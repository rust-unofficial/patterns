# `mem::{take(_), replace(_)}` to keep owned values in changed enums

## Description

Say we have a `&mut MyEnum` which has (at least) two variants,
`A { name: String, x: u8 }` and `B { name: String }`. Now we want to change
`MyEnum::A` to a `B` if `x` is zero, while keeping `MyEnum::B` intact.

We can do this without cloning the `name`.

## Example

```rust
use std::mem;

enum MyEnum {
    A { name: String, x: u8 },
    B { name: String }
}

fn a_to_b(e: &mut MyEnum) {
    if let MyEnum::A { name, x: 0 } = e {
        // this takes out our `name` and put in an empty String instead
        // (note that empty strings don't allocate).
        // Then, construct the new enum variant (which will
        // be assigned to `*e`).
        *e = MyEnum::B { name: mem::take(name) }
    }
}
```

This also works with more variants:

```rust
use std::mem;

enum MultiVariateEnum {
    A { name: String },
    B { name: String },
    C,
    D
}

fn swizzle(e: &mut MultiVariateEnum) {
    use MultiVariateEnum::*;
    *e = match e {
        // Ownership rules do not allow taking `name` by value, but we cannot
        // take the value out of a mutable reference, unless we replace it:
        A { name } => B { name: mem::take(name) },
        B { name } => A { name: mem::take(name) },
        C => D,
        D => C
    }
}
```

## Motivation

When working with enums, we may want to change an enum value in place, perhaps
to another variant. This is usually done in two phases to keep the borrow
checker happy. In the first phase, we observe the existing value and look at
its parts to decide what to do next. In the second phase we may conditionally
change the value (as in the example above).

The borrow checker won't allow us to take out `name` of the enum (because
*something* must be there.) We could of course `.clone()` name and put the clone
into our `MyEnum::B`, but that would be an instance of the [Clone to satisfy
the borrow checker](../anti_patterns/borrow_clone.md) anti-pattern. Anyway, we
can avoid the extra allocation by changing `e` with only a mutable borrow.

`mem::take` lets us swap out the value, replacing it with it's default value,
and returning the previous value. For `String`, the default value is an empty
`String`, which does not need to allocate. As a result, we get the original
`name` *as an owned value*. We can then wrap this in another enum.

__NOTE:__ `mem::replace` is very similar, but allows us to specify what to
replace the value with. An equivalent to our `mem::take` line would be
`mem::replace(name, String::new())`.

Note, however, that if we are using an `Option` and want to replace its
value with a `None`, `Option`’s `take()` method provides a shorter and
more idiomatic alternative.

## Advantages

Look ma, no allocation! Also you may feel like Indiana Jones while doing it.

## Disadvantages

This gets a bit wordy. Getting it wrong repeatedly will make you hate the
borrow checker. The compiler may fail to optimize away the double store,
resulting in reduced performance as opposed to what you'd do in unsafe
languages.

Furthermore, the type you are taking needs to implement the [`Default`
trait](./default.md). However, if the type you're working with doesn't
implement this, you can instead use `mem::replace`.

## Discussion

This pattern is only of interest in Rust. In GC'd languages, you'd take the
reference to the value by default (and the GC would keep track of refs), and in
other low-level languages like C you'd simply alias the pointer and fix things
later.

However, in Rust, we have to do a little more work to do this. An owned value
may only have one owner, so to take it out, we need to put something back in –
like Indiana Jones, replacing the artifact with a bag of sand.

## See also

This gets rid of the [Clone to satisfy the borrow checker](../anti_patterns/borrow_clone.md)
anti-pattern in a specific case.
