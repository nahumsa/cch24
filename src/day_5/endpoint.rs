use std::fmt;

use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;
use axum_extra::headers::ContentType;
use axum_extra::TypedHeader;
use cargo_manifest::Manifest;
use serde::Deserialize;
use serde_with::serde_as;

pub fn get_routes() -> Router {
    Router::new().route("/5/manifest", post(manifest_validation))
}

#[derive(Default, Deserialize)]
struct Metadata {
    #[serde(default)]
    pub orders: Vec<Order>,
}

#[serde_as]
#[derive(Deserialize, Debug)]
struct Order {
    pub item: String,
    #[serde_as(deserialize_as = "serde_with::DefaultOnError")]
    #[serde(default)]
    pub quantity: Option<u32>,
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.quantity {
            Some(num) => write!(f, "{}: {:?}", self.item, num),
            None => write!(f, ""),
        }
    }
}

pub async fn manifest_validation(
    TypedHeader(content_type_header): TypedHeader<ContentType>,
    body: String,
) -> impl IntoResponse {
    let content_type = content_type_header.to_string();

    let manifest = match content_type.as_str() {
        "application/toml" => {
            if let Ok(manifest) = toml::from_str::<Manifest<Metadata>>(&body) {
                manifest
            } else {
                return (axum::http::StatusCode::BAD_REQUEST, "Invalid manifest").into_response();
            }
        }
        "application/json" => {
            if let Ok(manifest) = serde_json::from_str::<Manifest<Metadata>>(&body) {
                manifest
            } else {
                return (axum::http::StatusCode::BAD_REQUEST, "Invalid manifest").into_response();
            }
        }
        "application/yaml" => {
            if let Ok(manifest) = serde_yaml::from_str::<Manifest<Metadata>>(&body) {
                manifest
            } else {
                return (axum::http::StatusCode::BAD_REQUEST, "Invalid manifest").into_response();
            }
        }
        _ => return (axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE,).into_response(),
    };

    let package = manifest.package.unwrap();

    if !package
        .keywords
        .and_then(|k| k.as_local())
        .map(|k| k.contains(&"Christmas 2024".to_string()))
        .unwrap_or_default()
    {
        return (
            axum::http::StatusCode::BAD_REQUEST,
            "Magic keyword not provided",
        )
            .into_response();
    }

    let metadata = match package.metadata {
        Some(metadata) => metadata,
        None => return (axum::http::StatusCode::NO_CONTENT,).into_response(),
    };

    let formatted_orders: String = metadata
        .orders
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
