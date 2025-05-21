---
outline: 'deep'
---

# eglib.fs

> 版本：>= 0.2.0

路径: `eglib.memory`

文件系统操作模块。

## 方法

### `fs:new(service_name: string) -> FsService` {#fs-new}

创建文件系统服务。

*返回:* [FsService](/zh/objects/FsService)

创建一个文件系统服务，用于操作文件系统。文件系统相关操作均在服务实例中。

创建服务而不是直接使用 `eglib.fs` 操作的目的是为了使每个脚本拥有独立的实例和服务命名，便于进行权限管理。
