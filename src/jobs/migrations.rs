use libsql::{Connection, Error};

const MIGRATION_TABLE_NAME: &str = "migrations";

/// Sets up the migration table if it doesn't exist.
///
/// # Arguments
///
/// * `conn` - A reference to the database connection.
///
/// # Returns
///
/// Returns `Ok(())` if the setup is successful, otherwise returns an `Error`.
async fn setup(conn: &Connection) -> Result<(), Error> {
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

    Ok(())
}

/// Runs an "up" migration. Skip if migration with same title has been previously
/// ran.
///
/// # Arguments
///
/// * `conn` - A reference to the database connection.
/// * `title` - The title of the migration.
/// * `migration_sql` - The SQL statement for the migration.
///
/// # Returns
///
/// Returns `Ok(())` if the migration is successful, otherwise returns an `Error`.
pub async fn run_up(conn: &Connection, title: &str, migration_sql: &str) -> Result<(), Error> {
    setup(conn).await?;
    let existing_migration_query = format!(
        "SELECT id FROM {} WHERE title = '{}';",
        MIGRATION_TABLE_NAME, title
    );
    let mut existing_migration_rows = conn.query(&existing_migration_query, ()).await?;
    match existing_migration_rows.next().await? {
        Some(_) => {
            println!("Skipping Migration '{}'", title);
            Ok(())
        }
        None => {
            println!("Running Migration '{}'", title);
            conn.query(migration_sql, ()).await?;

            let now = chrono::Utc::now().to_rfc3339();
            let uuid = uuid::Uuid::new_v4();
            let insert_ran_migration = format!(
                "INSERT INTO {} (id, title, created_at) VALUES ('{}', '{}', '{}');",
                MIGRATION_TABLE_NAME, uuid, title, now
            );
            conn.query(&insert_ran_migration, ()).await?;
            println!("Ran Migration '{}' Successfuly", title);
            Ok(())
        }
    }
}
