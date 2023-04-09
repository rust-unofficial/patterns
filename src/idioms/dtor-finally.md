# Finalisation in destructors

## Description

Rust does not provide the equivalent to `finally` blocks - code that will be
executed no matter how a function is exited. Instead, an object's destructor can
be used to run code that must be run before exit.

## Example

```rust,ignore
fn bar() -> Result<(), ()> {
    // These don't need to be defined inside the function.
    struct Foo;

    // Implement a destructor for Foo.
    impl Drop for Foo {
        fn drop(&mut self) {
            println!("exit");
        }
    }

    // The dtor of _exit will run however the function `bar` is exited.
    let _exit = Foo;
    // Implicit return with `?` operator.
    baz()?;
    // Normal return.
    Ok(())
}
```

## Motivation

If a function has multiple return points, then executing code on exit becomes
difficult and repetitive (and thus bug-prone). This is especially the case where
return is implicit due to a macro. A common case is the `?` operator which
returns if the result is an `Err`, but continues if it is `Ok`. `?` is used as
an exception handling mechanism, but unlike Java (which has `finally`), there is
no way to schedule code to run in both the normal and exceptional cases.
Panicking will also exit a function early.

## Advantages

Code in destructors will (nearly) always be run - copes with panics, early
returns, etc.

## Disadvantages

It is not guaranteed that destructors will run. For example, if there is an
infinite loop in a function or if running a function crashes before exit.
Destructors are also not run in the case of a panic in an already panicking
thread. Therefore, destructors cannot be relied on as finalizers where it is
absolutely essential that finalisation happens.

This pattern introduces some hard to notice, implicit code. Reading a function
gives no clear indication of destructors to be run on exit. This can make
debugging tricky.

Requiring an object and `Drop` impl just for finalisation is heavy on boilerplate.

## Discussion

There is some subtlety about how exactly to store the object used as a
finalizer. It must be kept alive until the end of the function and must then be
destroyed. The object must always be a value or uniquely owned pointer (e.g.,
`Box<Foo>`). If a shared pointer (such as `Rc`) is used, then the finalizer can
be kept alive beyond the lifetime of the function. For similar reasons, the
finalizer should not be moved or returned.

The finalizer must be assigned into a variable, otherwise it will be destroyed
immediately, rather than when it goes out of scope. The variable name must start
with `_` if the variable is only used as a finalizer, otherwise the compiler
will warn that the finalizer is never used. However, do not call the variable
`_` with no suffix - in that case it will be destroyed immediately.

In Rust, destructors are run when an object goes out of scope. This happens
whether we reach the end of block, there is an early return, or the program
panics. When panicking, Rust unwinds the stack running destructors for each
object in each stack frame. So, destructors get called even if the panic happens
in a function being called.

If a destructor panics while unwinding, there is no good action to take, so Rust
aborts the thread immediately, without running further destructors. This means
that destructors are not absolutely guaranteed to run. It also means that you
must take extra care in your destructors not to panic, since it could leave
resources in an unexpected state.

## See also

[RAII guards](../patterns/behavioural/RAII.md).
