---
title: 1. Project Setup
---

# 1. Project Setup

The first step is creating the project and adding some dependencies we will require. The project 
will depend on the `axum` and `tokio` libraries to implement the asynchronous web server.

## Creating the project

Using the `cargo` application we're creating a project of `bin` type.

```sh
cargo new --bin api-server
```

After running this command a new directory `api-server` is created. Inside it `cargo` generated
a basic hello world program. Enter the directory and try running the program to ensure project
was created properly.

```sh
cd api-server && cargo run
```

## Installing dependencies

We're going to install the following dependencies:

| Dependency | Description |
| --- | --- |
| tokio | Tokio is an async runtime engine for Rust applications. |
| axum | axum is a highly customizable web server framework. |
| serde | serde is a library for serializing and deserializing data |
| serde_json | serde_json allows us to serialize and deserialize json data with serde |
| tracing | tracing provides structured logging and traces support |
| tracing_subscriber | tracing_subscriber allows us displaying generated logs and traces |
| rand | random number generator |

Run the following cargo commands to install dependencies. Some of them require additional feature
flags so those add commands is done on separate lines.

```sh
cargo add tokio -F full
cargo add serde -F derive
cargo add tracing_subscriber -F env-filter
cargo add axum serde_json tracing rand
```


