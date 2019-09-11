use crate::Result;
use rpc_json_client::RpcClient;

mod block;
mod chain;
// mod mempool;
// mod mining;
// mod names;
// mod network;
// mod node;
// mod tx;

pub struct HandshakeRpcClient {
    client: RpcClient,
}

impl HandshakeRpcClient {
    pub fn new(uri: &str) -> Self {
        HandshakeRpcClient {
            client: RpcClient::new(uri),
        }
    }

    //TODO can we change params to be an Into<Value>? Then we can remove all those serde json
    //macros in all the requests.
    async fn call<T> (
        &self,
        method: &str,
        params: &[serde_json::Value],
    ) -> Result<T>
        where
        T: for<'a> serde::de::Deserialize<'a>,
        // V: Into<serde_json::Value>
    {
        let res = self.client.execute(method, params).await?;

        Ok(res)
    }
}
