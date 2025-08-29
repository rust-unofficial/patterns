# 独立した借用のための構造体分解

## 説明

大きな構造体が借用チェッカーで問題を引き起こすことがあります - フィールドは独立して借用できますが、
時には構造体全体が一度に使用されることになり、他の使用を妨げることがあります。
解決策は、構造体をいくつかの小さな構造体に分解することです。
そして、これらを元の構造体に再構成します。
そうすることで、各構造体を個別に借用でき、より柔軟な動作が可能になります。

これは多くの場合、他の面でもより良い設計につながります：
このデザインパターンを適用すると、より小さな機能単位が明らかになることがよくあります。

## 例

以下は、借用チェッカーが構造体の使用計画を妨げる作為的な例です：

```rust,ignore
struct Database {
    connection_string: String,
    timeout: u32,
    pool_size: u32,
}

fn print_database(database: &Database) {
    println!("Connection string: {}", database.connection_string);
    println!("Timeout: {}", database.timeout);
    println!("Pool size: {}", database.pool_size);
}

fn main() {
    let mut db = Database {
        connection_string: "initial string".to_string(),
        timeout: 30,
        pool_size: 100,
    };

    let connection_string = &mut db.connection_string;
    print_database(&db);
    *connection_string = "new string".to_string();
}
```

コンパイラは次のエラーを出力します：

```ignore
let connection_string = &mut db.connection_string;
                        ------------------------- 可変借用がここで発生
print_database(&db);
               ^^^ 不変借用がここで発生
*connection_string = "new string".to_string();
------------------ 可変借用が後でここで使用される
```

このデザインパターンを適用して、`Database`を3つの小さな構造体にリファクタリングすることで、
借用チェックの問題を解決できます：

```rust
// Databaseは3つの構造体 - ConnectionString、Timeout、PoolSizeで構成されるようになりました。
// より小さな構造体に分解しましょう
#[derive(Debug, Clone)]
struct ConnectionString(String);

#[derive(Debug, Clone, Copy)]
struct Timeout(u32);

#[derive(Debug, Clone, Copy)]
struct PoolSize(u32);

// これらの小さな構造体を`Database`に再構成します
struct Database {
    connection_string: ConnectionString,
    timeout: Timeout,
    pool_size: PoolSize,
}

// print_databaseは代わりにConnectionString、Timeout、PoolSize構造体を受け取れます
fn print_database(connection_str: ConnectionString, timeout: Timeout, pool_size: PoolSize) {
    println!("Connection string: {connection_str:?}");
    println!("Timeout: {timeout:?}");
    println!("Pool size: {pool_size:?}");
}

fn main() {
    // 3つの構造体でDatabaseを初期化
    let mut db = Database {
        connection_string: ConnectionString("localhost".to_string()),
        timeout: Timeout(30),
        pool_size: PoolSize(100),
    };

    let connection_string = &mut db.connection_string;
    print_database(connection_string.clone(), db.timeout, db.pool_size);
    *connection_string = ConnectionString("new string".to_string());
}
```

## 動機

このパターンは、独立して借用したい多くのフィールドを持つ構造体がある場合に最も有用です。
結果として、より柔軟な動作が可能になります。

## 利点

構造体の分解により、借用チェッカーの制限を回避できます。
そして、多くの場合、より良い設計が生まれます。

## 欠点

より冗長なコードになる可能性があります。
また、時には小さな構造体が良い抽象化ではなく、結果として悪い設計になることがあります。
これはおそらく「コードの匂い」であり、プログラムを何らかの方法でリファクタリングすべきことを示しています。

## 議論

このパターンは借用チェッカーを持たない言語では必要ないため、
その意味でRust独自のものです。しかし、より小さな機能単位を作ることは、
多くの場合よりクリーンなコードにつながります：これは言語に依存しない、
広く認められたソフトウェアエンジニアリングの原則です。

このパターンは、Rustの借用チェッカーがフィールドを互いに独立して借用できることに依存しています。
例では、借用チェッカーは`a.b`と`a.c`が別個であり、独立して借用できることを知っています。
`a`全体を借用しようとはしません。それではこのパターンは役に立たなくなってしまいます。