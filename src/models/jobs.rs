use crate::models::utils::create_paginator;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum JobType {
    SMScrape,
    Embed,
}

#[derive(Serialize, Deserialize)]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

pub fn create_job(job_type: JobType, job_status: JobStatus) -> String {
    let now = chrono::Utc::now();
    let uuid = uuid::Uuid::new_v4();

    format!(
        "INSERT INTO jobs (id, job_type, job_status, created_at, updated_at) VALUES ('{}', {}, {}, '{}', '{}');",
        uuid,
        job_type as i32,
        job_status as i32,
        now,
        now,
    )
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
    job_type INTEGER NOT NULL,
    job_status INTEGER NOT NULL,
    created_at timestampz NOT NULL,
    updated_at timestampz NOT NULL,
    completed_at timestampz
);
"#;
