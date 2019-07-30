# Ch3 - Programming Concepts

---

[3.1 - Variables and Mutability](#variables-and-mutability)

    [Variables vs Constants](#variables-vs-constants)

    [Shadowing](#shadowing)

[3.2 - Data Types](#data-types)

    [Scalar Types](#scalar-types)

        [Integers](#integer-types)
        [Floats](#floating-point-types)
        [Basic Math](#numeric-operations)
        [Booleans](#booleans)
        [Characters](#the-character-type)

    [Compound Types](#compound-types)

        [Tuples](#the-tuple-type)
        [Arrays](#arrays)

[3.3 - Functions](#functions)

        

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

> Rust is a _statically typed language_, which means that it must know the types of all variables at compile time.

Every value in Rust of a certain _data type_, which tells Rust what kind of data is being specified so it knows how to work with that data. We'll look at two subsets of data types: `scalar` and `compound`.

The compiler can usually infer what type we want based on the value and how it's used, but in cases when many types are possible, we must add a type notation, like so:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

If we don't add the type annotation above, the compiler will display `error[E0282]: type annotation needed`.

---

### Scalar Types

> A _scalar_ type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

#### Integer Types

- Integers are whole numbers (no fractional/decimal component)


|  Length | Signed | Unsigned |
|:-------:|:------:|:--------:|
| 8-bit   |   i8   |    u8    |
| 16-bit  |   i16  |    u16   |
| 32-bit  |   i32  |    u32   |
| 64-bit  |   i64  |    u64   |
| 128-bit |  i128  |   u128   |
| arch    |  isize |   usize  |

- Each type of integer can either be signed or unsigned and has an explicit size.

- _Signed_ = negative or positive

- _Unsigned_ = all positive

> Each signed variant can store numbers from -(2^n - 1) to 2^n - 1 - 1 inclusive, where n is the number of bits that variant uses. So an i8 can store numbers from -(2^7) to 2^7 - 1, which equals -128 to 127
> Unsigned variants can store numbers from 0 to 2^n - 1, so a u8 can store numbers from 0 to 2^8 - 1, which equals 0 to 255.
> Additionally, the `isize` and `usize` types depend on the kind of computer your program is running on: 64 bits on a 64-bit architecture, and 32 bits on a 32-bit architecture.

If you don't know what kind or size of integer to use, Rust's defaults are a safe bet, and integer types default to `i32` - this type is usually the fastest, even on 64-bit systems. `isize` or `usize` are mainly used when indexing some sort of collection.

If you have a variable of type `u8`, it can hold values between 0 and 255. If you try to change the variable to a value outside that range, like 256, **_integer overflow_** will occur. Compiling in debug mode will cause a _panic_ (AKA 'program exits with an error') at runtime if this occurs. Compiling in release mode, Rust instead performs _two's complement wrapping_. Values greater than the max value that the type can hold will loop back around, starting at the lowest value the integer can hold. 

> In the case of a `u8`, 256 becomes 0, 257 becomes 1, and so on.


#### Floating-Point Types

Rust also has two primitive types for _floating-point numbers_ (numbers with decimal points). Rust's floating-point types are `f32` and `f64`, which are 32 and 64 bits in size, respectively.

Rust's float default type is `f64` because on modern CPUs it's roughly the same speed as `f32` but is capable of more precision.

```rust
fn main(){
    let x = 2.0 // f64

    let y: f32 = 3.0 // f32
}
```

`f32` is a single-precision float, `f64` has double precision.


#### Numeric Operations

Rust supports all of the usual mathematical operations you'd expect.

`+`
`-`
`*`
`/`
`%`

#### Booleans

As in most other languages, Booleans in Rust have two possible values: `true`/`false`. Booleans are one byte in size. The Boolean type in Rust is specified using `bool`. It's usually inferred by the compiler, but can be explicitly annotated like so:

`let f: bool = false;`

#### The Character Type

Rust has a data type specifically for characters: `char`. `char`literals are specified with single quotes, as opposed to string literals, which use double quotes.

```rust
fn main(){
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';
}
```

Rust's `char` type is four byte in size and represents a Unicode Scalar Value, which means it can represent foreign languages and accented letters as well as ASCII.

### Compound Types

_Compound types_ can group multiple values into one type. Rust has two primitive compount types: tuples and arrays.

#### The Tuple Type

A tuple is a general way of grouping together some number of other values with a variety of types into one compound type.

*_Tuples have a fixed length: once declared, they cannot grow or shrink in size_*

Create a tuple in Rust by writing a comma-separated list of values inside parentheses. Each position in the tuple has a type, and they don't have to match. Type annotation is optional.

```rust
fn main(){
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

To get individual values out of a tuple, we can use pattern matching to destructure a tuple value, like so:

```rust
fn main(){
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
```

_Destructuring_ breaks down the tuple into individual variables. Attempting to do so with the wrong number of variables will throw an error at compile-time.

In addition to destructuring through pattern matching, we can access a tuple elementy directly by using a `.` followed by the index of the value we want to access.

```rust
fn main(){
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_four = x.1;

    let one = x.2;
}
```

#### Arrays

Unlike a tuple, every element of an array must have the same type. Arrays in Rust are different from arrays in some other languages because *_arrays in Rust have a fixed length, like tuples_*.

Arrays are written the same way in Rust as in most other languages: `let a = [1, 2, 3, 4, 5];`

Explicitly annotate variable type for an array like so:

```rust
    let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Here, `i32` is the type of each element. After the semicolon, `5` indicates that the array will have 5 items. This looks similar to an alternative syntax for creating an array: if you want to create an array that contains the same value for each element, you can specify the inital value, followed by a semicolon, and then the length of the array in square brackets:

```rust
    let a = [3; 5];
```

The array named `a` will contain `5` elements that will all be set to the value `3`. This is a more concise way to write `let a [3, 3, 3, 3, 3];`

Access elements of an array as in other languages: `arr[i]`. Index starts at 0.

If you attempt to access an index that is out of bounds of the array, code will compil but will exit with an error when it runs. In many low-level languages, this kind of check is not done.

---

## Functions


