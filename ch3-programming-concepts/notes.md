# Ch3 - Programming Concepts

---

[3.1 - Variables and Mutability](#variables-and-mutability)

- [Variables vs Constants](#variables-vs-constants)
- [Shadowing](#shadowing)

[3.2 - Data Types](#data-types)

- [Scalar Types](#scalar-types)
  - [Integers](#integer-types)
  - [Floats](#floating-point-types)
  - [Basic Math](#numeric-operations)
  - [Booleans](#booleans)
  - [Characters](#the-character-type)

- [Compound Types](#compound-types)
  - [Tuples](#the-tuple-type)
  - [Arrays](#arrays)

[3.3 - Functions](#functions)

- [Function Parameters](#function-parameters)
- [Statements vs Expressions](#statements-and-expressions)
- [Functions with Return Values](#functions-with-return-values)
- [Comments](#comments)
- [Control Flow](#control-flow)

[3.4 - Control Flow](#control-flow)

- [`if` Expressions](#if-expressions)
  - [Using `if` with `let`](#using-if-in-a-let-statement)
- [Loops](#repetition-with-loops)
  - [`loop` loops](#loop)
  - [`while` loops](#conditional-loops-with-while)
  - [`for` loops](#looping-through-a-collection-with-for)

[Exercises](#all-done)

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
`+=`
`-`
`-=`
`*`
`/`
`%`

Rust does _not_ support the `++` expression.


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

Declare a new function with the `fn` keyword.

Rust code uses `snake_case` as the conventional style for function and variable names.

Rust functions are enclosed in `{curly brackets}`

Rust doesn't care where you define your functions; you can reference all functions within your code regardless of location.

### Function Parameters

A parameter's type _must_ be declared in a function declaration. Separate multiple parameter declarations with commas:

```rust
    fn main(){
        my_function(5, 6.987);
    }

    fn my_function(x: i32, y: f64){
        println!("The value of x is {}", x);
        println!("The value of y is {}", y);
    }
```

Attempting to declare `fn my_function(x){ ... }` will prevent x from being initialized.

> Requiring type annotations in function definitions means the compiler almost never needs you to use them elsewhere in the code to figure out what you mean.

### Statements and Expressions

Function bodies are made up of a series of statements optionally ending in an expression. So far, we've only covered functions without an ending expression, but we have seen an expression as part of a statement.

Rust is an _expression-based language_, so this is an important distinction to understand.

_*Statements*_ are instructions that perform some action and do not return a value.
_*Expressions*_ return a value.

```rust
fn main(){
    let y = 6;
}
```

Variable and function declarations are statements. Statements do not return values. Therefore, you can't assign a `let` statement to another variable - ex: `let x = (let y = 6);`. This would produce an error.

Since the statement `let y = 6` does not return a value, there is nothing for x to bind to. This is different from other languages (C or Ruby for example), where variable assignment returns the value of the assignment.

_Expressions_ evaluate to something and make up most of the code that we will write in Rust. A simple example of an expression would be a basic math operation: `5 + 6`, which evaluates to `11`.

Expressions can be _part_ of a statement. In `let y = 6`, `6` is an expression that evaluates to the value `6`. Calling a function is an expression. Calling a macro is an expression. The block that we use to create new scopes, `{}`, is an expression. For example:

```rust
fn main(){
    let x = 5;

    let y = {
        let x = 3
        x + 1
    };

    println!("The value of y is {}", y);
}
```

The expression being assigned to our new variable, `y`, evaluates to `4`. Note that the `x + 1` line ends without a semicolon, which is unlike most lines we've seen thus far. Expressions do not include ending semicolons.

> If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value.

### Functions with Return Values

Functions can return values to the code that calls them. We don't name return values, but we do declare their type after an arrow (`->`).

In Rust, the return value of a function is the value of the final expression in the body of that function.

You can return early from a function by using the `return` keyword and specifying a value, but most functions return the last expression implicitly. Here's an example:

```rust
fn five() -> i32 {
    5
}

fn main(){
    let x = five();

    println!("The value of x is: {}", x);
}
```

There are no function calls, macros, `let` statements in the `five` function -- just the number 5 by itself. That's a perfectly valid function in Rust.

Note that:

- the function's return type is specified as `-> i32`.
- the final expression within `five()` ends _without_ a semicolon, indicating that we want to return the value of that expression

Let's look at another example:

```rust
fn main() {
    let x = plus_one(5)

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

Running this code will print `The value of x is: 6`. But if we placed a semicolon at the end of `x + 1`, we would change that line from an _expression_ to a _statement_, which would result in a `mismatched types` error on compile.

The definition of `plus_one()` says that it will return an `i32`, but statements don't evaluate to a value. The compiler will state that it expected an `i32` but found `()`, an empty tuple. Since nothing is being returned, the function definition is contradicted and the result is an error.

### Comments

In Rust, comments must start with `//` and continue to the end of the line. For multi-line comments, you'll need to include `//` on each line.

Comments can have their own lines, or be in-line with other code, but _everything_ AFTER `//` on that line will be considered a comment.

---

## Control Flow

_Control Flow_ is a term for deciding when to run code based on conditions. If/else statements, switch statements, and loops are the most common methods of control flow.

### `if` Expressions

```rust
fn main() {
    let number = 3;

    if number == 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

Blocks of code associated with the conditions in `if` expressions are sometimes called _arms_, just like the arms in `match` expressions that were discussed in Chapter 2.

Conditions _must_ be a `bool`. Languages like Ruby or JavaScript will try to convert non-Booleans, but Rust will not. 

Evaluators in Rust are similar to other languages: `=`, `<=`, `>=`, `==`, `!=`, `>`, `<`. Triple equal - `===` - does not exist in Rust and will error at compile time. `else` and `else if` expressions are optional of course.

Rust will stop checking `else if` statements as soon as it encounters one that is true. For example, in the following code, only the first `println!()` will print to the console:

```rust
fn main() {
    let num = 6;

    if num/2 > 2 {
        println!("The number is greater than 4");
    } else if num % 2 == 0 {
        println!("The number is even");
    } else {
        println!("The number is odd, and less than 4");
    }
}
```

> Using too many `else if` expressions can clutter your code, so if you have more than one, you might want to refactor your code. Consider a `match` statement for these cases (more on `match` in Chapter 6)

#### Using `if` in a `let` Statement

`if` is an _expression_. Because of this, we can use it on the right side of a `let` statement, like so:

```rust
fn main(){
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
```

Because `number` in the above block has the potential to be any of the values output by the expression, all possible outcomes must be of the same type. The `else` block, for example, could NOT be `"six"`. This would cause a compile-time error.

### Repetition with Loops

Rust has three kinds of loops: `loop`, `while`, and `for`.

#### `loop`

The `loop` keyword tells Rust to execute a block of code over and over forever or until you explicitly tell it to stop. You can place the `break` keyword within the loop to tell the program when to stop executing it.

#### Conditional Loops with `while`

```rust
fn main(){
    let mut num = 3;

    while num != 0 {
        println!("{}!", num);

        num -= 1;
    }

    println!("LIFTOFF!!");
}
```

> This construct eliminates a lot of nesting that would be necessary if you used `loop`, `if`, `else`, and `break`, and it’s clearer. While a condition holds true, the code runs; otherwise, it exits the loop.

#### Looping Through a Collection with `for`

`for` loops in Rust look like this:

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter(){
        println!("the value is: {}", element);
    }
}
```

`for` loops are the most commonly used loop construct in Rust due to their safety and conciseness.

We can rewrite the countdown loop from the last section with a `for` loop using a Range and the `rev()` method, like so:

```rust
fn main(){
    for num in (1..4).rev() {
        println!("{}!," num);
    }
    
    println!("LIFTOFF!");
}
```

---

## ALL DONE

Try the following exercises to reinforce some of the concepts learned in this chapter:

- Convert temperatures between Fahrenheit and Celsius
- Generate the nth Fibonacci number
- Print the lyrics to the Christmas carol "The Twelve Days of Christmas"
