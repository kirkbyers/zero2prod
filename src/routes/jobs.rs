use actix_web::{get, post, web, HttpResponse};

// GET /jobs
#[get("")]
pub async fn get_jobs() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// POST /jobs
#[post("")]
pub async fn start_job() -> HttpResponse {
    HttpResponse::Ok().finish()
}
