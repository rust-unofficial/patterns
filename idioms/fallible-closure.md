# Fallible closure inside infallible function

## Description

Build a fallible closure inside of an apparently infallible function.

## Example

```rust
use std::path::Path;

// this function hides its failure states
fn file_stem(path: &Path) -> &str {
    // this closure is fallible, so it can make use of `?`
    // fn() -> Option<&str>
    let inner = || Some(path.file_stem()?.to_str()?);
    inner().unwrap_or("untitled")
}
```

## Motivation

You may sometimes want to use the [`?`] operator inside an infallible function,
in those cases a closure inside your function could be the simple solution.

## Advantages

This allows using the terser syntax permitted by the [`?`] operator inside an
infallible context.

## Disadvantages

This is hiding failure states from the consumer of the API.

[`?`]: https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-question-mark-operator
