use actix_web::{get, HttpResponse};

#[get("/api/health_check")]
pub async fn health_check_route() -> HttpResponse {
    HttpResponse::Ok().finish()
}
