---
outline: 'deep'
---

# eglib.fs

> Version: >= 0.2.0

Path: `eglib.memory`

File system operations module.

## Methods

### `fs:new(service_name: string) -> FsService` {#fs-new}

Create a file system service.

*Returns:* [FsService](/objects/FsService)

Creates a file system service for file system operations. All file system related operations are performed through the service instance.

The purpose of creating services instead of using `eglib.fs` directly is to allow each script to have independent instances and service names, facilitating permission management.