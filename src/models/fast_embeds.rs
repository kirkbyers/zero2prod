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
