use crate::responses;

use jsonrpc::error::Error;

use crate::client::HSClient;

use serde_json;
use serde_json::json;

impl HSClient {
    pub fn getmempoolinfo(&mut self) -> Result<responses::GetMempoolInfo, Error> {
        self.call("getmempoolinfo", &[])
    }

    pub fn getmempoolancestors(&mut self, txhash: String) -> Result<responses::GetMempoolAncestors, Error> {
        self.call("getmempoolancestors", &[json!(txhash), json!(false)])
    }

    pub fn getmempoolancestorsverbose(&mut self, txhash: String) -> Result<responses::GetMempoolAncestorsVerbose, Error> {
        self.call("getmempoolancestors", &[json!(txhash), json!(true)])
    }

    pub fn getmempooldescendants(&mut self, txhash: String) -> Result<responses::GetMempoolDescendants, Error> {
        self.call("getmempooldescendants", &[json!(txhash), json!(false)])
    }

    pub fn getmempooldescendantsverbose(&mut self, txhash: String) -> Result<responses::GetMempoolDescendants, Error> {
        self.call("getmempooldescendants", &[json!(txhash), json!(true)])
    }

    pub fn getmempoolentry(&mut self, txhash: String) -> Result<responses::MempoolEntry, Error> {
        self.call("getmempoolentry", &[json!(txhash)])
    }

    pub fn getrawmempool(&mut self) -> Result<responses::GetRawMempool, Error> {
        self.call("getrawmempool", &[json!(false)])
    }

    pub fn getrawmempoolverbose(&mut self) -> Result<responses::GetRawMempoolVerbose, Error> {
        self.call("getrawmempool", &[json!(true)])
    }

    pub fn prioritisetransaction(&mut self, txhash: String, priority_delta: u32, fee_delta: u32) -> Result<responses::PrioritiseTransaction, Error> {
        self.call("prioritisetransaction", &[json!(txhash), json!(priority_delta), json!(fee_delta)])
    }

    pub fn estimatefee(&mut self, nblocks: u32) -> Result<responses::EstimateFee, Error> {
        self.call("estimatefee", &[json!(nblocks)])
    }

    pub fn estimatepriority(&mut self, nblocks: u32) -> Result<responses::EstimatePriority, Error> {
        self.call("estimatepriority", &[json!(nblocks)])
    }

    pub fn estimatesmartfee(&mut self, nblocks: u32) -> Result<responses::EstimateSmartFee, Error> {
        self.call("estimatesmartfee", &[json!(nblocks)])
    }

    pub fn estimatesmartpriority(&mut self, nblocks: u32) -> Result<responses::EstimateSmartPriority, Error> {
        self.call("estimatesmartpriority", &[json!(nblocks)])
    }
    

}