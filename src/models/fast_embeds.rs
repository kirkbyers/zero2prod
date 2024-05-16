use libsql::Connection;

#[derive(Debug)]
pub struct FastEmbed {
    pub id: String,
    pub doc_type: String,
    pub doc_id: String,
    pub embedding: Vec<u8>,
}

impl FastEmbed {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            doc_type: String::new(),
            doc_id: String::new(),
            embedding: Vec::new(),
        }
    }

    pub async fn insert(
        &self,
        conn: &Connection,
    ) -> Result<&FastEmbed, Box<dyn std::error::Error>> {
        let mut stmt = conn
            .prepare(
                r#"
            INSERT INTO fast_embeds (id, doc_type, doc_id, embedding)
            VALUES (?, ?, ?, ?)
        "#,
            )
            .await?;
        stmt.execute((
            self.id.to_string(),
            self.doc_type.to_string(),
            self.doc_id.to_string(),
            self.embedding.clone(),
        ))
        .await?;
        Ok(self)
    }
}

impl Default for FastEmbed {
    fn default() -> Self {
        Self::new()
    }
}

pub async fn get_page(
    conn: &Connection,
    limit: &u32,
    offset: &u32,
) -> Result<Vec<FastEmbed>, Box<dyn std::error::Error>> {
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
    let mut fast_embeds = Vec::new();
    while let Some(row) = rows.next().await? {
        let id: String = row.get(0)?;
        let doc_type: String = row.get(1)?;
        let doc_id: String = row.get(2)?;
        let embedding: Vec<u8> = row.get(3)?;
        let fast_embed = FastEmbed {
            id,
            doc_type,
            doc_id,
            embedding,
        };
        fast_embeds.push(fast_embed);
    }
    Ok(fast_embeds)
}
