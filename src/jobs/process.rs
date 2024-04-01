use crate::{
    db,
    jobs::{embed_scrapes::main as run_embed_scrapes, scrape_sm::main as run_scrape_sm},
    models::jobs,
};

pub async fn process_job(db_path: &str) -> Result<(), std::io::Error> {
    let db = match db::local_db(db_path).await {
        Ok(db) => db,
        Err(err) => {
            eprintln!("Failed to connect to database: {:?}", err);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to connect to database",
            ));
        }
    };

    let conn = match db.connect() {
        Ok(connection) => connection,
        Err(err) => {
            eprintln!("Failed to connect to database: {:?}", err);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to connect to database",
            ));
        }
    };

    let query = jobs::select_with_pagination(
        "id, job_type",
        &format!("job_status = {}", jobs::JobStatus::Pending as i32),
        "created_at",
        "ASC",
        1,
        0,
    );
    let pending_row = conn
        .query(&query, ())
        .await
        .expect("Failed to get pending job")
        .next()
        .await;
    let pending_id: String;
    let pending_job_type: i32;
    match pending_row {
        Ok(Some(row)) => {
            pending_id = row.get::<String>(0).unwrap();
            pending_job_type = row.get::<i32>(1).unwrap();
        }
        Ok(None) => {
            eprintln!("No pending jobs");
            return Ok(());
        }
        Err(e) => {
            eprintln!("Failed to get pending job: {:?}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to get pending job",
            ));
        }
    };
    let update_query = jobs::update_row(&pending_id, jobs::JobStatus::Running);
    conn.execute(&update_query, ())
        .await
        .expect("Failed to update job status");

    match pending_job_type {
        i if i == jobs::JobType::SMScrape.as_i32() => {
            eprintln!("SMScrape job running");
            run_scrape_sm().await;
        }
        i if i == jobs::JobType::Embed.as_i32() => {
            eprintln!("Embed job running");
            run_embed_scrapes().await;
        }
        _ => {
            eprintln!("Unknown job type");
            return Ok(());
        }
    }
    let complete_query = jobs::update_row(&pending_id, jobs::JobStatus::Completed);
    conn.execute(&complete_query, ())
        .await
        .expect("Failed to update job status");

    Ok(())
}
