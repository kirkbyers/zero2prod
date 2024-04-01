use actix_web::rt;
use std::net::TcpListener;

use zero2prod::{configuration::get_configuration, startup::run};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    rt::spawn(async {
        loop {
            println!("Hello from Tokio!");
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    });

    let config =
        get_configuration(Some("configuration.yaml")).expect("Failed to read configuration.");
    let address = format!("0.0.0.0:{}", config.application_port);
    let listener = TcpListener::bind(address)?;
    let db_path = config.database.local_file_path;
    run(listener, &db_path).await?.await
}
