// tests/mockito_test.rs

use mockito::{mock, server_url};

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Client;
    use tokio::test;

    #[tokio::test]
    async fn test_mocked_api() {
        // Start the mock server
        let _mock_server = mockito::start();

        // Create a mock endpoint
        let _mock = mock("GET", "/api/v1/ltp")
            .with_status(200)
            .with_body(r#"{"ltp": [{"pair": "BTC/USD", "amount": "50000.12"}]}"#)
            .create();

        // Make a request to the mock server
        let client = Client::new();
        let response = match client.get(&format!("{}/api/v1/ltp", server_url())).send().await {
            Ok(res) => res,
            Err(err) => panic!("Failed to send request: {}", err),
        };

        // Assert the response status
        assert!(response.status().is_success());

        // Read the response body as a string
        let body = match response.text().await {
            Ok(text) => text,
            Err(err) => panic!("Failed to read response body: {}", err),
        };
        println!("Response body: {}", body);
    }
}
