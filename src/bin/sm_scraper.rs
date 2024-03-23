use zero2prod::db::local_db;

#[tokio::main]
async fn main() {
    let db = local_db().await.unwrap();
    let _conn = db.connect().unwrap();
}
