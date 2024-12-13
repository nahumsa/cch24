use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};

pub fn get_routes() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/-1/seek", get(redirect))
}

pub async fn hello_world() -> &'static str {
    "Hello, bird!"
}

pub async fn redirect() -> impl IntoResponse {
    (
        StatusCode::FOUND,
        [(
            header::LOCATION,
            "https://www.youtube.com/watch?v=9Gc4QTqslN4",
        )],
    )
}
