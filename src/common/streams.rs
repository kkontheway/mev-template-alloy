use crate::common::calculate_next_block_base_fee::calculate_next_block_base_fee;

use alloy::primitives::*;
use alloy::providers::Provider;
use alloy::providers::RootProvider;
use alloy::pubsub::PubSubFrontend;
use alloy::rpc::types::Transaction;
// use std::sync::Arc;
use futures::StreamExt;
use tokio::sync::broadcast::Sender;
#[derive(Default, Debug, Clone)]
pub struct NewBlock {
    pub block_number: U64,
    pub base_fee: U256,
    pub next_base_fee: U256,
}

#[derive(Debug, Clone)]
pub struct NewPendingTx {
    pub added_block: Option<U64>,
    pub tx: Transaction,
}

impl Default for NewPendingTx {
    fn default() -> Self {
        Self {
            added_block: None,
            tx: Transaction::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    Block(NewBlock),
    PendingTx(NewPendingTx),
}

// A websocket connection made to get newly created blocks
pub async fn stream_new_blocks(
    provider: RootProvider<PubSubFrontend>,
    event_sender: Sender<Event>,
) {
    let mut stream = provider.subscribe_blocks().await.unwrap();
    let mut stream = stream.into_stream();
    while let Some(block) = stream.next().await {
        if let Some(number) = block.header.number {
            let new_block = NewBlock {
                block_number: U64::from(number),
                base_fee: U256::from(block.header.base_fee_per_gas.unwrap_or_default()),
                next_base_fee: U256::from(calculate_next_block_base_fee(
                    U256::from(block.header.gas_used),
                    U256::from(block.header.gas_limit),
                    U256::from(block.header.base_fee_per_gas.unwrap_or_default()),
                )),
            };
            let _ = event_sender.send(Event::Block(new_block));
        }
    }
}

// A websocket connection made to get new pending transactions
pub async fn stream_pending_transactions(
    provider: RootProvider<PubSubFrontend>,
    event_sender: Sender<Event>,
) {
    let stream = provider.subscribe_pending_transactions().await.unwrap();
    let mut stream = stream.into_stream();

    while let Some(tx_hash) = stream.next().await {
        if let Ok(Some(tx)) = provider.get_transaction_by_hash(tx_hash).await {
            let _ = event_sender.send(Event::PendingTx(NewPendingTx {
                added_block: None,
                tx,
            }));
        }
    }
}

// match result {
//     Ok(tx_hash) => {
//         if let Ok(tx) = provider.get_transaction(tx_hash).await {
//             let _ = event_sender.send(Event::PendingTx(NewPendingTx {
//                 added_block: None,
//                 tx,
//             }));
//         }
//     },
//     Err(_) => {}
// };
