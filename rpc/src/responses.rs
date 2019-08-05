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
    pub redeem_script: String,
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
    pub softforks: String,
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
    pub next_blockhash: Option<String>,
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

pub type GetChainTips = Vec<ChainTip>;

/// "getdifficulty"
pub type GetDifficulty = f64;

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
    pub blocks: u32,
}

/// "estimatesmartpriority
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EstimateSmartPriority {
    pub priority: f64,
    pub blocks: u32,
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
    pub time: u32,
}

//TODO needs some serious help with types and naming
/// "getblocktemplate" command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BlockTemplate {
    pub capabilities: Vec<String>,
    pub mutable: Vec<String>,
    pub version: u32,
    pub rules: Vec<String>,
    //Not going to work
    #[serde(rename = "vbavailable")]
    pub vb_available: VbAvailable,
    #[serde(rename = "vbrequired")]
    pub vb_required: u32,
    pub height: u32,
    #[serde(rename = "previousblockhash")]
    pub previous_blockhash: String,
    #[serde(rename = "treeroot")]
    pub tree_root: String,
    #[serde(rename = "filterroot")]
    pub filter_root: String,
    #[serde(rename = "reservedroot")]
    pub reserved_root: String,
    pub target: String,
    pub bits: String,
    #[serde(rename = "noncerange")]
    pub nonce_range: String,
    #[serde(rename = "curtime")]
    pub cur_time: u64,
    #[serde(rename = "mintime")]
    pub min_time: u64,
    #[serde(rename = "maxtime")]
    pub max_time: u64,
    pub expires: u64,
    #[serde(rename = "sigoplimit")]
    pub sig_op_limit: u64,
    #[serde(rename = "sizelimit")]
    pub size_limit: u64,
    #[serde(rename = "weightlimit")]
    pub weight_limit: u64,
    #[serde(rename = "longpollid")]
    pub long_poll_id: String,
    #[serde(rename = "submitold")]
    pub submit_old: bool,
    #[serde(rename = "coinbaseaux")]
    pub coinbase_aux: CoinbaseAux,
    #[serde(rename = "coinbasevalue")]
    pub coinbase_value: u64,
    pub claims: Vec<String>,
    pub airdrops: Vec<String>,
    pub transactions: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VbAvailable {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CoinbaseAux {
    pub flags: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Address {
    pub version: u32,
    pub hash: String,
}

/// "gettxout"
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetTxOut {
    pub bestblock: String,
    pub confirmations: u32,
    pub value: u64,
    pub address: Address,
    pub version: u32,
    pub coinbase: bool,
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
    pub total_burned: f64,
}

/// "getrawtransaction" verbose = false
pub type GetRawTransaction = String;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Vin {
    pub coinbase: bool,
    pub txid: String,
    pub vout: u64,
    pub txinwitness: Vec<String>,
    pub sequence: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Covenant {
    #[serde(rename = "type")]
    type_: u32,
    //TODO make this generic -> I think this will break on specific covenants.
    items: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Vout {
    pub value: u64,
    pub n: u32,
    pub address: Address,
    pub covenant: Covenant,
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
    pub blocktime: u64,
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

// --- Network Struct --- //

pub type ConnectionCount = u32;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PeerInfo {
    pub id: u32,
    pub addr: String,
    #[serde(rename = "addrlocal")]
    pub addr_local: String,
    pub name: String,
    pub services: String,
    #[serde(rename = "relaytxes")]
    pub relay_txes: bool,
    #[serde(rename = "lastsend")]
    pub last_send: u64,
    #[serde(rename = "lastrecv")]
    pub last_recv: u64,
    #[serde(rename = "bytessent")]
    pub bytes_sent: u64,
    #[serde(rename = "bytesrecv")]
    pub bytes_recv: u64,
    #[serde(rename = "conntime")]
    pub conn_time: u64,
    #[serde(rename = "timeoffset")]
    pub time_offset: u64,
    #[serde(rename = "pingtime")]
    pub ping_time: f64,
    #[serde(rename = "minping")]
    pub min_ping: u64,
    pub version: u32,
    #[serde(rename = "subver")]
    pub sub_ver: String,
    pub inbound: bool,
    #[serde(rename = "startingheight")]
    pub starting_height: u32,
    #[serde(rename = "besthash")]
    pub best_hash: String,
    #[serde(rename = "bestheight")]
    pub best_height: u32,
    #[serde(rename = "banscore")]
    pub ban_score: u32,
    pub inflight: Vec<String>,
    pub whitelisted: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NodeAddress {
    pub address: String,
    pub connected: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AddedNodeInfo {
    pub addednode: String,
    pub connected: bool,
    pub addresses: Vec<NodeAddress>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NetTotals {
    #[serde(rename = "totalbytesrecv")]
    total_bytes_recv: u64,
    #[serde(rename = "totalbytessent")]
    total_bytes_sent: u64,
    #[serde(rename = "timemillis")]
    time_millis: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LocalAddress {
    pub address: String,
    pub port: u32,
    pub score: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NetworkInfo {
    pub version: String,
    #[serde(rename = "subversion")]
    pub sub_version: String,
    #[serde(rename = "protocolversion")]
    pub protocol_version: String,
    #[serde(rename = "localservices")]
    pub local_services: String,
    #[serde(rename = "localrelay")]
    pub local_relay: bool,
    #[serde(rename = "timeoffset")]
    pub time_offset: u64,
    #[serde(rename = "networkactive")]
    pub network_active: bool,
    pub connections: u32,
    pub networks: Vec<String>,
    #[serde(rename = "relayfee")]
    pub relay_fee: f64,
    #[serde(rename = "incrementalfee")]
    pub incremental_fee: f64,
    #[serde(rename = "localaddresses")]
    pub local_addresses: Vec<LocalAddress>,
    pub warnings: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BannedNode {
    pub address: String,
    pub banned_until: u64,
    pub ban_created: u64,
    pub ban_reason: String,
}

// --- Naming Responses --- //

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateClaim {
    pub name: String,
    pub target: String,
    pub value: u64,
    pub size: u64,
    pub fee: u64,
    pub address: String,
    pub txt: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NameProof {
    pub hash: String,
    pub height: u32,
    pub root: String,
    pub name: String,
    pub key: String,
    pub proof: Proof,
}

//TODO needs to actually be an enum -> See: https://handshake-org.github.io/api-docs/#getnameproof
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Proof {
    #[serde(rename = "type")]
    pub type_: String,
    pub depth: u32,
    pub nodes: Vec<Vec<String>>,
    pub value: String,
}

//TODO no idea if this works -> Please test
pub type NameResource = HashMap<String, String>;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    pub name: String,
    pub name_hash: String,
    pub state: String,
    pub height: u32,
    pub renewal: u32,
    pub owner: NameOwner,
    pub value: u64,
    pub highest: u64,
    pub data: String,
    pub transfer: u32,
    pub revoked: u32,
    pub claimed: bool,
    pub weak: bool,
    pub stats: NameStats,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NameOwner {
    pub hash: String,
    pub index: u64,
}

//TODO make this an Enum for all posibilities
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NameStats {
    renewal_period_start: u32,
    renewal_period_end: u32,
    blocks_until_expire: u32,
    days_until_expire: f64,
}

//TODO check output types
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NameInfo {
    pub start: NameStart,
    pub info: Name,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NameStart {
    pub reserved: bool,
    pub week: u32,
    pub start: u32,
}
