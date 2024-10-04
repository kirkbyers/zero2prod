use crate::{
    db::start_db,
    models::{scrape::get_page, scrape_embeddings},
};
use anyhow::Result;
use fastembed::{InitOptions, TextEmbedding};
use uuid::Uuid;

pub async fn main() -> Result<()> {
    let db = start_db().await.unwrap();
    let conn = db.connect().unwrap();

    let model = TextEmbedding::try_new(InitOptions {
        show_download_progress: true,
        ..Default::default()
    })?;
    let limit = 10;
    let mut page = 0;
    let mut scrapes = get_page(conn.clone(), limit, limit * page, true)
        .await
        .unwrap();
    while !scrapes.is_empty() {
        println!(
            "Processing {:?} scrapes. Ids {:?}",
            scrapes.len(),
            scrapes
                .iter()
                .map(|f| f.id.to_string())
                .collect::<Vec<String>>()
        );
        let docs = scrapes.iter().map(|f| f.content.to_string()).collect();
        let embeddings = model.embed(docs, None)?;
        for (idx, scrape) in scrapes.iter().enumerate() {
            let res_embedding: Vec<u8> = embeddings[idx]
                .iter()
                .flat_map(|f| f.to_ne_bytes().to_vec())
                .collect();
            let new_fast_embed = scrape_embeddings::ScrapeEmbedding {
                id: Uuid::new_v4().to_string(),
                scrape_id: scrape.id.to_string(),
                embedding: res_embedding.clone(),
            };
            let _ = new_fast_embed.insert(&conn).await;
        }
        page += 1;
        scrapes = get_page(conn.clone(), limit, limit * page, true)
            .await
            .unwrap();
    }
    Ok(())
}
