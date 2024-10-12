use chrono::DateTime;
use libsql::{params, Connection, Error};

use crate::models::utils::create_paginator;

#[derive(Debug)]
pub struct ScrapeRow {
    pub id: String,
    pub url: String,
    pub content: String,
    pub scraped_at: chrono::DateTime<chrono::Utc>,
    pub embedding: Option<Vec<f32>>,
    pub region: String,
    pub processing: String,
    pub drying: String,
    pub arrival: String,
    pub lot_size: String,
    pub bag_size: String,
    pub packaging: String,
    pub farm_gate: String,
    pub cultivar_detail: String,
    pub grade: String,
    pub appearance: String,
    pub roast_rec: String,
    pub coffee_type: String,
    pub spro_rec: String,
}

impl ScrapeRow {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            url: String::new(),
            content: String::new(),
            scraped_at: chrono::Utc::now(),
            embedding: None,
            region: String::new(),
            processing: String::new(),
            drying: String::new(),
            arrival: String::new(),
            lot_size: String::new(),
            bag_size: String::new(),
            packaging: String::new(),
            farm_gate: String::new(),
            cultivar_detail: String::new(),
            grade: String::new(),
            appearance: String::new(),
            roast_rec: String::new(),
            coffee_type: String::new(),
            spro_rec: String::new(),
        }
    }
}

impl Default for ScrapeRow {
    fn default() -> Self {
        Self::new()
    }
}

pub fn select_with_pagination(
    columns: &str,
    q: &str,
    sort_by: &str,
    sort_direction: &str,
    limit: u32,
    offset: u32,
) -> String {
    create_paginator("scrapes")(columns, q, sort_by, sort_direction, limit, offset)
}

fn select_page(filter_non_null_embeddings: &bool, max_batch_id: &Option<&i32>) -> String {
    let mut where_stmts = vec![];
    let batch_id_str = match max_batch_id {
        Some(batch_id) => format!("s.batch_id = {}", batch_id).to_string(),
        None => "".to_string(),
    };

    let mut result = String::from(
        r#"
        SELECT s.id, s.url, s.content, s.scraped_at, se.embedding
        FROM scrapes AS s
        LEFT JOIN scrape_embeddings AS se
        ON se.scrape_id = s.id
    "#,
    );

    if *filter_non_null_embeddings {
        where_stmts.push("se.embedding IS NULL");
    }
    if !batch_id_str.is_empty() {
        where_stmts.push(&batch_id_str);
    }

    if !where_stmts.is_empty() {
        result.push_str(" WHERE ");
        result.push_str(&where_stmts.join(" AND "));
    }

    result.push_str(" ORDER BY scraped_at ASC LIMIT ? OFFSET ?");
    result
}

pub async fn get_page(
    conn: Connection,
    limit: u32,
    offset: u32,
    filter_non_null_embeddings: bool,
    max_batch_id: Option<&i32>,
) -> Result<Vec<ScrapeRow>, Error> {
    let mut stmt = conn
        .prepare(&select_page(&filter_non_null_embeddings, &max_batch_id))
        .await?;
    let mut rows = stmt.query((limit, offset)).await?;
    let scrapes: Vec<ScrapeRow> = rows_to_scrape_rows(&mut rows).await?;

    Ok(scrapes)
}

pub async fn rows_to_scrape_rows(rows: &mut libsql::Rows) -> Result<Vec<ScrapeRow>, Error> {
    let mut scrapes = Vec::new();

    while let Ok(Some(row)) = rows.next().await {
        let row = row;
        let id: String = row.get_str(0).unwrap().to_string();
        let url: String = row.get_str(1).unwrap().to_string();
        let content: String = row.get_str(2).unwrap().to_string();
        let scraped_at: chrono::DateTime<chrono::Utc> =
            DateTime::parse_from_rfc3339(row.get_str(3).unwrap())
                .unwrap()
                .into();
        let embedding_bytes: Option<Vec<u8>> = row.get::<Option<Vec<u8>>>(4).unwrap();
        let embedding: Option<Vec<f32>> = embedding_bytes.map(|bytes| {
            bytes
                .chunks_exact(4)
                .map(|b| f32::from_ne_bytes(b.try_into().unwrap()))
                .collect()
        });

        let mut result_row = ScrapeRow::new();
        result_row.id = id;
        result_row.url = url;
        result_row.content = content;
        result_row.scraped_at = scraped_at;
        result_row.embedding = embedding;
        scrapes.push(result_row);
    }

    Ok(scrapes)
}

pub async fn get_max_batch_id(conn: &Connection) -> Result<i32, Error> {
    let mut max_batch_id_rows = conn
        .query("SELECT COALESCE(MAX(batch_id), 0) FROM scrapes", params![])
        .await?;

    let max_batch_id: i32 = if let Ok(Some(row)) = max_batch_id_rows.next().await {
        row.get(0)?
    } else {
        0
    };

    Ok(max_batch_id)
}
