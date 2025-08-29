# 引数に借用型を使用する

## 説明

deref強制の対象を使用することで、関数引数にどの引数型を使用するかを決定する際に
コードの柔軟性を高めることができます。この方法で、関数はより多くの入力型を受け入れることができます。

これはスライス可能または太いポインタ型に限定されません。実際には、**借用された所有型**よりも
**借用型**を常に好むべきです。`&String`よりも`&str`、`&Vec<T>`よりも`&[T]`、
`&Box<T>`よりも`&T`などです。

借用型を使用することで、所有型がすでに間接化レイヤーを提供している場合に
間接化レイヤーを避けることができます。たとえば、`String`は間接化レイヤーを持つため、
`&String`は2つの間接化レイヤーを持つことになります。代わりに`&str`を使用し、
関数が呼び出されるたびに`&String`を`&str`に強制変換させることで、これを回避できます。

## 例

この例では、関数引数として`&String`を使用する場合と`&str`を使用する場合の
いくつかの違いを説明しますが、同じ考え方は`&Vec<T>`対`&[T]`の使用や
`&Box<T>`対`&T`の使用にも適用されます。

単語に3つの連続する母音が含まれているかどうかを判定したい例を考えてみましょう。
これを判定するために文字列を所有する必要はないので、参照を取ります。

コードは次のようになるかもしれません：

```rust
fn three_vowels(word: &String) -> bool {
    let mut vowel_count = 0;
    for c in word.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowel_count += 1;
                if vowel_count >= 3 {
                    return true;
                }
            }
            _ => vowel_count = 0,
        }
    }
    false
}

fn main() {
    let ferris = "Ferris".to_string();
    let curious = "Curious".to_string();
    println!("{}: {}", ferris, three_vowels(&ferris));
    println!("{}: {}", curious, three_vowels(&curious));

    // これは正常に動作しますが、以下の2行は失敗します：
    // println!("Ferris: {}", three_vowels("Ferris"));
    // println!("Curious: {}", three_vowels("Curious"));
}
```

これは`&String`型をパラメータとして渡しているため正常に動作します。最後の2行のコメントを
削除すると、例は失敗します。これは`&str`型が`&String`型に強制変換されないためです。
引数の型を単純に変更することでこれを修正できます。

たとえば、関数宣言を次のように変更すると：

```rust, ignore
fn three_vowels(word: &str) -> bool {
```

両方のバージョンがコンパイルされ、同じ出力が印刷されます。

```bash
Ferris: false
Curious: true
```

しかし、それだけではありません！この話にはもっと続きがあります。おそらくあなたは
「そんなことはどうでもいい、私は入力として`&'static str`を使うことはない（`"Ferris"`を使ったときのように）」
と言うかもしれません。この特別な例を無視しても、`&str`を使用すると`&String`を使用するよりも
柔軟性が向上することがわかるでしょう。

誰かが文章を与えてくれて、その文章の単語のいずれかに3つの連続する母音が含まれているか
判定したい例を考えてみましょう。すでに定義した関数を利用して、文章から各単語を
単純に入力すれば良いでしょう。

この例は次のようになるかもしれません：

```rust
fn three_vowels(word: &str) -> bool {
    let mut vowel_count = 0;
    for c in word.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowel_count += 1;
                if vowel_count >= 3 {
                    return true;
                }
            }
            _ => vowel_count = 0,
        }
    }
    false
}

fn main() {
    let sentence_string =
        "Once upon a time, there was a friendly curious crab named Ferris".to_string();
    for word in sentence_string.split(' ') {
        if three_vowels(word) {
            println!("{word} has three consecutive vowels!");
        }
    }
}
```

引数型`&str`で宣言された関数を使用してこの例を実行すると、次が得られます

```bash
curious has three consecutive vowels!
```

しかし、関数が引数型`&String`で宣言されている場合、この例は実行されません。
これは、文字列スライスが`&str`であり、`&String`ではないためです。`&String`への変換には
割り当てが必要で、これは暗黙的ではありません。一方、`String`から`&str`への変換は
安価で暗黙的です。

## 関連項目

- [Rust言語リファレンス - 型強制](https://doc.rust-lang.org/reference/type-coercions.html)
- `String`と`&str`の扱い方についてのさらなる議論については、Herman J. Radtke IIIによる
  [このブログシリーズ (2015)](https://web.archive.org/web/20201112023149/https://hermanradtke.com/2015/05/03/string-vs-str-in-rust-functions.html)を参照
- [Steve Klabnikのブログポスト「いつStringと&strを使うべきか？」](https://archive.ph/LBpD0)
