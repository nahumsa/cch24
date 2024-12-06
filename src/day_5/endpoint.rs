use axum::response::IntoResponse;
use axum_extra::headers::ContentType;
use axum_extra::TypedHeader;

use crate::day_5::manifest_parser::{Manifest, Order};

pub async fn manifest_validation(
    TypedHeader(content_type_header): TypedHeader<ContentType>,
    body: String,
) -> impl IntoResponse {
    let content_type = content_type_header.to_string();
    let mut return_body: Vec<Order> = Vec::new();

    if content_type.starts_with("application/toml") {
        match toml::from_str::<Manifest>(&body) {
            Ok(manifest) => match manifest.package.metadata {
                Some(metadata) => {
                    return_body.extend(metadata.orders.into_iter());
                }
                None => {
                    return (axum::http::StatusCode::NO_CONTENT,).into_response();
                }
            },

            Err(_) => {
                return (axum::http::StatusCode::NO_CONTENT,).into_response();
            }
        }
    } else {
        return (axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE,).into_response();
    }

    let formatted_orders: String = return_body
        .iter()
        .filter(|order| order.quantity.is_some())
        .map(|order| order.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    if formatted_orders.is_empty() {
        return (axum::http::StatusCode::NO_CONTENT,).into_response();
    }

    (axum::http::StatusCode::OK, formatted_orders).into_response()
}
