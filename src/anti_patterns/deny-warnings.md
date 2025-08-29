# `#![deny(warnings)]`

## 説明

善意のクレート作者が、警告なしでコードがビルドされることを確保したいと考えます。
そこで、クレートのルートに以下のアノテーションを付けます：

## 例

```rust
#![deny(warnings)]

// すべて順調。
```

## 利点

短く、何か問題があればビルドを停止します。

## 欠点

警告付きでのビルドをコンパイラに禁止することで、クレート作者は
Rustの有名な安定性から逸脱します。時々、新しい機能や古い誤機能は、
物事の行い方の変更を必要とし、したがって`deny`に変更される前の
一定の猶予期間`warn`するlintが書かれます。

例えば、型が同じメソッドを持つ2つの`impl`を持つことができることが発見されました。
これは悪い考えとみなされましたが、移行をスムーズにするために、
将来のリリースでハードエラーになる前に、この事実に遭遇する人に警告を与えるために
`overlapping-inherent-impls` lintが導入されました。

また、時々APIが非推奨になるため、以前は警告がなかった使用で警告が出力されます。

これらすべてが、何かが変更されるたびにビルドが破損する可能性があることを共謀します。

さらに、追加のlintを提供するクレート（例：[rust-clippy]）は、
アノテーションが削除されない限り、もはや使用できません。これは
[--cap-lints]で緩和されます。`--cap-lints=warn` コマンドライン引数は、
すべての`deny` lintエラーを警告に変えます。

## 代替案

この問題に取り組む方法は2つあります：第一に、ビルド設定をコードから
切り離すことができ、第二に、拒否したいlintを明示的に名前を挙げることができます。

以下のコマンドラインは、すべての警告を`deny`に設定してビルドします：

`RUSTFLAGS="-D warnings" cargo build`

これは、コードの変更を必要とせずに、個々の開発者（またはTravisのようなCIツールに設定される、
ただし何かが変更されたときにビルドが破損する可能性があることを覚えておいてください）によって行うことができます。

あるいは、コード内で`deny`したいlintを指定することができます。
以下は（願わくば）拒否しても安全な警告lintのリストです（rustc 1.48.0時点）：

```rust,ignore
#![deny(
    bad_style,
    const_err,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true
)]
```

加えて、以下の`allow`されたlintを`deny`するのは良いアイデアかもしれません：

```rust,ignore
#![deny(
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]
```

一部の人は`missing-copy-implementations`もリストに追加したいかもしれません。

将来的にもっと多くの非推奨APIがあることがかなり確実であるため、
明示的に`deprecated` lintは追加しなかったことに注意してください。

## 参照

- [すべてのclippy lintのコレクション](https://rust-lang.github.io/rust-clippy/master)
- [deprecate attribute] ドキュメント
- システム上のlintのリストについては`rustc -W help`と入力。また、
  一般的なオプションのリストについては`rustc --help`と入力
- [rust-clippy]は、より良いRustコードのためのlintのコレクション

[rust-clippy]: https://github.com/rust-lang/rust-clippy
[deprecate attribute]: https://doc.rust-lang.org/reference/attributes.html#deprecation
[--cap-lints]: https://doc.rust-lang.org/rustc/lints/levels.html#capping-lints
