# 関数型言語のオプティクス

オプティクスは関数型言語に共通するAPIデザインの一種です。これは純粋関数型の概念で、Rustで頻繁に使用されることはありません。

それでも、この概念を探求することは、[ビジター](../patterns/behavioural/visitor.md)などのRust APIの他のパターンを理解するのに役立つかもしれません。また、ニッチな使用例もあります。

これはかなり大きなトピックであり、その能力を完全に理解するには言語設計に関する実際の書籍が必要でしょう。しかし、Rustでの適用可能性ははるかに単純です。

概念の関連部分を説明するため、`Serde`-APIを例として使用します。これは、単にAPIドキュメントから理解するのが多くの人にとって困難なものだからです。

プロセスでは、オプティクスと呼ばれる様々な特定のパターンがカバーされます。これらは*アイソ*、*ポリ・アイソ*、および*プリズム*です。

## APIの例：Serde

*Serde*の動作方法をAPIを読むだけで理解しようとするのは困難です、特に初回は。新しいデータフォーマットを解析する任意のライブラリによって実装される`Deserializer`トレイトを考えてみましょう：

```rust,ignore
pub trait Deserializer<'de>: Sized {
    type Error: Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    // remainder omitted
}
```

And here's the definition of the `Visitor` trait passed in generically:

```rust,ignore
pub trait Visitor<'de>: Sized {
    type Value;

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: Error;

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error;

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error;

    // remainder omitted
}
```

ここでは多くの型消去が行われており、複数レベルの関連型が行き来しています。

しかし、全体像は何でしょうか？`Visitor`にストリーミAPIで呼び出し元が必要とするピースを返させ、それで終わりにしないのはなぜでしょうか？なぜこれらの余分なピースがあるのでしょうか？

これを理解する一つの方法は、*オプティクス*と呼ばれる関数型言語の概念を見ることです。

これは、Rustに共通するパターン、すなわち失敗、型変換などを促進するように設計された動作と属性の合成を行う方法です。[^1]

Rust言語は、これらを直接的にサポートすることはあまり得意ではありません。しかし、これらは言語自体の設計に現れており、その概念はRustのAPIの一部を理解するのに役立ちます。その結果、これはRustが行う方法で概念を説明しようと試みます。

これはおそらく、これらのAPIが何を達成しているのかを明らかにします：合成可能性の特定の属性です。

## 基本オプティクス

### アイソ

アイソは2つの型間の値変換器です。非常にシンプルですが、概念的に重要な構成要素です。

例として、ドキュメントのコンコーダンスとして使用されるカスタムハッシュテーブル構造があると仮定します。[^2]これはキーとして文字列（単語）を使用し、値としてインデックスのリスト（例えばファイルオフセット）を使用します。

重要な機能は、このフォーマットをディスクにシリアライズする能力です。「簡易で実用的」なアプローチは、JSONフォーマットの文字列との相互変換を実装することです。（エラーは今のところ無視され、後で処理されます。）

関数型言語ユーザーが期待する正規形で書くと：

```text
case class ConcordanceSerDe {
  serialize: Concordance -> String
  deserialize: String -> Concordance
}
```

アイソは、異なる型の値を変換する関数のペアです：`serialize`と`deserialize`。

簡単な実装：

```rust
use std::collections::HashMap;

struct Concordance {
    keys: HashMap<String, usize>,
    value_table: Vec<(usize, usize)>,
}

struct ConcordanceSerde {}

impl ConcordanceSerde {
    fn serialize(value: Concordance) -> String {
        todo!()
    }
    // 無効なコンコーダンスは空です
    fn deserialize(value: String) -> Concordance {
        todo!()
    }
}
```

これはばからしく見えるかもしれません。Rustでは、この種の動作は通常トレイトで行われます。結局のところ、標準ライブラリには`FromStr`と`ToString`があります。

しかし、ここで私たちの次の主題が登場します：ポリ・アイソ。

### Poly Isos

The previous example was simply converting between values of two fixed types.
This next block builds upon it with generics, and is more interesting.

Poly Isos allow an operation to be generic over any type while returning a
single type.

This brings us closer to parsing. Consider what a basic parser would do ignoring
error cases. Again, this is its normal form:

```text
case class Serde[T] {
    deserialize(String) -> T
    serialize(T) -> String
}
```

Here we have our first generic, the type `T` being converted.

In Rust, this could be implemented with a pair of traits in the standard
library: `FromStr` and `ToString`. The Rust version even handles errors:

```rust,ignore
pub trait FromStr: Sized {
    type Err;

    fn from_str(s: &str) -> Result<Self, Self::Err>;
}

pub trait ToString {
    fn to_string(&self) -> String;
}
```

Unlike the Iso, the Poly Iso allows application of multiple types, and returns
them generically. This is what you would want for a basic string parser.

