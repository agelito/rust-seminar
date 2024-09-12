---
title: 4. Prepare Server Routes
---

# 4. Prepare Server Routes

Let's implement scaffolding for all the server routes. The `/api/v1/status` route is already
completed from the previuous page.

| Endpoint | Description |
| --- | --- |
| `GET /api/v1/status` | Health check endpoint. |
| `GET /api/v1/movies` | Returns a list of all movies. |
| `GET /api/v1/movies/{:id}` | Returns a single movie by ID. |
| `PUT /api/v1/movies/{:id}` | Updates a single movie by ID. |
| `DELETE /api/v1/movies/{:id}` | Delete a movie by ID. |
| `POST /api/v1/movies` | Create a new movie. |

## Add endpoint functions

Add the following functions below the `get_status` function. The `todo!()` macro will cause a panic
if we try to call any of these functions but it also suppress any linter warnings and errors due to
missing return statement.

```rust
async fn get_movies() -> impl IntoResponse {
    todo!()
}

async fn get_movie() -> impl IntoResponse {
    todo!()
}

async fn update_movie() -> impl IntoResponse {
    todo!()
}

async fn delete_movie() -> impl IntoResponse {
    todo!()
}

async fn create_movie() -> impl IntoResponse {
    todo!()
}
```

## Update the router

Update the `Router` boilerplate to include the following routes. Remember you may have to add use
statements for the http verb functions `put`, `post`, `delete`:

```rust
 let router = Router::new()
     .route("/api/v1/status", get(get_status))
     .route("/api/v1/movies", get(get_movies))
     .route("/api/v1/movies/:id", get(get_movie))
     .route("/api/v1/movies/:id", put(update_movie))
     .route("/api/v1/movies/:id", delete(delete_movie))
     .route("/api/v1/movies", post(create_movie));
```

## Testing

If we try to call any of these functions the server will panic with `not yet implemented` because of
the `todo!()` macros. Note that the server keeps running since the panic occurred inside one of the
tokio worker threads.

```sh
❯ curl localhost:8000/api/v1/movies
curl: (52) Empty reply from server

❯ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running `E:\Projects\rust-seminar\target\debug\api-server.exe`
thread 'tokio-runtime-worker' panicked at api-server\src/main.rs:31:5:
not yet implemented
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
