use crate::responses;

use jsonrpc::error::Error;

use crate::client::RPCClient;

use serde_json;
use serde_json::json;

impl RPCClient {
    pub fn get_tx_out(
        &self,
        txid: &str,
        index: &u32,
        includemempool: bool,
    ) -> Result<responses::GetTxOut, Error> {
        self.call(
            "gettxout",
            &[json!(txid), json!(index), json!(includemempool)],
        )
    }

    pub fn get_tx_out_set_info(&self) -> Result<responses::GetTxOutSetInfo, Error> {
        self.call("gettxoutsetinfo", &[])
    }

    pub fn get_raw_transaction(&self, txhash: &str) -> Result<responses::GetRawTransaction, Error> {
        self.call("getrawtransaction", &[json!(txhash), json!(false)])
    }

    pub fn get_raw_transaction_verbose(
        &self,
        txhash: &str,
    ) -> Result<responses::RawTransaction, Error> {
        self.call("getrawtransaction", &[json!(txhash), json!(true)])
    }

    pub fn decode_raw_transaction(&self, rawtx: &str) -> Result<responses::RawTransaction, Error> {
        self.call("decoderawtransaction", &[json!(rawtx)])
    }

    pub fn decode_script(&self, script: &str) -> Result<responses::DecodeScript, Error> {
        self.call("decodescript", &[json!(script)])
    }

    pub fn send_raw_transaction(
        &self,
        rawtx: &str,
    ) -> Result<responses::SendRawTransaction, Error> {
        self.call("sendrawtransaction", &[json!(rawtx)])
    }

    ////Not sure how we are going to implement this one - TODO
    //// pub fn createrawtransaction(
    ////
    ////TODO signrawtransaction

    pub fn get_tx_out_proof(
        &self,
        txidlist: &Vec<String>,
        blockhash: &str,
    ) -> Result<responses::GetTxOutProof, Error> {
        self.call("gettxoutproof", &[json!(txidlist), json!(blockhash)])
    }

    pub fn verify_tx_out_proof(&self, proof: &str) -> Result<responses::VerifyTxOutProof, Error> {
        self.call("verifytxoutproof", &[json!(proof)])
    }
}
