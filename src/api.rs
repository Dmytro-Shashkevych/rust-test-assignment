use actix_web::{get, HttpResponse, Responder};
use serde::Serialize;

use crate::kraken::fetch_ltp;

#[derive(Serialize)]
struct Pair {
    pair: String,
    amount: Option<String>,
}

#[derive(Serialize)]
struct LTPResponse {
    ltp: Vec<Pair>,
}

#[get("/ltp")]
pub async fn get_ltp() -> impl Responder {
    let pairs = vec![
        ("BTC/USD".to_string(), fetch_ltp("BTCUSD").await.unwrap_or(None)),
        ("BTC/CHF".to_string(), fetch_ltp("BTCCHF").await.unwrap_or(None)),
        ("BTC/EUR".to_string(), fetch_ltp("BTCEUR").await.unwrap_or(None)),
    ];

    let ltp: Vec<Pair> = pairs
        .into_iter()
        .map(|(pair, amount)| Pair { pair, amount })
        .collect();

    HttpResponse::Ok().json(LTPResponse { ltp })
}
