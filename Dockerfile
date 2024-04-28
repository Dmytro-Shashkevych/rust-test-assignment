FROM rust

COPY ./ ./

RUN cargo build --release

EXPOSE 8080

CMD ["./target/release/price_fetching_service"]

