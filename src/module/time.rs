use mlua::prelude::*;

use super::LuaModule;

pub struct TimeModule;

impl LuaModule for TimeModule {
    fn register_library(_lua: &Lua, registry: &LuaTable) -> LuaResult<()> {
        registry.set("time", TimeModule)?;
        Ok(())
    }
}

impl LuaUserData for TimeModule {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_function_get("datetime", |_, _| Ok(LuaDateTimeModule));
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        // Get current time. Simple and high precision.
        methods.add_method("instant", |_, _, ()| {
            Ok(LuaInstant(std::time::Instant::now()))
        });
    }
}

struct LuaInstant(pub std::time::Instant);

impl LuaUserData for LuaInstant {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        // Get elapsed time since the instant was created.
        methods.add_method("elapsed", |_, this, ()| Ok(LuaDuration(this.0.elapsed())));
    }
}

struct LuaDuration(pub std::time::Duration);

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

struct LuaDateTimeModule;

impl LuaUserData for LuaDateTimeModule {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("now", |_, _, ()| Ok(LuaDateTime(chrono::Utc::now())));
        methods.add_method(
            "from_timestamp",
            |_, _, (secs, nsecs): (i64, Option<u32>)| {
                let datetime = chrono::DateTime::from_timestamp(secs, nsecs.unwrap_or(0))
                    .ok_or(LuaError::external("Invalid timestamp"))?;
                Ok(LuaDateTime(datetime))
            },
        );
        methods.add_method("parse_from_rfc3339", |_, _, s: String| {
            let datetime = chrono::DateTime::parse_from_rfc3339(&s)
                .map_err(|e| e.into_lua_err())?
                .to_utc();
            Ok(LuaDateTime(datetime))
        });
        methods.add_method("parse_from_str", |_, _, (s, fmt): (String, String)| {
            let datetime = chrono::DateTime::parse_from_str(&s, &fmt)
                .map_err(|e| e.into_lua_err())?
                .to_utc();
            Ok(LuaDateTime(datetime))
        });
    }
}

struct LuaDateTime(chrono::DateTime<chrono::Utc>);

impl LuaUserData for LuaDateTime {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("timestamp", |_, this, ()| Ok(this.0.timestamp()));
        methods.add_method("timestamp_millis", |_, this, ()| {
            Ok(this.0.timestamp_millis())
        });
        methods.add_method("to_rfc3339", |_, this, ()| Ok(this.0.to_rfc3339()));
        methods.add_method("format", |_, this, fmt: String| {
            Ok(format!("{}", this.0.format(&fmt)))
        });
    }
}
