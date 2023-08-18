# Generics as Type Classes

## Description

Rust's type system is designed more like functional languages (like Haskell)
rather than imperative languages (like Java and C++). As a result, Rust can turn
many kinds of programming problems into "static typing" problems. This is one of
the biggest wins of choosing a functional language, and is critical to many of
Rust's compile time guarantees.

A key part of this idea is the way generic types work. In C++ and Java, for
example, generic types are a meta-programming construct for the compiler.
`vector<int>` and `vector<char>` in C++ are just two different copies of the
same boilerplate code for a `vector` type (known as a `template`) with two
different types filled in.

In Rust, a generic type parameter creates what is known in functional languages
as a "type class constraint", and each different parameter filled in by an end
user *actually changes the type*. In other words, `Vec<isize>` and `Vec<char>`
*are two different types*, which are recognized as distinct by all parts of the
type system.

This is called **monomorphization**, where different types are created from
**polymorphic** code. This special behavior requires `impl` blocks to specify
generic parameters. Different values for the generic type cause different types,
and different types can have different `impl` blocks.

In object-oriented languages, classes can inherit behavior from their parents.
However, this allows the attachment of not only additional behavior to
particular members of a type class, but extra behavior as well.

The nearest equivalent is the runtime polymorphism in Javascript and Python,
where new members can be added to objects willy-nilly by any constructor.
However, unlike those languages, all of Rust's additional methods can be type
checked when they are used, because their generics are statically defined. That
makes them more usable while remaining safe.

## Example

Suppose you are designing a storage server for a series of lab machines. Because
of the software involved, there are two different protocols you need to support:
BOOTP (for PXE network boot), and NFS (for remote mount storage).

Your goal is to have one program, written in Rust, which can handle both of
them. It will have protocol handlers and listen for both kinds of requests. The
main application logic will then allow a lab administrator to configure storage
and security controls for the actual files.

The requests from machines in the lab for files contain the same basic
information, no matter what protocol they came from: an authentication method,
and a file name to retrieve. A straightforward implementation would look
something like this:

```rust,ignore
enum AuthInfo {
    Nfs(crate::nfs::AuthInfo),
    Bootp(crate::bootp::AuthInfo),
}

struct FileDownloadRequest {
    file_name: PathBuf,
    authentication: AuthInfo,
}
```

This design might work well enough. But now suppose you needed to support adding
metadata that was *protocol specific*. For example, with NFS, you wanted to
determine what their mount point was in order to enforce additional security
rules.

The way the current struct is designed leaves the protocol decision until
runtime. That means any method that applies to one protocol and not the other
requires the programmer to do a runtime check.

Here is how getting an NFS mount point would look:

```rust,ignore
struct FileDownloadRequest {
    file_name: PathBuf,
    authentication: AuthInfo,
    mount_point: Option<PathBuf>,
}

impl FileDownloadRequest {
    // ... other methods ...

    /// Gets an NFS mount point if this is an NFS request. Otherwise,
    /// return None.
    pub fn mount_point(&self) -> Option<&Path> {
        self.mount_point.as_ref()
    }
}
```

Every caller of `mount_point()` must check for `None` and write code to handle
it. This is true even if they know only NFS requests are ever used in a given
code path!

It would be far more optimal to cause a compile-time error if the different
request types were confused. After all, the entire path of the user's code,
including what functions from the library they use, will know whether a request
is an NFS request or a BOOTP request.

In Rust, this is actually possible! The solution is to *add a generic type* in
order to split the API.

Here is what that looks like:

```rust
use std::path::{Path, PathBuf};

mod nfs {
    #[derive(Clone)]
    pub(crate) struct AuthInfo(String); // NFS session management omitted
}

mod bootp {
    pub(crate) struct AuthInfo(); // no authentication in bootp
}

// private module, lest outside users invent their own protocol kinds!
mod proto_trait {
    use std::path::{Path, PathBuf};
    use super::{bootp, nfs};

    pub(crate) trait ProtoKind {
        type AuthInfo;
        fn auth_info(&self) -> Self::AuthInfo;
    }

    pub struct Nfs {
        auth: nfs::AuthInfo,
        mount_point: PathBuf,
    }

    impl Nfs {
        pub(crate) fn mount_point(&self) -> &Path {
            &self.mount_point
        }
    }

    impl ProtoKind for Nfs {
        type AuthInfo = nfs::AuthInfo;
        fn auth_info(&self) -> Self::AuthInfo {
            self.auth.clone()
        }
    }

    pub struct Bootp(); // no additional metadata

    impl ProtoKind for Bootp {
        type AuthInfo = bootp::AuthInfo;
        fn auth_info(&self) -> Self::AuthInfo {
            bootp::AuthInfo()
        }
    }
}

use proto_trait::ProtoKind; // keep internal to prevent impls
pub use proto_trait::{Nfs, Bootp}; // re-export so callers can see them

struct FileDownloadRequest<P: ProtoKind> {
    file_name: PathBuf,
    protocol: P,
}

// all common API parts go into a generic impl block
impl<P: ProtoKind> FileDownloadRequest<P> {
    fn file_path(&self) -> &Path {
        &self.file_name
    }

    fn auth_info(&self) -> P::AuthInfo {
        self.protocol.auth_info()
    }
}

// all protocol-specific impls go into their own block
impl FileDownloadRequest<Nfs> {
    fn mount_point(&self) -> &Path {
        self.protocol.mount_point()
    }
}

fn main() {
    // your code here
}
```

