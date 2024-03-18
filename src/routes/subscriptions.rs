use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct SubscriberData {
    email: String,
    name: String,
}

pub async fn subscribe(_json: web::Json<SubscriberData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
