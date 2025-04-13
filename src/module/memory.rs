use std::collections::HashMap;

use mlua::prelude::*;

use crate::error::{Error, Result};
use crate::memory::MemoryUtils;

use super::{LuaModule, luaptr::LuaPtr};

struct MemoryPatch {
    address: usize,
    size: usize,
    backup: Vec<u8>,
}

pub struct MemoryModule {
    patches: HashMap<usize, MemoryPatch>,
    scan_cache: HashMap<String, usize>,
    module_base: usize,
    module_size: usize,
}

impl LuaModule for MemoryModule {
    fn register_library(_lua: &mlua::Lua, registry: &mlua::Table) -> mlua::Result<()> {
        registry.set("memory", MemoryModule::new())?;
        Ok(())
    }
}

impl LuaUserData for MemoryModule {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("new_ptr", |_, _, ptr: LuaPtr| Ok(ptr));
        methods.add_method_mut("patch", |_, this, (ptr, bytes): (LuaPtr, Vec<u8>)| {
            this.new_patch(ptr.to_usize(), &bytes).into_lua_err()?;
            Ok(())
        });
        methods.add_method_mut("patch_nop", |_, this, (ptr, size): (LuaPtr, usize)| {
            this.new_patch_nop(ptr.to_usize(), size).into_lua_err()?;
            Ok(())
        });
        methods.add_method_mut("restore_patch", |_, this, ptr: LuaPtr| {
            let success = this.restore_patch(ptr.to_usize()).into_lua_err()?;
            Ok(success)
        });
        methods.add_method_mut(
            "scan",
            |_, this, (pattern, offset): (String, Option<isize>)| {
                let mut result = this.pattern_scan_first_cached(&pattern).into_lua_err()?;
                // apply offset
                if let Some(offset) = offset {
                    result = (result as isize + offset) as usize;
                }
                let result_ptr = LuaPtr::new(result as u64);
                Ok(result_ptr)
            },
        );
        methods.add_method_mut("scan_advanced", |_, this, options: PatternScanOptions| {
            let mut matches = this.pattern_scan_advanced(&options).into_lua_err()?;
            // apply offset
            if let Some(offset) = options.offset {
                matches.iter_mut().for_each(|ptr| {
                    *ptr = (*ptr as isize + offset) as usize;
                });
            }
            let matches_ptrs = matches
                .into_iter()
                .map(|ptr| LuaPtr::new(ptr as u64))
                .collect::<Vec<_>>();
            Ok(matches_ptrs)
        });
    }
}

impl MemoryModule {
    fn new() -> Self {
        Self {
            patches: HashMap::new(),
            scan_cache: HashMap::new(),
            module_base: 0,
            module_size: 0,
        }
    }

    fn pattern_scan_first_cached(&mut self, pattern: &str) -> Result<usize> {
        // use cache
        if let Some(address) = self.scan_cache.get(pattern).copied() {
            return Ok(address);
        }

        // scan and cache
        let result = MemoryUtils::scan_first(self.module_base, self.module_size, pattern)?;
        self.scan_cache.insert(pattern.to_string(), result);
        Ok(result)
    }

    fn pattern_scan_advanced(&mut self, options: &PatternScanOptions) -> Result<Vec<usize>> {
        if self.module_base == 0 || self.module_size == 0 {
            self.update_module_info()?;
        }
        let start_address = options
            .start
            .map(|ptr| ptr.to_usize())
            .unwrap_or(self.module_base);
        let length = options.length.unwrap_or(self.module_size);
        let matches = if options.all_matches {
            MemoryUtils::scan_all(start_address, length, &options.pattern)?
        } else {
            let match_address = MemoryUtils::scan_first(start_address, length, &options.pattern)?;
            vec![match_address]
        };
        Ok(matches)
    }

    fn update_module_info(&mut self) -> Result<()> {
        let (base, size) = unsafe { MemoryUtils::get_base_module_space()? };
        self.module_base = base;
        self.module_size = size;
        Ok(())
    }

    fn new_patch(&mut self, address: usize, data: &[u8]) -> Result<()> {
        if self.is_patch_exists(address, data.len()) {
            return Err(Error::PatchAlreadyExists(address));
        }

        let backup = MemoryUtils::patch(address, data)?;
        self.patches.insert(
            address,
            MemoryPatch {
                address,
                size: data.len(),
                backup,
            },
        );

        Ok(())
    }

    fn new_patch_nop(&mut self, address: usize, size: usize) -> Result<()> {
        if self.is_patch_exists(address, size) {
            return Err(Error::PatchAlreadyExists(address));
        }

        let backup = MemoryUtils::patch_repeat(address, 0x90, size)?;
        self.patches.insert(
            address,
            MemoryPatch {
                address,
                size,
                backup,
            },
        );

        Ok(())
    }

    fn restore_patch(&mut self, address: usize) -> Result<bool> {
        if let Some(patch) = self.patches.remove(&address) {
            MemoryUtils::patch(patch.address, &patch.backup)?;
            return Ok(true);
        }
        Ok(false)
    }

    fn is_patch_exists(&mut self, address: usize, size: usize) -> bool {
        for patch in self.patches.values() {
            let range1 = patch.address..(patch.address + patch.size);
            let range2 = address..(address + size);
            if self.range_overlaps(range1, range2) {
                return true;
            }
        }
        false
    }

    fn range_overlaps(
        &self,
        range1: std::ops::Range<usize>,
        range2: std::ops::Range<usize>,
    ) -> bool {
        range1.start < range2.end && range2.start < range1.end
    }
}

impl Drop for MemoryModule {
    fn drop(&mut self) {
        log::debug!("[DEBUG] MemoryModule dropped");
        let addresses = self.patches.keys().copied().collect::<Vec<_>>();
        for addr in addresses {
            if let Err(e) = self.restore_patch(addr) {
                log::error!("Failed to restore patch at 0x{:x}: {}", addr, e);
            };
        }
    }
}

struct PatternScanOptions {
    pattern: String,
    offset: Option<isize>,
    start: Option<LuaPtr>,
    length: Option<usize>,
    all_matches: bool,
}

impl FromLua for PatternScanOptions {
    fn from_lua(value: LuaValue, _lua: &Lua) -> LuaResult<Self> {
        let table = value.as_table().ok_or(LuaError::FromLuaConversionError {
            from: "table",
            to: "PatternScanOptions".to_string(),
            message: None,
        })?;
        let pattern: String = table
            .get("pattern")
            .map_err(|_| LuaError::external("PatternScanOptions missing string field 'pattern'"))?;
        let offset: Option<isize> = table.get::<Option<isize>>("offset")?;
        let start: Option<LuaPtr> = table.get::<Option<LuaPtr>>("start")?;
        let length: Option<usize> = table.get::<Option<usize>>("length")?;
        // all_matches defaults to false
        let all_matches: bool = table.get::<Option<bool>>("all_matches")?.unwrap_or(false);
        Ok(PatternScanOptions {
            pattern,
            offset,
            start,
            length,
            all_matches,
        })
    }
}
