use log::info;

use tokio::sync::broadcast::Sender;
// we'll update this part later, for now just import the necessary components
use crate::common::constants::{Env, WETH};
use crate::common::streams::{Event, NewBlock};
use eyre::Result;

pub async fn receive_event(event_sender: Sender<Event>) {
    let mut event_receiver = event_sender.subscribe();

    loop {
        match event_receiver.recv().await {
            Ok(event) => match event {
                Event::Block(block) => {
                    info!("收到新区块: {:?}", block);
                }
                Event::PendingTx(mut pending_tx) => {
                    info!("{:?}", pending_tx);
                }
            },
            Err(e) => {
                log::error!("接收事件时出错: {:?}", e);
                // 可以选择在这里 break 或继续循环
            }
        }
    }
}
