use alloy::providers::{ProviderBuilder, WsConnect};
use eyre::Result;

pub mod block_scanner;
pub mod mempool;

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
        block_scanner::block_scanner(provider).await;
    });

    mempool::mempool_scanner(ws_provider).await;

    Ok(())
}
