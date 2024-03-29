use zero2prod::{
    configuration::get_configuration, db::local_db, models::sm_scrape::get_page,
    services::open_ai::OpenAI,
};

#[tokio::main]
async fn main() {
    let config =
        get_configuration(Some("configuration.yaml")).expect("Failed to read configuration.");
    let db = local_db(&config.database.local_file_path).await.unwrap();
    let conn = db.connect().unwrap();

    let open_ai = OpenAI::new();

    let mut scrapes = get_page(conn.clone(), 10, 0, true).await.unwrap();
    while !scrapes.is_empty() {
        for scrape in scrapes {
            let embedding = open_ai.string_to_embedding(&scrape.content).await.unwrap();
            let blob_embedding: Vec<u8> = embedding
                .iter()
                .flat_map(|f| f.to_ne_bytes().to_vec())
                .collect();
            conn.execute(
                "UPDATE sm_scrapes SET embedding = ? WHERE id = ?",
                (blob_embedding, scrape.id),
            )
            .await
            .unwrap();
        }
        scrapes = get_page(conn.clone(), 10, 0, true).await.unwrap();
    }
}
