use jsonrpc::client::Client;
use jsonrpc::Error;

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;

mod node;
mod mining;

pub struct HSClient {
    client: Client
}

impl HSClient {
    //Create a new HSClient
    pub fn new(uri: &str) -> HSClient {
        HSClient {
            client: Client::new(uri.to_owned(), None, None),
        }
    }

    /// Generic call function for RPC calls.
    fn call<T: Serialize, U: DeserializeOwned>(
        &mut self,
        method: &str,
        input: T,
    ) -> Result<U, Error> {
        let params = serde_json::to_value(input)?;

        let arg = params.as_object().unwrap().iter().map(|(_, param)| param.clone()).collect::<Vec<_>>();

        let request = self.client.build_request(method, &arg);

        self.client
            .send_request(&request)
            .and_then(|res| res.into_result::<U>())

    }
}
