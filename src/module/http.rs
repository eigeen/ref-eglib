//! Http请求模块

use std::{collections::HashMap, time::Duration};

use mlua::prelude::*;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::module::LuaModule;
use crate::module::{
    EgLib,
    promise::{Promise, PromiseModule},
};

pub struct HttpModule;

impl LuaModule for HttpModule {
    fn register_library(_lua: &Lua, registry: &LuaTable) -> LuaResult<()> {
        registry.set("http", HttpModule)?;
        Ok(())
    }
}

impl LuaUserData for HttpModule {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        // 创建请求客户端
        methods.add_method("client", |_, _, ()| Ok(RequestClient::default()));
        // 快速get请求。返回 Promise<ResponseData>
        methods.add_method("get", |lua, _, config: LuaTable| {
            let promise = Self::quick_request(lua, reqwest::Method::GET, config)
                .map_err(LuaError::runtime)?;
            Ok(promise)
        });
        // 快速post请求。返回 Promise<ResponseData>
        methods.add_method("post", |lua, _, config: LuaTable| {
            let promise = Self::quick_request(lua, reqwest::Method::POST, config)
                .map_err(LuaError::runtime)?;
            Ok(promise)
        });
    }
}

impl HttpModule {
    /// 创建请求任务，返回 Promise<ResponseData>
    fn quick_request(lua: &Lua, method: reqwest::Method, lua_config: LuaTable) -> Result<Promise> {
        // 解析config
        let config: RequestConfig = lua.from_value(LuaValue::Table(lua_config))?;
        log::debug!("request config: {:?}", config);

        // let Ok(luavm) = CoreModule::get_current_luavm_helper(lua) else {
        //     return Err(LuaVMError::VMNotFound);
        // };
        // let luavm_weak = Arc::downgrade(&luavm.0);
        let lua_weak = lua.weak();

        let promise = PromiseModule::new_promise_async(lua, move |resolve, reject| {
            let method = method.clone();
            let config = config.clone();
            let lua_weak = lua_weak.clone();

            Box::pin(async move {
                let result = request_task(method, &config).await;

                if let Some(lua) = lua_weak.try_upgrade() {
                    EgLib::run_with_global_lock(&lua, |_lua| {
                        let call_result = match result {
                            Ok(resp) => {
                                if let Err(e) = resolve.call::<()>(resp) {
                                    // 尝试调用reject
                                    reject.call::<()>(e.to_string())
                                } else {
                                    Ok(())
                                }
                            }
                            Err(e) => reject.call::<()>(e.to_string()),
                        };
                        if let Err(e) = call_result {
                            log::error!("Http request: calling Promise callbacks error: {}", e);
                        }
                    });
                };
            })
        })?;

        Ok(promise)
    }
}

/// 请求配置结构
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
struct RequestConfig {
    url: String,
    #[serde(default)]
    headers: HashMap<String, String>,
    /// 请求超时时间，单位：秒
    timeout: Option<u64>,
    data: Option<RequestData>,
}

/// 请求负载结构
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", content = "payload")]
#[serde(rename_all = "snake_case")]
enum RequestData {
    Text(String),
    Json(serde_json::Value),
}

#[derive(Default)]
struct RequestClient {
    client: reqwest::Client,
}

impl LuaUserData for RequestClient {}

impl RequestClient {
    fn request(
        &self,
        method: reqwest::Method,
        url: &str,
        data: Option<&RequestData>,
    ) -> Result<()> {
        todo!()
    }
}

/// 响应数据结构
#[derive(Debug, Clone, Serialize)]
struct ResponseData {
    status_code: u16,
    headers: HashMap<String, String>,
    data: Option<Vec<u8>>,
}

impl LuaUserData for ResponseData {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        // 返回状态码，Integer
        methods.add_method("status_code", |_, this, ()| Ok(this.status_code));
        // 返回headers，Table
        methods.add_method("headers", |_, this, ()| Ok(this.headers.clone()));
        // 返回字节数组
        methods.add_method("data", |_, this, ()| Ok(this.data.clone()));
        // 作为json解析，返回Table
        methods.add_method("json", |lua, this, ()| {
            if let Some(data) = &this.data {
                let json_value: serde_json::Value =
                    serde_json::from_slice(data).map_err(LuaError::runtime)?;
                return lua.to_value(&json_value);
            }

            Ok(LuaNil)
        });
        // 作为字符串解析，返回String
        methods.add_method("text", |lua, this, ()| {
            if let Some(data) = &this.data {
                let text = String::from_utf8_lossy(data).to_string();
                return lua.to_value(&text);
            }

            Ok(LuaNil)
        });
    }
}

impl ResponseData {
    fn new(code: u16, headers: HashMap<String, String>) -> Self {
        Self {
            status_code: code,
            headers,
            data: None,
        }
    }
}

async fn request_task(
    method: reqwest::Method,
    config: &RequestConfig,
) -> std::result::Result<ResponseData, reqwest::Error> {
    let client = reqwest::Client::default();

    let mut req = client.request(method, &config.url);

    // 处理headers
    for (k, v) in &config.headers {
        req = req.header(k, v);
    }
    // 处理data
    if let Some(data) = &config.data {
        match data {
            RequestData::Text(value) => req = req.body(value.to_string()),
            RequestData::Json(value) => req = req.json(value),
        }
    }
    if let Some(timeout) = config.timeout {
        req = req.timeout(Duration::from_secs(timeout));
    }

    let resp = req.send().await?;

    // 包装响应
    let headers = resp
        .headers()
        .iter()
        .fold(HashMap::new(), |mut acc, (k, v)| {
            acc.insert(k.to_string(), v.to_str().unwrap_or_default().to_string());
            acc
        });
    let mut resp_data = ResponseData::new(resp.status().as_u16(), headers);
    // resp_data.data = resp.bytes().await.ok().map(|b| b.to_vec());
    match resp.bytes().await {
        Ok(b) => {
            resp_data.data = Some(b.to_vec());
            log::debug!("Http request: response: {:?}", b);
        }
        Err(e) => {
            log::error!("Http request: reading response bytes error: {}", e);
        }
    }

    Ok(resp_data)
}
