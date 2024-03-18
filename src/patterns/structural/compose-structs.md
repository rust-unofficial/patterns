# Struct decomposition for independent borrowing

## Description

Sometimes a large struct will cause issues with the borrow checker - although
fields can be borrowed independently, sometimes the whole struct ends up being
used at once, preventing other uses. A solution might be to decompose the struct
into several smaller structs. Then compose these together into the original
struct. Then each struct can be borrowed separately and have more flexible
behaviour.

This will often lead to a better design in other ways: applying this design
pattern often reveals smaller units of functionality.

## Example

Here is a contrived example of where the borrow checker foils us in our plan to
use a struct:

```rust
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
    print_database(&db); // Immutable borrow of `db` happens here
                         // *connection_string = "new string".to_string();  // Mutable borrow is used
                         // here
}
```

We can apply this design pattern and refactor `Database` into three smaller
structs, thus solving the borrow checking issue:

```rust
// Database is now composed of three structs - ConnectionString, Timeout and PoolSize.
// Let's decompose it into smaller structs
#[derive(Debug, Clone)]
struct ConnectionString(String);

#[derive(Debug, Clone, Copy)]
struct Timeout(u32);

#[derive(Debug, Clone, Copy)]
struct PoolSize(u32);

// We then compose these smaller structs back into `Database`
struct Database {
    connection_string: ConnectionString,
    timeout: Timeout,
    pool_size: PoolSize,
}

// print_database can then take ConnectionString, Timeout and Poolsize struct instead
fn print_database(connection_str: ConnectionString, timeout: Timeout, pool_size: PoolSize) {
    println!("Connection string: {connection_str:?}");
    println!("Timeout: {timeout:?}");
    println!("Pool size: {pool_size:?}");
}

fn main() {
    // Initialize the Database with the three structs
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

## Motivation

This pattern is most useful, when you have a struct that ended up with a lot of
fields that you want to borrow independently. Thus having a more flexible
behaviour in the end.

## Advantages

Decomposition of structs lets you work around limitations in the borrow checker.
And it often produces a better design.

## Disadvantages

It can lead to more verbose code. And sometimes, the smaller structs are not
good abstractions, and so we end up with a worse design. That is probably a
'code smell', indicating that the program should be refactored in some way.

## Discussion

This pattern is not required in languages that don't have a borrow checker, so
in that sense is unique to Rust. However, making smaller units of functionality
often leads to cleaner code: a widely acknowledged principle of software
engineering, independent of the language.

This pattern relies on Rust's borrow checker to be able to borrow fields
independently of each other. In the example, the borrow checker knows that `a.b`
and `a.c` are distinct and can be borrowed independently, it does not try to
borrow all of `a`, which would make this pattern useless.
