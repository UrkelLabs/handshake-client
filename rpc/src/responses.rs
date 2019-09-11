use extended_primitives::{Buffer, Hash, Uint256};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

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
