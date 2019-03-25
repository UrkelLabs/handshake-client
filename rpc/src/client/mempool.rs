use crate::responses;

use jsonrpc::error::Error;

use crate::client::RPCClient;

use serde_json;
use serde_json::json;

impl RPCClient {
    pub fn get_mempool_info(&self) -> Result<responses::GetMempoolInfo, Error> {
        self.call("getmempoolinfo", &[])
    }

    pub fn get_mempool_ancestors(
        &self,
        txhash: &str,
    ) -> Result<responses::GetMempoolAncestors, Error> {
        self.call("getmempoolancestors", &[json!(txhash), json!(false)])
    }

    pub fn get_mempool_ancestors_verbose(
        &self,
        txhash: &str,
    ) -> Result<responses::GetMempoolAncestorsVerbose, Error> {
        self.call("getmempoolancestors", &[json!(txhash), json!(true)])
    }

    pub fn get_mempool_descendants(
        &self,
        txhash: &str,
    ) -> Result<responses::GetMempoolDescendants, Error> {
        self.call("getmempooldescendants", &[json!(txhash), json!(false)])
    }

    pub fn get_mempool_descendants_verbose(
        &self,
        txhash: &str,
    ) -> Result<responses::GetMempoolDescendants, Error> {
        self.call("getmempooldescendants", &[json!(txhash), json!(true)])
    }

    pub fn get_mempool_entry(&self, txhash: &str) -> Result<responses::MempoolEntry, Error> {
        self.call("getmempoolentry", &[json!(txhash)])
    }

    pub fn get_raw_mempool(&self) -> Result<responses::GetRawMempool, Error> {
        self.call("getrawmempool", &[json!(false)])
    }

    pub fn get_raw_mempool_verbose(&self) -> Result<responses::GetRawMempoolVerbose, Error> {
        self.call("getrawmempool", &[json!(true)])
    }

    pub fn prioritise_transaction(
        &self,
        txhash: &str,
        priority_delta: u32,
        fee_delta: u32,
    ) -> Result<responses::PrioritiseTransaction, Error> {
        self.call(
            "prioritisetransaction",
            &[json!(txhash), json!(priority_delta), json!(fee_delta)],
        )
    }

    pub fn estimate_fee(&self, nblocks: &u32) -> Result<responses::EstimateFee, Error> {
        self.call("estimatefee", &[json!(nblocks)])
    }

    pub fn estimate_priority(&self, nblocks: &u32) -> Result<responses::EstimatePriority, Error> {
        self.call("estimatepriority", &[json!(nblocks)])
    }

    pub fn estimate_smart_fee(&self, nblocks: &u32) -> Result<responses::EstimateSmartFee, Error> {
        self.call("estimatesmartfee", &[json!(nblocks)])
    }

    pub fn estimate_smart_priority(
        &self,
        nblocks: &u32,
    ) -> Result<responses::EstimateSmartPriority, Error> {
        self.call("estimatesmartpriority", &[json!(nblocks)])
    }
}