At first glance, this seems like a good option for writing a parser. Let's see
it in action:

```rust,ignore
use anyhow;

use std::str::FromStr;

struct TestStruct {
    a: usize,
    b: String,
}

impl FromStr for TestStruct {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<TestStruct, Self::Err> {
        todo!()
    }
}

impl ToString for TestStruct {
    fn to_string(&self) -> String {
        todo!()
    }
}

fn main() {
    let a = TestStruct {
        a: 5,
        b: "hello".to_string(),
    };
    println!("Our Test Struct as JSON: {}", a.to_string());
}
```

That seems quite logical. However, there are two problems with this.

First, `to_string` does not indicate to API users, "this is JSON." Every type
would need to agree on a JSON representation, and many of the types in the Rust
standard library already don't. Using this is a poor fit. This can easily be
resolved with our own trait.

But there is a second, subtler problem: scaling.

When every type writes `to_string` by hand, this works. But if every single
person who wants their type to be serializable has to write a bunch of code --
and possibly different JSON libraries -- to do it themselves, it will turn into
a mess very quickly!

The answer is one of Serde's two key innovations: an independent data model to
represent Rust data in structures common to data serialization languages. The
result is that it can use Rust's code generation abilities to create an
intermediary conversion type it calls a `Visitor`.

This means, in normal form (again, skipping error handling for simplicity):

```text
case class Serde[T] {
    deserialize: Visitor[T] -> T
    serialize: T -> Visitor[T]
}

case class Visitor[T] {
    toJson: Visitor[T] -> String
    fromJson: String -> Visitor[T]
}
```

The result is one Poly Iso and one Iso (respectively). Both of these can be
implemented with traits:

```rust
trait Serde {
    type V;
    fn deserialize(visitor: Self::V) -> Self;
    fn serialize(self) -> Self::V;
}

trait Visitor {
    fn to_json(self) -> String;
    fn from_json(json: String) -> Self;
}
```

Because there is a uniform set of rules to transform Rust structures to the
independent form, it is even possible to have code generation creating the
`Visitor` associated with type `T`:

```rust,ignore
#[derive(Default, Serde)] // the "Serde" derive creates the trait impl block
struct TestStruct {
    a: usize,
    b: String,
}

// user writes this macro to generate an associated visitor type
generate_visitor!(TestStruct);
```

But let's actually try that approach.

```rust,ignore
fn main() {
    let a = TestStruct { a: 5, b: "hello".to_string() };
    let a_data = a.serialize().to_json();
    println!("Our Test Struct as JSON: {a_data}");
    let b = TestStruct::deserialize(
        generated_visitor_for!(TestStruct)::from_json(a_data));
}
```

It turns out that the conversion isn't symmetric after all! On paper it is, but
with the auto-generated code the name of the actual type necessary to convert
all the way from `String` is hidden. We'd need some kind of
`generated_visitor_for!` macro to obtain the type name.

It's wonky, but it works... until we get to the elephant in the room.

The only format currently supported is JSON. How would we support more formats?

The current design requires completely re-writing all of the code generation and
creating a new Serde trait. That is quite terrible and not extensible at all!

In order to solve that, we need something more powerful.

## Prism

To take format into account, we need something in normal form like this:

```text
case class Serde[T, F] {
    serialize: T, F -> String
    deserialize: String, F -> Result[T, Error]
}
```

This construct is called a Prism. It is "one level higher" in generics than Poly
Isos (in this case, the "intersecting" type F is the key).

Unfortunately because `Visitor` is a trait (since each incarnation requires its
own custom code), this would require a kind of generic type boundary that Rust
does not support.

Fortunately, we still have that `Visitor` type from before. What is the
`Visitor` doing? It is attempting to allow each data structure to define the way
it is itself parsed.

Well what if we could add one more interface for the generic format? Then the
`Visitor` is just an implementation detail, and it would "bridge" the two APIs.

In normal form:

```text
case class Serde[T] {
    serialize: F -> String
    deserialize F, String -> Result[T, Error]
}

case class VisitorForT {
    build: F, String -> Result[T, Error]
    decompose: F, T -> String
}

case class SerdeFormat[T, V] {
    toString: T, V -> String
    fromString: V, String -> Result[T, Error]
}
```

And what do you know, a pair of Poly Isos at the bottom which can be implemented
as traits!

Thus we have the Serde API:

1. Each type to be serialized implements `Deserialize` or `Serialize`,
   equivalent to the `Serde` class
1. They get a type (well two, one for each direction) implementing the `Visitor`
   trait, which is usually (but not always) done through code generated by a
   derive macro. This contains the logic to construct or destruct between the
   data type and the format of the Serde data model.
1. The type implementing the `Deserializer` trait handles all details specific
   to the format, being "driven by" the `Visitor`.

