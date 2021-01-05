# FFI Idioms

Writing FFI code is an entire course in itself.
However, there are several idioms here that can act as pointers, and avoid traps for inexperienced users of `unsafe` Rust.

This section contains idioms that may be useful when doing FFI.

1. [Idiomatic Errors](./ffi-errors.md) - Error handling with integer codes and sentinel return values (such as NULL pointers)

2. [Accepting Strings](./ffi-accepting-strings.md) with minimal unsafe code

3. [Passing Strings](./ffi-passing-strings.md) to FFI functions
