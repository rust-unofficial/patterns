# Pass clone to closure

## Description

By default, closures capture their environment by borrowing. Or you can use `move`-closure
to move environment. However, often you want to give copy of some data to closure.
Use variable rebinding in separate scope for that.


## Example

```rust
let mut num = Rc::new(5);
let closure = {
	let num = num.clone();
	move || {
		*num + 10
	}
};

// Instead of
let num_copy = num.clone();
let closure = move || { *num_copy + 10 };
```


## Advantages

Copied data are grouped together with closure definition, so their purpose is more clear
and they will be dropped immediately even if they are not consumed by closure.

Closure uses same variable names as surrounding code whether data are copied or moved.

