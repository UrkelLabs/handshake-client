use serde_derive::{Deserialize, Serialize};

/// "getinfo" command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetInfo {
    pub version: String,
    #[serde(rename = "protocolversion")]
    pub protocol_version: u32,
    #[serde(rename = "walletversion")]
    pub wallet_version: u32,
    pub balance: u64,
    pub blocks: u64,
    pub timeoffset: i64,
    pub connections: u32,
    pub proxy: String,
    pub difficulty: f64,
    pub testnet: bool,
    #[serde(rename = "keypoololdest")]
    pub key_pool_oldest: u32,
    #[serde(rename = "keypoolsize")]
    pub key_pool_size: u32,
    pub unlocked_until: u32,
    #[serde(rename = "paytxfee")]
    pub pay_tx_fee: f64,
    #[serde(rename = "relayfee")]
    pub relay_fee: f64,
    pub errors: String
}

/// "getmemoryinfo" command
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMemoryInfo {
    pub total: u32,
    pub js_heap: u32,
    pub js_heap_total: u32,
    pub native_heap: u32,
    pub external: u32
}

/// "validateaddress" command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ValidateAddress {
     #[serde(rename = "isvalid")]
     pub is_valid: bool,
    // TODO transition to hash type
     pub address: Option<String>,
     #[serde(rename = "ismine")]
     pub is_mine: Option<bool>,
    #[serde(rename = "iswatchonly")]
    pub is_watch_only: Option<bool>
}

