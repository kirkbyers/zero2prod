use crate::models::utils::create_paginator;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum JobType {
    SMScrape = 0,
    Embed = 1,
}

impl JobType {
    pub fn as_i32(&self) -> i32 {
        match self {
            JobType::SMScrape => 0,
            JobType::Embed => 1,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Copy, Clone)]
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

pub fn update_row(id: &str, job_status: JobStatus) -> String {
    let now = chrono::Utc::now();
    let mut result = format!(
        "UPDATE jobs SET job_status = {}, updated_at = '{}' ",
        job_status as i32, now,
    );

    if job_status == JobStatus::Completed {
        result = format!("{}, completed_at = '{}' ", result, now);
    }

    result = format!("{} WHERE id = '{}';", result, id);

    result
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

// TODO: timestampz isn't easily comparable.
pub const INIT_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS jobs (
    id TEXT PRIMARY KEY,
    job_type INTEGER NOT NULL,
    job_status INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    completed_at TEXT
);
"#;
