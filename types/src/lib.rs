// use extended_primitives::{Buffer, Hash, Uint256};
use extended_primitives::{Buffer, Hash};
use handshake_primitives::{Address, Covenant};
use handshake_types::{amount, Amount, Compact};
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
    #[serde(with = "amount::as_doos")]
    pub balance: Amount,
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
    #[serde(with = "amount::as_hns")]
    pub pay_tx_fee: Amount,
    #[serde(rename = "relayfee")]
    #[serde(with = "amount::as_hns")]
    pub relay_fee: Amount,
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
    pub hash: Hash,
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
    pub merkle_root: Hash,
    #[serde(rename = "witnessroot")]
    pub witness_root: Hash,
    #[serde(rename = "treeroot")]
    pub tree_root: Hash,
    #[serde(rename = "reservedroot")]
    pub reserved_root: Hash,
    pub mask: Hash,
    pub coinbase: Vec<String>,
    pub tx: Vec<String>,
    #[serde(rename = "nTx")]
    pub num_tx: u32,
    pub time: u64,
    pub mediantime: u64,
    pub nonce: u32,
    #[serde(rename = "extranonce")]
    pub extra_nonce: Buffer,
    pub bits: Compact,
    pub difficulty: f64,
    pub chainwork: String,
    #[serde(rename = "previousblockhash")]
    pub previous_blockhash: Option<Hash>,
    #[serde(rename = "nextblockhash")]
    pub next_blockhash: Option<Hash>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetBlockDetailed {
    pub hash: Hash,
    pub confirmations: u32,
    #[serde(rename = "strippedsize")]
    pub stripped_size: u64,
    pub size: u64,
    pub weight: u64,
    pub height: u32,
    pub version: u32,
    #[serde(rename = "versionHex")]
    pub verion_hex: Buffer,
    #[serde(rename = "merkleroot")]
    pub merkle_root: Hash,
    #[serde(rename = "witnessroot")]
    pub witness_root: Hash,
    #[serde(rename = "treeroot")]
    pub tree_root: Hash,
    #[serde(rename = "reservedroot")]
    pub reserved_root: Hash,
    pub mask: Hash,
    pub tx: Vec<Transaction>,
    #[serde(rename = "nTx")]
    pub num_tx: u32,
    pub time: u64,
    pub mediantime: u64,
    pub nonce: u32,
    #[serde(rename = "extranonce")]
    pub extra_nonce: Buffer,
    pub bits: Compact,
    pub difficulty: f64,
    pub chainwork: String,
    #[serde(rename = "previousblockhash")]
    pub previous_blockhash: Option<Hash>,
    #[serde(rename = "nextblockhash")]
    pub next_blockhash: Option<Hash>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Transaction {
    pub txid: Hash,
    pub hash: Hash,
    pub size: u32,
    pub vsize: u32,
    pub version: u32,
    pub locktime: u32,
    pub vin: Vec<VirtualInput>,
    pub vout: Vec<VirtualOutput>,
    pub blockhash: Option<Hash>,
    pub confirmations: u32,
    pub time: u64,
    pub blocktime: u64,
    pub hex: Option<String>,
}

//@todo txid and vout will be Prevout.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VirtualInput {
    pub coinbase: bool,
    pub txid: Hash,
    pub vout: usize,
    pub txinwitness: Vec<String>,
    pub sequence: u32,
    pub link: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VirtualOutput {
    #[serde(with = "amount::as_hns")]
    pub value: Amount,
    pub n: usize,
    pub address: Address,
    pub covenant: Covenant,
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
    #[serde(rename = "reservedroot")]
    pub reserved_root: String,
    pub mask: String,
    pub time: u64,
    pub mediantime: u64,
    pub nonce: u32,
    #[serde(rename = "extranonce")]
    pub extra_nonce: Buffer,
    pub bits: Compact,
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
    #[serde(with = "amount::as_hns")]
    pub mempool_min_fee: Amount,
}

//@todo this is probably be exposed from rsd.
/// "getmempoolentry"
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MempoolEntry {
    pub size: u32,
    #[serde(with = "amount::as_hns")]
    pub fee: Amount,
    #[serde(rename = "modifiedfee")]
    #[serde(with = "amount::as_hns")]
    pub modified_fee: Amount,
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
    #[serde(with = "amount::as_doos")]
    pub descendant_fees: Amount,
    #[serde(rename = "ancestorcount")]
    pub ancestor_count: u32,
    #[serde(rename = "ancestorsize")]
    pub ancestor_size: u32,
    #[serde(rename = "ancestorfees")]
    #[serde(with = "amount::as_doos")]
    pub ancestor_fees: Amount,
    pub depends: Vec<String>,
}

/// "estimatesmartfee"
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EstimateSmartFee {
    //@todo this can be a -1 so we should almost make a type of Option<Amount> where -1 = None.
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

// --- Network Struct --- //

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PeerInfo {
    pub id: u32,
    pub addr: String,
    #[serde(rename = "addrlocal")]
    pub addr_local: String,
    pub name: Option<String>,
    pub services: String,
    #[serde(rename = "relaytxes")]
    pub relay_txes: bool,
    #[serde(rename = "lastsend")]
    pub last_send: i64,
    #[serde(rename = "lastrecv")]
    pub last_recv: i64,
    #[serde(rename = "bytessent")]
    pub bytes_sent: i64,
    #[serde(rename = "bytesrecv")]
    pub bytes_recv: i64,
    #[serde(rename = "conntime")]
    pub conn_time: i64,
    #[serde(rename = "timeoffset")]
    pub time_offset: i64,
    #[serde(rename = "pingtime")]
    pub ping_time: f64,
    #[serde(rename = "minping")]
    pub min_ping: f64,
    pub version: u32,
    #[serde(rename = "subver")]
    pub sub_ver: String,
    pub inbound: bool,
    #[serde(rename = "startingheight")]
    pub starting_height: i32,
    #[serde(rename = "besthash")]
    pub best_hash: String,
    #[serde(rename = "bestheight")]
    pub best_height: i32,
    #[serde(rename = "banscore")]
    pub ban_score: i32,
    pub inflight: Vec<String>,
    pub whitelisted: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BannedNode {
    pub address: String,
    pub banned_until: u64,
    pub ban_created: u64,
    pub ban_reason: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NetworkInfo {
    pub version: String,
    #[serde(rename = "subversion")]
    pub sub_version: String,
    #[serde(rename = "protocolversion")]
    pub protocol_version: u32,
    #[serde(rename = "identitykey")]
    pub identity_key: String,
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
    #[serde(with = "amount::as_hns")]
    pub relay_fee: Amount,
    #[serde(rename = "incrementalfee")]
    #[serde(with = "amount::as_hns")]
    pub incremental_fee: Amount,
    #[serde(rename = "localaddresses")]
    pub local_addresses: Vec<LocalAddress>,
    pub warnings: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LocalAddress {
    pub address: String,
    pub port: u32,
    pub score: u32,
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
    pub total_bytes_recv: u64,
    #[serde(rename = "totalbytessent")]
    pub total_bytes_sent: u64,
    #[serde(rename = "timemillis")]
    pub time_millis: u64,
}

/// "createmultisig" command
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMultiSig {
    pub address: String,
    pub redeem_script: String,
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateClaim {
    pub name: String,
    pub target: String,
    #[serde(with = "amount::as_doos")]
    pub value: Amount,
    pub size: u64,
    #[serde(with = "amount::as_doos")]
    pub fee: Amount,
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

//@todo needs to actually be an enum -> See: https://handshake-org.github.io/api-docs/#getnameproof
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
    pub claimed: u32,
    pub renewals: u32,
    pub registered: bool,
    pub expired: bool,
    pub weak: bool,
    pub stats: Option<NameStats>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NameOwner {
    pub hash: String,
    pub index: u64,
}

//TODO make this an Enum for all posibilities
// #[derive(Debug, Clone, Deserialize, Serialize)]
// #[serde(rename_all = "camelCase")]
// pub struct NameStats {
//     renewal_period_start: u32,
//     renewal_period_end: u32,
//     blocks_until_expire: u32,
//     days_until_expire: f64,
// }

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum NameStats {
    Open(OpeningStats),
    Locked(LockedStats),
    Bid(BiddingStats),
    Reveal(RevealStats),
    Revoke(RevokedStats),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpeningStats {
    open_period_start: u32,
    open_period_end: u32,
    blocks_until_bidding: u32,
    hours_until_bidding: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LockedStats {
    lockup_period_start: u32,
    lockup_period_end: u32,
    blocks_until_closed: u32,
    hours_until_closed: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BiddingStats {
    bid_period_start: u32,
    bid_period_end: u32,
    blocks_until_reveal: u32,
    hours_until_reveal: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RevealStats {
    reveal_period_start: u32,
    reveal_period_end: u32,
    blocks_until_close: u32,
    hours_until_close: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RevokedStats {
    revoke_period_start: u32,
    revoke_period_end: u32,
    blocks_until_reopen: u32,
    hours_until_reopen: f64,
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

// fn as_doos<'de, D>(deserializer: D) -> Result<T, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     // This is a Visitor that forwards string types to T's `FromStr` impl and
//     // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
//     // keep the compiler from complaining about T being an unused generic type
//     // parameter. We need T in order to know the Value type for the Visitor
//     // impl.
//     struct StringOrStruct<T>(PhantomData<fn() -> T>);

//     impl<'de, T> Visitor<'de> for StringOrStruct<T>
//     where
//         T: Deserialize<'de> + FromStr<Err = Void>,
//     {
//         type Value = T;

//         fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//             formatter.write_str("string or map")
//         }

//         fn visit_str<E>(self, value: &str) -> Result<T, E>
//         where
//             E: de::Error,
//         {
//             Ok(FromStr::from_str(value).unwrap())
//         }

//         fn visit_map<M>(self, map: M) -> Result<T, M::Error>
//         where
//             M: MapAccess<'de>,
//         {
//             // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
//             // into a `Deserializer`, allowing it to be used as the input to T's
//             // `Deserialize` implementation. T then deserializes itself using
//             // the entries from the map visitor.
//             Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
//         }
//     }

//     deserializer.deserialize_any(StringOrStruct(PhantomData))
// }
//
