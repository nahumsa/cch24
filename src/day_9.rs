use std::{ops::DerefMut, sync::Arc, time::Duration};

use axum::{
    body::Body,
    extract::{Request, State},
    http::{header::CONTENT_TYPE, Response, StatusCode},
    response::IntoResponse,
    Json, RequestExt,
};
use leaky_bucket::RateLimiter;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct UnitConverter {
    #[serde(default)]
    gallons: Option<f32>,
    #[serde(default)]
    liters: Option<f32>,
    #[serde(default)]
    pints: Option<f32>,
    #[serde(default)]
    litres: Option<f32>,
}

impl UnitConverter {
    pub fn is_valid(&self) -> bool {
        let mut counter = 0;
        if self.gallons.is_some() {
            counter += 1;
        }
        if self.liters.is_some() {
            counter += 1;
        }
        if self.litres.is_some() {
            counter += 1;
        }
        if self.pints.is_some() {
            counter += 1;
        }

        counter == 1
    }

    pub fn convert(&self) -> Result<UnitConverter, String> {
        if let Some(liter_qnt) = self.liters {
            let converted_gallons = liter_qnt * 0.264172;
            return Ok(UnitConverter {
                gallons: Some(converted_gallons),
                liters: None,
                pints: None,
                litres: None,
            });
        } else if let Some(gallon_qnt) = self.gallons {
            let converted_liters = gallon_qnt * 3.785412;
            return Ok(UnitConverter {
                gallons: None,
                liters: Some(converted_liters),
                pints: None,
                litres: None,
            });
        } else if let Some(litres_qnt) = self.litres {
            let converted_pints = litres_qnt * 1.759754;
            return Ok(UnitConverter {
                gallons: None,
                liters: None,
                pints: Some(converted_pints),
                litres: None,
            });
        } else if let Some(pints_qnt) = self.pints {
            let converted_litres = pints_qnt * 0.56826;
            return Ok(UnitConverter {
                gallons: None,
                liters: None,
                pints: None,
                litres: Some(converted_litres),
            });
        }

        Err("all variables are None".to_string())
    }
}

pub async fn leaky_milk(
    State(state): State<Arc<Mutex<RateLimiter>>>,
    req: Request,
) -> impl IntoResponse {
    let has_milk = state.lock().await.try_acquire(1);

    if !has_milk {
        println!("milk not available");
        return Response::builder()
            .status(StatusCode::TOO_MANY_REQUESTS)
            .body(Body::from("No milk available\n"))
            .unwrap();
    }

    println!("milk available");
    let content_type_header = req.headers().get(CONTENT_TYPE);
    let content_type = content_type_header
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");

    match content_type {
        "application/json" => {
            let Json(payload) = match req.extract::<Json<UnitConverter>, _>().await {
                Ok(res) => res,
                Err(_) => return (axum::http::StatusCode::BAD_REQUEST,).into_response(),
            };
            println!("{:?}", payload);

            if !payload.is_valid() {
                println!("{:?}", payload);
                println!("invalid payload");
                return (axum::http::StatusCode::BAD_REQUEST,).into_response();
            }

            println!("{:?}", payload.convert().unwrap());
            return (axum::http::StatusCode::OK, Json(payload.convert().unwrap())).into_response();
        }
        _ => println!("no content_type"),
    };

    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("Milk withdrawn\n"))
        .unwrap()
}

pub async fn refill_milk(State(state): State<Arc<Mutex<RateLimiter>>>) -> impl IntoResponse {
    let mut lock = state.lock().await;
    let deref_state = lock.deref_mut();

    *deref_state = RateLimiter::builder()
        .initial(5)
        .refill(1)
        .max(5)
        .interval(Duration::from_secs(1))
        .build();

    println!("refilled");

    (StatusCode::OK).into_response()
}
