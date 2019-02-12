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
