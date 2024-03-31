use libsql::{Builder, Connection, Database, Error};

use crate::models::{
    sm_scrape::INIT_TABLE as SCRAPE_INIT, 
    subscriptions::INIT_TABLE as SUBSCRIPTIONS_INIT,
    jobs::INIT_TABLE as JOBS_INIT,
};

pub async fn local_db(db_path: &str) -> Result<Database, Error> {
    let db_path_parts: Vec<&str> = db_path.split('/').collect();
    let new_db_path = db_path_parts[..db_path_parts.len() - 1].join("/");
    std::fs::create_dir_all(new_db_path).expect("Failed to create database directory.");
    let db = Builder::new_local(db_path)
        .build()
        .await
        .expect("Failed to create database.");
    let conn = db.connect().expect("Failed to connect to database.");
    init_schema(&conn)
        .await
        .expect("Failed to initialize schema.");

    Ok(db)
}

async fn init_schema(conn: &Connection) -> Result<(), Error> {
    conn.execute(SUBSCRIPTIONS_INIT, ()).await?;
    conn.execute(SCRAPE_INIT, ()).await?;
    conn.execute(JOBS_INIT, ()).await?;
    Ok(())
}
