use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post, put};
use axum::Router;
use tokio::net::TcpListener;

mod movie_repository;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("couldn't create tcp listener");

    let router = Router::new()
        .route("/api/v1/status", get(get_status))
        .route("/api/v1/movies", get(get_movies))
        .route("/api/v1/movies/:id", get(get_movie))
        .route("/api/v1/movies/:id", put(update_movie))
        .route("/api/v1/movies/:id", delete(delete_movie))
        .route("/api/v1/movies", post(create_movie));

    axum::serve(listener, router)
        .await
        .expect("couldn't serve api");
}

async fn get_status() -> impl IntoResponse {
    (StatusCode::OK, "SERVER_OK")
}

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
