use actix_web::{get, HttpResponse};
use tera::{Context, Tera};

#[get("/")]
async fn home() -> HttpResponse {
    let tera_context = Context::new();
    let rendered = match Tera::one_off(include_str!("../templates/index.html"), &tera_context, true)
    {
        Ok(rendered) => rendered,
        Err(e) => {
            eprintln!("Failed to render template: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };
    HttpResponse::Ok().body(rendered)
}
