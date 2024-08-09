---
title: 2. Rust Crates
---

# Rust Crates

Rust projects are called `crates`. A crate can be either a binary application or a library. When developing projects using Rust installing publicly available crates is very common.

## Where to find Crates?
The official repository for Rust crates is [crates.io](https://crates.io/). The crates found here can be installed or upgraded using the `cargo` CLI utility. 

All published Rust crates follows the same standard documentation process and style. Documentation for crates and Rust itself can be found at [docs.rs](https://docs.rs/).

## Anatomy of a Crate
Every crate in Rust follows the same basic structure. The `cargo` utility will help setting up the structure when creating a new project.

```sh
cargo new --bin my-project
```

```
my-project
- src
  - main.rs
- Cargo.toml
- Cargo.lock
```

The example above is a binary application crate. All source code should be added to the `src` directory. The `Cargo.toml` contains crate properties and dependencies, the `Cargo.lock` contains the installed dependency versions and checksums.

## Modules
Each Rust crate can contain multiple modules. Modules can be defined in a few different ways.

### 1. As a file inside src

```
my-project
- src
  - main.rs (main module)
  - types.rs (types module)
```

In this example the rust compiler will treat `types.rs` as a separate module within the `my-project` crate. To ensure the `types` module is compiled it must be referenced from the `main` module.

```rust
// main.rs

// declares the types module
mod types;

fn main() {

}
```

### 2. As a directory inside src
```
my-project
- src
  - main.rs (main module)
  - types
    - mod.rs
```

In this example the `types` module is added as a separate directory. It's important the main module file inside the directory is called `mod.rs`, otherwise the compiler will not know where to find it.

### 3. As a block witin another file

Modules can also be defined inline inside files.

```rust
// main.rs

fn main() {
    my_module::bar();
}

mod my_module {
   pub fn bar() { } 
}
```

### Module Visibility
Visibility of modules is private by default. If a module should be publicly visible the `pub` keyword can be used.

```rust
pub mod types;
pub(crate) mod types;
```

### Using a module
To actually use a module the `use` keyword must be used to import the module.

```rust
// imports the `MyCustomType` type from `types` module
use types::MyCustomType;

// declares the types module
mod types;
```

Types can also be re-exported by using the `pub` keyword.

```rust
pub use types::MyCustomType;
```


