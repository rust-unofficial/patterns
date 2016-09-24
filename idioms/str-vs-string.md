# Using &str instead of &String

## Description

It is usually recommended to prefer a `&str` type over a `&String` type for arguments.  In most cases `&str` has more flexibility.  An `&String` will coerce to a `&str` type when required -- however, the oppositive is not true.  

## Example

Let's look at a few examples.  Consider an example where we wish to determine if a word contains three consecutive vowels.  We certainly don't need to own the string to determine this so we will take a reference.  The code might look something like this:

``` rust
fn three_vowels(word: &String) -> bool {
    let mut vowel_count = 0;
    for char in word.chars() {
        match char {
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
```

This example will work fine, as shown here:

``` rust
fn main() {
    let ferris = "Ferris".to_string();
    let curious = "Curious".to_string();
    println!("{}: {}", ferris, three_vowels(&ferris));
    println!("{}: {}", curious, three_vowels(&curious));
}
```

which will print

``` bash
Ferris: false
Curious: true
```

However, by using a `&String` type in our arguent we will find the following example fails:

``` 
println!("Ferris: {}", three_vowels("Ferris"));
```

This example fails because a `&str` type will not coerce to a `&String` type.  We can fix this by simply modifying the type for our argument.  For instance, if we change our function declaration to:

``` rust
fn three_vowels(word: &str) -> bool {
```

then the previous example will now print

``` bash
Ferris: false
```

But wait, that's not all!  There is more to this story.  It's likely that you may say to yourself: that doesn't matter, I will never be using a `&'static str` as an input anways (as we did when we used `"Ferris"`).  Even ignoring this special example, you may still find that using `&str` will give you more flexibility than using a `&String`.  Let's now take an example where someone gives us a sentence, and we want to determine if any of the words in the sentence has a word that contains three consecutive vowels.  We probably should make use of the function we have already defined and simply feed in each word from the sentence.  An example of this could look like this:

``` rust
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

Running this example using our function declared with an argument type `&str` will yield

``` bash
curious has three consecutive vowels!
```

However, this example will not run when our function is declared with an argument type `&String`.  This is because string slices are a `&str` and not a `&String`.  One way you can think about why this is the case: to have a `&String` you will need a reference to a `String` object which requires three things: a pointer to your data, the length of your data, and how much capacity you have left to insert new data.  This last entry doesn't make much sense when you are talking about string slices. A string slices only cares about a window of your data, needing only: a pointer to your data, and the length of your data slice.

## See also

For more discussion on how to handle `String` and `&str` see [this blog series](http://hermanradtke.com/2015/05/03/string-vs-str-in-rust-functions.html) by Herman J. Radtke III.
