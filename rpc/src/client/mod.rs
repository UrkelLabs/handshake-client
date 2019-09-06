use rpc_json_client::RpcClient;
use crate::Result;


mod block;
mod chain;
mod mempool;
mod mining;
mod network;
mod node;
mod names;
mod tx;

//TODO can we use runtime here, and force our own executor into hyper?

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
    async fn call<T: for<'a> serde::de::Deserialize<'a>>(
        &self,
        method: &str,
        params: &[serde_json::Value],
    ) -> Result<T> {
        let res =  self.client.execute(method, params).await?;

        Ok(res)
    }
}
