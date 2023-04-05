# FFI Patterns

Writing FFI code is an entire course in itself.
However, there are several idioms here that can act as pointers, and avoid traps
for inexperienced users of unsafe Rust.

This section contains design patterns that may be useful when doing FFI.

1. [Object-Based API](./export.md) design that has good memory safety characteristics,
  and a clean boundary of what is safe and what is unsafe

2. [Type Consolidation into Wrappers](./wrappers.md) - group multiple Rust types
  together into an opaque "object"
