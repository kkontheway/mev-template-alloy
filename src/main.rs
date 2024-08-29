use mev_template_alloy::run;
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    run().await;
}
