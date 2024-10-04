use dotenvy::dotenv;
use zero2prod::jobs;

#[tokio::main]
async fn main() {
    dotenv().expect("No .env file found");
    jobs::scrape_sm::main().await;
}
