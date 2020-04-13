use crate::Result;
use rpc_json_client::{ClientBuilder, RpcClient};
use std::sync::Arc;

#[derive(Clone)]
pub struct HandshakeWalletRpcClient {
    client: Arc<RpcClient>,
}

impl HandshakeWalletRpcClient {
    pub fn new(uri: &str) -> Self {
        let client = ClientBuilder::new(uri).with_retry().build();

        HandshakeWalletRpcClient {
            client: Arc::new(client),
        }
    }

    //TODO can we change params to be an Into<Value>? Then we can remove all those serde json
    //macros in all the requests.
    pub(crate) async fn call<T>(&self, method: &str, params: &[serde_json::Value]) -> Result<T>
    where
        T: for<'a> serde::de::Deserialize<'a>,
        // V: Into<serde_json::Value>
    {
        let res = self.client.execute(method, params).await?;

        Ok(res)
    }
}
