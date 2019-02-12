use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;

use crate::responses;
use crate::requests;

use jsonrpc::client::Client;
use jsonrpc::error::Error;

pub struct HSClient {
    client: Client
}

//Todo use Handshake Network settings.
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

    /// Show information about this node.
    pub fn getinfo(&mut self) -> Result<responses::GetInfo, Error> {
        self.call("getinfo", requests::GetInfo {})
    }

    /// Set the log level on the node.
    pub fn setloglevel(&mut self, level: String) -> Result<(), Error> {
        self.call("setloglevel", requests::SetLogLevel { level })
    }


}

