# Accepting Strings

## Description

When accepting strings via FFI through pointers, there are two principles that
should be followed:

1. Keep foreign strings "borrowed", rather than copying them directly.
2. Minimize the amount of complexity and `unsafe` code involved in converting
   from a C-style string to native Rust strings.

## Motivation

The strings used in C have different behaviours to those used in Rust, namely:

- C strings are null-terminated while Rust strings store their length
- C strings can contain any arbitrary non-zero byte while Rust strings must be
  UTF-8
- C strings are accessed and manipulated using `unsafe` pointer operations while
  interactions with Rust strings go through safe methods

The Rust standard library comes with C equivalents of Rust's `String` and `&str`
called `CString` and `&CStr`, that allow us to avoid a lot of the complexity and
`unsafe` code involved in converting between C strings and Rust strings.

The `&CStr` type also allows us to work with borrowed data, meaning passing
strings between Rust and C is a zero-cost operation.

## Code Example

```rust,ignore
pub mod unsafe_module {

    // other module content

    /// Log a message at the specified level.
    ///
    /// # Safety
    ///
    /// It is the caller's guarantee to ensure `msg`:
    ///
    /// - is not a null pointer
    /// - points to valid, initialized data
    /// - points to memory ending in a null byte
    /// - won't be mutated for the duration of this function call
    #[no_mangle]
    pub unsafe extern "C" fn mylib_log(msg: *const libc::c_char, level: libc::c_int) {
        let level: crate::LogLevel = match level { /* ... */ };

        // SAFETY: The caller has already guaranteed this is okay (see the
        // `# Safety` section of the doc-comment).
        let msg_str: &str = match std::ffi::CStr::from_ptr(msg).to_str() {
            Ok(s) => s,
            Err(e) => {
                crate::log_error("FFI string conversion failed");
                return;
            }
        };

        crate::log(msg_str, level);
    }
}
```

## Advantages

The example is written to ensure that:

1. The `unsafe` block is as small as possible.
2. The pointer with an "untracked" lifetime becomes a "tracked" shared reference

Consider an alternative, where the string is actually copied:

```rust,ignore
pub mod unsafe_module {

    // other module content

    pub extern "C" fn mylib_log(msg: *const libc::c_char, level: libc::c_int) {
        // DO NOT USE THIS CODE.
        // IT IS UGLY, VERBOSE, AND CONTAINS A SUBTLE BUG.

        let level: crate::LogLevel = match level { /* ... */ };

        let msg_len = unsafe { /* SAFETY: strlen is what it is, I guess? */
            libc::strlen(msg)
        };

        let mut msg_data = Vec::with_capacity(msg_len + 1);

        let msg_cstr: std::ffi::CString = unsafe {
            // SAFETY: copying from a foreign pointer expected to live
            // for the entire stack frame into owned memory
            std::ptr::copy_nonoverlapping(msg, msg_data.as_mut(), msg_len);

            msg_data.set_len(msg_len + 1);

            std::ffi::CString::from_vec_with_nul(msg_data).unwrap()
        }

        let msg_str: String = unsafe {
            match msg_cstr.into_string() {
                Ok(s) => s,
                Err(e) => {
                    crate::log_error("FFI string conversion failed");
                    return;
                }
            }
        };

        crate::log(&msg_str, level);
    }
}
```

This code in inferior to the original in two respects:

1. There is much more `unsafe` code, and more importantly, more invariants it
   must uphold.
2. Due to the extensive arithmetic required, there is a bug in this version that
   cases Rust `undefined behaviour`.

The bug here is a simple mistake in pointer arithmetic: the string was copied,
all `msg_len` bytes of it. However, the `NUL` terminator at the end was not.

The Vector then had its size *set* to the length of the *zero padded string* --
rather than *resized* to it, which could have added a zero at the end. As a
result, the last byte in the Vector is uninitialized memory. When the `CString`
is created at the bottom of the block, its read of the Vector will cause
`undefined behaviour`!

Like many such issues, this would be difficult issue to track down. Sometimes it
would panic because the string was not `UTF-8`, sometimes it would put a weird
character at the end of the string, sometimes it would just completely crash.

## Disadvantages

None?
