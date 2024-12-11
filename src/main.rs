use std::sync::Arc;
use std::time::Duration;

use ::shuttlings_cch24::day_minus_1::{hello_world, redirect};
use axum::routing::post;
use axum::{routing::get, Router};
use leaky_bucket::RateLimiter;
use shuttlings_cch24::day_2::{get_key_ipv4, get_key_ipv6, get_to_ipv4, get_to_ipv6};
use shuttlings_cch24::day_5::manifest_validation;
use shuttlings_cch24::day_9::leaky_milk;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let limiter = Arc::new(
        RateLimiter::builder()
            .initial(5)
            .refill(1)
            .max(5)
            .interval(Duration::from_secs(1))
            .build(),
    );

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/seek", get(redirect))
        .route("/2/dest", get(get_to_ipv4))
        .route("/2/key", get(get_key_ipv4))
        .route("/2/v6/dest", get(get_to_ipv6))
        .route("/2/v6/key", get(get_key_ipv6))
        .route("/5/manifest", post(manifest_validation))
        .route("/9/milk", post(leaky_milk))
        .with_state(limiter);

    Ok(router.into())
}
