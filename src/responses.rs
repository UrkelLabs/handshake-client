use serde_derive::{Deserialize, Serialize};

use std::collections::HashMap;

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

/// "getmemoryinfo" command
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMemoryInfo {
    pub total: u32,
    pub js_heap: u32,
    pub js_heap_total: u32,
    pub native_heap: u32,
    pub external: u32,
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
    pub is_watch_only: Option<bool>,
}

/// 'stop' command
pub type Stop = String;

/// "createmultisig" command
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMultiSig {
    pub address: String,
    pub redeem_script: String
}

/// "signmessagewithprivkey" command
pub type SignMessageWithPrivKey = String;

/// "verifymessage" command
pub type VerifyMessage = bool;

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
    //Soft forks we need to develop out that struct XXX TODO.
    #[serde(skip)]
    pub softforks: String
}

/// "getbestblockhash"
pub type GetBestBlockHash = String;

/// "getblockcount"
pub type GetBlockCount = u32;

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
    pub next_blockhash: Option<String>
}

/// "getblockhash" 
pub type GetBlockHash = String;

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
    pub next_blockhash: Option<String>
}

/// "getchaintips"
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChainTip {
    pub height: u32,
    pub hash: String,
    pub branchlen: u32,
    pub status: String 
}

pub type GetChainTips = Vec<ChainTip>;

/// "getdifficulty"
pub type GetDifficulty = f64;


// --- Mempool Responses --- //

/// "getmempoolinfo"
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMempoolInfo{
    pub size: u32,
    pub bytes: u64,
    pub usage: u64,
    pub maxmempool: u64,
    #[serde(rename = "mempoolminfee")]
    pub mempool_min_fee: f64
}

/// "getmempoolancestors" verbose = 0
pub type GetMempoolAncestors = Vec<String>;

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

/// "getrawmempool"
pub type GetRawMempool = Vec<String>;

/// "getrawmempool"
pub type GetRawMempoolVerbose = HashMap<String, MempoolEntry>;

/// "getmempoolancestors" verbose = 1
pub type GetMempoolAncestorsVerbose = Vec<MempoolEntry>;

/// "getmempooldescendants" verbose = 0
pub type GetMempoolDescendants = Vec<String>;

/// "getmempooldescendants" verbose = 1
pub type GetMempoolDescendentsVerbose = Vec<MempoolEntry>;

/// "prioritisetransaction"
pub type PrioritiseTransaction = bool;

/// "estimatefee"
pub type EstimateFee = f64;

/// "estimatepriority"
pub type EstimatePriority = f64;

/// "estimatesmartfee"
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EstimateSmartFee {
    pub fee: f64,
    pub blocks: u32
}

/// "estimatesmartpriority
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EstimateSmartPriority {
    pub priority: f64,
    pub blocks: u32
}


// --- Mining Responses --- //
/// "getnetworkhashps" command
pub type GetNetworkHashps = f32;

/// "getmininginfo" command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMiningInfo {
    pub blocks: u32,
    #[serde(rename = "currentblocksize")]
    pub current_block_size: u32,
    #[serde(rename = "currentblockweight")]
    pub current_block_weight: u32,
    #[serde(rename = "currentblocktx")]
    pub currrent_block_tx: u32,
    pub difficulty: f32,
    pub errors: String,
    #[serde(rename = "genproclimit")]
    pub genproc_limit: u32,
    #[serde(rename = "networkhashps")]
    pub network_hashps: f32,
    #[serde(rename = "pooledtx")]
    pub pooled_tx: u32,
    pub testnet: bool,
    pub chain: String,
    pub generate: bool,
}

/// "getwork" command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetWork {
    pub network: String,
    pub data: String,
    pub target: String,
    pub height: u32,
    pub time: u32
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Address {
    pub version: u32,
    pub hash: String
}

/// "gettxout"
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetTxOut {
    pub bestblock: String,
    pub confirmations: u32,
    pub value: u64,
    pub address: Address,
    pub version: u32,
    pub coinbase: bool
}

/// "gettxoutsetinfo"
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetTxOutSetInfo {
    pub height: u32,
    pub bestblock: String,
    pub transactions: u32,
    pub txouts: u32,
    pub bytes_serialized: u32,
    pub hash_serialized: u32,
    pub total_amount: f64,
    pub total_burned: f64
}


/// "getrawtransaction" verbose = false
pub type GetRawTransaction = String;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Vin {
    pub coinbase: bool,
    pub txid: String,
    pub vout: u64,
    pub txinwitness: Vec<String>,
    pub sequence: u64
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Covenant {
    #[serde(rename = "type")]
    type_: u32,
    //TODO make this generic -> I think this will break on specific covenants.
    items: Vec<String>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Vout {
    pub value: u64,
    pub n: u32,
    pub address: Address,
    pub covenant: Covenant
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RawTransaction {
    pub txid: String,
    pub hash: String,
    pub size: u32,
    pub vsize: u32,
    pub version: u32,
    pub locktime: u64,
    pub vin: Vec<Vin>,
    pub vout: Vec<Vout>,
    pub blockhash: Option<String>,
    pub confirmations: u32,
    pub time: u64,
    pub blocktime: u64
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DecodeScript {
    pub asm: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "reqSigs")]
    pub req_sigs: u32,
    pub p2sh: String,
}

pub type SendRawTransaction = String;

pub type GetTxOutProof = String;

pub type VerifyTxOutProof = Vec<String>;
