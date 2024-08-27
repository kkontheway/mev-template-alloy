use alloy::providers::{Provider, ProviderBuilder, RootProvider};
use alloy::transports::BoxTransport;
use std::sync::Arc;
pub struct Config {
    #[allow(dead_code)]
    pub http: Arc<RootProvider<BoxTransport>>,
    #[allow(dead_code)]
    pub ws: Arc<RootProvider<BoxTransport>>,
}

impl Config {
    pub async fn new() -> Self {
        let network = std::env::var("NETWORK_RPC").expect("missing NETWORK_RPC");
    }
}

pub async fn run() {}
