use crate::error::Error;
// use jsonrpc::client::Client;
use hyper::client::{Client, HttpConnector};
use futures::lock::Mutex;
use std::sync::Arc;

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;

mod block;
mod chain;
mod mempool;
mod mining;
mod network;
mod node;
mod tx;

#[derive(Debug, PartialEq, Serialize)]
pub struct Request {
    pub method: String,
    pub params: Vec<serde_json::Value>,
    pub id: serde_json::Value,
    pub jsonrpc: Option<String>,
}

pub struct RPCClient {
    client: Client<HttpConnector>,
    id: Arc<Mutex<u64>>,
}

impl RPCClient {
    //Create a new HSClient
    pub fn new(uri: &str) -> RPCClient {
        RPCClient {
            // client: Client::new(uri.to_owned(), None, None),
            client: Client::new(),
            id: Arc::new(Mutex::new(0)),
        }
    }

    async fn build_request(&self, name: &str, params: &[serde_json::Value]) -> Request {
        let mut id = self.id.lock().await;
        *id += 1;
        Request {
            method: name.to_owned(),
            params: params.to_vec(),
            id: From::from(*id),
            jsonrpc: Some("2.0".to_owned()),
        }


    }

    /// Generic call function for RPC calls.
    fn call<U: DeserializeOwned>(
        &self,
        method: &str,
        // input: T,
        args: &[serde_json::Value],
    ) -> Result<U, Error> {
        // let params = serde_json::to_value(input)?;

        // dbg!(&params);

        // let arg = params.as_object().unwrap().iter().map(|(_, param)| param.clone()).collect::<Vec<_>>();

        // dbg!(&arg);

        let request = self.build_request(method, args).await;

        // self.client
        //     .send_request(&request)
        //     .and_then(|res| res.into_result::<U>())
        //     .map_err(Error::from)
    }
}
