use ::shuttlings_cch24::day_minus_1::{hello_world, redirect};
use axum::routing::post;
use axum::{routing::get, Router};
use shuttlings_cch24::day_2::{get_key_ipv4, get_key_ipv6, get_to_ipv4, get_to_ipv6};
use shuttlings_cch24::day_5::manifest_validation;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/seek", get(redirect))
        .route("/2/dest", get(get_to_ipv4))
        .route("/2/key", get(get_key_ipv4))
        .route("/2/v6/dest", get(get_to_ipv6))
        .route("/2/v6/key", get(get_key_ipv6))
        .route("/5/manifest", post(manifest_validation));

    Ok(router.into())
}
