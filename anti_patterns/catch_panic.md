# panic::catch_unwind for exceptions

## Description

This antipattern arises when the method for catching (`panic::catch_unwind`) an
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

There are scenarios where using panic::catch_unwind is the correct choice, eg, a
web server implementation wants to save an unwinding thread in order to send a
valid response if the route for that request (as in: logic outside of the web server
implementor's control) is producing a panic.

Expected errors should not result in stack unwinding. Instead, expected errors
should be handled through the Result and Option types. [The Rust Book's chapter
on Error Handling](https://doc.rust-lang.org/book/error-handling.html) elaborates further on this.

## See also

[The Rust Book: Error Handling](https://doc.rust-lang.org/book/error-handling.html)
[Rust 1.9 announcement, which contains a description of this antipattern](http://blog.rust-lang.org/2016/05/26/Rust-1.9.html)
[Result documentation](http://doc.rust-lang.org/std/result/enum.Result.html)
[panic::catch_unwind documentation](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html)
