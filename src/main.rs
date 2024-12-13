use axum::Router;
use shuttlings_cch24::day_2::{self};
use shuttlings_cch24::day_5::{self};
use shuttlings_cch24::day_9::{self};
use shuttlings_cch24::day_minus_1;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .merge(day_2::get_routes())
        .merge(day_minus_1::get_routes())
        .merge(day_5::get_routes())
        .merge(day_9::get_routes());

    Ok(router.into())
}
