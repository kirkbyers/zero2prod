use zero2prod::jobs;

#[tokio::main]
async fn main() {
    jobs::scrape_sm::main().await;
}
