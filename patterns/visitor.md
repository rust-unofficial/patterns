# Visitor

## Description

A visitor encapsulates an algorithm that operates over a heterogeneous
collection of objects. It allows multiple different algorithms to be written
over the same data without having to modify the data (or their primary
behaviour).

Furthermore, the visitor pattern allows separating the traversal of
a collection of objects from the operations performed on each object.


## Example

```rust
// The data we will visit
mod ast {
    pub enum Stmt {
        Expr(Expr),
        Let(Name, Expr),
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

// The abstract visitor
mod visit {
    use ast::*;

    pub trait Visitor<T> {
        fn visit_name(&mut self, n: &Name) -> T;
        fn visit_stmt(&mut self, s: &Stmt) -> T;
        fn visit_expr(&mut self, e: &Expr) -> T;
    }
}

use visit::*;
use ast::*;

// An example concrete implementation - walks the AST interpreting it as code.
struct Interpreter;
impl Visitor<i64> for Interpreter {
    fn visit_name(&mut self, n: &Name) -> i64 { panic!() }
    fn visit_stmt(&mut self, s: &Stmt) -> i64 {
        match *s {
            Stmt::Expr(ref e) => self.visit_expr(e),
            Stmt::Let(..) => unimplemented!(),
        }
    }

    fn visit_expr(&mut self, e: &Expr) -> i64 {
        match *e {
            Expr::IntLit(n) => n,
            Expr::Add(ref lhs, ref rhs) => self.visit_expr(lhs) + self.visit_expr(rhs),
            Expr::Sub(ref lhs, ref rhs) => self.visit_expr(lhs) - self.visit_expr(rhs),
        }
    }
}
```

One could implement further visitors, for example a type checker, without having
to modify the AST data.


## Motivation

The visitor pattern is useful anywhere that you want to apply an algorithm to
heterogeneous data. If data is homogeneous, you can use an iterator-like pattern.
Using a visitor object (rather than a functional approach) allows the visitor to
be stateful and thus communicate information between nodes.


## Discussion

It is common for the `visit_*` methods to return void (as opposed to in the
example). In that case it is possible to factor out the traversal code and share
it between algorithms (and also to provide noop default methods). In Rust, the
common way to do this is to provide `walk_*` functions for each datum. For
example,

```rust
pub fn walk_expr(visitor: &mut Visitor, e: &Expr) {
    match *e {
        Expr::IntLit(_) => {},
        Expr::Add(ref lhs, ref rhs) => {
            visitor.visit_expr(lhs);
            visitor.visit_expr(rhs);
        }
        Expr::Sub(ref lhs, ref rhs) => {
            visitor.visit_expr(lhs);
            visitor.visit_expr(rhs);
        }
    }
}
```

In other languages (e.g., Java) it is common for data to have an `accept` method
which performs the same duty.

## See also

The visitor pattern is a common pattern in most OO languages.

[Wikipedia article](https://en.wikipedia.org/wiki/Visitor_pattern)

The [fold](fold.md) pattern is similar to visitor but produces a new version of
the visited data structure.
