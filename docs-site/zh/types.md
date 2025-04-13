---
outline: 'deep'
---

# 类型

为了便于理解，我们将在后续文档中使用一些类型别名。这些类型本质上都是Lua已有的类型。在Lua中，例如`Table`类型既可以作为列表、哈希表甚至 Class 存在，所以我们需要一些精确的定义。

## integer

实际类型: `number`

必须是整数或可以无损转换为整数，否则可能导致精度丢失或错误。

## SeqTable\<T> | List\<T>

实际类型: `Table<number, T>`

列表是一个可以通过数字索引的表，索引从`1`开始。可以通过 Lua 的`ipairs`函数遍历列表。

## Map\<K, V>

实际类型: `Table<K, V>`

哈希表结构。

## AsLuaPtr

实际类型: `number | string | LuaPtr`

这不是 Lua 中实际存在的类型，准确地说，`AsLuaPtr`是一个约束条件。它只能作为函数的参数类型出现。

任何可以转换为[LuaPtr](/zh/objects/luaptr)对象的类型都可以传递给此类型。

## u8 | i8 | u16 | i16 | u32 | i32 | u64 | i64 | f32 | f64 {#number-types}

实际类型: `number`

> u32: 32位无符号整数，CPP中的unsigned int或uint32_t。
> 
> f32: C语言中的float。
> 
> f64: C语言中的double。

这些类型名称来自Rust。

虽然 Lua 的 number 类型没有细分，但在内存操作时需要精确处理类型长度。请注意数字的大小以避免溢出，这可能导致隐式转换或错误。
