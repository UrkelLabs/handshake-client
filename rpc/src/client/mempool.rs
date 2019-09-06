use crate::client::HandshakeRpcClient;
use crate::responses;
use crate::Result;
use serde_json::json;

impl HandshakeRpcClient {
    pub async fn get_mempool_info(&self) -> Result<responses::GetMempoolInfo> {
        self.call("getmempoolinfo", &[]).await
    }

    pub async fn get_mempool_ancestors(
        &self,
        txhash: &str,
    ) -> Result<responses::GetMempoolAncestors> {
        let params = vec![json!(txhash), json!(false)];
        self.call("getmempoolancestors", &params).await
    }

    pub async fn get_mempool_ancestors_verbose(
        &self,
        txhash: &str,
    ) -> Result<responses::GetMempoolAncestorsVerbose> {
        let params = vec![json!(txhash), json!(true)];
        self.call("getmempoolancestors", &params).await
    }

    pub async fn get_mempool_descendants(
        &self,
        txhash: &str,
    ) -> Result<responses::GetMempoolDescendants> {
        let params = vec![json!(txhash), json!(false)];
        self.call("getmempooldescendants", &params).await
    }

    pub async fn get_mempool_descendants_verbose(
        &self,
        txhash: &str,
    ) -> Result<responses::GetMempoolDescendants> {
        let params = vec![json!(txhash), json!(true)];
        self.call("getmempooldescendants", &params).await
    }

    pub async fn get_mempool_entry(&self, txhash: &str) -> Result<responses::MempoolEntry> {
        let params = vec![json!(txhash)];
        self.call("getmempoolentry", &params).await
    }

    pub async fn get_raw_mempool(&self) -> Result<responses::GetRawMempool> {
        let params = vec![json!(false)];
        self.call("getrawmempool", &params).await
    }

    pub async fn get_raw_mempool_verbose(&self) -> Result<responses::GetRawMempoolVerbose> {
        let params = vec![json!(true)];
        self.call("getrawmempool", &params).await
    }

    pub async fn prioritise_transaction(
        &self,
        txhash: &str,
        priority_delta: u32,
        fee_delta: u32,
    ) -> Result<responses::PrioritiseTransaction> {
        let params = vec![json!(txhash), json!(priority_delta), json!(fee_delta)];
        self.call("prioritisetransaction", &params).await
    }

    pub async fn estimate_fee(&self, nblocks: u32) -> Result<responses::EstimateFee> {
        let params = vec![json!(nblocks)];
        self.call("estimatefee", &params).await
    }

    pub async fn estimate_priority(&self, nblocks: u32) -> Result<responses::EstimatePriority> {
        let params = vec![json!(nblocks)];
        self.call("estimatepriority", &params).await
    }

    pub async fn estimate_smart_fee(&self, nblocks: u32) -> Result<responses::EstimateSmartFee> {
        let params = vec![json!(nblocks)];
        self.call("estimatesmartfee", &params).await
    }

    pub async fn estimate_smart_priority(
        &self,
        nblocks: u32,
    ) -> Result<responses::EstimateSmartPriority> {
        let params = vec![json!(nblocks)];
        self.call("estimatesmartpriority", &params).await
    }
}
