use mlua::prelude::*;

use super::LuaModule;

pub struct TimeModule;

impl LuaModule for TimeModule {
    fn register_library(_lua: &mlua::Lua, registry: &mlua::Table) -> mlua::Result<()> {
        registry.set("time", TimeModule)?;
        Ok(())
    }
}

impl LuaUserData for TimeModule {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        // Get current time. Simple and high precision.
        methods.add_function("instant", |_, _this: LuaValue| {
            Ok(LuaInstant(std::time::Instant::now()))
        });
    }
}

pub struct LuaInstant(pub std::time::Instant);

impl LuaUserData for LuaInstant {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        // Get elapsed time since the instant was created.
        methods.add_method("elapsed", |_, this, ()| Ok(LuaDuration(this.0.elapsed())));
    }
}

pub struct LuaDuration(pub std::time::Duration);

impl LuaUserData for LuaDuration {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("as_secs_f64", |_, this, ()| Ok(this.0.as_secs_f64()));
        methods.add_method("as_secs", |_, this, ()| Ok(this.0.as_secs()));
        methods.add_method("as_millis", |_, this, ()| {
            // u128 to i64 cast
            let num = this.0.as_millis();
            if num > i64::MAX as u128 {
                return Err(LuaError::external(
                    "Duration too large to fit in i64".to_string(),
                ));
            }
            Ok(num)
        });
        methods.add_method("as_micros", |_, this, ()| {
            let num = this.0.as_micros();
            if num > i64::MAX as u128 {
                return Err(LuaError::external(
                    "Duration too large to fit in i64".to_string(),
                ));
            }
            Ok(num)
        });
        methods.add_method("as_nanos", |_, this, ()| {
            let num = this.0.as_nanos();
            if num > i64::MAX as u128 {
                return Err(LuaError::external(
                    "Duration too large to fit in i64".to_string(),
                ));
            }
            Ok(num)
        });
    }
}
