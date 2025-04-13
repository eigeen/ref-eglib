---
outline: 'deep'
---

# Duration

## Methods

### `obj:as_secs_f64() -> f64`

Returns the duration in seconds as a number value.

### `obj:as_secs() -> integer`

Returns the duration in seconds as an integer value.

### `obj:as_millis() -> integer`

Returns the duration in milliseconds as an integer value.

Errors if duration > i64::MAX.

### `obj:as_micros() -> integer`

Returns the duration in microseconds as an integer value.

Errors if duration > i64::MAX.

### `obj:as_nanos() -> integer`

Returns the duration in nanoseconds as an integer value.

Errors if duration > i64::MAX.