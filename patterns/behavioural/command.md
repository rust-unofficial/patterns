# Command

## Description

Suppose we have a sequence of actions or transactions
encapsulated as objects.
We want these actions or commands to be
executed or invoked in some order later at different time.
These commands may also be triggered as a result
of some event. For example, when a user pushes a button,
or on arrival of a data packet.
In addition, these commands might be be undoable.
This may come in useful for operations of an editor.
We might want to store logs of executed commands
so that we could reapply the changes later if the system crashes.

## Motivation

Define two database operations
`create table` and `add field`.
Each of these operations is a command
which knows how to undo the command, e.g.,
`drop table` and `remove field`.
When a user invokes a database migration
operation then each command is executed in the defined order,
and when the user invokes the rollback operation
then each undo operation is invoked in reverse order.

## First approach. Using trait objects

We define a common trait which encapsulates our command
with two operations `execute` and `rollback`. All command
`structs` must implement this trait.

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
            .rev()
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

## Second approach. Using function pointers

We could follow another solution by
creating each individual command as
a different function and store function pointers
to invoke these functions later at a different time.
Since function pointers implement all three traits
`Fn`, `FnMut`, and `FnOnce` we could as well pass and store
closures instead of function pointers.

```rust
type FnPtr<'a> = fn() -> &'a str;
struct Command<'a> {
    execute: FnPtr<'a>,
    rollback: FnPtr<'a>,
}

struct Schema<'a> {
    commands: Vec<Box<Command<'a>>>,
}

impl<'a> Schema<'a> {
    fn new() -> Self {
        Self { commands: vec![] }
    }
    fn add_migration(&mut self, execute: FnPtr<'a>, rollback: FnPtr<'a>) {
        self.commands.push(Box::new(Command { execute, rollback }));
    }
    fn execute(&self) -> Vec<&str> {
        self.commands.iter().map(|cmd| (cmd.execute)()).collect()
    }
    fn rollback(&self) -> Vec<&str> {
        self.commands
            .iter()
            .rev()
            .map(|cmd| (cmd.rollback)())
            .collect()
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

## Third approach. Using `Fn` trait objects

Finally, instead of defining a common command
trait we simply store each command implementing
`Fn` trait separately in vectors.

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

## Discussion

If our commands are small and may be defined
as functions or passed as a closure then the
second approach might be preferable since it
does not exploit dynamic dispatch.
But if our command is a whole struct with a bunch
of functions and variables defined as seperate module
then the first approach would be more suitable.
As an example, the first approach is used in
the [`actix`](https://actix.rs/) when we register handler function
for routes.

## See also

- [Command pattern](https://en.wikipedia.org/wiki/Command_pattern)
- [A powerful, pragmatic, and extremely fast web framework for Rust](https://actix.rs/)
