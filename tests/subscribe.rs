mod common;

use common::spawn_app;

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
    }
}