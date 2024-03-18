mod common;

use common::spawn_app;
use zero2prod::db::local_db;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let address = spawn_app().await;
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

    let db = local_db().await.unwrap();
    let conn = db.connect().unwrap();

    let mut rows = conn
        .query("SELECT email, name FROM subscriptions LIMIT 1", ())
        .await
        .unwrap();
    let row = rows.next().await.expect("No rows returned").expect("Failed to get row");
    let email_value = row
        .get_value(0)
        .expect("Failed to get email");
    let email = email_value.as_text().expect("Failed to get email text");
    let name_value = row
        .get_value(1)
        .expect("Failed to get name");
    let name = name_value
        .as_text()
        .expect("Failed to get name text");
    assert_eq!(email, "some@email.co");
    assert_eq!(name, "leeroy");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("{}", "missing both name and email"),
        (
            r#"
        {
            "email": "some@email.co"
        }"#,
            "missing name",
        ),
        (
            r#"
        {
            "name": "leeroy"
        }"#,
            "missing email",
        ),
    ];

    for (invalid_body, _error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &address))
            .header("Content-Type", "application/json")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 400 Bad Request when the payload was {}.",
            invalid_body
        );
    }
}
