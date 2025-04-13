mod luaptr;
mod memory;
mod time;

use std::sync::{Arc, LazyLock};

use mlua::prelude::*;
use parking_lot::Mutex;
use reframework_api_rs::RefAPI;

const LIB_MODULE_NAME: &str = "eglib";

pub trait LuaModule {
    fn register_library(lua: &mlua::Lua, registry: &mlua::Table) -> mlua::Result<()>;
}

pub type SharedLua = Arc<Lua>;

#[derive(Default)]
pub struct EgLib {
    lua_state: Mutex<Option<SharedLua>>,
}

impl LuaModule for EgLib {
    fn register_library(lua: &mlua::Lua, registry: &mlua::Table) -> mlua::Result<()> {
        // eglib
        let core_table = lua.create_table()?;
        // sub modules
        time::TimeModule::register_library(lua, &core_table)?;
        luaptr::LuaPtr::register_library(lua, &core_table)?;
        memory::MemoryModule::register_library(lua, &core_table)?;

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
        EgLib::register_library(&lua, &globals)?;

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
}
