mod fs;
mod luaptr;
mod memory;
mod promise;
mod time;

use std::sync::{Arc, LazyLock};

use mlua::prelude::*;
use parking_lot::Mutex;
use rand::Rng;
use reframework_api_rs::RefAPI;

const LIB_MODULE_NAME: &str = "eglib";
const LUA_SCRIPT: &str = include_str!("script.lua");
const KEY_PRIVILEGED: &str = "__privileged";

static PRIVILEGED: LazyLock<Mutex<Privileged>> = LazyLock::new(|| Mutex::new(Privileged::new()));

pub trait LuaModule {
    fn register_library(lua: &mlua::Lua, registry: &mlua::Table) -> mlua::Result<()>;
}

pub type SharedLua = Arc<Lua>;

#[derive(Default)]
pub struct EgLib {
    lua_state: Mutex<Option<SharedLua>>,
}

impl LuaModule for EgLib {
    fn register_library(lua: &Lua, registry: &LuaTable) -> LuaResult<()> {
        // eglib
        let core_table = lua.create_table()?;
        // sub modules
        promise::PromiseModule::register_library(lua, &core_table)?;
        time::TimeModule::register_library(lua, &core_table)?;
        luaptr::LuaPtr::register_library(lua, &core_table)?;
        memory::MemoryModule::register_library(lua, &core_table)?;
        fs::FsModule::register_library(lua, &core_table)?;
        // privileged instructions
        core_table.set(KEY_PRIVILEGED, lua.create_userdata(Privileged::new())?)?;
        core_table.set(
            "__get_privileged_key",
            lua.create_function(|_, _: ()| Ok(PRIVILEGED.lock().get_key()))?,
        )?;
        core_table.set(
            "__with_privileged",
            lua.create_function(|lua, (key, cb): (String, LuaFunction)| {
                let module = EgLib::get_module(lua)?;
                let mut privileged = module.get::<LuaUserDataRefMut<Privileged>>(KEY_PRIVILEGED)?;
                if !privileged.validate_key(&key) {
                    return Err(LuaError::external("Invalid key"));
                }

                privileged.privileged = true;
                let result = cb.call::<()>(());
                privileged.privileged = false;

                result
            })?,
        )?;

        registry.set(LIB_MODULE_NAME, core_table)?;
        // // _G
        // let globals = lua.globals();

        Ok(())
    }
}

impl EgLib {
    pub fn instance() -> &'static Self {
        static EGLIB: LazyLock<EgLib> = LazyLock::new(EgLib::default);
        &EGLIB
    }

    pub fn mount(&self, lua_state: *mut mlua::ffi::lua_State) -> LuaResult<()> {
        let lua = unsafe { Lua::init_from_ptr(lua_state) };
        let globals = lua.globals();
        // reset privileged key
        PRIVILEGED.lock().reset();
        // register eglib module
        EgLib::register_library(&lua, &globals)?;
        // run lua scripts
        if let Err(e) = lua.load(LUA_SCRIPT).exec() {
            log::error!("Failed to run eglib lua script: {}", e);
        }

        let mut states = self.lua_state.lock();
        states.replace(Arc::new(lua));

        Ok(())
    }

    pub fn unmount(&self, _lua_state: *mut mlua::ffi::lua_State) {
        let mut state = self.lua_state.lock();
        state.take();
    }

    /// Runs a closure with ReFramework global Lua lock.
    pub fn run_with_global_lock<F>(lua: &Lua, f: F) -> LuaResult<()>
    where
        F: FnOnce(&Lua) -> LuaResult<()>,
    {
        let refapi = RefAPI::instance().unwrap();
        // 临时处理，有开销，后续需要修改
        let lua_mtx = refapi.new_lua_mutex(());
        let _l = lua_mtx.lock();
        f(lua)
    }

    fn get_module(lua: &Lua) -> LuaResult<LuaTable> {
        let globals = lua.globals();
        globals.get(LIB_MODULE_NAME)
    }

    fn is_privileged(lua: &Lua) -> LuaResult<bool> {
        let module = Self::get_module(lua)?;
        let privileged = module.get::<LuaUserDataRef<Privileged>>(KEY_PRIVILEGED)?;
        Ok(privileged.privileged())
    }
}

struct Privileged {
    key: String,
    key_once: bool,
    privileged: bool,
}

impl LuaUserData for Privileged {}

impl Privileged {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789!@#$%^&*-_+=";
    const KEY_LENGTH: usize = 16;

    fn new() -> Self {
        Self {
            key: String::new(),
            key_once: false,
            privileged: false,
        }
    }

    fn get_key(&mut self) -> String {
        // can only get key once
        if self.key_once {
            return String::new();
        }

        self.key = (0..Self::KEY_LENGTH)
            .map(|_| {
                let idx = rand::rng().random_range(0..Self::CHARSET.len());
                Self::CHARSET[idx] as char
            })
            .collect();
        self.key_once = true;

        self.key.clone()
    }

    fn reset(&mut self) {
        self.key_once = false;
        self.key.clear();
        self.privileged = false;
    }

    fn privileged(&self) -> bool {
        self.privileged
    }

    fn validate_key(&self, key: &str) -> bool {
        self.key == key
    }
}
