use crate::client::HandshakeRpcClient;
use crate::responses;
use crate::Result;
use serde_json::json;

impl HandshakeRpcClient {
    pub async fn get_tx_out(
        &self,
        txid: &str,
        index: &u32,
        includemempool: bool,
    ) -> Result<responses::GetTxOut> {
        let params = vec![json!(txid), json!(index), json!(includemempool)];
        self.call("gettxout", &params).await
    }

    pub async fn get_tx_out_set_info(&self) -> Result<responses::GetTxOutSetInfo> {
        self.call("gettxoutsetinfo", &[]).await
    }

    pub async fn get_raw_transaction(&self, txhash: &str) -> Result<responses::GetRawTransaction> {
        let params = vec![json!(txhash), json!(false)];
        self.call("getrawtransaction", &params).await
    }

    pub async fn get_raw_transaction_verbose(
        &self,
        txhash: &str,
    ) -> Result<responses::RawTransaction> {
        let params = vec![json!(txhash), json!(true)];
        self.call("getrawtransaction", &params).await
    }

    pub async fn decode_raw_transaction(&self, rawtx: &str) -> Result<responses::RawTransaction> {
        let params = vec![json!(rawtx)];
        self.call("decoderawtransaction", &params).await
    }

    pub async fn decode_script(&self, script: &str) -> Result<responses::DecodeScript> {
        let params = vec![json!(script)];
        self.call("decodescript", &params).await
    }

    pub async fn send_raw_transaction(&self, rawtx: &str) -> Result<responses::SendRawTransaction> {
        let params = vec![json!(rawtx)];
        self.call("sendrawtransaction", &params).await
    }

    ////Not sure how we are going to implement this one - TODO
    //// pub fn createrawtransaction(
    ////
    ////TODO signrawtransaction

    pub async fn get_tx_out_proof(
        &self,
        txidlist: &Vec<String>,
        blockhash: &str,
    ) -> Result<responses::GetTxOutProof> {
        let params = vec![json!(txidlist), json!(blockhash)];
        self.call("gettxoutproof", &params).await
    }

    pub async fn verify_tx_out_proof(&self, proof: &str) -> Result<responses::VerifyTxOutProof> {
        let params = vec![json!(proof)];
        self.call("verifytxoutproof", &params).await
    }
}
