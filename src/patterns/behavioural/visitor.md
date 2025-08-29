# ビジターパターン

## 説明

ビジターは、異種のオブジェクトのコレクションに対して動作するアルゴリズムをカプセル化します。
データ（またはその主要な動作）を変更することなく、同じデータに対して複数の異なるアルゴリズムを
書くことができます。

さらに、ビジターパターンは、オブジェクトのコレクションの走査と
各オブジェクトに対して実行される操作を分離することを可能にします。

## Example

```rust,ignore
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

use ast::*;
use visit::*;

// An example concrete implementation - walks the AST interpreting it as code.
struct Interpreter;
impl Visitor<i64> for Interpreter {
    fn visit_name(&mut self, n: &Name) -> i64 {
        panic!()
    }
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

ASTデータを変更することなく、例えば型チェッカーなどの追加のビジターを実装することができます。

## 動機

ビジターパターンは、異種データにアルゴリズムを適用したい場合に有用です。
データが同種の場合は、イテレータのようなパターンを使用できます。
（関数型アプローチではなく）ビジターオブジェクトを使用することで、
ビジターがステートフルになり、ノード間で情報を通信できます。

## 議論

`visit_*`メソッドがvoidを返すことが一般的です（例とは対照的に）。
その場合、走査コードを分離してアルゴリズム間で共有することが可能になります
（また、noop デフォルトメソッドを提供することも可能です）。
Rustでは、各データに対して`walk_*`関数を提供することが一般的な方法です。
例えば：

```rust,ignore
pub fn walk_expr(visitor: &mut Visitor, e: &Expr) {
    match *e {
        Expr::IntLit(_) => {}
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

他の言語（例：Java）では、データが同じ役割を果たす`accept`メソッドを持つことが一般的です。

## 参照

ビジターパターンは、ほとんどのOO言語で共通のパターンです。

[Wikipedia記事](https://en.wikipedia.org/wiki/Visitor_pattern)

[fold](../creational/fold.md)パターンはビジターに似ていますが、
訪問されたデータ構造の新しいバージョンを生成します。
