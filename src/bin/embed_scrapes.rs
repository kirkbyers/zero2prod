use zero2prod::{configuration::get_configuration, db::local_db, models::get_page};

#[tokio::main]
async fn main() {
    let config =
        get_configuration(Some("configuration.yaml")).expect("Failed to read configuration.");
    let db = local_db(&config.database.local_file_path).await.unwrap();
    let _conn = db.connect().unwrap();

    let scrapes = get_page(_conn, 10, 0).await.unwrap();
    for scrape in scrapes {
        println!("{:?}", scrape);
    }
}
