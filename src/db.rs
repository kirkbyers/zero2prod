use libsql::{Builder, Connection, Database, Error};

pub async fn new_local_db(path: &str) -> Result<Database, Error> {
    let db = Builder::new_local(path).build().await?;
    let conn = db.connect()?;
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
}
