use zero2prod::jobs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match jobs::fast_embed_scrapes::main().await {
        Ok(v) => Ok(v),
        Err(e) => {
            println!("{:?}", e);
            Err(e)
        }
    }
}
