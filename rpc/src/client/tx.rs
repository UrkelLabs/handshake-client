use crate::responses;
use crate::client::HandshakeRpcClient;
use serde_json::json;
use crate::Result;

impl HandshakeRpcClient {
    pub async fn get_tx_out(
        &self,
        txid: &str,
        index: &u32,
        includemempool: bool,
    ) -> Result<responses::GetTxOut> {
        self.call(
            "gettxout",
            &[json!(txid), json!(index), json!(includemempool)],
        ).await
    }

    pub async fn get_tx_out_set_info(&self) -> Result<responses::GetTxOutSetInfo> {
        self.call("gettxoutsetinfo", &[]).await
    }

    pub async fn get_raw_transaction(&self, txhash: &str) -> Result<responses::GetRawTransaction> {
        self.call("getrawtransaction", &[json!(txhash), json!(false)]).await
    }

    pub async fn get_raw_transaction_verbose(
        &self,
        txhash: &str,
    ) -> Result<responses::RawTransaction> {
        self.call("getrawtransaction", &[json!(txhash), json!(true)]).await
    }

    pub async fn decode_raw_transaction(&self, rawtx: &str) -> Result<responses::RawTransaction> {
        self.call("decoderawtransaction", &[json!(rawtx)]).await
    }

    pub async fn decode_script(&self, script: &str) -> Result<responses::DecodeScript> {
        self.call("decodescript", &[json!(script)]).await
    }

    pub async fn send_raw_transaction(
        &self,
        rawtx: &str,
    ) -> Result<responses::SendRawTransaction> {
        self.call("sendrawtransaction", &[json!(rawtx)]).await
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
        self.call("gettxoutproof", &[json!(txidlist), json!(blockhash)]).await
    }

    pub async fn verify_tx_out_proof(&self, proof: &str) -> Result<responses::VerifyTxOutProof> {
        self.call("verifytxoutproof", &[json!(proof)]).await
    }
}
