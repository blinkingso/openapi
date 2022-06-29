async fn spawn_app() -> std::io::Result<()> {
    openapi::run().await
}

#[tokio::test]
async fn health_check_works() {
    spawn_app().await.expect("failed to spawn app.");

    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:8080/health")
        .send()
        .await
        .expect("failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(2));
}
