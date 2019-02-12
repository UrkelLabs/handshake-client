use serde_derive::{Deserialize, Serialize};

/// 'getinfo' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetInfo {}

/// 'getmemoryinfo' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMemoryInfo {}

/// 'setloglevel' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SetLogLevel {
    pub level: String,
}

/// 'validateaddress' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ValidateAddress {
    pub address: String,
}

// --- Mining Requests --- //
/// 'getnetworkhasps' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetNetworkHashps {
    pub blocks: u32,
    pub height: u32
}

/// 'getmininginfo' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMiningInfo {}

/// 'getwork' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetWork {
    // pub data: Option<String>,
}
// TODO this data looks like it CAN be passed...need to talk about optional params
