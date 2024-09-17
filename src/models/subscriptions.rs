pub const INIT_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS subscriptions (
    id TEXT PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    subscribed_at TEXT NOT NULL
);
"#;
