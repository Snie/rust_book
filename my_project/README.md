# my_project

This example demonstrates how to organize a Rust project using packages, crates, and modules.

## Structure

```
my_project/
├── Cargo.toml
├── src
│   ├── main.rs         # Binary crate root (entry point)
│   ├── lib.rs          # Library crate root
│   ├── garden.rs       # Top-level module for the library
│   └── garden
│       └── vegetables.rs  # Submodule of garden
```

- **Cargo.toml**: Project manifest, defines package metadata and dependencies.
- **src/main.rs**: Entry point for the binary crate. Uses the library crate.
- **src/lib.rs**: Root of the library crate. Declares and exposes modules.
- **src/garden.rs**: Declares the `vegetables` submodule.
- **src/garden/vegetables.rs**: Implements the `Asparagus` struct.

## Usage

- Run the binary:  
  `cargo run`
- Add more modules or binaries as your project grows.
- Use `pub` to expose items from modules as needed.

See [The Rust Book, Chapter 7](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html) for more details.
