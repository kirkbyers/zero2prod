use crate::models::jobs;
use actix_web::{get, post, web, HttpResponse};
use libsql::Rows;
use serde::Serialize;

// GET /jobs
#[get("")]
pub async fn get_jobs(conn: web::Data<libsql::Connection>) -> HttpResponse {
    let query = jobs::select_with_pagination(
        "id, type, status, created_at, completed_at",
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
#[post("")]
pub async fn start_job() -> HttpResponse {
    HttpResponse::Ok().finish()
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
    completed_at: String,
}

async fn rows_to_response(mut rows: Rows) -> Vec<ParsedJobRow> {
    let mut result = Vec::new();
    while let Ok(Some(row)) = rows.next().await {
        result.push(ParsedJobRow {
            id: row_to_string!(row, 0),
            job_type: row_to_string!(row, 1),
            status: row_to_string!(row, 2),
            created_at: row_to_string!(row, 3),
            completed_at: row_to_string!(row, 4),
        });
    }
    result
}
