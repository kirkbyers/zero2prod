use libsql::{Builder, Database, Error};

pub async fn new_local_db(path: &str) -> Result<Database, Error> {
    Builder::new_local(path)
        .build()
        .await
}