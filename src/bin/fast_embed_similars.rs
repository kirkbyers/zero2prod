use dotenvy::dotenv;
use zero2prod::{db::start_db, models::scrape_embeddings};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect("No .env file found");

    let db = start_db().await.unwrap();
    let conn = db.connect().unwrap();

    let limit = 10;
    let mut page: u32 = 0;

    let mut fast_embed_recs = scrape_embeddings::get_page(&conn, &limit, &(limit * page))
        .await
        .unwrap();
    let mut all_fast_embeds: Vec<scrape_embeddings::ScrapeEmbedding> = vec![];
    while !fast_embed_recs.is_empty() {
        all_fast_embeds.append(&mut fast_embed_recs);
        page += 1;
        fast_embed_recs = scrape_embeddings::get_page(&conn, &limit, &(limit * page))
            .await
            .unwrap();
    }

    for i in 0..all_fast_embeds.len() {
        let mut max_similarity: f64 = 0.0;
        let mut similar_index = i;
        for j in 0..all_fast_embeds.len() {
            if i == j {
                continue;
            }

            let similarity =
                cosine_similarity(&all_fast_embeds[i].embedding, &all_fast_embeds[j].embedding);

            if similarity > max_similarity {
                max_similarity = similarity;
                similar_index = j;
            }

            // If similarity is 1 then there is a dup in the set and its pointless to continue checking the record
            if max_similarity >= 1.0 {
                break;
            }
        }
        println!(
            "{:?} - {max_similarity:} - {:?}",
            all_fast_embeds[i].scrape_id, all_fast_embeds[similar_index].scrape_id
        );
    }

    Ok(())
}

fn cosine_similarity(vec1: &[u8], vec2: &[u8]) -> f64 {
    let dot_product = dot_product(vec1, vec2);
    let magnitude1 = magnitude(vec1);
    let magnitude2 = magnitude(vec2);
    dot_product / (magnitude1 * magnitude2)
}

fn dot_product(vec1: &[u8], vec2: &[u8]) -> f64 {
    let mut result = 0.0;
    for i in 0..vec1.len() {
        result += (vec1[i] as f64) * (vec2[i] as f64);
    }
    result
}

fn magnitude(vec: &[u8]) -> f64 {
    let mut result = 0.0;
    for i in vec {
        result += (*i as f64) * (*i as f64);
    }
    result.sqrt()
}
