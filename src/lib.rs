pub mod db;

use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
struct SubscriberData {
    email: String,
    name: String,
}

async fn subscribe(_json: web::Json<SubscriberData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
