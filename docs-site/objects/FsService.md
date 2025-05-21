---
outline: 'deep'
---

# FsService

> Version: >= 0.2.0

File system service object for managing file system related operations.

Can be constructed via [fs:new](/modules/fs#fs-new).

## Methods

### `obj:request_access(options: RequestAccessOptions) -> List<string>`

Parameters: [RequestAccessOptions](#requestaccessoptions)

Returns: List of paths.

Requests file access permissions. This will open a system file selection dialog, allowing the user to select files to access, and returns the selected file paths list.

The file selection process serves as the authorization process - only authorized files or directories can be accessed through subsequent operations.

Authorization lifecycle: Takes effect from authorization until script reload ends.

### `obj:read_text_file(path: string) -> string`

Requires `r` permission.

Reads text file content.

### `obj:write_text_file(path: string, data: string)`

Requires `w` permission.

Writes string content to file. If the path doesn't exist, it will create folders and files. If the file exists, it will overwrite the original file.

# RequestAccessOptions
  
| Field Name     | Type                                 | Default | Description                                                                                                                                     |
| -------------- | ------------------------------------ | ------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| **permission** | string                               |         | **Required**, permission type, options are `"r"`, `"w"`, `"rw"`                                                                                 |
| directory      | string                               | `"."`   | Default selected directory path                                                                                                                 |
| file_name      | string                               | `""`    | Default selected filename                                                                                                                       |
| filters        | List\<[DialogFilter](#dialogfilter)> | `None`  | File extension filters                                                                                                                          |
| title          | string                               | `""`    | Dialog title                                                                                                                                    |
| folder         | bool                                 | `false` | Folder selection mode                                                                                                                           |
| multiple       | bool                                 | `false` | Whether multiple files can be selected                                                                                                          |
| recursive      | bool                                 | `false` | Whether to recursively grant directory permissions (only when `folder` is true): if true, grants access to the directory and its subdirectories |
| auto_grant     | bool                                 | `false` | Whether to auto-grant permissions (only when `multiple` is false): if true, no dialog will pop up if the directory already has permissions      |

# DialogFilter

| Field Name     | Type          | Description              |
| -------------- | ------------- | ------------------------ |
| **name**       | string        | **Required**, name       |
| **extensions** | List\<string> | **Required**, extensions |
