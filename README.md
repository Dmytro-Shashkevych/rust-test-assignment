# rust-test-assignment

# Kraken Last Traded Price (LTP) API

This project implements a simple API service in Rust using Actix-web to fetch the Last Traded Price (LTP) of Bitcoin for three currency pairs (BTC/USD, BTC/CHF, BTC/EUR) from the Kraken exchange.

### Installation

1. Clone this repository:
```sh
git clone https://github.com/Dmytro-Shashkevych/rust-test-assignment

cd rust-test-assignment
 ```
2. Build and run the project:
```sh
cargo run
 ```
The API server will start running at http://0.0.0.0:8080/api/v1/ltp

### Usage

To fetch the Last Traded Price (LTP) for BTC/USD, BTC/CHF, and BTC/EUR pairs:

 ```sh
curl http://0.0.0.0:8080/api/v1/ltp
 ```

The response: 

 ```
{
  "ltp": [
    {
      "pair": "BTC/USD",
      "amount": "63741.30000"
    },
    {
      "pair": "BTC/CHF",
      "amount": "58262.70000"
    },
    {
      "pair": "BTC/EUR",
      "amount": "59593.50000"
    }
  ]
}
 ```

### Docker

You can also run the project using Docker:

1. Build the Docker image
```sh
docker build -t rust-test-assignment .
```

2. Run the Docker container
```sh
docker run -p 8080:8080 rust-test-assignment
```

