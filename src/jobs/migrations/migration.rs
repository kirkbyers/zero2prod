use libsql::{Connection, Error, Rows};

const MIGRATION_TABLE_NAME: &str = "migrations";

pub struct Migration {
    title: String,
    up: String,
    down: String,
}

pub async fn setup(conn: &Connection) -> Result<(), Error> {
    let table_exists_query = format!("SELECT to_regclass('public.{}');", MIGRATION_TABLE_NAME);
    let mut rows: Rows = conn.query(&table_exists_query, ()).await?;
    match rows.next().await? {
        Some(_) => (),
        None => {
            conn.query(
                &format!(
                    r#"
                CREATE TABLE IF NOT EXISTS {} (
                    id uuid NOT NULL PRIMARY KEY,
                    title TEXT NOT NULL,
                    created_at TEXT
                );
            "#,
                    MIGRATION_TABLE_NAME
                ),
                (),
            )
            .await?;
        }
    };

    Ok(())
}
