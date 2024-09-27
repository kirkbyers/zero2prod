use std::{thread::sleep, time::Duration};

use crate::{db::start_db, services::scraper};
use chrono::Utc;
use libsql::params;
use uuid::Uuid;

macro_rules! unwrap_table_data {
    ($table_data:expr, $key:expr) => {
        $table_data.get($key).unwrap_or(&String::new()).to_string()
    };
}

pub async fn main() {
    let db = start_db().await.unwrap();
    let conn = db.connect().unwrap();

    let mut rows = conn
        .query("SELECT COALESCE(MAX(batch_id), 0) FROM scrapes", params![])
        .await
        .expect("Failed to query max batch_id");

    let max_batch_id: i32 = if let Ok(Some(row)) = rows.next().await {
        row.get(0).unwrap()
    } else {
        0
    };
    let batch_id = max_batch_id + 1;

    let scraper = scraper::Scraper::new();
    let res = scraper
        .get_url("https://www.sweetmarias.com/green-coffee.html?product_list_limit=all&sm_status=1")
        .await
        .expect("Failed to get URL");

    let links = scraper.parse_directory_html(&res);
    println!("Found {} links", links.len());
    sleep(Duration::from_secs(5));
    for (i, link) in links.iter().enumerate() {
        println!("Scraping link #{}: {}", i, link);
        let item_html = scraper
            .get_url(link.as_str())
            .await
            .expect("Failed to get URL");

        let item_text = scraper.strip_html_tags(&item_html);
        let id = Uuid::new_v4();
        let now = Utc::now();
        let table_data = scraper.sm_item_listing_to_details(&item_html);
        let region: String = unwrap_table_data!(table_data, "region");
        let processing: String = unwrap_table_data!(table_data, "processing");
        let drying: String = unwrap_table_data!(table_data, "drying");
        let arrival: String = unwrap_table_data!(table_data, "arrival");
        let lot_size: String = unwrap_table_data!(table_data, "lot_size");
        let bag_size: String = unwrap_table_data!(table_data, "bag_size");
        let packaging: String = unwrap_table_data!(table_data, "packaging");
        let farm_gate: String = unwrap_table_data!(table_data, "farm_gate");
        let cultivar_detail: String = unwrap_table_data!(table_data, "cultivar_detail");
        let grade: String = unwrap_table_data!(table_data, "grade");
        let appearance: String = unwrap_table_data!(table_data, "appearance");
        let roast_rec: String = unwrap_table_data!(table_data, "roast_rec");
        let coffee_type: String = unwrap_table_data!(table_data, "coffee_type");
        let spro_rec: String = unwrap_table_data!(table_data, "spro_rec");
        let score: String = unwrap_table_data!(table_data, "score");

        // Save the scraped item to the database
        let mut stmt = conn
            .prepare(INSERT_QUERY)
            .await
            .expect("Failed to prepare query.");
        match stmt
            .execute(vec![
                id.to_string(),
                link.clone(),
                item_text,
                now.to_rfc3339(),
                item_html,
                region,
                processing,
                drying,
                arrival,
                lot_size,
                bag_size,
                packaging,
                farm_gate,
                cultivar_detail,
                grade,
                appearance,
                roast_rec,
                coffee_type,
                spro_rec,
                score,
                batch_id.to_string()
            ])
            .await
        {
            Ok(_) => println!("Scraped and saved item #{}", i),
            Err(e) => eprintln!("Failed to execute query: {:?}", e),
        }
        sleep(Duration::from_secs(2));
    }
}

const INSERT_QUERY: &str = r#"
    INSERT INTO scrapes (
        id, 
        url, 
        content, 
        scraped_at, 
        original, 
        region, 
        processing,
        drying,
        arrival,
        lot_size,
        bag_size,
        packaging,
        farm_gate,
        cultivar_detail,
        grade,
        appearance,
        roast_rec,
        coffee_type,
        spro_rec,
        score,
        batch_id
    )
    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21);
"#;
