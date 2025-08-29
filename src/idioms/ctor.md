# コンストラクタ

## 説明

Rustは言語構成としてコンストラクタを持ちません。代わりに、オブジェクトを作成するために
[関連関数][associated function]`new`を使用することが慣例です：

````rust
/// 秒単位の時間。
///
/// # 例
///
/// ```
/// let s = Second::new(42);
/// assert_eq!(42, s.value());
/// ```
pub struct Second {
    value: u64,
}

impl Second {
    // [`Second`]の新しいインスタンスを構築します。
    // これは関連関数であることに注意 - selfはありません。
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    /// 秒単位の値を返します。
    pub fn value(&self) -> u64 {
        self.value
    }
}
````

## デフォルトコンストラクタ

Rustは[`Default`][std-default]トレイトでデフォルトコンストラクタをサポートします：

````rust
/// 秒単位の時間。
///
/// # 例
///
/// ```
/// let s = Second::default();
/// assert_eq!(0, s.value());
/// ```
pub struct Second {
    value: u64,
}

impl Second {
    /// 秒単位の値を返します。
    pub fn value(&self) -> u64 {
        self.value
    }
}

impl Default for Second {
    fn default() -> Self {
        Self { value: 0 }
    }
}
````

`Default`は、`Second`のように、すべてのフィールドのすべての型が`Default`を実装している場合は、
導出することもできます：

````rust
/// 秒単位の時間。
///
/// # 例
///
/// ```
/// let s = Second::default();
/// assert_eq!(0, s.value());
/// ```
#[derive(Default)]
pub struct Second {
    value: u64,
}

impl Second {
    /// 秒単位の値を返します。
    pub fn value(&self) -> u64 {
        self.value
    }
}
````

**注意:** 型が`Default`と空の`new`コンストラクタの両方を実装することは一般的であり、
期待されることです。`new`はRustのコンストラクタ慣例であり、ユーザーはその存在を期待しています。
したがって、基本コンストラクタが引数を取らないことが合理的であるならば、
機能的にdefaultと同一であっても、そうすべきです。

**ヒント:** `Default`を実装または導出する利点は、`Default`実装が必要な場所であなたの型が
使用できるようになることです。最も目立つのは、
[標準ライブラリの`*or_default`関数][std-or-default]のいずれかです。

## 関連項目

- `Default`トレイトのより詳細な説明については、[デフォルトの慣用句](default.md)を参照。

- 複数の設定があるオブジェクトを構築するための[ビルダーパターン](../patterns/creational/builder.md)。

- `Default`と`new`の両方を実装するための[API Guidelines/C-COMMON-TRAITS][API Guidelines/C-COMMON-TRAITS]。

[associated function]: https://doc.rust-lang.org/stable/book/ch05-03-method-syntax.html#associated-functions
[std-default]: https://doc.rust-lang.org/stable/std/default/trait.Default.html
[std-or-default]: https://doc.rust-lang.org/stable/std/?search=or_default
[API Guidelines/C-COMMON-TRAITS]: https://rust-lang.github.io/api-guidelines/interoperability.html#types-eagerly-implement-common-traits-c-common-traits
