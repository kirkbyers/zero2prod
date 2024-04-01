use crate::models::sm_scrape;
use actix_web::{get, web, HttpResponse};
use libsql::Rows;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct QueryParams {
    q: Option<String>,
    page: Option<usize>,
    per_page: Option<usize>,
    sort_by: Option<SortOptions>,
    sort_direction: Option<SortDirection>,
}

#[derive(Deserialize)]
enum SortOptions {
    Score,
    Arrival,
    BagSize,
    LotSize,
}

#[derive(Deserialize)]
enum SortDirection {
    Asc,
    Desc,
}

#[get("/scrapes")]
async fn get_scrapes(
    query: web::Query<QueryParams>,
    conn: web::Data<libsql::Connection>,
) -> HttpResponse {
    let q = match &query.q {
        Some(q) => q,
        None => "",
    };
    let page = query.page.unwrap_or(1);
    let per_page = std::cmp::min(query.per_page.unwrap_or(10), 200);
    let sort_by = match &query.sort_by {
        Some(SortOptions::Score) => "score",
        Some(SortOptions::Arrival) => "scraped_at",
        Some(SortOptions::BagSize) => "bag_size",
        Some(SortOptions::LotSize) => "lot_size",
        None => "",
    };
    let sort_direction = match &query.sort_direction {
        Some(SortDirection::Asc) => "ASC",
        Some(SortDirection::Desc) => "DESC",
        None => "",
    };

    let select_page = sm_scrape::select_with_pagination(
        "id, url, arrival, lot_size, bag_size, score, packaging, cultivar_detail, spro_rec",
        q,
        sort_by,
        sort_direction,
        per_page as u32,
        (page - 1) as u32 * per_page as u32,
    );

    let scrapes = match conn.get_ref().query(&select_page, ()).await {
        Ok(scrapes) => scrapes,
        Err(e) => {
            eprintln!("Error: {}", e);
            return HttpResponse::InternalServerError().body("Error querying the database");
        }
    };
    let response = rows_to_response(scrapes).await;

    HttpResponse::Ok().json(response)
}

#[derive(Serialize, Debug)]
pub struct Response {
    pub scrapes: Vec<Scrape>,
    pub total: u32,
}

#[derive(Serialize, Clone, Debug)]
pub struct Scrape {
    id: String,
    url: String,
    arrival: String,
    lot_size: String,
    bag_size: String,
    score: String,
    packaging: String,
    cultivar_detail: String,
    spro_rec: String,
}

macro_rules! row_to_string {
    ($row:expr, $index:expr) => {
        $row.get_str($index).unwrap().to_string()
    };
}

macro_rules! real_row_to_string {
    ($row:expr, $index:expr, $default:expr) => {
        $row.get_value($index)
            .unwrap()
            .as_real()
            .unwrap_or($default)
            .to_string()
    };
}

pub async fn rows_to_response(mut rows: Rows) -> Response {
    let mut scrapes = Vec::new();
    while let Ok(Some(row)) = rows.next().await {
        let scrape = Scrape {
            id: row_to_string!(row, 0),
            url: row_to_string!(row, 1),
            arrival: row_to_string!(row, 2),
            lot_size: row_to_string!(row, 3),
            bag_size: row_to_string!(row, 4),
            score: real_row_to_string!(row, 5, &0.0),
            packaging: row_to_string!(row, 6),
            cultivar_detail: row_to_string!(row, 7),
            spro_rec: row_to_string!(row, 8),
        };
        scrapes.push(scrape);
    }
    let length = scrapes.len() as u32;
    Response {
        scrapes,
        total: length,
    }
}
