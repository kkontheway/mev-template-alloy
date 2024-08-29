use alloy::{
    providers::{Provider, RootProvider},
    pubsub::PubSubFrontend,
};
use futures_util::StreamExt;

pub async fn mempool_scanner(ws_provider: RootProvider<PubSubFrontend>) {
    let sub = ws_provider.subscribe_pending_transactions().await.unwrap();
    let mut tx_stream = sub.into_stream();
    println!("---------- MONITORING MEMPOOL ----------");

    while let Some(tx_hash) = tx_stream.next().await {
        if let Ok(tx) = ws_provider.get_transaction_by_hash(tx_hash).await {
            println!("Transaction details: {tx:#?}");
        }
    }
}
