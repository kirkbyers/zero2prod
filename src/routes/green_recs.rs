use crate::models::scrape;
use actix_web::{error::ErrorInternalServerError, post, web, Error, HttpResponse};
use fastembed::{InitOptions, TextEmbedding};
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Deserialize)]
pub struct GreenRecData {
    description: String,
}

#[derive(Serialize, Debug)]
pub struct GreenRecRes {
    pub id: String,
    pub url: String,
    pub similarity: f32,
}

#[derive(Deserialize)]
pub struct GreenRecQuery {
    similarity: Option<SimilarityOptions>,
}

#[post("/green_recs")]
pub async fn make_green_rec(
    query: web::Query<GreenRecQuery>,
    json: web::Json<GreenRecData>,
    conn: web::Data<libsql::Connection>,
) -> Result<HttpResponse, Error> {
    let fast_embed_model = TextEmbedding::try_new(InitOptions {
        show_download_progress: true,
        ..Default::default()
    }).map_err(|e| ErrorInternalServerError(e))?;

    let input_embedding = fast_embed_model.embed(vec![&json.description], Some(1))
        .map_err(|e| ErrorInternalServerError(e))?;

    let scrapes = scrape::get_page(conn.get_ref().clone(), 150, 0, false)
        .await
        .unwrap();

    let similarity_option = match &query.similarity {
        Some(similarity) => similarity,
        None => &SimilarityOptions::Cosine,
    };
    let closest_scrapes = find_closest_similarity(input_embedding[0].clone(), scrapes, similarity_option);

    Ok(HttpResponse::Ok().json(closest_scrapes))
}

#[derive(Deserialize)]
enum SimilarityOptions {
    Cosine,
    Euclidean,
    Manhattan,
}

fn find_closest_similarity(
    inp: Vec<f32>,
    scrapes: Vec<scrape::ScrapeRow>,
    similarity_option: &SimilarityOptions,
) -> Vec<GreenRecRes> {
    let mut result: Vec<GreenRecRes> = Vec::new();
    let similarity_fn = match similarity_option {
        SimilarityOptions::Cosine => cosine_similarity,
        SimilarityOptions::Euclidean => euclidean_distance,
        SimilarityOptions::Manhattan => manhattan_distance,
    };
    for scrape in scrapes {
        let similarity = similarity_fn(
            &inp,
            match &scrape.embedding.clone() {
                Some(embedding) => embedding,
                None => continue,
            },
        );
        result.push(GreenRecRes {
            id: scrape.id,
            url: scrape.url,
            similarity,
        });
    }
    result.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap());
    result.truncate(5);

    result
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot_product = a.iter().zip(b.iter()).map(|(a, b)| a * b).sum::<f32>();
    let norm_a = a.iter().map(|a| a * a).sum::<f32>().sqrt();
    let norm_b = b.iter().map(|b| b * b).sum::<f32>().sqrt();
    dot_product / (norm_a * norm_b)
}

fn euclidean_distance(a: &[f32], b: &[f32]) -> f32 {
    a.iter()
        .zip(b.iter())
        .map(|(a, b)| (a - b) * (a - b))
        .sum::<f32>()
        .sqrt()
}

fn manhattan_distance(a: &[f32], b: &[f32]) -> f32 {
    a.iter()
        .zip(b.iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<f32>()
}
