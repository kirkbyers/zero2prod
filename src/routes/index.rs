use actix_web::{get, web, HttpResponse};
use libsql::params;
use tera::{Context, Tera};

use crate::{
    models::scrape,
    routes::scrapes,
};

#[get("/")]
async fn home(conn: web::Data<libsql::Connection>) -> HttpResponse {
    let mut tera_context = Context::new();

    let mut max_batch_id_rows = conn
        .query("SELECT COALESCE(MAX(batch_id), 0) FROM scrapes", params![])
        .await
        .expect("Failed to query max batch_id");

    let max_batch_id: i32 = if let Ok(Some(row)) = max_batch_id_rows.next().await {
        row.get(0).unwrap()
    } else {
        0
    };

    let select_page = scrape::select_with_pagination(
        "id, url, arrival, lot_size, bag_size, score, packaging, cultivar_detail, spro_rec",
        &format!(
            "score != '' AND batch_id = {}",
            max_batch_id
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
