use std::{env, time::Duration};

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

    let db_path_parts: Vec<&str> = db_file_path.split('/').collect();
    let new_db_path = db_path_parts[..db_path_parts.len() - 1].join("/");
    std::fs::create_dir_all(new_db_path).map_err(|_| {
        Error::ConnectionFailed(String::from("Failed to create database directory."))
    })?;

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

    let db = match Builder::new_remote_replica(db_path, db_url.to_string(), db_auth_token)
        .read_your_writes(true)
        .sync_interval(Duration::from_secs(300))
        .build()
        .await
    {
        Ok(db) => db,
        Err(e) => return Err(e),
    };
    let conn = match db.connect() {
        Ok(con) => con,
        Err(e) => return Err(e),
    };

    match init_schema(&conn).await {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    Ok(db)
}

async fn local_db(db_path: &str) -> Result<Database, Error> {
    let db = Builder::new_local(db_path)
        .build()
        .await
        .map_err(|_| Error::ConnectionFailed(String::from("Failed to create database.")))?;
    let conn = db
        .connect()
        .map_err(|_| Error::ConnectionFailed(String::from("Failed to connect to database.")))?;
    init_schema(&conn)
        .await
        .map_err(|_| Error::ConnectionFailed(String::from("Failed to initialize schema.")))?;

    Ok(db)
}

async fn init_schema(conn: &Connection) -> Result<(), Error> {
    conn.execute(SUBSCRIPTIONS_INIT, ()).await?;
    conn.execute(SCRAPE_INIT, ()).await?;
    conn.execute(JOBS_INIT, ()).await?;
    Ok(())
}
