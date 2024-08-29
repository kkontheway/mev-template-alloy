use alloy::{
    providers::{Provider, RootProvider},
    transports::http::{Client, Http},
};

use std::time::Duration;
use tokio::time::sleep;

pub async fn block_scanner(provider: RootProvider<Http<Client>>) {
    let mut latest_block = 0;
    loop {
        if let Ok(block) = provider.get_block_number().await {
            if block > latest_block {
                latest_block = block;
                println!("\n---------- 新区块: {} ----------", block);
            }
        }

        sleep(Duration::from_millis(1)).await;
    }
}
