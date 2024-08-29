use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use clap::builder::Str;
use eyre::Result;

pub mod block_scanner;
pub mod mempool;
use std::io::{self, Write};
// ... 现有的导入语句 ...

#[derive(Debug)]
struct Config {
    chain: String,
    connection_type: String,
    endpoint: String,
}

#[tokio::main]
async fn main() {
    println!("MEV_101");

    let config = get_user_config();
    println!("用户配置: {:?}", config);

    // 这里可以根据配置进行后续操作
    // 例如：根据 connection_type 创建相应的 provider
    match config.connection_type.as_str() {
        "HTTP" => {
            let provider = ProviderBuilder::new().on_http(config.endpoint.parse().unwrap());
            let last_block = provider.get_block_number().await.unwrap();
            println!("最新区块: {}", last_block);
        }
        "WS" => {
            println!("WebSocket 连接暂未实现");
            // 实现 WebSocket 连接逻辑
        }
        "IPC" => {
            println!("IPC 连接暂未实现");
            // 实现 IPC 连接逻辑
        }
        _ => println!("不支持的连接类型"),
    }
}

fn get_user_config() -> Config {
    let chain = select_chain();
    let connection_type = select_provider();
    let endpoint = prompt("请输入节点地址: ");

    Config {
        chain,
        connection_type,
        endpoint,
    }
}

// ... select_chain, select_provider, prompt 函数保持不变 ...
fn select_provider() -> String {
    loop {
        println!("\nChoose a provider:");
        println!("1. HTTP");
        println!("2. WS");
        println!("3. IPC");
        print!("请输入选项 (1-3): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => return "HTTP".to_string(),
            "2" => return "WS".to_string(),
            "3" => return "IPC".to_string(),

            _ => println!("无效选项，请重新选择"),
        }
    }
}

fn select_chain() -> String {
    loop {
        println!("\nChoose a chain:");
        println!("1. ETH");
        println!("2. BSC");
        println!("3. SOLANA");

        print!("Input your choice (1-3): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => return "ETH".to_string(),
            "2" => return "BSC".to_string(),
            "3" => return "SOLANA".to_string(),

            _ => println!("无效选项，请重新选择"),
        }
    }
}

fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
// #[tokio::main]
// async fn main() -> Result<()> {
//     let rpc_url =
//         "https://eth-mainnet.g.alchemy.com/v2/Q9tQTNzKb5PvbMwh5mcuj4p2eTMWea1M".parse()?;
//     let provider = ProviderBuilder::new().on_http(rpc_url);

//     let ws_url = "wss://eth-mainnet.g.alchemy.com/v2/Q9tQTNzKb5PvbMwh5mcuj4p2eTMWea1M";

//     let ws = WsConnect::new(ws_url);
//     let ws_provider = ProviderBuilder::new().on_ws(ws).await?;
//     println!("程序开始运行...");
//     tokio::spawn(async move {
//         block_scanner::block_scanner(provider).await;
//     });

//     mempool::mempool_scanner(ws_provider).await;

//     Ok(())
// }
