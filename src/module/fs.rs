use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
    sync::LazyLock,
};

use bitflags::bitflags;
use mlua::prelude::*;
use parking_lot::Mutex;
use serde::Deserialize;

use crate::utils;

use super::LuaModule;

static FS_MODULE: LazyLock<Mutex<FsModule>> = LazyLock::new(|| Mutex::new(FsModule::new()));

pub struct FsModule {
    /// User granted accesses of services.
    granted: HashMap<String, GrantState>,
}

impl LuaModule for FsModule {
    fn register_library(lua: &Lua, registry: &LuaTable) -> LuaResult<()> {
        let fs = lua.create_table()?;
        fs.set(
            "new",
            lua.create_function(|_, (_this, name): (LuaValue, String)| Ok(FsService::new(name)))?,
        )?;
        fs.set(
            "get_granted_access",
            lua.create_function(|lua, _: LuaValue| {
                let module = FsModule::get_module().lock();
                let table = lua.create_table()?;
                for (name, state) in module.granted.iter() {
                    table.set(name.clone(), state.clone().into_lua(lua)?)?;
                }

                Ok(table)
            })?,
        )?;

        registry.set("fs", fs)?;
        Ok(())
    }
}

impl FsModule {
    fn new() -> Self {
        Self {
            granted: HashMap::new(),
        }
    }

    fn get_module() -> &'static Mutex<FsModule> {
        &FS_MODULE
    }

    fn accept_access(&mut self, service_name: &str, access: Access) {
        let acceptions = &mut self
            .granted
            .entry(service_name.to_string())
            .or_default()
            .acceptions;
        if !acceptions
            .iter()
            .any(|a| a.path == access.path && a.permissions == access.permissions)
        {
            acceptions.push(access);
        }
    }

    fn reject_access(&mut self, service_name: &str, access: Access) {
        self.granted
            .entry(service_name.to_string())
            .or_default()
            .rejections
            .push(access);
    }

    fn clear_access(&mut self, service_name: Option<&str>) {
        if let Some(srv_name) = service_name {
            self.granted.remove(srv_name);
        } else {
            self.granted.clear();
        }
    }

    fn is_access_allowed(
        &self,
        service_name: &str,
        path: impl AsRef<Path>,
        perm: Permissions,
    ) -> (bool, String) {
        let path = path.as_ref();

        let Some(state) = self.granted.get(service_name) else {
            log::warn!("Service not found: {}", service_name);
            return (false, String::new());
        };

        let abs_path = utils::normalize_path(path);
        let abs_path_str = abs_path.to_string_lossy().to_string();

        // Helper function to check path matches
        fn path_matches(pattern: &str, target: &str) -> bool {
            if pattern.ends_with("/**") {
                let base = pattern.trim_end_matches("**").trim_end_matches('/');
                target.starts_with(base)
                    && (target == base || target.starts_with(&format!("{}/", base)))
            } else if pattern.ends_with("/*") {
                let base = pattern.trim_end_matches('*').trim_end_matches('/');
                target.starts_with(base) && !target[base.len()..].contains('/')
            } else {
                pattern == target
            }
        }

        // Check rejections first
        for reject in &state.rejections {
            if reject.permissions.contains(perm) && path_matches(&reject.path, &abs_path_str) {
                return (false, abs_path_str);
            }
        }

        // Then check acceptions
        for accept in &state.acceptions {
            if accept.permissions.contains(perm) && path_matches(&accept.path, &abs_path_str) {
                return (true, abs_path_str);
            }
        }

        (false, abs_path_str)
    }
}

#[derive(Clone, Default)]
struct GrantState {
    acceptions: Vec<Access>,
    rejections: Vec<Access>,
}

impl IntoLua for GrantState {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let table = lua.create_table()?;
        table.set("acceptions", self.acceptions.into_lua(lua)?)?;
        table.set("rejections", self.rejections.into_lua(lua)?)?;
        Ok(LuaValue::Table(table))
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Permissions: u8 {
        const READ = 0b01;
        const WRITE = 0b10;
    }
}

impl IntoLua for Permissions {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let mut string = String::new();
        if self.contains(Permissions::READ) {
            string.push('r');
        }
        if self.contains(Permissions::WRITE) {
            string.push('w');
        }

        string.into_lua(lua)
    }
}

impl Permissions {
    fn from_char(c: char) -> Option<Permissions> {
        match c {
            'r' => Some(Permissions::READ),
            'w' => Some(Permissions::WRITE),
            _ => None,
        }
    }

