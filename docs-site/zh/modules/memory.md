---
outline: 'deep'
---

# eglib.memory

路径: `eglib.memory`

直接内存访问方法。

## 方法

### `memory:new_ptr(address: number | string | LuaPtr) -> LuaPtr` {#memory-new-ptr}

*返回:* [LuaPtr](/zh/objects/luaptr)

创建一个新的[LuaPtr](/zh/objects/luaptr)包装给定的地址值。

可接受的地址值:
- 表示内存地址的数字。例如 `0x12345678`(十六进制) `12345678`(十进制)
- 表示内存地址的字符串。例如 `"0x12345678"`(十六进制必须以`"0x"`开头) `"12345678"`(十进制)
- 另一个[LuaPtr](/zh/objects/luaptr)对象。

### `memory:patch(ptr: AsLuaPtr, bytes: List<u8>)`

用给定的字节修补指定地址的内存。

支持修补只读区域，如指令、数据和代码。

::: tip
此指令会存储原始字节，可以通过[memory:restore_patch](#memory-restore-patch)手动恢复。

当REFramework脚本重新加载时，所有修补将被**自动恢复**。
:::

::: warning
多次修补同一内存区域会引发错误。
:::

### `memory:patch_nop(ptr: AsLuaPtr, size: usize)`

用NOP指令(Windows AMD64中为`0x90`)修补指定地址的内存。

### `memory:restore_patch(ptr: AsLuaPtr) -> bool` {#memory-restore-patch}

*返回:* 如果修补成功恢复则返回`true`。

在修补后恢复指定地址的原始字节。

如果只是想重新加载脚本时恢复修补，则不需要调用此方法。修补会自动恢复。

::: warning
此方法目前不是线程安全的。建议在hook或其他线程安全函数中运行。
:::
