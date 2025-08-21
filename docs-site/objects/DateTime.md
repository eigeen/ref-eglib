---
outline: 'deep'
---

# DateTime

> Version: >= 0.4.0

A DateTime object representing a specific point in time.

## Methods

### `obj:timestamp() -> i64`

Returns the Unix timestamp in seconds.

### `obj:timestamp_millis() -> i64`

Returns the Unix timestamp in milliseconds.

### `obj:to_rfc3339() -> string`

Returns the time as a string in RFC 3339 format.

### `obj:format(fmt: string) -> string`

Formats the time according to the given format string.

For the `fmt` string syntax, see: [struct.DateTime.html#method.format](https://docs.rs/chrono/latest/chrono/struct.DateTime.html#method.format)
