# Temporary mutability

## Description

Often it is necessary to prepare and process some data, but after that data are
only inspected and never modified. The intention can be made explicit by
redefining the mutable variable as immutable.

It can be done either by processing data within a nested block or by redefining
the variable.

## Example

Say, vector must be sorted before usage.

Using nested block:

```rust,ignore
let data = {
    let mut data = get_vec();
    data.sort();
    data
};

// Here `data` is immutable.
```

Using variable rebinding:

```rust,ignore
let mut data = get_vec();
data.sort();
let data = data;

// Here `data` is immutable.
```

## Advantages

Compiler ensures that you don't accidentally mutate data after some point.

## Disadvantages

Nested block requires additional indentation of block body. One more line to
return data from block or redefine variable.
