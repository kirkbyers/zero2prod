use crate::{
    db,
    routes::{health_check, subscribe},
};

use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};

pub async fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    db::new_local_db().await.expect("Failed to create database");
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
