---
outline: 'deep'
---

# LuaPtr

A native pointer wrapper object.

Can be constructed by [memory:new_ptr](/modules/memory#memory-new-ptr)

## Methods

### `obj:to_integer() -> integer`

Get the Lua integer value of the pointer.

Errors if the integer value is not in [0, i64::MAX].

### `obj:read_bytes(size: integer) -> List<u8>`

Read `size` of bytes from the memory.

### Read Number Methods

`obj:read_[type]() -> number`

| Type Name | Byte Length |
| --------- | ----------- |
| `u8`      | 1           |
| `i8`      | 1           |
| `u16`     | 2           |
| `i16`     | 2           |
| `u32`     | 4           |
| `i32`     | 4           |
| `u64`     | 8           |
| `i64`     | 8           |
| `f32`     | 4           |
| `f64`     | 8           |

Example:

```lua
-- memory at 0x20000: 0x12 0x34 0x56 0x78
local ptr = eglib.memory:new_ptr(0x20000)
ptr:read_u8()  -- returns 0x12
ptr:read_u16() -- returns 0x3412
ptr:read_u32() -- returns 0x78563412
```

### `obj:write_bytes(bytes: List<u8>)`

Write `bytes` to the memory.

### Write Number Methods

`obj:write_[type](value: number)`

Type names same as in [Read Number Methods](#read-number-methods).

Example:

```lua
local ptr = eglib.memory:new_ptr(0x20000)
ptr:write_u8(127)
ptr:write_u16(30000)
ptr:write_f32(3.14159)
```

### `obj:read_ptr() -> LuaPtr`

*Returns:* A new `LuaPtr` object.

Reads a value equivalent to the pointer length and returns it as a new `LuaPtr` object.

```lua
local new_ptr = ptr:read_ptr()
-- equivalent to:
local new_ptr = eglib.memory:new_ptr(ptr:read_u64())
```

### String Methods

unimplemented for now!

### `obj:offset(...offsets) -> LuaPtr` {#offset}

*Parameters:* Can be a list of offsets or variadic offset arguments.

*Returns:* A new `LuaPtr` object.

Offsets the pointer multiple times.

```lua
local new_ptr = ptr:offset(0x1000, 0x2000) -- or ptr:offset({0x1000, 0x2000})
-- equivalent to:
local ptr = ptr:offset(0x1000):read_ptr():offset(0x2000)
```

### `obj:offset_ce(...offsets) -> LuaPtr`

*Parameters:* Can be a list of offsets or variadic offset arguments.

*Returns:* A new `LuaPtr` object.

Offsets the pointer multiple times, like Cheat Engine.

Compared to [obj:offset](#offset), this method uses the same offset calculation as Cheat Engine.

Equivalent to `obj:read_ptr():offset(...)`

## Meta Methods

### __tostring

```lua
local ptr = eglib.memory:new_ptr(0x20000)
tostring(ptr) -- returns "0x20000"
```

### __eq

### __add

Add a number to the object, returns a new object.

```lua
local ptr = eglib.memory:new_ptr(0x20000)
local new_ptr = ptr + 0x1000 -- new_ptr points to 0x21000
```

### __sub

Subtract a number to the object, returns a new object.

```lua
local ptr = eglib.memory:new_ptr(0x20000)
local new_ptr = ptr - 0x1000 -- new_ptr points to 0x19000
```