This splitting and Rust type erasure is really to achieve a Prism through
indirection.

You can see it on the `Deserializer` trait

```rust,ignore
pub trait Deserializer<'de>: Sized {
    type Error: Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    // remainder omitted
}
```

And the visitor:

```rust,ignore
pub trait Visitor<'de>: Sized {
    type Value;

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: Error;

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error;

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error;

    // remainder omitted
}
```

And the trait `Deserialize` implemented by the macros:

```rust,ignore
pub trait Deserialize<'de>: Sized {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;
}
```

This has been abstract, so let's look at a concrete example.

How does actual Serde deserialize a bit of JSON into `struct Concordance` from
earlier?

1. The user would call a library function to deserialize the data. This would
   create a `Deserializer` based on the JSON format.
1. Based on the fields in the struct, a `Visitor` would be created (more on that
   in a moment) which knows how to create each type in a generic data model that
   was needed to represent it: `Vec` (list), `u64` and `String`.
1. The deserializer would make calls to the `Visitor` as it parsed items.
1. The `Visitor` would indicate if the items found were expected, and if not,
   raise an error to indicate deserialization has failed.

For our very simple structure above, the expected pattern would be:

1. Begin visiting a map (*Serde*'s equivalent to `HashMap` or JSON's
   dictionary).
1. Visit a string key called "keys".
1. Begin visiting a map value.
1. For each item, visit a string key then an integer value.
1. Visit the end of the map.
1. Store the map into the `keys` field of the data structure.
1. Visit a string key called "value_table".
1. Begin visiting a list value.
1. For each item, visit an integer.
1. Visit the end of the list
1. Store the list into the `value_table` field.
1. Visit the end of the map.

But what determines which "observation" pattern is expected?

A functional programming language would be able to use currying to create
reflection of each type based on the type itself. Rust does not support that, so
every single type would need to have its own code written based on its fields
and their properties.

*Serde* solves this usability challenge with a derive macro:

```rust,ignore
use serde::Deserialize;

#[derive(Deserialize)]
struct IdRecord {
    name: String,
    customer_id: String,
}
```

That macro simply generates an impl block causing the struct to implement a
trait called `Deserialize`.

This is the function that determines how to create the struct itself. Code is
generated based on the struct's fields. When the parsing library is called - in
our example, a JSON parsing library - it creates a `Deserializer` and calls
`Type::deserialize` with it as a parameter.

The `deserialize` code will then create a `Visitor` which will have its calls
"refracted" by the `Deserializer`. If everything goes well, eventually that
`Visitor` will construct a value corresponding to the type being parsed and
return it.

For a complete example, see the
[*Serde* documentation](https://serde.rs/deserialize-struct.html).

The result is that types to be deserialized only implement the "top layer" of
the API, and file formats only need to implement the "bottom layer". Each piece
can then "just work" with the rest of the ecosystem, since generic types will
bridge them.

結論として、このAPI設計で示されるように、Rustのジェネリックスにインスパイアされた型システムは、これらの概念に近づけ、その力を使用することができます。しかし、ジェネリックスのためのブリッジを作成するために手続きマクロが必要になる場合もあります。

このトピックについてさらに学びたい場合は、次のセクションをご確認ください。

## 関連項目

- これらの例よりもクリーンなインターフェースを持つ、事前に構築されたレンズ実装のための[lens-rsクレート](https://crates.io/crates/lens-rs)
- 詳細を理解する必要なく、エンドユーザー（すなわち構造体の定義）にとってこれらの概念を直感的にする[Serde](https://serde.rs)自体
- コンピューターグラフィックスを描画するためのクレートで、ジェネリックのまま異なるピクセルタイプのバッファのための完全なプリズムを作成する手続きマクロを含む、類似のAPI設計を使用する[luminance](https://github.com/phaazon/luminance-rs)
- Scalaの専門知識がなくても非常に読みやすい[Scalaのレンズに関する記事](https://web.archive.org/web/20221128185849/https://medium.com/zyseme-technology/functional-references-lens-and-other-optics-in-scala-e5f7e2fdafe)
- [論文：Profunctor Optics: Modular Data Accessors](https://web.archive.org/web/20220701102832/https://arxiv.org/ftp/arxiv/papers/1703/1703.10857.pdf)
- 異なるアプローチで類似の構造を使用しようと試みるライブラリで、例えばビジターを去った[Musli](https://github.com/udoprog/musli)

[^1]: [School of Haskell: A Little Lens Starter Tutorial](https://web.archive.org/web/20221128190041/https://www.schoolofhaskell.com/school/to-infinity-and-beyond/pick-of-the-week/a-little-lens-starter-tutorial)

[^2]: [Concordance on Wikipedia](https://en.wikipedia.org/wiki/Concordance_(publishing))
