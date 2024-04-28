use mockito::{mock, server_url};

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Client;
    use serde_json::{json, Value};
    use tokio::test;

    #[tokio::test]
    async fn test_mocked_api() {
        let _mock_server = mockito::start();

        let _mock = mock("GET", "/api/v1/ltp")
            .with_status(200)
            .with_body(r#"{"ltp": [{"pair": "BTC/USD", "amount": "50000.12"}]}"#)
            .create();

        let client = Client::new();
        let response = match client.get(&format!("{}/api/v1/ltp", server_url())).send().await {
            Ok(res) => res,
            Err(err) => panic!("Failed to send request: {}", err),
        };

        assert!(response.status().is_success());

        let body = match response.text().await {
            Ok(text) => text,
            Err(err) => panic!("Failed to read response body: {}", err),
        };

        let value: Value = serde_json::from_str(&body).unwrap();

        assert_eq!(
            value,
            json!({
                "ltp": [
                    {
                        "pair": "BTC/USD",
                        "amount": "50000.12"
                    }
                ]
            })
        );
    }
}
