use libsql::Connection;

#[derive(Debug)]
pub struct ScrapeEmbedding {
    pub id: String,
    pub scrape_id: String,
    pub embedding: Vec<u8>,
}

impl ScrapeEmbedding {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            scrape_id: String::new(),
            embedding: Vec::new(),
        }
    }

    pub async fn insert(
        &self,
        conn: &Connection,
    ) -> Result<&ScrapeEmbedding, Box<dyn std::error::Error>> {
        let mut stmt = conn
            .prepare(
                r#"
            INSERT INTO scrape_embeddings (id, scrape_id, embedding)
            VALUES (?, ?, ?)
        "#,
            )
            .await?;
        stmt.execute((
            self.id.to_string(),
            self.scrape_id.to_string(),
            self.embedding.clone(),
        ))
        .await?;
        Ok(self)
    }
}

impl Default for ScrapeEmbedding {
    fn default() -> Self {
        Self::new()
    }
}

pub async fn get_page(
    conn: &Connection,
    limit: &u32,
    offset: &u32,
) -> Result<Vec<ScrapeEmbedding>, Box<dyn std::error::Error>> {
    let mut stmt = conn
        .prepare(
            r#"
            SELECT id, doc_type, doc_id, embedding
            FROM fast_embeds
            LIMIT ? OFFSET ?
        "#,
        )
        .await?;
    let mut rows = stmt.query((*limit, *offset)).await?;
    let mut scrape_embeds = Vec::new();
    while let Some(row) = rows.next().await? {
        let id: String = row.get(0)?;
        let scrape_id: String = row.get(1)?;
        let embedding: Vec<u8> = row.get(2)?;
        let fast_embed = ScrapeEmbedding {
            id,
            scrape_id,
            embedding,
        };
        scrape_embeds.push(fast_embed);
    }
    Ok(scrape_embeds)
}
