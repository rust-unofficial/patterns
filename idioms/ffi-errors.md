# Error Handling in FFI

## Description

In foreign languages like C, errors are represented by return codes. However, Rust's type system allows much more rich error information to be captured a propogated through a full type.

This best practice shows different kinds of error codes, and how to expose them in a usable way:
1. Flat Enums should be converted to integers and returned as codes.
1. Structured Enums should be converted to an integer code with a string error message for detail.
1. Custom Error Types should become "transparent", with a C representation.

## Code Example

### Flat Enums

```rust
enum DatabaseError {
	IsReadOnly = 1, // user attempted a write operation
	IOError = 2, // user should read the C errno() for what it was
	FileCorrupted = 3, // user should run a repair tool to recover it
}

impl From<DatabaseError> for libc::c_int {
	fn from(e: DatabaseError) -> libc::c_int {
		(e as u8).into()
	}
}
```

### Structured Enums

```rust
enum DatabaseError {
	IsReadOnly,
	IOError(std::io::Error),
	FileCorrupted(String), // message describing the issue
}

impl From<DatabaseError> for libc::c_int {
	fn from(e: DatabaseError) -> libc::c_int {
		match e {
			DatabaseError::IsReadOnly => 1,
			DatabaseError::IOError(_) => 2,
			DatabaseError::FileCorrupted(_) => 3,
		}
	}
}

#[no_mangle]
pub extern "C" fn db_error_description(error: *const DatabaseError) -> *mut libc::c_char {
	let error: &DatabaseError = unsafe {
		/* SAFETY: pointer lifetime is greater than the current stack frame */
		&*e
	};
	let error_str: String = match e {
		DatabaseError::IsReadOnly => format!("cannot write to read-only database"),
		DatabaseError::IOError(e) => format!("I/O Error: {}", e),
		DatabaseError::FileCorrupted(s) => format!("File corrupted, run repair: {}", &s),
	};
	let c_error = /* copy error_str to an allocated buffer with a NUL character at the end */;
	c_error
}
```

### Custom Error Types

```rust
struct ParseError {
	expected: char,
	line: u32,
	char: u16
}

impl ParseError { /* ... */ }

/* Create a second version which is exposed as a C structure */
#[repr(C)]
pub struct parse_error {
	pub expected: char,
	pub line: u32,
	pub char: u16
}

impl From<ParseError> for parse_error {
	from(e: ParseError) -> parse_error {
		let ParseError { expected, line, char } = e;
		parse_error { expected, line, char }
	}
}
```

## Advantages

This ensures that the foreign language has clear access to error information while not compromising the Rust code's API at all.

## Disadvantages

It's a lot of typing, and some types may not be able to be converted easily to C.
