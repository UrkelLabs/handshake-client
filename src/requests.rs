use serde_derive::{Deserialize, Serialize};

/// 'getinfo' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetInfo {}

/// 'setloglevel' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SetLogLevel {
    pub level: String,
}
