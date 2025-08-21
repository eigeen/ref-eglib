---
outline: 'deep'
---

# FsService

> 版本：>= 0.2.0

文件系统服务对象，用于管理文件系统相关的操作。

可通过 [fs:new](/zh/modules/fs#fs-new) 构造。

## 方法

### `obj:request_access(options: RequestAccessOptions) -> List<string>`

参数：[RequestAccessOptions](#requestaccessoptions)

返回：路径列表。

请求访问文件权限，将会打开一个系统文件选择Dialog，用户选择需要访问的文件，并返回选择的文件的路径列表。

用户选择文件的过程即授权的过程，仅有授权的文件或目录才可通过后续操作访问。

授权生命周期：从授权开始生效，直到重载脚本结束。

### `obj:read_text_file(path: string) -> string`

需要 `r` 权限。

读取文本文件内容。

### `obj:write_text_file(path: string, data: string)`

需要 `w` 权限。

写入字符串内容到文件。如果路径不存在，则会创建文件夹和文件。如果文件存在，则覆盖原有文件。

### `obj:mkdir(path: string, recursive: bool)`

> 版本：0.3.0

需要 `w` 权限。

创建目录。如果 `recursive` 为 true，则会创建目录及其子目录。

### `obj:remove(path: string)`

> 版本：0.3.0

需要 `w` 权限。

删除文件或目录。

### `obj:read_dir(path: string) -> (List<string>, List<string>)`

> 版本：0.3.0

返回：(目录列表, 文件列表)

需要 `r` 权限。

读取目录内容。返回目录和文件的元组。

# RequestAccessOptions
 
| 字段名         | 类型                                 | 默认值  | 说明                                                                                          |
| -------------- | ------------------------------------ | ------- | --------------------------------------------------------------------------------------------- |
| **permission** | string                               |         | **必填**，权限类型，可选值为 `"r"`, `"w"`, `"rw"`                                             |
| directory      | string                               | `"."`   | 默认选择的文件夹路径                                                                          |
| file_name      | string                               | `""`    | 默认选择的文件名                                                                              |
| filters        | List\<[DialogFilter](#dialogfilter)> | `None`  | 选择文件后缀名的过滤器                                                                        |
| title          | string                               | `""`    | Dialog标题                                                                                    |
| folder         | bool                                 | `false` | 选择文件夹模式                                                                                |
| multiple       | bool                                 | `false` | 是否可以选择多个文件                                                                          |
| recursive      | bool                                 | `false` | 是否递归授权目录权限，仅在`folder`为true时有效：如果为true，则该目录及其子目录均授权访问      |
| auto_grant     | bool                                 | `false` | 是否自动授权，仅`multiple`为false时有效：如果为true，则如果该目录具有权限，无需弹出Dialog询问 |


# DialogFilter

| 字段名         | 类型          | 说明             |
| -------------- | ------------- | ---------------- |
| **name**       | string        | **必填**，名称   |
| **extensions** | List\<string> | **必填**，后缀名 |
