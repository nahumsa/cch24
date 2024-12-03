use axum::{
    body::Body,
    extract::Query,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use serde::Deserialize;
use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Deserialize)]
pub struct IPParam<T> {
    from: T,
    key: T,
}

#[derive(Deserialize)]
pub struct KeyParam<T> {
    from: T,
    to: T,
}

fn operate_over_ipv4<F>(ip1: Ipv4Addr, ip2: Ipv4Addr, operation: F) -> Ipv4Addr
where
    F: Fn(u8, u8) -> u8,
{
    let result: [u8; 4] = ip1
        .octets()
        .iter()
        .zip(ip2.octets())
        .map(|(&a, b)| operation(a, b))
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

pub async fn get_to_ipv4(ip_param: Query<IPParam<Ipv4Addr>>) -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(
            operate_over_ipv4(ip_param.from, ip_param.key, |x: u8, y: u8| {
                x.wrapping_add(y)
            })
            .to_string(),
        ))
        .unwrap()
}

pub async fn get_key_ipv4(ip_param: Query<KeyParam<Ipv4Addr>>) -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(
            operate_over_ipv4(ip_param.to, ip_param.from, |x: u8, y: u8| x.wrapping_sub(y))
                .to_string(),
        ))
        .unwrap()
}

pub async fn get_to_ipv6(ip_param: Query<IPParam<Ipv6Addr>>) -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(
            xor_ipv6(ip_param.from, ip_param.key).to_string(),
        ))
        .unwrap()
}

pub async fn get_key_ipv6(ip_param: Query<KeyParam<Ipv6Addr>>) -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(xor_ipv6(ip_param.to, ip_param.from).to_string()))
        .unwrap()
}
