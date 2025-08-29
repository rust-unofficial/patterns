# コマンドパターン

## 説明

コマンドパターンの基本的な考え方は、アクションを独自のオブジェクトに分離し、
パラメータとして渡すことです。

## 動機

オブジェクトとしてカプセル化された一連のアクションまたはトランザクションがあるとします。
これらのアクションまたはコマンドを、異なる時刻に何らかの順序で実行または呼び出したいと考えています。
これらのコマンドは、何らかのイベントの結果としてもトリガーされる可能性があります。
例えば、ユーザーがボタンを押したとき、またはデータパケットが到着したときです。
さらに、これらのコマンドは元に戻すことができるかもしれません。
これは、エディタの操作に役立つ可能性があります。実行されたコマンドのログを保存して、
システムがクラッシュした場合に後で変更を再適用できるようにしたいかもしれません。

## 例

2つのデータベース操作`create table`と`add field`を定義します。
これらの操作はそれぞれ、コマンドを元に戻す方法を知っているコマンドです。
例えば、`drop table`と`remove field`です。
ユーザーがデータベースマイグレーション操作を呼び出すと、
各コマンドが定義された順序で実行され、ユーザーがロールバック操作を呼び出すと、
コマンド全体のセットが逆順で呼び出されます。

## アプローチ: トレイトオブジェクトを使用

2つの操作`execute`と`rollback`でコマンドをカプセル化する共通のトレイトを定義します。
すべてのコマンド`structs`はこのトレイトを実装する必要があります。

```rust
pub trait Migration {
    fn execute(&self) -> &str;
    fn rollback(&self) -> &str;
}

pub struct CreateTable;
impl Migration for CreateTable {
    fn execute(&self) -> &str {
        "create table"
    }
    fn rollback(&self) -> &str {
        "drop table"
    }
}

pub struct AddField;
impl Migration for AddField {
    fn execute(&self) -> &str {
        "add field"
    }
    fn rollback(&self) -> &str {
        "remove field"
    }
}

struct Schema {
    commands: Vec<Box<dyn Migration>>,
}

impl Schema {
    fn new() -> Self {
        Self { commands: vec![] }
    }

    fn add_migration(&mut self, cmd: Box<dyn Migration>) {
        self.commands.push(cmd);
    }

    fn execute(&self) -> Vec<&str> {
        self.commands.iter().map(|cmd| cmd.execute()).collect()
    }
    fn rollback(&self) -> Vec<&str> {
        self.commands
            .iter()
            .rev() // reverse iterator's direction
            .map(|cmd| cmd.rollback())
            .collect()
    }
}

fn main() {
    let mut schema = Schema::new();

    let cmd = Box::new(CreateTable);
    schema.add_migration(cmd);
    let cmd = Box::new(AddField);
    schema.add_migration(cmd);

    assert_eq!(vec!["create table", "add field"], schema.execute());
    assert_eq!(vec!["remove field", "drop table"], schema.rollback());
}
```

## アプローチ: 関数ポインタを使用

個々のコマンドを異なる関数として作成し、
異なる時刻にこれらの関数を後で呼び出すために関数ポインタを格納することで、
別のアプローチに従うことができます。
関数ポインタは3つのトレイト`Fn`、`FnMut`、`FnOnce`をすべて実装するため、
関数ポインタの代わりにクロージャを渡して格納することもできます。

```rust
type FnPtr = fn() -> String;
struct Command {
    execute: FnPtr,
    rollback: FnPtr,
}

struct Schema {
    commands: Vec<Command>,
}

impl Schema {
    fn new() -> Self {
        Self { commands: vec![] }
    }
    fn add_migration(&mut self, execute: FnPtr, rollback: FnPtr) {
        self.commands.push(Command { execute, rollback });
    }
    fn execute(&self) -> Vec<String> {
        self.commands.iter().map(|cmd| (cmd.execute)()).collect()
    }
    fn rollback(&self) -> Vec<String> {
        self.commands
            .iter()
            .rev()
            .map(|cmd| (cmd.rollback)())
            .collect()
    }
}

fn add_field() -> String {
    "add field".to_string()
}

fn remove_field() -> String {
    "remove field".to_string()
}

fn main() {
    let mut schema = Schema::new();
    schema.add_migration(|| "create table".to_string(), || "drop table".to_string());
    schema.add_migration(add_field, remove_field);
    assert_eq!(vec!["create table", "add field"], schema.execute());
    assert_eq!(vec!["remove field", "drop table"], schema.rollback());
}
```

## アプローチ: `Fn`トレイトオブジェクトを使用

最後に、共通のコマンドトレイトを定義する代わりに、
`Fn`トレイトを実装する各コマンドをベクトルに別々に格納することができます。

```rust
type Migration<'a> = Box<dyn Fn() -> &'a str>;

struct Schema<'a> {
    executes: Vec<Migration<'a>>,
    rollbacks: Vec<Migration<'a>>,
}

impl<'a> Schema<'a> {
    fn new() -> Self {
        Self {
            executes: vec![],
            rollbacks: vec![],
        }
    }
    fn add_migration<E, R>(&mut self, execute: E, rollback: R)
    where
        E: Fn() -> &'a str + 'static,
        R: Fn() -> &'a str + 'static,
    {
        self.executes.push(Box::new(execute));
        self.rollbacks.push(Box::new(rollback));
    }
    fn execute(&self) -> Vec<&str> {
        self.executes.iter().map(|cmd| cmd()).collect()
    }
    fn rollback(&self) -> Vec<&str> {
        self.rollbacks.iter().rev().map(|cmd| cmd()).collect()
    }
}

fn add_field() -> &'static str {
    "add field"
}

fn remove_field() -> &'static str {
    "remove field"
}

fn main() {
    let mut schema = Schema::new();
    schema.add_migration(|| "create table", || "drop table");
    schema.add_migration(add_field, remove_field);
    assert_eq!(vec!["create table", "add field"], schema.execute());
    assert_eq!(vec!["remove field", "drop table"], schema.rollback());
}
```

## 議論

コマンドが小さく、関数として定義されたり、クロージャとして渡されたりする場合、
動的ディスパッチを利用しないため、関数ポインタを使用することが好ましいかもしれません。
しかし、コマンドが分離されたモジュールとして定義された関数や変数の束を持つ
完全な構造体である場合、トレイトオブジェクトを使用する方が適しているでしょう。
アプリケーションの事例は[`actix`](https://actix.rs/)で見つけることができ、
ルートのハンドラ関数を登録する際にトレイトオブジェクトを使用しています。
`Fn`トレイトオブジェクトを使用する場合、
関数ポインタの場合と同じ方法でコマンドを作成して使用することができます。

パフォーマンスについて、パフォーマンスとコードの単純性および組織化の間には
常にトレードオフがあります。静的ディスパッチはより高速なパフォーマンスを提供し、
動的ディスパッチはアプリケーションを構造化する際の柔軟性を提供します。

## 参照

- [Command pattern](https://en.wikipedia.org/wiki/Command_pattern)

- [`command`パターンの別の例](https://web.archive.org/web/20210223131236/https://chercher.tech/rust/command-design-pattern-rust)
