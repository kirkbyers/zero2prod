use libsql::{Builder, Connection, Database, Error};

use crate::configuration::get_configuration;

pub async fn local_db() -> Result<Database, Error> {
    let config = get_configuration(None).expect("Failed to read configuration.");
    let path = config.database.local_file_path;
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
