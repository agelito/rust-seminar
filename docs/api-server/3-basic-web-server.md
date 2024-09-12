---
title: 3. Basic Web Server
---

# 3. Basic Web Server

Now that `tokio` is working it's time to initialize and run the `axum` server framework. There's
some boilerplate code to set up the tcp listener and router.

Replace the `main.rs` content with the following:

```rust
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("couldn't create tcp listener");

    let router = Router::new().route("/api/v1/status", get(get_status));

    axum::serve(listener, router)
        .await
        .expect("couldn't serve api");
}

async fn get_status() -> impl IntoResponse {
    (StatusCode::OK, "SERVER_OK")
}
```

Run the application using `cargo run` and then try sending a `GET` request to
`localhost:8000/api/v1/status`. The output should look like this:

```sh
❯ curl localhost:8000/api/v1/status
SERVER_OK
```
