# Temporary mutability

## Description

Often it is necessary to prepare and process some data, but after that data are only inspected
and never modified. It would be cool to turn mutable variable into immutable one.

It can be done either by processing data within nested block or by rebinding variable.


## Example

Say, vector must be sorted before usage.

Using nested block:

```rust
let data = {
	let mut data = get_vec();
	data.sort();
	data
};

// Here `data` is immutable.
```

Using variable rebinding:

```rust
let mut data = get_vec();
data.sort();
let data = data;

// Here `data` is immutable.
```


## Advantages

Compiler ensures that you don't accidentally mutate data after some point.


## Disadvantages

Nested block requires additional indentation of block body.
One more line to return data from block or rebind variable (but it worth it).

