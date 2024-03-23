use libsql::{Builder, Connection, Database, Error};

use crate::configuration::get_configuration;

pub async fn local_db() -> Result<Database, Error> {
    let config = get_configuration(None).expect("Failed to read configuration.");
    let path = config.database.local_file_path;
    let db = Builder::new_local(path).build().await.expect("Failed to create database.");
    let conn = db.connect().expect("Failed to connect to database.");
    init_schema(&conn).await;

    Ok(db)
}

async fn init_schema(conn: &Connection) {
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS subscriptions (
            id uuid NOT NULL PRIMARY KEY,
            email TEXT NOT NULL UNIQUE,
            name TEXT NOT NULL,
            subscribed_at timestampz NOT NULL
        );
        "#,
        (),
    )
    .await
    .unwrap();
    conn.execute(
        r#"
    CREATE TABLE IF NOT EXISTS sm_scrapes (
        id uuid NOT NULL PRIMARY KEY,
        url TEXT NOT NULL UNIQUE,
        content TEXT NOT NULL,
        scraped_at timestampz NOT NULL
    );
    "#,
        (),
    )
    .await
    .unwrap();
}