With this approach, if the user were to make a mistake and use the wrong type;

```rust,ignore
fn main() {
    let mut socket = crate::bootp::listen()?;
    while let Some(request) = socket.next_request()? {
        match request.mount_point().as_ref()
            "/secure" => socket.send("Access denied"),
            _ => {} // continue on...
        }
        // Rest of the code here
    }
}
```

They would get a syntax error. The type `FileDownloadRequest<Bootp>` does not
implement `mount_point()`, only the type `FileDownloadRequest<Nfs>` does. And
that is created by the NFS module, not the BOOTP module of course!

## Advantages

First, it allows fields that are common to multiple states to be de-duplicated.
By making the non-shared fields generic, they are implemented once.

Second, it makes the `impl` blocks easier to read, because they are broken down
by state. Methods common to all states are typed once in one block, and methods
unique to one state are in a separate block.

Both of these mean there are fewer lines of code, and they are better organized.

## Disadvantages

This currently increases the size of the binary, due to the way monomorphization
is implemented in the compiler. Hopefully the implementation will be able to
improve in the future.

## Alternatives

- If a type seems to need a "split API" due to construction or partial
  initialization, consider the
  [Builder Pattern](../patterns/creational/builder.md) instead.

- If the API between types does not change -- only the behavior does -- then the
  [Strategy Pattern](../patterns/behavioural/strategy.md) is better used
  instead.

## See also

This pattern is used throughout the standard library:

- `Vec<u8>` can be cast from a String, unlike every other type of `Vec<T>`.[^1]
- They can also be cast into a binary heap, but only if they contain a type that
  implements the `Ord` trait.[^2]
- The `to_string` method was specialized for `Cow` only of type `str`.[^3]

It is also used by several popular crates to allow API flexibility:

- The `embedded-hal` ecosystem used for embedded devices makes extensive use of
  this pattern. For example, it allows statically verifying the configuration of
  device registers used to control embedded pins. When a pin is put into a mode,
  it returns a `Pin<MODE>` struct, whose generic determines the functions usable
  in that mode, which are not on the `Pin` itself. [^4]

- The `hyper` HTTP client library uses this to expose rich APIs for different
  pluggable requests. Clients with different connectors have different methods
  on them as well as different trait implementations, while a core set of
  methods apply to any connector. [^5]

- The "type state" pattern -- where an object gains and loses API based on an
  internal state or invariant -- is implemented in Rust using the same basic
  concept, and a slightly different technique. [^6]

[^1]: See:
[impl From\<CString\> for Vec\<u8\>](https://doc.rust-lang.org/1.59.0/src/std/ffi/c_str.rs.html#803-811)

[^2]: See:
[impl\<T: Ord\> FromIterator\<T\> for BinaryHeap\<T\>](https://web.archive.org/web/20201030132806/https://doc.rust-lang.org/stable/src/alloc/collections/binary_heap.rs.html#1330-1335)

[^3]: See:
[impl\<'\_\> ToString for Cow\<'\_, str>](https://doc.rust-lang.org/stable/src/alloc/string.rs.html#2235-2240)

[^4]: Example:
[https://docs.rs/stm32f30x-hal/0.1.0/stm32f30x_hal/gpio/gpioa/struct.PA0.html](https://docs.rs/stm32f30x-hal/0.1.0/stm32f30x_hal/gpio/gpioa/struct.PA0.html)

[^5]: See:
[https://docs.rs/hyper/0.14.5/hyper/client/struct.Client.html](https://docs.rs/hyper/0.14.5/hyper/client/struct.Client.html)

[^6]: See:
[The Case for the Type State Pattern](https://web.archive.org/web/20210325065112/https://www.novatec-gmbh.de/en/blog/the-case-for-the-typestate-pattern-the-typestate-pattern-itself/)
and
[Rusty Typestate Series (an extensive thesis)](https://web.archive.org/web/20210328164854/https://rustype.github.io/notes/notes/rust-typestate-series/rust-typestate-index)
