use crate::responses;

use jsonrpc::error::Error;

use crate::client::HSClient;

use serde_json;
use serde_json::json;

impl HSClient {

    pub fn gettxout(&mut self, txid: String, index: u32, includemempool: bool) -> Result<responses::GetTxOut, Error>
    {
        self.call("gettxout", &[json!(txid), json!(index), json!(includemempool)])
    }

    pub fn gettxoutsetinfo(&mut self) -> Result<responses::GetTxOutSetInfo, Error> {
        self.call("gettxoutsetinfo", &[])
    }

    pub fn getrawtransaction(&mut self, txhash: String) -> Result<responses::GetRawTransaction, Error> {
        self.call("getrawtransaction", &[json!(txhash), json!(false)])
    }

    pub fn getrawtransactionverbose(&mut self, txhash: String) -> Result<responses::RawTransaction, Error> {
        self.call("getrawtransaction", &[json!(txhash), json!(true)])
    }

    pub fn decoderawtransaction(&mut self, rawtx: String) -> Result<responses::RawTransaction, Error> {
        self.call("decoderawtransaction", &[json!(rawtx)])
    }

    pub fn decodescript(&mut self, script: String) -> Result<responses::DecodeScript, Error> {
        self.call("decodescript", &[json!(script)])
    }

    pub fn sendrawtransaction(&mut self, rawtx: String) -> Result<responses::SendRawTransaction, Error> {
        self.call("sendrawtransaction", &[json!(rawtx)])
    }

    ////Not sure how we are going to implement this one - TODO
    //// pub fn createrawtransaction(
    ////
    ////TODO signrawtransaction

    pub fn gettxoutproof(&mut self, txidlist: Vec<String>, blockhash: String) -> Result<responses::GetTxOutProof, Error> {
        self.call("gettxoutproof", &[json!(txidlist), json!(blockhash)])
    }

    pub fn verifytxoutproof(&mut self, proof: String) -> Result<responses::VerifyTxOutProof, Error> {
        self.call("verifytxoutproof", &[json!(proof)])
    }

}
