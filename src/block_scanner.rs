use alloy::providers::Provider;
use alloy::providers::RootProvider;
use alloy::transports::BoxTransport;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

pub async fn loop_blocks(http_provider: Arc<RootProvider<BoxTransport>>) {
    let mut last_number = 0;

    loop {
        if let Ok(block) = http_provider.get_block_number().await {
            if block > last_number {
                last_number = block;
                println!("\n---------- BLOCK: {:?} ----------", block);
            }
        }
        sleep(Duration::from_millis(1)).await;
    }
}