    fn from_str(s: &str) -> Option<Permissions> {
        let mut perm = Permissions::empty();
        for c in s.chars() {
            if let Some(p) = Permissions::from_char(c) {
                perm |= p;
            } else {
                return None;
            }
        }
        Some(perm)
    }
}

#[derive(Debug, Clone)]
struct Access {
    path: String,
    permissions: Permissions,
}

impl IntoLua for Access {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let table = lua.create_table()?;
        table.set("path", self.path.into_lua(lua)?)?;
        table.set("permissions", self.permissions.into_lua(lua)?)?;
        Ok(LuaValue::Table(table))
    }
}

#[derive(Deserialize)]
struct RequestAccessOptions {
    permission: String,
    directory: Option<String>,
    file_name: Option<String>,
    filters: Option<Vec<DialogFilter>>,
    title: Option<String>,
    #[serde(default)]
    folder: bool,
    #[serde(default)]
    multiple: bool,
    #[serde(default)]
    recursive: bool,
    /// If user granted before, grant automatically without dialog.
    #[serde(default)]
    auto_grant: bool,
    /// Save file mode, allows to select an unexisting file.
    #[serde(default)]
    save_file: bool,
}

#[derive(Deserialize)]
struct DialogFilter {
    name: String,
    extensions: Vec<String>,
}

impl FromLua for RequestAccessOptions {
    fn from_lua(value: LuaValue, lua: &Lua) -> LuaResult<Self> {
        lua.from_value(value)
    }
}

struct FsService {
    name: String,
}

