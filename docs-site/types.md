---
outline: 'deep'
---

# Types

For ease of understanding, we will use some type aliases in subsequent documents. These types are essentially existing Lua types. In Lua, for example, the `Table` type can also exist as a list, map, or even class, so we need some precise definitions.

## integer

Real type: `number`

Must be an integer or can be lossless converted to an integer, otherwise may cause precision loss or errors.

## SeqTable\<T> | List\<T>

Real type: `Table<number, T>`

A sequence table is a table that can be indexed by a number, starting from `1`. Can iterate using `ipairs`. 

## Map\<K, V>

Real type: `Table<K, V>`

A map structure.

## AsLuaPtr

Real type: `number | string | LuaPtr`

It's not a actual type in Lua, precisely, `AsLuaPtr` is a constraint. It can only appears as a parameter of functions.

Any type can be converted to a [LuaPtr](/objects/luaptr) object can be passed to this type.

## u8 | i8 | u16 | i16 | u32 | i32 | u64 | i64 | f32 | f64 {#number-types}

Real type: `number`

> u32: unsigned integer of 32 bits, unsigned int or uint32_t in CPP.
> 
> f32: float in C.
> 
> f64: double in C.

The type name comes from Rust.

Although Lua's number type is not subdivided, precise handling of type length is required during memory operations. Please pay attention to the size of numbers to avoid overflow, which may cause implicit conversions or errors.
