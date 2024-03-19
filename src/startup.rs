use crate::{
    db,
    routes::{health_check, subscribe},
};

use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};

pub async fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let db = db::local_db().await.expect("Failed to create database");
    let connection = web::Data::new(db.connect().expect("Failed to connect to database"));
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
