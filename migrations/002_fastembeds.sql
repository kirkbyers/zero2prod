CREATE TABLE IF NOT EXISTS fast_embeds (
  id TEXT PRIMARY KEY,
  doc_type TEXT,
  doc_id TEXT,
  embedding BLOB
);