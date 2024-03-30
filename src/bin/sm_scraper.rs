use std::{thread::sleep, time::Duration};

use chrono::Utc;
use uuid::Uuid;
use zero2prod::{configuration::get_configuration, db::local_db, services::scraper};

macro_rules! unwrap_table_data {
    ($table_data:expr, $key:expr) => {
        $table_data.get($key).unwrap_or(&String::new()).to_string()
    };
}

#[tokio::main]
async fn main() {
    let config =
        get_configuration(Some("configuration.yaml")).expect("Failed to read configuration.");
    let db = local_db(&config.database.local_file_path).await.unwrap();
    let conn = db.connect().unwrap();

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
            ])
            .await
        {
            Ok(_) => println!("Scraped and saved item #{}", i),
            Err(e) => eprintln!("Failed to execute query: {:?}", e),
        }
        sleep(Duration::from_secs(7));
    }
}

const INSERT_QUERY: &str = r#"
    INSERT INTO sm_scrapes (
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
        spro_rec
    )
    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19);
"#;
