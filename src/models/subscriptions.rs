pub const INIT_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS subscriptions (
    id uuid NOT NULL PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    subscribed_at timestampz NOT NULL
);
"#;
