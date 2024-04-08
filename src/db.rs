use std::env;

use libsql::{Builder, Connection, Database, Error};

use crate::models::{
    jobs::INIT_TABLE as JOBS_INIT, sm_scrape::INIT_TABLE as SCRAPE_INIT,
    subscriptions::INIT_TABLE as SUBSCRIPTIONS_INIT,
};

pub async fn start_db() -> Result<Database, Error> {
    let db_url = match env::var("DB_URL") {
        Ok(url) => url,
        Err(_) => String::new(),
    };
    let db_file_path = match env::var("DB_FILE_PATH") {
        Ok(file_path) => file_path,
        Err(_) => {
            return Err(Error::ConnectionFailed(String::from(
                "DB_FILE_PATH must be set",
            )))
        }
    };
    if db_url.is_empty() {
        return local_db(&db_file_path).await;
    }

    replica_db(&db_file_path, &db_url).await
}

async fn replica_db(db_path: &str, db_url: &str) -> Result<Database, Error> {
    let db_auth_token = match env::var("DB_AUTH_TOKEN") {
        Ok(token) => token,
        Err(_) => {
            return Err(Error::ConnectionFailed(String::from(
                "DB_AUTH_TOKEN must be provided with DB_URL",
            )))
        }
    };

    Builder::new_remote_replica(db_path, db_url.to_string(), db_auth_token)
        .build()
        .await
}

async fn local_db(db_path: &str) -> Result<Database, Error> {
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
