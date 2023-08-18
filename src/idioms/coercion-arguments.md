# Use borrowed types for arguments

## Description

Using a target of a deref coercion can increase the flexibility of your code
when you are deciding which argument type to use for a function argument. In
this way, the function will accept more input types.

This is not limited to slice-able or fat pointer types. In fact, you should
always prefer using the **borrowed type** over **borrowing the owned type**.
Such as `&str` over `&String`, `&[T]` over `&Vec<T>`, or `&T` over `&Box<T>`.

Using borrowed types you can avoid layers of indirection for those instances
where the owned type already provides a layer of indirection. For instance, a
`String` has a layer of indirection, so a `&String` will have two layers of
indirection. We can avoid this by using `&str` instead, and letting `&String`
coerce to a `&str` whenever the function is invoked.

## Example

For this example, we will illustrate some differences for using `&String` as a
function argument versus using a `&str`, but the ideas apply as well to using
`&Vec<T>` versus using a `&[T]` or using a `&Box<T>` versus a `&T`.

Consider an example where we wish to determine if a word contains three
consecutive vowels. We don't need to own the string to determine this, so we
will take a reference.

The code might look something like this:

```rust
fn three_vowels(word: &String) -> bool {
    let mut vowel_count = 0;
    for c in word.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowel_count += 1;
                if vowel_count >= 3 {
                    return true
                }
            }
            _ => vowel_count = 0
        }
    }
    false
}

fn main() {
    let ferris = "Ferris".to_string();
    let curious = "Curious".to_string();
    println!("{}: {}", ferris, three_vowels(&ferris));
    println!("{}: {}", curious, three_vowels(&curious));

    // This works fine, but the following two lines would fail:
    // println!("Ferris: {}", three_vowels("Ferris"));
    // println!("Curious: {}", three_vowels("Curious"));

}
```

This works fine because we are passing a `&String` type as a parameter. If we
remove the comments on the last two lines, the example will fail. This is
because a `&str` type will not coerce to a `&String` type. We can fix this by
simply modifying the type for our argument.

For instance, if we change our function declaration to:

```rust, ignore
fn three_vowels(word: &str) -> bool {
```

then both versions will compile and print the same output.

```bash
Ferris: false
Curious: true
```

But wait, that's not all! There is more to this story. It's likely that you may
say to yourself: that doesn't matter, I will never be using a `&'static str` as
an input anyways (as we did when we used `"Ferris"`). Even ignoring this special
example, you may still find that using `&str` will give you more flexibility
than using a `&String`.

Let's now take an example where someone gives us a sentence, and we want to
determine if any of the words in the sentence contain three consecutive vowels.
We probably should make use of the function we have already defined and simply
feed in each word from the sentence.

An example of this could look like this:

```rust
fn three_vowels(word: &str) -> bool {
    let mut vowel_count = 0;
    for c in word.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowel_count += 1;
                if vowel_count >= 3 {
                    return true
                }
            }
            _ => vowel_count = 0
        }
    }
    false
}

fn main() {
    let sentence_string =
        "Once upon a time, there was a friendly curious crab named Ferris".to_string();
    for word in sentence_string.split(' ') {
        if three_vowels(word) {
            println!("{} has three consecutive vowels!", word);
        }
    }
}
```

Running this example using our function declared with an argument type `&str`
will yield

```bash
curious has three consecutive vowels!
```

However, this example will not run when our function is declared with an
argument type `&String`. This is because string slices are a `&str` and not a
`&String` which would require an allocation to be converted to `&String` which
is not implicit, whereas converting from `String` to `&str` is cheap and
implicit.

## See also

- [Rust Language Reference on Type Coercions](https://doc.rust-lang.org/reference/type-coercions.html)
- For more discussion on how to handle `String` and `&str` see
  [this blog series (2015)](https://web.archive.org/web/20201112023149/https://hermanradtke.com/2015/05/03/string-vs-str-in-rust-functions.html)
  by Herman J. Radtke III
