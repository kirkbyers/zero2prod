use crate::{
    db,
    routes::{
        get_jobs, get_scrapes, health_check_route, home, make_green_rec, start_job, subscribe,
    },
};
use actix_web::{dev::Server, web, App, HttpServer};
use std::net::TcpListener;

pub async fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let db = match db::start_db().await {
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
            .service(
                web::scope("/api")
                    .service(health_check_route)
                    .service(subscribe)
                    .service(make_green_rec)
                    .service(get_scrapes)
                    .service(web::scope("/jobs").service(get_jobs).service(start_job)),
            )
            .service(home)
            .app_data(connection_data.clone())
    })
    .listen(listener)?
    .run();
    Ok(app)
}
