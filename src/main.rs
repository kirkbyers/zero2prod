use actix_web::rt;
use dotenvy::dotenv;
use std::net::TcpListener;

use zero2prod::{configuration::get_configuration, jobs::process::process_job, startup::run};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let config =
        get_configuration(Some("configuration.yaml")).expect("Failed to read configuration.");
    dotenv().expect("No .env file found");
    let address = format!("0.0.0.0:{}", config.application_port);
    let listener = TcpListener::bind(address)?;

    rt::spawn(async move {
        loop {
            process_job().await.expect("Failed to process job");
            tokio::time::sleep(std::time::Duration::from_secs(30)).await;
        }
    });

    run(listener).await?.await
}
