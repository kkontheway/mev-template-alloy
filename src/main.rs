use alloy::{
    providers::{Provider, ProviderBuilder, RootProvider, WsConnect},
    pubsub::PubSubFrontend,
    transports::http::{Client, Http},
};
use eyre::Result;
use futures_util::StreamExt;
use std::{mem, thread, time::Duration};
#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url =
        "https://eth-mainnet.g.alchemy.com/v2/Q9tQTNzKb5PvbMwh5mcuj4p2eTMWea1M".parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);

    let ws_url = "wss://eth-mainnet.g.alchemy.com/v2/Q9tQTNzKb5PvbMwh5mcuj4p2eTMWea1M";

    let ws = WsConnect::new(ws_url);
    let ws_provider = ProviderBuilder::new().on_ws(ws).await?;
    println!("程序开始运行...");
    tokio::spawn(async move {
        block_scanner(provider).await;
    });

    mempool_scanner(ws_provider).await;

    Ok(())
}

async fn block_scanner(provider: RootProvider<Http<Client>>) {
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

async fn mempool_scanner(ws_provider: RootProvider<PubSubFrontend>) {
    let sub = ws_provider.subscribe_pending_transactions().await.unwrap();

    let mut tx_stream = sub.into_stream().take(3);
    let handle = tokio::spawn(async move {
        while let Some(tx_hash) = tx_stream.next().await {
            if let Ok(tx) = ws_provider.get_transaction_by_hash(tx_hash).await {
                println!("Transaction details: {tx:#?}");
            }
        }
    });
    handle.await.unwrap();
}
