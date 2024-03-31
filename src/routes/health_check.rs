use actix_web::{get, HttpResponse};

#[get("/health_check")]
pub async fn health_check_route() -> HttpResponse {
    HttpResponse::Ok().finish()
}
