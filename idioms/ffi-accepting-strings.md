# Accepting Strings via FFI

## Description

When accepting strings via FFI through pointers, there are two principles that should be followed:
1. Keep foreign strings "borrowed", rather than copying them directly.
2. Minimize `unsafe` code during the conversion.

## Motivation

Rust has built-in support for C-style strings with its `CString` and `CStr` types. However, there are different approaches one can take with strings that are being accepted from a foreign caller of a Rust function.

The best practice is simple: use `CStr` in such a way as to minimize unsafe code, and created a borrowed slice. If an owned String is needed, call `to_string()` on the string slice.

## Code Example

```rust
#[no_mangle]
pub extern "C" fn mylib_log(msg: *const libc::c_char, level: libc::c_int) {
    let level: crate::LogLevel = match level { /* ... */ };
    let msg_str: &str = unsafe {
        // SAFETY: accessing raw pointers expected to live for the call, and creating a shared
        // reference that does not outlive the current stack frame.
        match std::ffi::CStr::from_ptr(path).to_str() {
            Ok(s) => s,
            Err(e) => {
                crate::log_error("FFI string conversion failed");
                return;
            }
        }
    };
    crate::log(msg_str, level);
}
```

## Advantages

The example is is written to ensure that:
1. The `unsafe` block is as small as possible.
1. The pointer with an "untracked" lifetime becomes a "tracked" shared reference

Consider an alternative, where the string is actually copied:

```rust
pub extern "C" fn mylib_log(msg: *const libc::c_char, level: libc::c_int) {
    /* DO NOT USE THIS CODE. IT IS UGLY, VERBOSE, AND CONTAINS A SUBTLE BUG. */

    let level: crate::LogLevel = match level { /* ... */ };

    let msg_len = unsafe { /* SAFETY: strlen is what it is, I guess? */ libc::strlen(msg) + 1 };
    let mut msg_data = Vec::with_capacity(msg_len);
    let msg_cstr: CString = unsafe {
        // SAFETY: copying from a foreign pointer expected to live for the entire stack frame into 
        // owned memory
        std::ptr::copy_nonoverlapping(msg, msg_data.as_mut(), msg_len + 1);
        // SAFETY: setting vector length to copied size, so always ininitialized
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
```

This code in inferior to the original in two respects:

1. There is much more `unsafe` code, and more importantly, more invariants it must uphold.
2. Due to the extensive arithmetic required, there is a bug in this version that cases Rust UB.

## Disadvantages

None?
