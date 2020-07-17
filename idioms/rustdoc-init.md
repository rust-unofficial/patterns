# Easy doc initialization

## Description

If a struct takes significant effort to initialize, when writing docs it can be quicker to wrap your example with a 
function the struct as an argument.

## Motivation
Sometimes there is a struct with multiple or complicated parameters and several methods.
Each of these methods should have examples. 

For example:

```rust
struct ExampleStruct {
    foo: Box<Foo>,
    count: AtomicI64,
}

impl ExampleStruct {
    /// Does something important.
    /// # Example
    /// ```
    /// # //This is some really borring boiler plate to get an example working.
    /// # let baz = Baz::new(Bat::new(0, 1), 2, "Foo".to_owned());
    /// # let foo = Box::new(Foo::new(baz));
    /// # let example_struct = ExampleStruct{ foo: foo, count: AtomicI64::new(3) };
    /// let result = example_struct.bar();
    /// // do stuff with result.
    /// ```
    fn bar() -> u64 {
        
    }
    
    /// Oh no all that boiler plate needs to be repeated here !!!
    fn other_method() {
    }
}
```

## Example
Instead of typing all of this boiler plate to create an `ExampleStruct` it is easier to just create a dummy method to pass one in:
```rust
struct ExampleStruct {
    foo: Box<Foo>,
    count: AtomicI64,
}
impl ExampleStruct {
    /// Does something important.
    /// # Example
    /// ```
    /// # fn call_bar(example_struct: ExampleStruct) {
    ///   let result = example_struct.bar();
    /// // do stuff with result.
    /// # }
}
```
## Advantages

This is much more concise and avoids repetitive code in examples.

## Disadvantages

Because the example is in a function, the code won't actually be tested. 
It still will be compiled when running a `cargo test` but assertions can't be used to verify properties.

## Discussion

If assertions are not required this pattern works well. 
If they are, an alternative can be to create a method to create a dummy instance which is annotated with:

`#[doc(hidden)]`

