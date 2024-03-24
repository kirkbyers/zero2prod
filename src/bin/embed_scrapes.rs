use zero2prod::{configuration::get_configuration, db::local_db};

#[tokio::main]
async fn main() {
    let config =
        get_configuration(Some("configuration.yaml")).expect("Failed to read configuration.");
    let db = local_db(&config.database.local_file_path).await.unwrap();
    let _conn = db.connect().unwrap();
}
