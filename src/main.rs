use zero2prod::{run, db};


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    db::new_local_db("./.data/mydb").await.expect("Failed to create database");
    let listener = std::net::TcpListener::bind("127.0.0.1:8000")
        .expect("Failed to bind to port 8000");
    run(listener)?.await
}