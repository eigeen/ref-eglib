---
outline: 'deep'
---

# Duration

## 方法

### `obj:as_secs_f64() -> f64`

以浮点数形式返回持续时间(秒)。

### `obj:as_secs() -> integer`

以整数形式返回持续时间(秒)。

### `obj:as_millis() -> integer`

以整数形式返回持续时间(毫秒)。

如果持续时间 > i64::MAX 会报错。

### `obj:as_micros() -> integer`

以整数形式返回持续时间(微秒)。

如果持续时间 > i64::MAX 会报错。

### `obj:as_nanos() -> integer`

以整数形式返回持续时间(纳秒)。

如果持续时间 > i64::MAX 会报错。