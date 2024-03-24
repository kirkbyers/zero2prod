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
    for (i, link) in links.iter().enumerate() {
        println!("Scraping link #{}: {}", i, link);
        let item_html = scraper
            .get_url(link.as_str())
            .await
            .expect("Failed to get URL");
        let item_text = scraper.strip_html_tags(&item_html);

        let id = Uuid::new_v4();
        let now = Utc::now();
        let mut stmt = conn
            .prepare(INSERT_QUERY)
            .await
            .expect("Failed to prepare query.");

        match stmt
            .execute((id.to_string(), link.clone(), item_text, now.to_rfc3339()))
            .await
        {
            Ok(_) => println!("Scraped and saved item #{}", i),
            Err(e) => eprintln!("Failed to execute query: {:?}", e),
        }
        sleep(Duration::from_secs(7));
    }
}

const INSERT_QUERY: &str = r#"
    INSERT INTO sm_scrapes (id, url, content, scraped_at)
    VALUES (?1, ?2, ?3, ?4);
"#;
