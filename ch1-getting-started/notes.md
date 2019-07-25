# Installation

`sudo curl https://sh.rustup.rs -sSf | sh`

This script automatically adds Rust to system PATH after next login. To start using Rust right away, run following command:

`source $HOME/.cargo/env`

To update or uninstall:

`rustup update`
`rustup self uninstall`

Installation includes a local version of the docs, run the following to see those:

`rustup doc`

To install Cargo (Rust build system and package manager),run:

`sudo apt-get cargo`

To verify Cargo installation and version, run:

`cargo --version` (Full word! no lazy `--v` allowed)

---

## Anatomy of a Rust Program

```rust
fn main() {
    println!("Hello world!");
}
```

- `snake_case` is the Rust naming convention
- Four spaces rather than one tab is the Rust spacing convention
- The `main` function is special: it gets run FIRST in every executable Rust program no matter what.
- Rust requires `{curly brackets}` around all function bodies.
- Using a `!` after a 'function' call means you're actually calling a `macro` rather than a function.
- Semicolons `;` end an expression. Most lines of Rust code end with a semicolon

---

## Compiling and Running

> Before running a Rust program, you must compile it using the Rust compiler by entering the rustc command and passing it the name of your source file, like this:

`rustc main.rs`

After compiling successfully, Rust outputs a binary executable.

`Dynamic` languages (like Ruby, Python, JS) do not compile and run in separate steps. Rust is an _ahead-of-time compiled_ language. The executable that is generated after compiling can be sent to and run by any machine, whether it has Rust installed or not! The previously mentioned languages would require those languages to be installed locally to run.

---

## Cargo Basics

With Cargo installed, running 

`cargo new {project_name}`

will create a directory, `{project_name}`, inside of which will be a `src` directory (containing the `main.rs` file), and a `Cargo.toml` file.

Inside of the project directory, running

`cargo build`

in the projecct directory will compile the project, and place the executable in `./target/debug/{project_name}`. This will also create a `Cargo.lock` file (which keeps track of the versions of dependencies in the project) at the root directory of the project.

Running

`cargo run`

will compile and immediately run the generated executable.

Running

`cargo check`

will check the code to make sure it compiles, but not actually compile into an executable. `check` is much faster than `build`, and will allow you to more quickly check your work throughout the process of coding.

*TOML* (_Tom's Obvious, Minimal Language_) is Cargo's configuration format. `Cargo.toml` will contain config information for compiling the project, as well as a list of dependencies.

In Rust, packages of code are referred to as `crates`

Cargo expects all of the source files to stay in the `/src`. The root directory of the project is for READMEs, license information, config files, and anything else not related to the code.

Cargo doesn't provide a lot of value over just using `rustc` in simple projects, but will prove its worth as programs become more intricate.

---

## Building for Release

> When your project is finally ready for release, you can use `cargo build --release` to compile it with optimizations. This command will create an executable in `/target/release` instead of `/target/debug`. The optimizations make your Rust code run faster, but turning them on lengthens the time it takes for your program to compile. This is why there are two different profiles: one for development, when you want to rebuild quickly and often, and another for building the final program you’ll give to a user that won’t be rebuilt repeatedly and that will run as fast as possible. If you’re benchmarking your code’s running time, be sure to run `cargo build --release` and benchmark with the executable in `/target/release`.

---

## SUMMARY

- Installing Rust and Cargo
- `rustup` commands for updating, viewing a local version of the language docs, and uninstalling
- Write, compile, and run a Hello, world! using `rustc`
- Creating, compiling, and running a project using Cargo
