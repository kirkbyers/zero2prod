use std::net::TcpListener;

use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config =
        get_configuration(Some("configuration.yaml")).expect("Failed to read configuration.");
    let address = format!("0.0.0.0:{}", config.application_port);
    let listener = TcpListener::bind(address)?;
    let db_path = config.database.local_file_path;
    run(listener, &db_path).await?.await
}
