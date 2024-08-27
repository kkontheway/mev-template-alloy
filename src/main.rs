#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    run().await;
}
