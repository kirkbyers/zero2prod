use actix_web::{get, web, HttpResponse};
use tera::{Context, Tera};

use crate::{
    models::{jobs, scrape},
    routes::scrapes,
};

#[get("/")]
async fn home(conn: web::Data<libsql::Connection>) -> HttpResponse {
    let mut tera_context = Context::new();

    let recent_scrape_job = jobs::select_with_pagination(
        "created_at, completed_at",
        &format!("job_type = {}", jobs::JobType::SMScrape.as_i32()),
        "created_at",
        "DESC",
        1,
        0,
    );
    let scrape_row = match conn.get_ref().query(&recent_scrape_job, ()).await {
        Ok(mut row) => match row.next().await {
            Ok(row) => row,
            Err(e) => {
                eprintln!("Error: {:?}", e);
                return HttpResponse::InternalServerError().body("Error parsing db results");
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            return HttpResponse::InternalServerError().body("Error querying the database");
        }
    };
    let scrape_created_at: String = match scrape_row {
        Some(scrape) => scrape.get::<String>(0).unwrap_or(String::new()),
        None => String::new(),
    };

    let select_page = scrape::select_with_pagination(
        "id, url, arrival, lot_size, bag_size, score, packaging, cultivar_detail, spro_rec",
        &format!(
            "score != '' AND strftime(scraped_at) > strftime('{}')",
            scrape_created_at
        ),
        "score",
        "DESC",
        200,
        0,
    );
    let rows = match conn.get_ref().query(&select_page, ()).await {
        Ok(rows) => rows,
        Err(e) => {
            eprintln!("Error: {}", e);
            return HttpResponse::InternalServerError().body("Error querying the database");
        }
    };
    let res = scrapes::rows_to_response(rows).await;
    tera_context.insert("scrapes", &res.scrapes);

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
