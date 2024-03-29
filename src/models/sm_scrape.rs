use chrono::DateTime;
use libsql::{Connection, Error};

#[derive(Debug)]
pub struct SMScrapeRow {
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

impl SMScrapeRow {
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

impl Default for SMScrapeRow {
    fn default() -> Self {
        Self::new()
    }
}

fn select_page(filter_non_null_embeddings: bool) -> String {
    let mut result = String::from(
        r#"
        SELECT id, url, content, scraped_at, embedding
        FROM sm_scrapes
    "#,
    );
    if filter_non_null_embeddings {
        result.push_str(" WHERE embedding IS NULL");
    }
    result.push_str(" ORDER BY scraped_at ASC LIMIT ? OFFSET ?");
    result
}

pub async fn get_page(
    conn: Connection,
    limit: u32,
    offset: u32,
    filter_non_null_embeddings: bool,
) -> Result<Vec<SMScrapeRow>, Error> {
    let mut stmt = conn
        .prepare(&select_page(filter_non_null_embeddings))
        .await?;
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

        let mut new_row = SMScrapeRow::default();
        new_row.id = id;
        new_row.url = url;
        new_row.content = content;
        new_row.scraped_at = scraped_at;
        new_row.embedding = embedding;
        scrapes.push(new_row);
    }

    Ok(scrapes)
}

pub const INIT_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS sm_scrapes (
    id uuid NOT NULL PRIMARY KEY,
    url TEXT NOT NULL,
    original TEXT,
    content TEXT,
    scraped_at timestampz NOT NULL,
    embedding BLOB,
    region TEXT,
    processing TEXT,
    drying TEXT,
    arrival TEXT,
    lot_size TEXT,
    bag_size TEXT,
    packaging TEXT,
    farm_gate TEXT,
    cultivar_detail TEXT,
    grade TEXT,
    appearance TEXT,
    roast_rec TEXT,
    coffee_type TEXT,
    spro_rec TEXT
);
"#;
