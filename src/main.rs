mod api;
mod kraken;

use actix_web::web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(web::scope("/api/v1").service(api::get_ltp))
    })
        .bind("0.0.0.0:8080")?
        .run();

    println!("Server running at http://0.0.0.0:8080/api/v1/ltp");
    server.await
}
