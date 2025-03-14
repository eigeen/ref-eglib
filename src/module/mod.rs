mod luaptr;
mod time;

use std::{
    collections::HashMap,
    sync::{Arc, LazyLock},
};

use log::error;
use mlua::prelude::*;
use parking_lot::Mutex;
use reframework_api_rs::RefAPI;

const ON_DESTROY_FUNC: &str = "__on_destroy";
const GET_STATE_PTR_FUNC: &str = "__get_state_ptr";
const REGISTER_PTR_KEY: &str = "__register_ptr";

pub trait LuaModule {
    fn register_library(lua: &mlua::Lua, registry: &mlua::Table) -> mlua::Result<()>;
}

pub type SharedLua = Arc<Lua>;

#[derive(Default)]
pub struct EgLib {
    lua_states: Mutex<HashMap<u64, SharedLua>>,
}

impl LuaModule for EgLib {
    fn register_library(lua: &mlua::Lua, registry: &mlua::Table) -> mlua::Result<()> {
        // eglib
        let core_table = lua.create_table()?;
        // sub modules
        time::TimeModule::register_library(lua, &core_table)?;
        luaptr::LuaPtr::register_library(lua, &core_table)?;

        registry.set("eglib", core_table)?;

        // _G
        let globals = lua.globals();
        unsafe {
            globals.set(
                GET_STATE_PTR_FUNC,
                lua.create_c_function(lua_get_state_ptr)?,
            )?;
        }
        globals.set(REGISTER_PTR_KEY, EgLib::get_state_ptr(lua)?)?;

        Ok(())
    }
}

impl EgLib {
    pub fn instance() -> &'static Self {
        static EGLIB: LazyLock<EgLib> = LazyLock::new(EgLib::default);
        &EGLIB
    }

    pub fn register_lua(&self, lua_state: *mut mlua::ffi::lua_State) -> LuaResult<()> {
        let lua = unsafe { Lua::init_from_ptr(lua_state) };
        let globals = lua.globals();
        EgLib::register_library(&lua, &globals)?;

        let mut states = self.lua_states.lock();
        states.insert(lua_state as u64, Arc::new(lua));

        Ok(())
    }

    pub fn destroy_lua(&self, lua_state: *mut mlua::ffi::lua_State) {
        let mut states = self.lua_states.lock();
        let state_addr = if states.contains_key(&(lua_state as u64)) {
            lua_state as u64
        } else {
            // try to use register ptr
            let lua = unsafe { Lua::init_from_ptr(lua_state) };
            EgLib::get_state_register_ptr(&lua).unwrap_or(0)
        };

        let removed = states.remove(&state_addr);
        if let Some(shared_lua) = removed {
            let result = EgLib::run_with_global_lock(&shared_lua, EgLib::invoke_on_destroy);
            if let Err(e) = result {
                error!("Failed to invoke on_destroy callback: {}", e);
            }
        }
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

    /// 获取 lua_State 指针
    pub fn get_state_ptr(lua: &Lua) -> LuaResult<u64> {
        let get_state_ptr = lua.globals().get::<LuaFunction>(GET_STATE_PTR_FUNC)?;
        let result: u64 = get_state_ptr.call(())?;

        Ok(result)
    }

    /// Get custom id of lua state.
    ///
    /// The id is actually the pointer to when lua_state is registered.
    /// The address obtained by [EgLib::get_state_ptr] may change due to coroutine.
    /// So it's recommended to use [EgLib::get_state_register_ptr] instead.
    pub fn get_state_register_ptr(lua: &Lua) -> LuaResult<u64> {
        lua.globals().get::<u64>(REGISTER_PTR_KEY)
    }

    /// Invokes on_destroy callback.
    /// This functions is **no lock**, should lock it outside.
    pub fn invoke_on_destroy(lua: &Lua) -> LuaResult<()> {
        if let Ok(on_destroy) = lua.globals().get::<LuaFunction>(ON_DESTROY_FUNC) {
            on_destroy.call::<()>(())?;
        }
        Ok(())
    }
}

#[allow(non_snake_case)]
unsafe extern "C-unwind" fn lua_get_state_ptr(L: *mut mlua::ffi::lua_State) -> std::ffi::c_int {
    // lua_State 指针作为返回值 u64 类型
    let lua_state_ptr: i64 = L as i64;
    unsafe {
        mlua::ffi::lua_pushinteger(L, lua_state_ptr);
    }

    1
}
