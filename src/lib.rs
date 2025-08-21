#![allow(clippy::missing_safety_doc)]

use std::{ffi::c_void, sync::LazyLock};

use reframework_api_rs::prelude::*;

use log::{error, info};

mod error;
mod memory;
mod module;
mod util;

pub static TOKIO_RUNTIME: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1) // 只使用一个工作线程，模拟单线程行为
        .enable_all()
        .build()
        .unwrap()
});

fn main_entry() -> anyhow::Result<()> {
    let refapi = RefAPI::instance().unwrap();

    // 初始化Tokio运行时
    let _ = *TOKIO_RUNTIME;

    // 初始化lua回调
    let ok = refapi.param().on_lua_state_created(on_lua_state_created);
    if !ok {
        return Err(anyhow::anyhow!(
            "Failed to register on_lua_state_created hook"
        ));
    }

    let ok = refapi
        .param()
        .on_lua_state_destroyed(on_lua_state_destroyed);
    if !ok {
        return Err(anyhow::anyhow!(
            "Failed to register on_lua_state_destroyed hook"
        ));
    }

    Ok(())
}

unsafe extern "C" fn on_lua_state_created(lua_state_ptr: *mut c_void) {
    if let Err(e) = module::EgLib::instance().mount(lua_state_ptr as *mut mlua::ffi::lua_State) {
        error!("Failed to initialize lua module: {}", e);
    };
}

unsafe extern "C" fn on_lua_state_destroyed(lua_state_ptr: *mut c_void) {
    module::EgLib::instance().unmount(lua_state_ptr as *mut mlua::ffi::lua_State);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reframework_plugin_required_version(
    version: &mut REFrameworkPluginVersion,
) {
    version.major = REFRAMEWORK_PLUGIN_VERSION_MAJOR;
    version.minor = REFRAMEWORK_PLUGIN_VERSION_MINOR;
    version.patch = REFRAMEWORK_PLUGIN_VERSION_PATCH;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reframework_plugin_initialize(
    param: *const REFrameworkPluginInitializeParam,
) -> bool {
    unsafe {
        if RefAPI::initialize(param).is_none() {
            return false;
        };

        RefAPI::init_log(env!("CARGO_PKG_NAME"), log::LevelFilter::Debug);

        info!(
            "{} v{} initializing...",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        );

        if let Err(e) = main_entry() {
            error!("runtime error: {}", e);

            // remove hooks if exists

            return false;
        }

        true
    }
}