impl LuaUserData for FsService {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method(
            "request_access",
            |_, this, mut options: RequestAccessOptions| {
                // validate permission string
                let Some(perm) = Permissions::from_str(&options.permission) else {
                    return Err(LuaError::external(format!(
                        "Invalid permission string: {}. Use r/w/rw.",
                        options.permission
                    )));
                };

                if let Some(ref dir) = options.directory {
                    let abs_dir = utils::normalize_path(dir);
                    options
                        .directory
                        .replace(abs_dir.to_string_lossy().to_string());
                }

                let mut module = FsModule::get_module().lock();

                // check if access already granted
                if options.auto_grant && !options.multiple {
                    if options.folder && options.directory.is_some() {
                        let (ok, path) = module.is_access_allowed(
                            &this.name,
                            options.directory.as_ref().unwrap(),
                            perm,
                        );
                        if ok {
                            return Ok(vec![path]);
                        }
                    } else if !options.folder
                        && options.directory.is_some()
                        && options.file_name.is_some()
                    {
                        let dir_path = options.directory.as_ref().unwrap();
                        let file_name = options.file_name.as_ref().unwrap();
                        let (ok, path) = module.is_access_allowed(
                            &this.name,
                            Path::new(dir_path).join(file_name).to_str().unwrap(),
                            perm,
                        );
                        if ok {
                            return Ok(vec![path]);
                        }
                    }
                }

                let mut dialog = rfd::FileDialog::new();
                if let Some(filters) = options.filters {
                    for filter in filters {
                        dialog = dialog.add_filter(filter.name, &filter.extensions);
                    }
                }
                if let Some(dir) = options.directory {
                    dialog = dialog.set_directory(dir);
                }
                if let Some(file_name) = options.file_name {
                    dialog = dialog.set_file_name(file_name);
                }
                if let Some(title) = options.title {
                    let title = if options.recursive {
                        format!("[Recursive] {}", title)
                    } else {
                        title
                    };
                    dialog = dialog.set_title(title);
                }

                let result = if options.save_file {
                    dialog.save_file().map(|p| vec![p])
                } else {
                    match (options.folder, options.multiple) {
                        (true, true) => dialog.pick_folders(),
                        (true, false) => dialog.pick_folder().map(|p| vec![p]),
                        (false, true) => dialog.pick_files(),
                        (false, false) => dialog.pick_file().map(|p| vec![p]),
                    }
                };

                match result {
                    Some(paths) => {
                        for path in &paths {
                            let abs_path = utils::normalize_path(path);
                            let path_str = if options.folder {
                                let mut path = abs_path.to_string_lossy().to_string();
                                path.push_str(if options.recursive { "/**" } else { "/*" });
                                path
                            } else {
                                abs_path.to_string_lossy().to_string()
                            };

                            let access = Access {
                                path: path_str,
                                permissions: perm,
                            };
                            module.accept_access(&this.name, access);
                        }
                        Ok(paths
                            .iter()
                            .map(|p| p.to_string_lossy().to_string())
                            .collect())
                    }
                    None => Ok(vec![]),
                }
            },
        );
        methods.add_method("read_text_file", |_, this, path_str: String| {
            let mut file = OpenFileOptions::new(&path_str)
                .with_service(&this.name)
                .read()
                .map_err(|e| e.into_lua_err())?;

            let mut content = String::new();
            file.read_to_string(&mut content).map_err(|e| {
                LuaError::external(format!("Failed to read file {}: {}", path_str, e))
            })?;

            Ok(content)
        });
        methods.add_method(
            "write_text_file",
            |_, this, (path_str, content): (String, String)| {
                let mut file = OpenFileOptions::new(&path_str)
                    .with_service(&this.name)
                    .create()
                    .map_err(|e| e.into_lua_err())?;

                file.write_all(content.as_bytes()).map_err(|e| {
                    LuaError::external(format!("Failed to write file {}: {}", path_str, e))
                })?;

                Ok(())
            },
        );
        methods.add_method("mkdir", |_, this, (path_str, recursive): (String, bool)| {
            let module = FsModule::get_module().lock();
            let (ok, abs_path) =
                module.is_access_allowed(&this.name, &path_str, Permissions::WRITE);
            if !ok {
                return Err(LuaError::external(format!(
                    "Access denied to path {}.",
                    abs_path
                )));
            };

            if recursive {
                std::fs::create_dir_all(&abs_path)
            } else {
                std::fs::create_dir(&abs_path)
            }
            .map_err(|e| e.into_lua_err())
        });
        methods.add_method("remove", |_, this, path_str: String| {
            let module = FsModule::get_module().lock();
            let (ok, abs_path) =
                module.is_access_allowed(&this.name, &path_str, Permissions::WRITE);
            if !ok {
                return Err(LuaError::external(format!(
                    "Access denied to path {}.",
                    abs_path
                )));
            };

            let path = Path::new(&abs_path);
            if path.is_dir() {
                std::fs::remove_dir_all(&abs_path)
            } else {
                std::fs::remove_file(&abs_path)
            }
            .map_err(|e| e.into_lua_err())
        });
        methods.add_method("read_dir", |_, this, path_str: String| {
            let module = FsModule::get_module().lock();
            let (ok, abs_path) = module.is_access_allowed(&this.name, &path_str, Permissions::READ);
            if !ok {
                return Err(LuaError::external(format!(
                    "Access denied to path {}.",
                    abs_path
                )));
            };

            let mut files = vec![];
            let mut dirs = vec![];
            for entry in std::fs::read_dir(&abs_path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    files.push(path.to_string_lossy().to_string());
                } else {
                    dirs.push(path.to_string_lossy().to_string());
                }
            }

            Ok((dirs, files))
        });
    }
}

impl FsService {
    fn new(name: String) -> Self {
        Self { name }
    }
}

struct OpenFileOptions {
    path: PathBuf,
    service_name: String,
}

impl OpenFileOptions {
    fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            service_name: String::new(),
        }
    }

    fn with_service(mut self, service_name: &str) -> Self {
        self.service_name = service_name.to_string();
        self
    }

    fn read(self) -> anyhow::Result<File> {
        let module = FsModule::get_module().lock();
        let (ok, abs_path) =
            module.is_access_allowed(&self.service_name, &self.path, Permissions::READ);
        if !ok {
            anyhow::bail!("Access denied to file {}.", abs_path);
        };

        if !self.path.is_file() {
            anyhow::bail!("File {} not found.", self.path.display());
        }

        Ok(File::open(&self.path)?)
    }

    fn create(self) -> anyhow::Result<File> {
        let module = FsModule::get_module().lock();
        let (ok, abs_path) =
            module.is_access_allowed(&self.service_name, &self.path, Permissions::WRITE);
        if !ok {
            anyhow::bail!("Access denied to file {}.", abs_path);
        };

        let parent = Path::new(&abs_path).parent().unwrap();
        if !parent.exists() {
            std::fs::create_dir_all(parent).map_err(|e| {
                LuaError::external(format!(
                    "Failed to create directory {}: {}",
                    parent.display(),
                    e
                ))
            })?;
        }

        Ok(File::create(&self.path)?)
    }
}
