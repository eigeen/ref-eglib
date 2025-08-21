---
layout: doc
---

# EgLib API 参考文档

插件版本：0.4.0

## API 设计

由于Lua静态方法调用（使用`.`）和实例方法调用（使用`:`）可能造成混淆，因此设计尽可能让模块函数使用实例方法（即用`:`调用），即使该方法原本是静态的。

## 错误处理

REFramework确实会自动捕获脚本异常。但由于一些技术问题，虽然调用`EgLib`产生的错误会被捕获，但错误信息会丢失，这对调试非常不利。

建议使用`pcall`或其他方法包装可能出错的函数，然后打印出错误信息。

示例：

```lua{3}
local ok, result = pcall(function()
    -- 你的代码在这里
    eglib.memory.patch(0x0, {0x90, 0x90}) -- 这行会引发错误
end)
if not ok then
    -- 在这里处理错误，或者重新抛出
    error(tostring(result))
end
