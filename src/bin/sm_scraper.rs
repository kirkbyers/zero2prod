use std::{thread::sleep, time::Duration};

use chrono::Utc;
use uuid::Uuid;
use zero2prod::{configuration::get_configuration, db::local_db, services::scraper};

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
    for (i, link) in vec![links[0].clone()].iter().enumerate() {
        println!("Scraping link #{}: {}", i, link);
        let item_html = scraper
            .get_url(link.as_str())
            .await
            .expect("Failed to get URL");

        let item_text = scraper.strip_html_tags(&item_html);
        let id = Uuid::new_v4();
        let now = Utc::now();
        let table_data = scraper.sm_item_listing_to_details(&item_html);
        let region: String = table_data
            .get("region")
            .unwrap_or(&String::new())
            .to_string();
        let processing: String = table_data
            .get("processing")
            .unwrap_or(&String::new())
            .to_string();

        // Save the scraped item to the database
        let mut stmt = conn
            .prepare(INSERT_QUERY)
            .await
            .expect("Failed to prepare query.");
        match stmt
            .execute((
                id.to_string(),
                link.clone(),
                item_text,
                now.to_rfc3339(),
                item_html,
                region,
                processing,
            ))
            .await
        {
            Ok(_) => println!("Scraped and saved item #{}", i),
            Err(e) => eprintln!("Failed to execute query: {:?}", e),
        }
        sleep(Duration::from_secs(7));
    }
}

const INSERT_QUERY: &str = r#"
    INSERT INTO sm_scrapes (id, url, content, scraped_at, original, region, processing)
    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7);
"#;
