use zero2prod::startup::spawn_app;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute the health_check request!");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
