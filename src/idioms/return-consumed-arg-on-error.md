# Return consumed argument on error

## Description

If a fallible function consumes (moves) an argument, return that argument back
inside an error.

## Example

```rust
pub fn send(value: String) -> Result<(), SendError> {
    println!("using {value} in a meaningful way");
    // Simulate non-deterministic fallible action.
    use std::time::SystemTime;
    let period = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    if period.subsec_nanos() % 2 == 1 {
        Ok(())
    } else {
        Err(SendError(value))
    }
}

pub struct SendError(String);

fn main() {
    let mut value = "imagine this is very long string".to_string();

    let success = 's: {
        // Try to send value two times.
        for _ in 0..2 {
            value = match send(value) {
                Ok(()) => break 's true,
                Err(SendError(value)) => value,
            }
        }
        false
    };

    println!("success: {success}");
}
```

## Motivation

In case of error you may want to try some alternative way or to retry action in
case of non-deterministic function. But if the argument is always consumed, you
are forced to clone it on every call, which is not very efficient.

The standard library uses this approach in e.g. `String::from_utf8` method. When
given a vector that doesn't contain valid UTF-8, a `FromUtf8Error` is returned.
You can get original vector back using `FromUtf8Error::into_bytes` method.

## Advantages

Better performance because of moving arguments whenever possible.

## Disadvantages

Slightly more complex error types.
