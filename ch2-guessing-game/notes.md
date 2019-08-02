# Guessing Game

---

## Using Standard Libraries

By default, Rust brings only a few types into the scope of every program in _[the prelude](https://doc.rust-lang.org/stable/std/prelude/index.html)_. If a type you want to use isn't in the prelude, you have to bring that type into scope with a `use` statement, like so:

`use std::io;`

The `std::io` library provides several in/output features.


---

## Intro to Variables, Associated Functions, and Expecting Errors

Variables are declared like so:

```rust
let foo = 5;  //immutable
let mut bar = 5;  //mutable
let mut guess = String::new;
```

By default, variables in Rust are _immutable_. As such, if you wish to change a variable in the future you must specify `let mut` instead of just `let`. 

`::` denotes an _associated function_. An associated function is implemented on a type (in this case the `String` type) rather than on an individual instance of an object. In some languages this is called a _static method_.

```rust
io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
```

After calling `use std::io`, we have quick access to `io`'s associated functions. Without `use`, this call also could have been made with `std::io::stdin`, but in an application where we are using `std::io` many times, we want to avoid repeating ourselves unnecessarily.

`read_line` takes whatever the user types and places it into a string. Pass an object in as a parameter to store the input string into it.

`&` indicates that an argument is a _reference_, which allows us to give "multiple parts of our code access to one piece of data without needing to copy that data into memory multiple times." Like variables, references are immutable by default - hence `&mut guess` rather than just `&guess`.

`read_line` will also return a value - in this case, an `io::Result`. Result types are _[enumerations](https://doc.rust-lang.org/stable/book/ch06-00-enums.html)_, which are a type that can have a fixed set of values, called the enum's _variants_.

For `Result`, the variants are `Ok` or `Err`. The `Ok` variant indicates the operation was successful, and inside `Ok` is the successfully generated value. `Err` means it failed, and contains information about how or why. In the code above, `.expect` will handle the `Err` by printing out "Failed to read line".

---

## String Placeholders

```rust
println!("You guessed: {}", guess);
```

Instead of interpolation, Rust has _placeholders_, which will then be filled in, from left to right, by the parameters that are passed in next.

```rust
let x = 5;
let y = 10;
println!("x = {} and y = {}", x, y)
```
`=> x = 5 and y = 10`

---

## Using Crates

To import and use a crate, we need to edit our _Cargo.toml_ file, like so:

```rust
[dependencies]

rand = "0.3.14"
```

On run of `cargo build`, cargo will fetch the latest versions of every listed dependency from the registry, which is a copy of data from [Crates.io](https://crates.io/). Crates.io is Rust's equivalent of npm.

Cargo will only use the specified versions of dependencies until indicated otherwise. Cargo will reference the `Cargo.lock` file to determine which versions of dependencies to use

If you _do_ want to update a crate, you can do so with

`cargo update`

Which will ignore the `Cargo.lock` file and figure out all the latest versions that fit the specifications in `Cargo.toml`. If that works, Cargo will write those versions to `Cargo.lock`.

---

## Using rand::Rng

```rust
let secret_number = rand::thread_rng().gen_range(1, 101);
```

`gen_range` takes two arguments and generates a random number between them. It is inclusive on the lower bound, but exlusive on the upper bound, so we need to pass in `101` to get a number between 1 and 100.

---

## Comparing Numbers using  std::cmp::Ordering and `match`

```rust
use std::cmp::Ordering;

match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small"),
    Ordering::Greater => println!("Too big"),
    Ordering::Equal => println!("You win")
}
```

Like `Result`, `Ordering` is another enum, but the variants for `Ordering` are `Less`, `Greater`, and `Equal`. These are the possible outcomes when comparing two values.

We use a `match` expression to decide what to do based on which variant of `Ordering` was returned from calling `cmp` to compare `guess` against a reference to `secret_number`. 

A `match` expression is made up for _arms_. An arm consists of a _pattern_ and the code that should be run if the value given to the beginning of the `match` fit's that arm's pattern. Rust takes the value given to match and looks through each arm in turn.

---

## Changing Strings to Nums

```rust
let mut guess = String::new();
let guess: u32 = guess.trim().parse()
    .expect("Please type a number!");
```

Even though we already created a variable called `guess`, Rust allows us to _shadow_ the previous value of `guess` with a new one. This is generally used in a situation in which you want to convert a value from one type to another. Shadowing lets us reuse the `guess` variable name instead of forcing us to create two unique variables.

`.trim()` will cut off all white space, including a `\n` that gets added to the string when the user presses enter to submit their text. `parse()` on a string will turn it into some kind of number. `:` after a variable name denotes that we will be specifying the variable's type. There are a few different types of numbers in Rust, so we specify `guess: u32` to make it an unsigned, 32-bit integer.

---

## Intro to Looping

```rust
loop {
    // snip
    break;
}
```

`loop` will initiate an infinite loop, and must be broken forcefully.

---

## Handling Invalid Input

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
}
```

Switching from an `expect` call to a match expression is how you generally move from crashing on an error to handling the error. `parse` returns a `Result` type and `Result` is an enum that has the variants `Ok` or `Err`. We're using a `match` here, as we did with the `Ordering` result of `cmp`.

If `parse` is able to successfully turn the string into a number, it will return an `Ok` value that contains the resulting number. That `Ok` value will match the first arm's pattern, and the `match` expression will just return the `num` value that `parse` produced and placed inside `Ok`.

If `parse` is _not_ able to turn the string into a number, it will return an `Err` value that contains more information about the error. The `Err` value doesn't match the `Ok(num)` pattern in the first match arm, but it does match the `Err(_)` pattern in the second arm. 

`_` is a catchall value; in the example above, we're saying we want to match all `Err` values. So the program will execute the second arm's code, `continue`, which tells the program to go to the next iteration of the loop, skipping all errors.
