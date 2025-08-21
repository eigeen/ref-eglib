use std::pin::Pin;

use mlua::prelude::*;

use crate::{module::EgLib, util};

use super::LuaModule;

const PROMISE_SCRIPT: &str = include_str!("promise.lua");

pub type Promise = LuaTable;

pub struct PromiseModule;

impl LuaModule for PromiseModule {
    fn register_library(lua: &Lua, registry: &LuaTable) -> LuaResult<()> {
        let promise: LuaValue = lua.load(PROMISE_SCRIPT).eval()?;
        registry.set("Promise", promise)?;

        Ok(())
    }
}

impl PromiseModule {
    /// 从运行环境获取 Promise 模块
    pub fn get_promise_module(lua: &Lua) -> LuaResult<Promise> {
        EgLib::get_module(lua)?.get::<LuaTable>("Promise")
    }

    /// 创建异步任务，带Lua操作回调
    ///
    /// lua: Lua上下文\
    /// task_future: 异步耗时任务，不阻塞Lua运行时，无锁，需内部实现加锁\
    /// result_handler: 异步任务结束后执行的回调函数，运行时获取Lua上下文。有锁
    pub fn new_async_task<T, F, C>(
        lua: &Lua,
        task_future: F,
        result_handler: C,
    ) -> crate::error::Result<LuaTaskHandle>
    where
        T: Send,
        F: Future<Output = T> + Send + 'static,
        C: FnOnce(&Lua, T) + Send + 'static,
    {
        let lua_weak = lua.weak();

        // 创建请求任务
        let lua_task_handle = LuaTaskHandle::new();
        let id = lua_task_handle.id;

        let runtime = &crate::TOKIO_RUNTIME;

        // !!!多线程操作，注意操作Lua对象加锁!!!
        let handle = runtime.spawn(async move {
            log::debug!("Promise: task {} started", id);
            // 耗时异步任务（无锁，需内部实现加锁）
            let result = task_future.await;

            if let Some(lua) = lua_weak.try_upgrade() {
                EgLib::run_with_global_lock(&lua, |lua| result_handler(lua, result));
                // 移除句柄
                EgLib::instance().remove_task_handle(id);
            } else {
                log::error!("Promise: Lua state not found via weak reference");
            }
            log::debug!("Promise: task {} finished", id);
        });

        // 保存句柄
        EgLib::instance().add_task_handle(id, handle);

        Ok(lua_task_handle)
    }

    /// 创建一个新的 promise 对象，包含一个异步执行器。
    ///
    /// 该执行器执行时，在外部不会对Lua线程上锁，防止阻塞Lua线程。
    ///
    /// 内部调用Lua函数时需手动加锁，防止竞争。
    ///
    /// executor: 接收 resolve, reject 回调函数，返回一个 Future 对象。
    ///
    /// # Example
    ///
    /// ```rs
    /// module.new_promise_async(lua, |resolve, reject| async move {
    ///     // 执行异步任务
    ///     let result = do_some_async_task().await;
    ///     // 在此处加锁（模拟）
    ///     let _lock = lua.lock();
    ///     match result {
    ///         Ok(value) => resolve(value),
    ///         Err(error) => reject(error),
    ///     };
    /// });
    /// ```
    pub fn new_promise_async<F>(lua: &Lua, executor: F) -> LuaResult<Promise>
    where
        F: Fn(LuaFunction, LuaFunction) -> Pin<Box<dyn Future<Output = ()> + Send>>
            + Send
            + 'static,
    {
        let promise_module = Self::get_promise_module(lua)?;

        let executor_lua =
            lua.create_function(move |lua, (resolve, reject): (LuaFunction, LuaFunction)| {
                let future = executor(resolve, reject);
                Self::new_async_task(lua, future, |_, _| {}).map_err(LuaError::runtime)?;

                Ok(())
            });

        let promise_new_fn = promise_module.get::<LuaFunction>("new")?;
        let promise = promise_new_fn.call::<LuaTable>(executor_lua)?;

        Ok(promise)
    }
}

/// Lua可操作的任务句柄
pub struct LuaTaskHandle {
    id: u64,
}

impl LuaUserData for LuaTaskHandle {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("abort", |_, this, ()| {
            this.abort();
            Ok(())
        });
        // methods.add_method("join", |_, this, ()| {
        //     this.join();
        //     Ok(())
        // });
    }
}

// 未实现Drop，允许丢弃时不释放资源

impl LuaTaskHandle {
    fn new() -> Self {
        Self {
            id: util::new_random_id(),
        }
    }

    /// 试图终止任务，不会报错
    fn abort(&self) {
        EgLib::instance().remove_task_handle(self.id);
    }

    // /// 试图阻塞等待任务完成，不会报错
    // fn join(&self) {
    //     if let Some(luavm) = self.belongs_to.upgrade() {
    //         if let Some(handle) = luavm.take_task_handle(self.id) {
    //             if handle.is_finished() {
    //                 return;
    //             }
    //             crate::TOKIO_RUNTIME.block_on(async move {
    //                 if let Err(e) = handle.await {
    //                     error!("Task handle join error: id={} error={:?}", self.id, e);
    //                 };
    //             });
    //         };
    //     }
    // }
}
