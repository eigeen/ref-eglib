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
    }
}

impl MemoryModule {
    fn new() -> Self {
        Self {
            patches: HashMap::new(),
        }
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
