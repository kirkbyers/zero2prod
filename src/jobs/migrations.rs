use std::{env, fs};

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
            conn.execute_batch(migration_sql).await?;

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

pub async fn run_all_in_dir(
    conn: &Connection,
    dir_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = env::current_dir()?;
    let mut migration_files: Vec<String> = vec![];
    let path_res = current_dir.join(dir_path);

    if path_res.is_dir() {
        for entry in fs::read_dir(path_res).unwrap() {
            let entry = entry.unwrap();
            let entry_path = entry.path();
            if entry_path.is_file() {
                migration_files.push(entry_path.to_str().unwrap().to_string())
            }
        }
    } else {
        migration_files.push(path_res.to_str().unwrap().to_string());
    }

    migration_files.sort();

    for migration_file in migration_files {
        let file_content = fs::read_to_string(&migration_file)?;
        run_up(&conn, &migration_file, &file_content).await.unwrap();
    }
    Ok(())
}
