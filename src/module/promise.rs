use mlua::prelude::*;

use super::LuaModule;

const PROMISE_SCRIPT: &str = include_str!("promise.lua");

pub struct PromiseModule;

impl LuaModule for PromiseModule {
    fn register_library(lua: &Lua, registry: &LuaTable) -> LuaResult<()> {
        let promise: LuaValue = lua.load(PROMISE_SCRIPT).eval()?;
        registry.set("Promise", promise)?;

        Ok(())
    }
}
