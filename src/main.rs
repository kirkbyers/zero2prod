use actix_web::rt;
use dotenvy::dotenv;
use std::{env, net::TcpListener};

use zero2prod::{jobs::process::process_job, startup::run};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().expect("No .env file found");

    let application_port = env::var("APPLICATION_PORT").unwrap_or("8000".to_string());

    let address = format!("0.0.0.0:{}", application_port);
    println!("Listening on port {application_port}");
    let listener = TcpListener::bind(address)?;

    rt::spawn(async move {
        loop {
            dotenv().expect("No .env file found");
            process_job().await.expect("Failed to process job");
            tokio::time::sleep(std::time::Duration::from_secs(30)).await;
        }
    });

    run(listener).await?.await
}
