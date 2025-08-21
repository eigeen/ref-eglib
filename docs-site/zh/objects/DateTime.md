---
outline: 'deep'
---

# DateTime

> 版本：>= 0.4.0

日期时间对象，表示一个具体的时间点。

## 方法

### `obj:timestamp() -> i64`

返回秒级时间戳。

### `obj:timestamp_millis() -> i64`

返回毫秒级时间戳。

### `obj:to_rfc3339() -> string`

返回 RFC 3339 格式的时间字符串。

### `obj:format(fmt: string) -> string`

使用指定的格式化字符串格式化时间。

fmt 字符串格式可参考：[struct.DateTime.html#method.format](https://docs.rs/chrono/latest/chrono/struct.DateTime.html#method.format)
