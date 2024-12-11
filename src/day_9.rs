use std::sync::Arc;

use axum::{
    body::Body,
    extract::State,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use leaky_bucket::RateLimiter;

pub async fn leaky_milk(State(state): State<Arc<RateLimiter>>) -> impl IntoResponse {
    if state.try_acquire(1) {
        println!("milk available");
        return Response::builder()
            .status(StatusCode::OK)
            .body(Body::from("Milk withdrawn\n"))
            .unwrap();
    }

    println!("milk not available");
    Response::builder()
        .status(StatusCode::TOO_MANY_REQUESTS)
        .body(Body::from("No milk available\n"))
        .unwrap()
}
