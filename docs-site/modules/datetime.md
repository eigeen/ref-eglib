---
outline: 'deep'
---

# eglib.time.datetime

> Version: >= 0.4.0

Path: `eglib.time.datetime`

## Methods

### `datetime:now() -> DateTime`

Get the current time, based on the system's current UTC time.

*Returns:*  A [Datetime](/objects/DateTime) object.

### `datetime:from_timestamp(secs: i64, nsecs?: u32) -> DateTime`

Create a `DateTime` object from a Unix timestamp.

*Returns:*  A [Datetime](/objects/DateTime) object.

### `datetime:parse_from_rfc3339(s: string) -> DateTime`

Parse a time from an RFC 3339 formatted string.

*Returns:*  A [Datetime](/objects/DateTime) object.

### `datetime:parse_from_str(s: string, fmt: string) -> DateTime`

Parse a time from a string with a specified format.

For the `fmt` string syntax, see [struct.DateTime.html#method.parse_from_str](https://docs.rs/chrono/latest/chrono/struct.DateTime.html#method.parse_from_str)

*Returns:*  A [Datetime](/objects/DateTime) object.
