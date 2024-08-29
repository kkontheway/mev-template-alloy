mod common;
use crate::common::constants::Env;

use crate::common::logger::setup_logger;
use crate::common::receive::receive_event;
use crate::common::streams::{stream_new_blocks, stream_pending_transactions, Event};
use alloy::providers::{ProviderBuilder, WsConnect};
use eyre::Result;
use log::info;
use tokio::sync::broadcast;

pub async fn run() -> Result<()> {
    setup_logger()?;
    info!("Starting MEV");
    let env = Env::new().map_err(|e| eyre::eyre!(e))?;

    let wss_url = env.wss_url;
    let wss = WsConnect::new(wss_url);
    let wss_provider = ProviderBuilder::new().on_ws(wss).await?;

    // // 创建一个广播通道
    let (tx, _) = broadcast::channel(100);
    // 在一个新的任务中运行stream_new_blocks
    tokio::spawn(stream_new_blocks(wss_provider.clone(), tx.clone()));
    tokio::spawn(stream_pending_transactions(
        wss_provider.clone(),
        tx.clone(),
    ));

    // 运行sandwich策略
    receive_event(tx).await; // 添加 ? 来传播错误
    Ok(())
}
