use crate::models::utils::create_paginator;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
enum JobType {
    SMScrape,
    Embed,
}

#[derive(Serialize, Deserialize)]
enum JobStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

pub fn select_with_pagination(
    columns: &str,
    q: &str,
    sort_by: &str,
    sort_direction: &str,
    limit: u32,
    offset: u32,
) -> String {
    create_paginator("jobs")(columns, q, sort_by, sort_direction, limit, offset)
}

pub const INIT_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS jobs (
    id uuid NOT NULL PRIMARY KEY,
    type INTEGER NOT NULL,
    status INTEGER NOT NULL,
    created_at timestampz NOT NULL,
    updated_at timestampz NOT NULL,
    completed_at timestampz
);
"#;
