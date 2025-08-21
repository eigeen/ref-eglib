---
outline: 'deep'
---

# eglib.time.datetime

> 版本：>= 0.4.0

路径: `eglib.time.datetime`

## 方法

### `datetime:now() -> DateTime`

获取当前时间。基于当前系统UTC时间。

*返回:*  [Datetime](/zh/objects/DateTime) 对象。

### `datetime:from_timestamp(secs: i64, nsecs?: u32) -> DateTime`

从时间戳创建一个 `DateTime` 对象。

*返回:*  [Datetime](/zh/objects/DateTime) 对象。

### `datetime:parse_from_rfc3339(s: string) -> DateTime`

从RFC 3339格式的字符串解析时间。

*返回:*  [Datetime](/zh/objects/DateTime) 对象。

### `datetime:parse_from_str(s: string, fmt: string) -> DateTime`

从指定格式的字符串解析时间。

fmt 字符串格式可参考：[struct.DateTime.html#method.parse_from_str](https://docs.rs/chrono/latest/chrono/struct.DateTime.html#method.parse_from_str)

*返回:*  [Datetime](/zh/objects/DateTime) 对象。
