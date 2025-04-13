---
outline: 'deep'
---

# LuaPtr

原生指针包装对象。

可通过[memory:new_ptr](/zh/modules/memory#memory-new-ptr)构造。

## 方法

### `obj:to_integer() -> integer`

获取指针的Lua整数值。

如果整数值不在[0, i64::MAX]范围内会报错。

### `obj:read_bytes(size: integer) -> List<u8>`

从内存中读取`size`字节。

### 读取数值方法

`obj:read_[type]() -> number`

| 类型名称 | 字节长度 |
| -------- | -------- |
| `u8`     | 1        |
| `i8`     | 1        |
| `u16`    | 2        |
| `i16`    | 2        |
| `u32`    | 4        |
| `i32`    | 4        |
| `u64`    | 8        |
| `i64`    | 8        |
| `f32`    | 4        |
| `f64`    | 8        |

示例:

```lua
-- 内存地址0x20000处: 0x12 0x34 0x56 0x78
local ptr = eglib.memory:new_ptr(0x20000)
ptr:read_u8()  -- 返回0x12
ptr:read_u16() -- 返回0x3412
ptr:read_u32() -- 返回0x78563412
```

### `obj:write_bytes(bytes: List<u8>)`

将`bytes`写入内存。

### 写入数值方法

`obj:write_[type](value: number)`

类型名称与[读取数值方法](#read-number-methods)相同。

示例:

```lua
local ptr = eglib.memory:new_ptr(0x20000)
ptr:write_u8(127)
ptr:write_u16(30000)
ptr:write_f32(3.14159)
```

### `obj:read_ptr() -> LuaPtr`

*返回:* 一个新的`LuaPtr`对象。

读取一个与指针长度相等的值并返回为新的`LuaPtr`对象。

```lua
local new_ptr = ptr:read_ptr()
-- 等同于:
local new_ptr = eglib.memory:new_ptr(ptr:read_u64())
```

### 字符串方法

暂未实现!

### `obj:offset(...offsets) -> LuaPtr` {#offset}

*参数:* 可以是偏移量列表或可变偏移量参数。

*返回:* 一个新的`LuaPtr`对象。

对指针进行多次偏移。

```lua
local new_ptr = ptr:offset(0x1000, 0x2000) -- 或 ptr:offset({0x1000, 0x2000})
-- 等同于:
local ptr = ptr:offset(0x1000):read_ptr():offset(0x2000)
```

### `obj:offset_ce(...offsets) -> LuaPtr`

*参数:* 可以是偏移量列表或可变偏移量参数。

*返回:* 一个新的`LuaPtr`对象。

像Cheat Engine一样对指针进行多次偏移。

与[obj:offset](#offset)相比，此方法使用与Cheat Engine相同的偏移计算方式。

等同于`obj:read_ptr():offset(...)`

## 元方法

### __tostring

```lua
local ptr = eglib.memory:new_ptr(0x20000)
tostring(ptr) -- 返回"0x20000"
```

### __eq

### __add

指针偏移给定数值，返回新对象。

```lua
local ptr = eglib.memory:new_ptr(0x20000)
local new_ptr = ptr + 0x1000 -- new_ptr指向0x21000
```

### __sub

指针反向偏移给定数值，返回新对象。

```lua
local ptr = eglib.memory:new_ptr(0x20000)
local new_ptr = ptr - 0x1000 -- new_ptr指向0x19000