use crate::client::HandshakeRpcClient;
use crate::Result;
use handshake_client_types::{
    EstimateSmartFee, EstimateSmartPriority, GetMempoolInfo, MempoolEntry,
};
use serde_json::json;
use std::collections::HashMap;

impl HandshakeRpcClient {
    pub async fn get_mempool_info(&self) -> Result<GetMempoolInfo> {
        self.call("getmempoolinfo", &[]).await
    }

    //@todo should this be txhashes?
    pub async fn get_mempool_ancestors(&self, txhash: &str) -> Result<Vec<String>> {
        let params = vec![json!(txhash), json!(false)];
        self.call("getmempoolancestors", &params).await
    }

    pub async fn get_mempool_ancestors_verbose(
        &self,
        txhash: &str,
    ) -> Result<HashMap<String, MempoolEntry>> {
        let params = vec![json!(txhash), json!(true)];
        self.call("getmempoolancestors", &params).await
    }

    pub async fn get_mempool_descendants(&self, txhash: &str) -> Result<Vec<String>> {
        let params = vec![json!(txhash), json!(false)];
        self.call("getmempooldescendants", &params).await
    }

    pub async fn get_mempool_descendants_verbose(&self, txhash: &str) -> Result<Vec<MempoolEntry>> {
        let params = vec![json!(txhash), json!(true)];
        self.call("getmempooldescendants", &params).await
    }

    pub async fn get_mempool_entry(&self, txhash: &str) -> Result<MempoolEntry> {
        let params = vec![json!(txhash)];
        self.call("getmempoolentry", &params).await
    }

    pub async fn get_raw_mempool(&self) -> Result<Vec<String>> {
        let params = vec![json!(false)];
        self.call("getrawmempool", &params).await
    }

    pub async fn get_raw_mempool_verbose(&self) -> Result<HashMap<String, MempoolEntry>> {
        let params = vec![json!(true)];
        self.call("getrawmempool", &params).await
    }

    pub async fn prioritise_transaction(
        &self,
        txhash: &str,
        priority_delta: u32,
        fee_delta: u32,
    ) -> Result<bool> {
        let params = vec![json!(txhash), json!(priority_delta), json!(fee_delta)];
        self.call("prioritisetransaction", &params).await
    }

    //@todo return Amount from RSD
    pub async fn estimate_fee(&self, nblocks: u32) -> Result<f64> {
        let params = vec![json!(nblocks)];
        self.call("estimatefee", &params).await
    }

    pub async fn estimate_priority(&self, nblocks: u32) -> Result<f64> {
        let params = vec![json!(nblocks)];
        self.call("estimatepriority", &params).await
    }

    //@todo return Amount from RSD
    pub async fn estimate_smart_fee(&self, nblocks: u32) -> Result<EstimateSmartFee> {
        let params = vec![json!(nblocks)];
        self.call("estimatesmartfee", &params).await
    }

    pub async fn estimate_smart_priority(&self, nblocks: u32) -> Result<EstimateSmartPriority> {
        let params = vec![json!(nblocks)];
        self.call("estimatesmartpriority", &params).await
    }
}
