---
title: 4. Rust Examples
---

# Rust Examples
I have included some example crates for this seminar. The examples can be found at the GitHub repository [rust-seminar](https://github.com/agelito/rust-seminar).

### hello-world
The `hello-world` crate is just a basic hello world application.

```sh
cargo run --bin hello-world
```

### my-library
The `my-library` crate is a library exposing some basic arithmetic functions. The library is imported and used by the `calculator` crate.

### calculator
The `calculator` crate is a CLI application which demonstrates using a crate from `crates.io`. It's also using the locally available `my-library` crate.

```sh
# Add 4 with 4
cargo run --bin calculator -- add 4 4

# Output:
# 4 + 4 = 8

# Print a help message
cargo run --bin calculator -- --help

# Output:
# A basic calculator app
#
# Usage: calculator.exe <COMMAND>
#
# Commands:
#   add       Add two numbers
#   multiply  Multiply two numbers
#   subtract  Subtract two numbers
#   divide    Divide two numbers
#   help      Print this message or the help of the given subcommand(s)
#
# Options:
#   -h, --help  Print help
```
