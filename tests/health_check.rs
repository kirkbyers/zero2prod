
#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    let body = r#"
    {
        "name": "leeroy",
        "email": "some@email.co"
    }"#;

    let response = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("{}", "missing both name and email"),
        (r#"
        {
            "email": "some@email.co"
        }"#, "missing name"),
        (r#"
        {
            "name": "leeroy"
        }"#, "missing email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &address))
            .header("Content-Type", "application/json")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(400, response.status().as_u16(), "The API did not return a 400 Bad Request when the payload was {}.", invalid_body);
        assert_eq!(
            Some("".to_string()),
            response.headers().get("content-length").map(|v| v.to_str().unwrap().to_string())
        );
        let response_text = response.text().await.expect("Failed to read response body.");
        assert_eq!(response_text, format!("{{\"error\":\"{}\"}}", error_message));
    }
}

fn spawn_app() -> String {
    let listener = std::net::TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}