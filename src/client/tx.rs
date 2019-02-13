use crate::responses;

use jsonrpc::error::Error;

use crate::client::HSClient;

use serde_json;
use serde_json::json;

impl HSClient {

    pub fn gettxout(&self, txid: &str, index: &u32, includemempool: bool) -> Result<responses::GetTxOut, Error>
    {
        self.call("gettxout", &[json!(txid), json!(index), json!(includemempool)])
    }

    pub fn gettxoutsetinfo(&self) -> Result<responses::GetTxOutSetInfo, Error> {
        self.call("gettxoutsetinfo", &[])
    }

    pub fn getrawtransaction(&self, txhash: &str) -> Result<responses::GetRawTransaction, Error> {
        self.call("getrawtransaction", &[json!(txhash), json!(false)])
    }

    pub fn getrawtransactionverbose(&self, txhash: &str) -> Result<responses::RawTransaction, Error> {
        self.call("getrawtransaction", &[json!(txhash), json!(true)])
    }

    pub fn decoderawtransaction(&self, rawtx: &str) -> Result<responses::RawTransaction, Error> {
        self.call("decoderawtransaction", &[json!(rawtx)])
    }

    pub fn decodescript(&self, script: &str) -> Result<responses::DecodeScript, Error> {
        self.call("decodescript", &[json!(script)])
    }

    pub fn sendrawtransaction(&self, rawtx: &str) -> Result<responses::SendRawTransaction, Error> {
        self.call("sendrawtransaction", &[json!(rawtx)])
    }

    ////Not sure how we are going to implement this one - TODO
    //// pub fn createrawtransaction(
    ////
    ////TODO signrawtransaction

    pub fn gettxoutproof(&self, txidlist: &Vec<String>, blockhash: &str) -> Result<responses::GetTxOutProof, Error> {
        self.call("gettxoutproof", &[json!(txidlist), json!(blockhash)])
    }

    pub fn verifytxoutproof(&self, proof: &str) -> Result<responses::VerifyTxOutProof, Error> {
        self.call("verifytxoutproof", &[json!(proof)])
    }

}
