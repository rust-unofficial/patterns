# catch_unwind for exceptions

## Description

This anti-pattern arises when the method for catching ([panic::catch_unwind](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html)) an
unexpected problem (implementation bug) is used to handle an expected problem,
such as a missing file.

## Example

```rust
use std::io::prelude::*;
use std::fs::File;
use std::panic;

fn main() {
    // panic::catch_unwind catches any panic that occurs within the
    // function passed to it
    let result = panic::catch_unwind(|| {
        let mut file_result = File::open("foo.txt");
        file_result.unwrap(); // causes a panic if the file is not found
    });

    // the program continues running despite the panic
    println!("potential error: {:?}", result);
    assert!(result.is_err());
}
```


## Motivation

In rust, there are two ways an operation can fail: An expected problem, like a
file not being found, or a HTTP request timing out. Or, an unexpected problem,
such as an index being out of bounds, or an assert!() failing. These are
unexpected because they are bugs that should not happen. It would not make sense
to handle an error for QuickSort returning an incorrectly unsorted array, as
this should not happen.

## Advantages

There are scenarios where using `panic::catch_unwind` is the correct choice, e.g. a
web server implementation wants to save an unwinding thread in order to send a
valid response if the route for that request (as in: logic outside of the web server
implementor's control) is producing a panic.

## Disadvantages
​
`panic::catch_unwind` may not catch all panics in Rust. A panic in Rust is not always
implemented via unwinding, but can be implemented by aborting the process as well.
`panic::catch_unwind` only catches unwinding panics, not those that abort the process.

Also note that unwinding into Rust code with a foreign exception
(e.g. a an exception thrown from C++ code) is undefined behavior.

TODO: since Result::unwrap() converts the error to a string, it's harder to distinguish
between different kinds of errors than if we had matched the result directly.

## Discussion

TODO:
?-operator to propagate errors
explain why unwinding is bad
other disadvantages of panic::catch_unwind
+ "The example could be improved by adding a function and which panics and catching the panic
in the caller, then matching the Result. Describing the example you could show how by returning
a Result, the Result-ness of the function is described in the signature."

Expected errors should not result in stack unwinding. Instead, expected errors
should be handled through the Result and Option types. [The Rust Book's chapter
on Error Handling](https://doc.rust-lang.org/book/error-handling.html) elaborates further on this.

## See also

- [The Rust Book: Error Handling](https://doc.rust-lang.org/book/error-handling.html)
- [Rust 1.9 announcement, which contains a description of this antipattern](http://blog.rust-lang.org/2016/05/26/Rust-1.9.html)
- [Result documentation](http://doc.rust-lang.org/std/result/enum.Result.html)
- [panic::catch_unwind documentation](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html)
