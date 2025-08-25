# Rust
[web version of rust book](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

## content of this repo
[variables](variables/): variables, mutability and data types operations  
[functions](functions/): some example functions with related syntax explanations    
[ownership](ownership/): deepdive into mutability, ownership and references  
[structenums](structenums/): deepdive into structs, enums and pattern matching    
[my_project](my_project/): example package structure

## rustup & toolchains
`rustup` is the Rust toolchain installer. It manages multiple Rust versions and associated tools.  
To update rustup and all installed toolchains, run: `rustup update`  

`rustc` is the Rust compiler. to compile a Rust program, use: `rustc <filename>.rs`, this produces an executable that can be run directly.

## Cargo 

| Command         | Description                                      |
|-----------------|--------------------------------------------------|
| `cargo build`   | Compiles the current package                     |
| `cargo run`     | Builds and runs the main binary of the package   |
| `cargo check`   | Quickly checks code to catch errors without producing an executable |
| `cargo test`    | Runs the tests for the package                    |
| `cargo new`     | Creates a new Cargo package                      |
| `cargo add <crate>` | Adds a dependency to the Cargo.toml file     |
| `cargo update`  | Updates dependencies as recorded in the Cargo.lock file |

Cargo is Rust’s build system and package manager.It handles building, running, testing, and managing dependencies for your Rust projects.  
The **default package index is** [crates.io](https://crates.io/).


## rust toolchain components

| Command                      | Description                        |
|------------------------------|------------------------------------|
| `rustup component add rustfmt`| Installs the rustfmt component     |
| `cargo fmt`                  | Formats the code in the workspace  |

Rust tools such as `rustfmt` are distributed as components, they can be installed via `rustup`.
You can add components to your toolchain using `rustup component add <component-name>`.  
For example, to enable automatic code formatting, install the `rustfmt` component as shown above.