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
    pub errors: String,
}

// --- Block Responses --- //

/// "getblockchaininfo" command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetBlockchainInfo {
    pub chain: String,
    pub blocks: u32,
    pub headers: u32,
    #[serde(rename = "bestblockhash")]
    pub best_blockhash: String,
    pub difficulty: f64,
    pub mediantime: u64,
    #[serde(rename = "verificationprogress")]
    pub verification_progress: f64,
    pub chainwork: String,
    pub pruned: bool,
    #[serde(rename = "pruneheight")]
    pub prune_height: Option<u32>,
    //Soft forks we need to develop out that struct @todo.
    #[serde(skip)]
    pub softforks: String,
}

/// "getblock" and "getblockbyheight"
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetBlock {
    pub hash: String,
    pub confirmations: u32,
    #[serde(rename = "strippedsize")]
    pub stripped_size: u32,
    pub size: u32,
    pub weight: u32,
    pub height: u32,
    pub version: u32,
    #[serde(rename = "versionHex")]
    pub verion_hex: String,
    #[serde(rename = "merkleroot")]
    pub merkle_root: String,
    #[serde(rename = "witnessroot")]
    pub witness_root: String,
    #[serde(rename = "treeroot")]
    pub tree_root: String,
    #[serde(rename = "filterroot")]
    pub filter_root: String,
    #[serde(rename = "reservedroot")]
    pub reserved_root: String,
    pub coinbase: Vec<String>,
    pub tx: Vec<String>,
    pub time: u64,
    pub mediantime: u64,
    pub bits: u64,
    pub difficulty: f64,
    pub chainwork: String,
    #[serde(rename = "previousblockhash")]
    pub previous_blockhash: String,
    #[serde(rename = "nextblockhash")]
    pub next_blockhash: Option<String>,
}

/// "getchaintips"
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChainTip {
    pub height: u32,
    pub hash: String,
    pub branchlen: u32,
    pub status: String,
}

/// "getblockheader"
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetBlockHeader {
    pub hash: String,
    pub confirmations: u32,
    pub height: u32,
    pub version: u32,
    #[serde(rename = "versionHex")]
    pub verion_hex: String,
    #[serde(rename = "merkleroot")]
    pub merkle_root: String,
    #[serde(rename = "witnessroot")]
    pub witness_root: String,
    #[serde(rename = "treeroot")]
    pub tree_root: String,
    #[serde(rename = "filterroot")]
    pub filter_root: String,
    #[serde(rename = "reservedroot")]
    pub reserved_root: String,
    pub time: u64,
    pub mediantime: u64,
    pub bits: u64,
    pub difficulty: f64,
    pub chainwork: String,
    #[serde(rename = "previousblockhash")]
    pub previous_blockhash: String,
    #[serde(rename = "nextblockhash")]
    pub next_blockhash: Option<String>,
}

// --- Mempool Responses --- //

/// "getmempoolinfo"
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMempoolInfo {
    pub size: u32,
    pub bytes: u64,
    pub usage: u64,
    pub maxmempool: u64,
    #[serde(rename = "mempoolminfee")]
    pub mempool_min_fee: f64,
}

//@todo this is probably be exposed from rsd.
/// "getmempoolentry"
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MempoolEntry {
    pub size: u32,
    pub fee: f64,
    #[serde(rename = "modifiedfee")]
    pub modified_fee: f64,
    pub time: u64,
    pub height: u32,
    //Double check if these should be floats XXX
    #[serde(rename = "startingpriority")]
    pub starting_priority: f64,
    #[serde(rename = "currentpriority")]
    pub current_priority: f64,
    #[serde(rename = "descendantcount")]
    pub descendant_count: u32,
    #[serde(rename = "descendantsize")]
    pub descendant_size: u32,
    #[serde(rename = "descendantfees")]
    pub descendant_fees: f64,
    #[serde(rename = "ancestorcount")]
    pub ancestor_count: u32,
    #[serde(rename = "ancestorsize")]
    pub ancestor_size: u32,
    #[serde(rename = "ancestorfees")]
    pub ancestor_fees: f64,
    pub depends: Vec<String>,
}

/// "estimatesmartfee"
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EstimateSmartFee {
    pub fee: f64,
    pub blocks: u32,
}

/// "estimatesmartpriority
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EstimateSmartPriority {
    pub priority: f64,
    pub blocks: u32,
}
