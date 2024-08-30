use crate::common::streams::{NewBlock, NewPendingTx};
use alloy::primitives::{B160, B256};
use alloy::providers::{ProviderBuilder, RootProvider};
use alloy::pubsub::PubSubFrontend;
use alloy::rpc::types::trace::geth::{
    GethDebugBuiltInTracerType, GethDebugTracerType, GethDebugTracingOptions,
    GethDefaultTracingOptions,
};
use alloy::rpc::types::AccessList;
use alloy::sol_types::*;
use eyre::Result;
use log::info;
use reth::rpc::types::trace::geth::GethDebugTracingCallOptions;
#[derive(Debug, Clone, Default)]
pub struct PendingTxInfo {
    pub pending_tx: NewPendingTx,
    pub touched_pairs: Vec<SwapInfo>,
}

#[derive(Debug, Clone)]
pub enum SwapDirection {
    Buy,
    Sell,
}
#[derive(Debug, Clone)]
pub struct SwapInfo {
    pub tx_hash: B256,
    pub target_pair: B160,
    pub main_currency: B160,
    pub target_token: B160,
    pub version: u8,
    pub token0_is_main: bool,
    pub direction: SwapDirection,
}

pub static V2_SWAP_EVENT_ID: &str = "0xd78ad95f";

pub async fn debug_trace_call(
    provider: RootProvider<PubSubFrontend>,
    new_block: &NewBlock,
    pending_tx: &NewPendingTx,
) -> Result<Option<CallFrame>> {
    let mut opts = GethDebugTracingCallOptions::default();
}
