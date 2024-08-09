# Rust Seminar

This project contains presentation material and some Rust example projects.

## Dependencies

This repository uses `doctave` to build or serve the presentation page. Doctave can be installed using `cargo`. 

```sh
cargo install --git https://github.com/Doctave/doctave --tag 0.4.2
```

## Presenting the Seminar

Use the `doctave` tool to serve the presentation website.

```sh
doctave serve
```

## Running the Examples

```sh
cargo run --bin calculator
cargo run --bin hello-world
```
