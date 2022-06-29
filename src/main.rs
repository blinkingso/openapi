use openapi::run;

#[tokio::main]
async fn main() {
    let _ = run().await.expect("failed to run server");
}
