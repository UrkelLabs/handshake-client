use crate::Result;
use futures::future::try_join_all;
use rpc_json_client::{ClientBuilder, RpcClient};
use std::sync::Arc;

#[derive(Clone)]
pub struct HandshakeRpcClient {
    client: Arc<RpcClient>,
}

impl HandshakeRpcClient {
    pub fn new(uri: &str) -> Self {
        //@todo probably expose this with a with_builder as well here.
        let client = ClientBuilder::new(uri).with_retry().build();
        HandshakeRpcClient {
            client: Arc::new(client),
        }
    }

    pub fn new_with_backups(uri: &str, backups: Vec<String>) -> Self {
        let client = ClientBuilder::new(uri)
            .with_retry()
            .with_backups(backups)
            .build();
        HandshakeRpcClient {
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

    pub(crate) async fn batch<T>(
        &self,
        method: &str,
        params_set: &[Vec<serde_json::Value>],
    ) -> Result<Vec<T>>
    where
        T: for<'a> serde::de::Deserialize<'a>,
        // V: Into<serde_json::Value>
    {
        let mut requests = Vec::new();
        for params in params_set {
            requests.push(self.client.execute(method, params));
        }

        Ok(try_join_all(requests).await?)
    }
}
