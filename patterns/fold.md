# Fold

## Description

Run an algorithm over each item in a collection of data to create a new item,
thus creating a whole new collection.

The etymology here is unclear to me. The terms 'fold' and 'folder' are used
in the Rust compiler, although it appears to me to be more like a map than a
fold in the usual sense. See the discussion below for more details.

## Example

```rust
// The data we will fold, a simple AST.
mod ast {
    pub enum Stmt {
        Expr(Box<Expr>),
        Let(Box<Name>, Box<Expr>),
    }

    pub struct Name {
        value: String,
    }

    pub enum Expr {
        IntLit(i64),
        Add(Box<Expr>, Box<Expr>),
        Sub(Box<Expr>, Box<Expr>),
    }
}

// The abstract folder
mod fold {
    use ast::*;

    pub trait Folder {
        // A leaf node just returns the node itself. In some cases, we can do this
        // to inner nodes too.
        fn fold_name(&mut self, n: Box<Name>) -> Box<Name> { n }
        // Create a new inner node by folding its children.
        fn fold_stmt(&mut self, s: Box<Stmt>) -> Box<Stmt> {
            match *s {
                Stmt::Expr(e) => Box::new(Stmt::Expr(self.fold_expr(e))),
                Stmt::Let(n, e) => Box::new(Stmt::Let(self.fold_name(n), self.fold_expr(e))),
            }
        }
        fn fold_expr(&mut self, e: Box<Expr>) -> Box<Expr> { ... }
    }
}

use fold::*;
use ast::*;

// An example concrete implementation - renames every name to 'foo'.
struct Renamer;
impl Folder for Renamer {
    fn fold_name(&mut self, n: Box<Name>) -> Box<Name> {
        Box::new(Name { value: "foo".to_owned() })
    }
    // Use the default methods for the other nodes.
}
```

The result of running the `Renamer` on an AST is a new AST identical to the old
one, but with every name changed to `foo`. A real life folder might have some
state preserved between nodes in the struct itself.

A folder can also be defined to map one data structure to a different (but
usually similar) data structure. For example, we could fold an AST into a HIR
tree (HIR stands for high-level intermediate representation).


## Motivation

It is common to want to map a data structure by performing some operation on
each node in the structure. For simple operations on simple data structures,
this can be done using `Iterator::map`. For more complex operations, perhaps
where earlier nodes can affect the operation on later nodes, or where iteration
over the data structure is non-trivial, using the fold pattern is more
appropriate.

Like the visitor pattern, the fold pattern allows us to separate traversal of a
data structure from the operations performed to each node.


## Discussion

Mapping data structures in this fashion is common in functional languages. In OO
languages, it would be more common to mutate the data structure in place. The
'functional' approach is common in Rust, mostly due to the preference for
immutability. Using fresh data structures, rather than mutating old ones, makes
reasoning about the code easier in most circumstances.

The trade-off between efficiency and reusability can be tweaked by changing how
nodes are accepted by the `fold_*` methods.

In the above example we operate on `Box` pointers. Since these own their data
exclusively, the original copy of the data structure cannot be re-used. On the
other hand if a node is not changed, reusing it is very efficient.

If we were to operate on borrowed references, the original data structure can be
reused, however, if a node is unchanged it must be cloned, which can be
expensive.

Using a reference counted pointer gives the best of both worlds - we can reuse
the original data structure and we don't need to clone unchanged nodes. However,
they are less ergonomic to use and mean that the data structures cannot be
mutable.


## See also

Iterators have a `fold` method, however this folds a data structure into a
value, rather than into a new data structure. An iterator's `map` is more like
this fold pattern.

In other languages, fold is usually used in the sense of Rust's iterators,
rather than this pattern. Some functional languages have powerful constructs for
performing flexible maps over data structures.

The [visitor](visitor.md) pattern is closely related to fold. They share the
concept of walking a data structure performing an operation on each node.
However, the visitor does not create a new data structure nor consume the old
one.
