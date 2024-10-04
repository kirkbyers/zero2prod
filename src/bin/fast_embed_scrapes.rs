use anyhow::Result;
use dotenvy::dotenv;
use zero2prod::jobs;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().expect("No .env file found");
    match jobs::embed_scrapes::main().await {
        Ok(v) => Ok(v),
        Err(e) => {
            println!("{:?}", e);
            Err(e)
        }
    }
}
