use anyhow::Result;
use zero2prod::jobs;

#[tokio::main]
async fn main() -> Result<()> {
    match jobs::embed_scrapes::main().await {
        Ok(v) => Ok(v),
        Err(e) => {
            println!("{:?}", e);
            Err(e)
        }
    }
}
