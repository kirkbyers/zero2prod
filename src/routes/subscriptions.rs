use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct SubscriberData {
    email: String,
    name: String,
}

const INSERT_QUERY: &str = r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES (?1, ?2, ?3, ?4);
"#;

pub async fn subscribe(
    json: web::Json<SubscriberData>,
    connection: web::Data<libsql::Connection>,
) -> HttpResponse {
    let id = Uuid::new_v4();
    let now = Utc::now();
    let mut stmt = connection
        .prepare(INSERT_QUERY)
        .await
        .expect("Failed to prepare query.");
    match stmt
        .execute((
            id.to_string(),
            json.email.clone(),
            json.name.clone(),
            now.to_rfc3339(),
        ))
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
