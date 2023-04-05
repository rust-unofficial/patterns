# `Deref` polymorphism

## Description

Misuse the `Deref` trait to emulate inheritance between structs, and thus reuse
methods.

## Example

Sometimes we want to emulate the following common pattern from OO languages such
as Java:

```java
class Foo {
    void m() { ... }
}

class Bar extends Foo {}

public static void main(String[] args) {
    Bar b = new Bar();
    b.m();
}
```

We can use the deref polymorphism anti-pattern to do so:

```rust
use std::ops::Deref;

struct Foo {}

impl Foo {
    fn m(&self) {
        //..
    }
}

struct Bar {
    f: Foo,
}

impl Deref for Bar {
    type Target = Foo;
    fn deref(&self) -> &Foo {
        &self.f
    }
}

fn main() {
    let b = Bar { f: Foo {} };
    b.m();
}
```

There is no struct inheritance in Rust. Instead we use composition and include
an instance of `Foo` in `Bar` (since the field is a value, it is stored inline,
so if there were fields, they would have the same layout in memory as the Java
version (probably, you should use `#[repr(C)]` if you want to be sure)).

In order to make the method call work we implement `Deref` for `Bar` with `Foo`
as the target (returning the embedded `Foo` field). That means that when we
dereference a `Bar` (for example, using `*`) then we will get a `Foo`. That is
pretty weird. Dereferencing usually gives a `T` from a reference to `T`, here we
have two unrelated types. However, since the dot operator does implicit
dereferencing, it means that the method call will search for methods on `Foo` as
well as `Bar`.

## Advantages

You save a little boilerplate, e.g.,

```rust,ignore
impl Bar {
    fn m(&self) {
        self.f.m()
    }
}
```

## Disadvantages

Most importantly this is a surprising idiom - future programmers reading this in
code will not expect this to happen. That's because we are misusing the `Deref`
trait rather than using it as intended (and documented, etc.). It's also because
the mechanism here is completely implicit.

This pattern does not introduce subtyping between `Foo` and `Bar` like
inheritance in Java or C++ does. Furthermore, traits implemented by `Foo` are
not automatically implemented for `Bar`, so this pattern interacts badly with
bounds checking and thus generic programming.

Using this pattern gives subtly different semantics from most OO languages with
regards to `self`. Usually it remains a reference to the sub-class, with this
pattern it will be the 'class' where the method is defined.

Finally, this pattern only supports single inheritance, and has no notion of
interfaces, class-based privacy, or other inheritance-related features. So, it
gives an experience that will be subtly surprising to programmers used to Java
inheritance, etc.

## Discussion

There is no one good alternative. Depending on the exact circumstances it might
be better to re-implement using traits or to write out the facade methods to
dispatch to `Foo` manually. We do intend to add a mechanism for inheritance
similar to this to Rust, but it is likely to be some time before it reaches
stable Rust. See these [blog](http://aturon.github.io/blog/2015/09/18/reuse/)
[posts](http://smallcultfollowing.com/babysteps/blog/2015/10/08/virtual-structs-part-4-extended-enums-and-thin-traits/)
and this [RFC issue](https://github.com/rust-lang/rfcs/issues/349) for more details.

The `Deref` trait is designed for the implementation of custom pointer types.
The intention is that it will take a pointer-to-`T` to a `T`, not convert
between different types. It is a shame that this isn't (probably cannot be)
enforced by the trait definition.

Rust tries to strike a careful balance between explicit and implicit mechanisms,
favouring explicit conversions between types. Automatic dereferencing in the dot
operator is a case where the ergonomics strongly favour an implicit mechanism,
but the intention is that this is limited to degrees of indirection, not
conversion between arbitrary types.

## See also

- [Collections are smart pointers idiom](../idioms/deref.md).
- Delegation crates for less boilerplate like [delegate](https://crates.io/crates/delegate)
  or [ambassador](https://crates.io/crates/ambassador)
- [Documentation for `Deref` trait](https://doc.rust-lang.org/std/ops/trait.Deref.html).
