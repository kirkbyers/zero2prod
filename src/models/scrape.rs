use chrono::DateTime;
use libsql::{Connection, Error};

#[derive(Debug)]
pub struct ScrapeRow {
    pub id: String,
    pub url: String,
    pub content: String,
    pub scraped_at: chrono::DateTime<chrono::Utc>,
    pub embedding: Option<Vec<f32>>,
}

fn select_page(where_cluase: bool) -> String {
    let mut result = String::from(
        r#"
        SELECT id, url, content, scraped_at, embedding
        FROM sm_scrapes
    "#,
    );
    if where_cluase {
        result.push_str(" WHERE embedding IS NULL");
    }
    result.push_str(" ORDER BY scraped_at ASC LIMIT ? OFFSET ?");
    result
}

pub async fn get_page(conn: Connection, limit: u32, offset: u32) -> Result<Vec<ScrapeRow>, Error> {
    let mut stmt = conn.prepare(&select_page(true)).await?;
    let mut rows = stmt.query((limit, offset)).await?;
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
        scrapes.push(ScrapeRow {
            id,
            url,
            content,
            scraped_at,
            embedding,
        });
    }

    Ok(scrapes)
}
