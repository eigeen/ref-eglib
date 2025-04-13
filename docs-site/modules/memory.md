---
outline: 'deep'
---

# eglib.memory

Path: `eglib.memory`

Direct memory access methods.

## Methods

### `memory:new_ptr(address: number | string | LuaPtr) -> LuaPtr` {#memory-new-ptr}

*Returns:* [LuaPtr](/objects/luaptr)

Create a new [LuaPtr](/objects/luaptr) wrapping the given address value.

Acceptable address values:

- A number representing the memory address. e.g. `0x12345678`(hex) `12345678`(decimal)
- A string representing the memory address. e.g. `"0x12345678"`(hex must start with `"0x"`) `"12345678"`(decimal)
- Another [LuaPtr](/objects/luaptr) object.

### `memory:patch(ptr: AsLuaPtr, bytes: List<u8>)`

Patch the memory at the given address with the given bytes.

Supports patching read-only regions, such as instructions, data, and code.

::: tip
This instruction will store the original bytes, and can be restored manually by [memory:restore_patch](#memory-restore-patch).

When REFramework scripts reloads, all patches will be **automatically restored**.
:::

::: warning
Patch the same memory region multiple times will raise an error.
:::

### `memory:patch_nop(ptr: AsLuaPtr, size: usize)`

Patch the memory at the given address with NOP instructions (`0x90` in Windows AMD64).

### `memory:restore_patch(ptr: AsLuaPtr) -> bool` {#memory-restore-patch}

*Returns:* `true` if the patch was successfully restored.

Restore the original bytes at the given address after a patch.

You don't need to call this method if you just want to restore the patch when the script reloads. The patches will be restored automatically.

::: warning
This method is not thread-safe currently. Run it in hooks or other thread-safe functions is recommended.
:::
