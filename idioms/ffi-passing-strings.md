# Passing Strings

## Description

When passing strings to FFI functions, there are four principles that should be
followed:

1. Make the lifetime of owned strings as long as possible.
2. Minimize `unsafe` code during the conversion.
3. If the C code can modify the string data, use `Vec` instead of `CString`.
4. Unless the Foreign Function API requires it, the ownership of the string
  should not transfer to the callee.

## Motivation

Rust has built-in support for C-style strings with its `CString` and `CStr`
types. However, there are different approaches one can take with strings that
are being sent to a foreign function call from a Rust function.

The best practice is simple: use `CString` in such a way as to minimize
`unsafe` code. However, a secondary caveat is that
*the object must live long enough*, meaning the lifetime should be maximized.
In addition, the documentation explains that "round-tripping" a `CString` after
modification is UB, so additional work is necessary in that case.

## Code Example

```rust,ignore
pub mod unsafe_module {

    // other module content

    extern "C" {
        fn seterr(message: *const libc::c_char);
        fn geterr(buffer: *mut libc::c_char, size: libc::c_int) -> libc::c_int;
    }

    fn report_error_to_ffi<S: Into<String>>(
        err: S
    ) -> Result<(), std::ffi::NulError>{
        let c_err = std::ffi::CString::new(err.into())?;

        unsafe {
            // SAFETY: calling an FFI whose documentation says the pointer is
            // const, so no modification should occur
            seterr(c_err.as_ptr());
        }

        Ok(())
        // The lifetime of c_err continues until here
    }

    fn get_error_from_ffi() -> Result<String, std::ffi::IntoStringError> {
        let mut buffer = vec![0u8; 1024];
        unsafe {
            // SAFETY: calling an FFI whose documentation implies
            // that the input need only live as long as the call
            let written: usize = geterr(buffer.as_mut_ptr(), 1023).into();

            buffer.truncate(written + 1);
        }

        std::ffi::CString::new(buffer).unwrap().into_string()
    }
}
```

## Advantages

The example is written in a way to ensure that:

1. The `unsafe` block is as small as possible.
2. The `CString` lives long enough.
3. Errors with typecasts are always propagated when possible.

A common mistake (so common it's in the documentation) is to not use the
variable in the first block:

```rust,ignore
pub mod unsafe_module {

    // other module content

    fn report_error<S: Into<String>>(err: S) -> Result<(), std::ffi::NulError> {
        unsafe {
            // SAFETY: whoops, this contains a dangling pointer!
            seterr(std::ffi::CString::new(err.into())?.as_ptr());
        }
        Ok(())
    }
}
```

This code will result in a dangling pointer, because the lifetime of the
`CString` is not extended by the pointer creation, unlike if a reference were
created.

Another issue frequently raised is that the initialization of a 1k vector of
zeroes is "slow". However, recent versions of Rust actually optimize that
particular macro to a call to `zmalloc`, meaning it is as fast as the operating
system's ability to return zeroed memory (which is quite fast).

## Disadvantages

None?
