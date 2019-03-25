use jsonrpc::error::Error;

use crate::client::HSClient;

use serde_json;
use serde_json::json;

impl HSClient {
    pub fn prune_blockchain(&self) -> Result<(), Error> {
        self.call("pruneblockchain", &[])
    }

    pub fn invalidate_block(&self, blockhash: &str) -> Result<(), Error> {
        self.call("invalidateblock", &[json!(blockhash)])
    }

    pub fn reconsider_block(&self, blockhash: &str) -> Result<(), Error> {
        self.call("reconsiderblock", &[json!(blockhash)])
    }

}
