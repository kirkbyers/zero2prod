use std::env;

pub async fn spawn_app() -> String {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    env::set_var("DB_FILE_PATH", "./.data/tests.db");
    let server = zero2prod::startup::run(listener)
        .await
        .expect("Failed to bind address");
    tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
