use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
enum JobType {
    SMScrape,
    Embed
}

#[derive(Serialize, Deserialize)]
enum JobStatus {
    Pending,
    Running,
    Completed,
    Failed
}

pub const INIT_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS jobs (
    id uuid NOT NULL PRIMARY KEY,
    type INTEGER NOT NULL,
    status INTEGER NOT NULL,
    created_at timestampz NOT NULL,
    updated_at timestampz NOT NULL,
    completed_at timestampz,
);
"#;