use std::{env, fs};

use dotenvy::dotenv;
use zero2prod::{db, jobs};

/// Runs database migrations using the provided migration files.
///
/// # Arguments
///
/// - path to migrations directory
///
/// # Example
///
/// `cargo run --bin migrate ./migrations`
#[tokio::main]
async fn main() {
    dotenv().expect("No .env file found");
    let args: Vec<String> = env::args().collect();
    assert!(
        args.len() > 1,
        "A path to the migrations directory must be provided"
    );

    let db = db::start_db().await.unwrap();
    let conn = db.connect().unwrap();

    let current_dir = env::current_dir().unwrap();
    for file_path in &args[1..] {
        let mut migration_files: Vec<String> = vec![];
        let path_res = current_dir.join(file_path);

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
            println!("{migration_file:?}");
            let file_content = fs::read_to_string(&migration_file).unwrap();
            jobs::migrations::run_up(&conn, &migration_file, &file_content)
                .await
                .unwrap();
        }
    }
}
