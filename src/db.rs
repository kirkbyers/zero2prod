use libsql::{Builder, Connection, Database, Error};

pub async fn local_db(db_path: &str) -> Result<Database, Error> {
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
    .await?;
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS sm_scrapes (
            id uuid NOT NULL PRIMARY KEY,
            url TEXT NOT NULL,
            content TEXT NOT NULL,
            scraped_at timestampz NOT NULL,
            embedding BLOB
        );
        "#,
        (),
    )
    .await?;
    Ok(())
}
