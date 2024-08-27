use alloy::{
    providers::{Provider, RootProvider},
    transports::http::{Client, Http},
};

use std::thread;
use std::time::Duration;

pub async fn block_scanner(provider: RootProvider<Http<Client>>) {
    let mut latest_block = 0;
    loop {
        let block = provider.get_block_number().await.expect("获取区块号失败");
        if block > latest_block {
            latest_block = block;
            println!("\n---------- 新区块: {} ----------", block);
        }

        thread::sleep(Duration::from_secs(5));
    }
}
