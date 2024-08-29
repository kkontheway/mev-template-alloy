use std::env;
use std::str::FromStr;
pub static PROJECT_NAME: &str = "mev_template_alloy";
/// 从环境变量中获取值，如果不存在则返回 None
pub fn get_env(key: &str) -> Option<String> {
    env::var(key).ok()
}

/// 从环境变量中获取值并解析为指定类型，如果失败则返回 None
pub fn parse_env<T: FromStr>(key: &str) -> Option<T> {
    get_env(key).and_then(|v| v.parse().ok())
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Env {
    pub https_url: String,
    pub wss_url: String,
    // pub bot_address: String,
    // pub private_key: String,
    // pub identity_key: String,
    // pub telegram_token: String,
    // pub telegram_chat_id: String,
    // pub use_alert: bool,
    // pub debug: bool,
}

impl Env {
    /// 创建新的 Env 实例，从环境变量中加载配置
    pub fn new() -> Result<Self, String> {
        Ok(Env {
            https_url: get_env("HTTPS_URL").unwrap_or_default(),
            wss_url: get_env("WSS_URL").unwrap_or_default(),
            // bot_address: get_env("BOT_ADDRESS").unwrap_or_default(),
            // private_key: get_env("PRIVATE_KEY").unwrap_or_default(),
            // identity_key: get_env("IDENTITY_KEY").unwrap_or_default(),
            // telegram_token: get_env("TELEGRAM_TOKEN").unwrap_or_default(),
            // telegram_chat_id: get_env("TELEGRAM_CHAT_ID").unwrap_or_default(),
            // use_alert: parse_env("USE_ALERT").unwrap_or(false),
            // debug: parse_env("DEBUG").unwrap_or(false),
        })
    }
}

/// Flashbots Builder 的地址
pub const COINBASE: &str = "0xDAFEA492D9c6733ae3d56b7Ed1ADB60692c98Bc5";

/// WETH 合约地址
pub const WETH: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";

/// WETH 余额存储槽
pub const WETH_BALANCE_SLOT: i32 = 3;

/// WETH 小数位数
pub const WETH_DECIMALS: u8 = 18;
