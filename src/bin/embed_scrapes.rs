use zero2prod::jobs;

#[tokio::main]
async fn main() {
    jobs::embed_scrapes::main().await;
}
