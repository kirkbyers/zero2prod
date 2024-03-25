use crate::{
    db,
    routes::{create_green_rec, health_check, subscribe},
};

use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};

pub async fn run(listener: TcpListener, db_path: &str) -> Result<Server, std::io::Error> {
    let db = match db::local_db(db_path).await {
        Ok(db) => db,
        Err(err) => {
            eprintln!("Failed to connect to database: {:?}", err);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to connect to database",
            ));
        }
    };

    let connection = match db.connect() {
        Ok(connection) => connection,
        Err(err) => {
            eprintln!("Failed to connect to database: {:?}", err);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to connect to database",
            ));
        }
    };
    let connection_data = web::Data::new(connection);
    let app = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .route("/green_recs", web::post().to(create_green_rec))
            .app_data(connection_data.clone())
    })
    .listen(listener)?
    .run();
    Ok(app)
}
