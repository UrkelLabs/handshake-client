use jsonrpc::error::Error;

use crate::client::HSClient;

use serde_json;
use serde_json::json;

impl HSClient {
    pub fn pruneblockchain(&mut self) -> Result<(), Error> {
        self.call("pruneblockchain", &[])
    }

    pub fn invalidateblock(&mut self, blockhash: String) -> Result<(), Error> {
        self.call("invalidateblock", &[json!(blockhash)])
    }

    pub fn reconsiderblock(&mut self, blockhash: String) -> Result<(), Error> {
        self.call("reconsiderblock", &[json!(blockhash)])
    }

}
