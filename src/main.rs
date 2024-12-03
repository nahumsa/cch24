use std::net::{Ipv4Addr, Ipv6Addr};

use axum::{
    body::Body,
    extract::Query,
    http::{header, Response, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use serde::Deserialize;

async fn hello_world() -> &'static str {
    "Hello, bird!"
}

async fn redirect() -> impl IntoResponse {
    (
        StatusCode::FOUND,
        [(
            header::LOCATION,
            "https://www.youtube.com/watch?v=9Gc4QTqslN4",
        )],
    )
}

#[derive(Deserialize)]
struct IPParam {
    from: String,
    key: String,
}

#[derive(Deserialize)]
struct KeyParam {
    from: String,
    to: String,
}

fn add_ipv4_octets(ip1: Ipv4Addr, ip2: Ipv4Addr) -> Ipv4Addr {
    let result: [u8; 4] = ip1
        .octets()
        .iter()
        .zip(ip2.octets())
        .map(|(a, b)| a.wrapping_add(b))
        .collect::<Vec<u8>>()
        .try_into()
        .expect("expected a 4-byte tuple");

    Ipv4Addr::from(result)
}

fn subtract_ipv4_octets(ip1: Ipv4Addr, ip2: Ipv4Addr) -> Ipv4Addr {
    let result: [u8; 4] = ip1
        .octets()
        .iter()
        .zip(ip2.octets())
        .map(|(a, b)| a.wrapping_sub(b))
        .collect::<Vec<u8>>()
        .try_into()
        .expect("expected a 4-byte tuple");

    Ipv4Addr::from(result)
}

fn xor_ipv6(ip1: Ipv6Addr, ip2: Ipv6Addr) -> Ipv6Addr {
    let result: [u8; 16] = ip1
        .octets()
        .iter()
        .zip(ip2.octets().iter())
        .map(|(&a, &b)| a ^ b)
        .collect::<Vec<u8>>()
        .try_into()
        .expect("expected a 16-byte tuple");

    Ipv6Addr::from(result)
}

async fn get_to_ipv4(ip_param: Query<IPParam>) -> impl IntoResponse {
    let from = match ip_param.from.parse::<Ipv4Addr>() {
        Ok(from) => from,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    let key = match ip_param.key.parse::<Ipv4Addr>() {
        Ok(from) => from,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(add_ipv4_octets(from, key).to_string()))
        .unwrap()
}

async fn get_key_ipv4(ip_param: Query<KeyParam>) -> impl IntoResponse {
    let from = match ip_param.from.parse::<Ipv4Addr>() {
        Ok(from) => from,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    let to = match ip_param.to.parse::<Ipv4Addr>() {
        Ok(from) => from,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(subtract_ipv4_octets(to, from).to_string()))
        .unwrap()
}

async fn get_to_ipv6(ip_param: Query<IPParam>) -> impl IntoResponse {
    let from = match ip_param.from.parse::<Ipv6Addr>() {
        Ok(from) => from,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    let key = match ip_param.key.parse::<Ipv6Addr>() {
        Ok(from) => from,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(xor_ipv6(from, key).to_string()))
        .unwrap()
}

async fn get_key_ipv6(ip_param: Query<KeyParam>) -> impl IntoResponse {
    let from = match ip_param.from.parse::<Ipv6Addr>() {
        Ok(from) => from,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    let to = match ip_param.to.parse::<Ipv6Addr>() {
        Ok(from) => from,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(xor_ipv6(to, from).to_string()))
        .unwrap()
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/seek", get(redirect))
        .route("/2/dest", get(get_to_ipv4))
        .route("/2/key", get(get_key_ipv4))
        .route("/2/v6/dest", get(get_to_ipv6))
        .route("/2/v6/key", get(get_key_ipv6));

    Ok(router.into())
}
