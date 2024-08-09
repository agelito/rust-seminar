---
title: 1. Getting Started
---

# Getting Started

## The Rust Programming Language
The best way to get started with Rust is to read *The Rust Programming Language* book. The book is available for free online and provides a very good introduction to the language.

[The Rust Programming Language](https://doc.rust-lang.org/book/)

I recommend reading the book at a glance first, skipping over things that seem too complicated until feeling ready to learn them. Start making smaller applications, services, or tools while working through the book. Use the book as a reference for syntax or Rust programming concepts.

## Installing Rust
The recommended way to install the rust compiler and other associated tools is to use the `rustup` utility. After installed `rustup` can be used to upgrade, list, remove different versions of Rust.

See the [Installation](https://doc.rust-lang.org/book/ch01-01-installation.html) section of the book for more information.

## Cargo
The `rustup` utility will install the `Cargo` command line tool. `Cargo` is the build system and package manager of Rust.

### Creating Rust Projects
```sh
# Creates a new binary crate called `hello-world`
cargo new hello-world

# Creates a new library crate called `my-library`
cargo new --lib my-library
```

### Installing Dependencies
```sh
# Add the dependencies `axum` and `serde`
cargo add axum serde
```

### Running Project
```sh
# Run without arguments
cargo run

# Run with arguments
cargo run -- arg1 arg2
```

### Running Tests
```sh
cargo test
```

### Generate and Open Documentation
```sh
cargo doc --open
```

Read more about `Cargo` in the book at the [Cargo](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html) section.


