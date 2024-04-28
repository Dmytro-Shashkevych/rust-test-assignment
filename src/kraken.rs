use reqwest::Error;

pub async fn fetch_ltp(pair: &str) -> Result<Option<String>, Error> {
    let url = format!("https://api.kraken.com/0/public/Ticker?pair={}", pair);
    let response = reqwest::get(&url).await?;

    if !response.status().is_success() {
        return Ok(None);
    }

    let body = response.bytes().await?;
    let json_data: serde_json::Value = serde_json::from_slice(&body).unwrap_or_default();

    let mapped_pair = map_pair(pair);

    if let Some(result) = json_data.get("result") {
        if let Some(pair_data) = result.get(mapped_pair) {
            if let Some(c) = pair_data.get("c") {
                if let Some(price) = c.as_array().and_then(|arr| arr.get(0).and_then(|val| val.as_str())) {
                    return Ok(Some(price.to_string()));
                }
            }
        }
    }

    Ok(None)
}

fn map_pair(pair: &str) -> &str {
    match pair {
        "BTCUSD" => "XXBTZUSD",
        "BTCCHF" => "XBTCHF",
        "BTCEUR" => "XXBTZEUR",
        _ => pair,
    }
}
