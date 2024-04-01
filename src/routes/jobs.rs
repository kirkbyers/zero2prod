use crate::models::jobs;
use actix_web::{get, post, web, HttpResponse};
use libsql::Rows;
use serde::Serialize;

// GET /jobs
#[get("")]
pub async fn get_jobs(conn: web::Data<libsql::Connection>) -> HttpResponse {
    let query = jobs::select_with_pagination(
        "id, job_type, job_status, created_at, completed_at",
        "",
        "created_at",
        "DESC",
        10,
        0,
    );
    let rows = match conn.get_ref().query(&query, ()).await {
        Ok(rows) => rows,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };
    let response = rows_to_response(rows).await;

    HttpResponse::Ok().json(response)
}

// POST /jobs
// Start a job. If there is a pending job, return a 409 Conflict.
#[post("")]
pub async fn start_job(conn: web::Data<libsql::Connection>) -> HttpResponse {
    let query = jobs::select_with_pagination(
        "id",
        &format!("job_status = {}", jobs::JobStatus::Pending as i32),
        "created_at",
        "ASC",
        1,
        0,
    );
    let pending_row = match conn.get_ref().query(&query, ()).await {
        Ok(mut rows) => rows.next().await,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    match pending_row {
        Ok(Some(_row)) => {
            return HttpResponse::Conflict().body("A job is already running");
        }
        Ok(None) => (),
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    }

    let create_query = jobs::create_job(jobs::JobType::SMScrape, jobs::JobStatus::Pending);
    match conn.get_ref().execute(&create_query, ()).await {
        Ok(_) => (),
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    }

    let confirm_query = jobs::select_with_pagination(
        "id, job_type, job_status, created_at, completed_at",
        "",
        "created_at",
        "DESC",
        1,
        0,
    );
    let confirm_rows = match conn.get_ref().query(&confirm_query, ()).await {
        Ok(rows) => rows,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };
    let response = rows_to_response(confirm_rows).await;

    HttpResponse::Ok().json(response)
}

macro_rules! row_to_string {
    ($row:expr, $index:expr) => {
        $row.get_str($index).unwrap().to_string()
    };
}

#[derive(Serialize, Clone)]
struct ParsedJobRow {
    id: String,
    job_type: String,
    status: String,
    created_at: String,
    completed_at: Option<String>,
}

async fn rows_to_response(mut rows: Rows) -> Vec<ParsedJobRow> {
    let mut result = Vec::new();
    while let Ok(Some(row)) = rows.next().await {
        result.push(ParsedJobRow {
            id: row_to_string!(row, 0),
            job_type: row_to_string!(row, 1),
            status: row_to_string!(row, 2),
            created_at: row_to_string!(row, 3),
            completed_at: match row.get_str(4) {
                Ok(value) => Some(value.to_string()),
                Err(_) => None,
            },
        });
    }
    result
}
