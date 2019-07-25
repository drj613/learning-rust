# Ch3 - Programming Concepts

    [3.1 - Variables and Mutability](#variables-and-mutability)
        [Variables vs Constants](#variables-vs-constants)
        [Shadowing](#shadowing)
    [3.2 - Data Types](#data-types)

---

## Variables and Mutability

- Variables are immutable by default.

To make a variable mutable, add `mut` to its initialization, like so:

```rust
    fn main(){
        let mut x = 5;
        println!("The value of x is {}", x);
        x = 6
        println!("The value of x is {}", x);
    }
```

`=> The value of x is 5`
`=> The value of x is 6`

---

### Variables vs Constants

- Constants aren't just immutable by default, they are _always_ immutable

- Declare constants using the `const` keyword instead of the `let` keyword

- You _must_ declare the type of the value for constants (e.g. `const VALUE: u32 = 168`)

- Constants may be set only to a constant expression, not the result of a function call or any other value that could only be computed at runtime

- Rust's naming convention for constants it to use all UPPERCASE_WITH_UNDERSCORES between words. Underscores can also be inserted in numeric literals to improve readability:

```rust
const MAX_POINTS: u32 = 100_000;
```

- Naming hardcoded values used throughout a program as constants is useful in converying the meaning of that value to future maintainers of the code

---

### Shadowing

> You can declare a new variable with the same name as a previous variable, and the new variable will shadow the previous variable. The first variable is _shadowed_ by the second. When the variable is called, the value of the most recent shadow will be used


```rust
fn main() {
    let x = 5;

    let x = x + 1;
    // x = 5 + 1
    let x = x * 2;
    // x = (5 + 1) * 2
    println!("The value of x is: {}", x);
}
```

`=> The value of x is 12`

> Shadowing is different from marking a variable as `mut`, because we'll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword
> By using `let`, we can perform transformations on a value but have the variable be immutable after those transformations have been completed
> The other difference between `mut` and shadowing is that because we're creating a new variable when we use the `let` keyword again, we can change the _type_ of the value but reuse the same name.

```rust
let spaces = "    ";
let spaces = spaces.len();
```

This is allowed because shadowing creates a brand-new variable with the same name rather than changing the value of the `spaces` variable. If we tried to use `mut` for this, as shown here, we'd get a mismatched types error at compile time:

```rust
let mut spaces = "   ";
spaces = spaces.len();
```

> Shadowing thus spares us from having to come up with different names, such as `spaces_str` and `spaces_num`; instead we can reuse the simpler `spaces` name.

---

## Data Types
