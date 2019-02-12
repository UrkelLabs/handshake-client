use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;
use serde_json::json;

use crate::responses;
use crate::requests;

use jsonrpc::client::Client;
use jsonrpc::error::Error;

//TODO this will not support https at the moment. 
//We'll like need to deploy an enum that is either httpConnector or httpsconnector, but
//for the mean time I don't think anyone will be connecting to a daemon using https.
pub struct HSClient {
    client: Client
}

// let mut client = jsonrpc::client::Client::new("example.org".to_owned(), None, None);
//     let request = client.build_request("getmystruct".to_owned(), vec![]);
//     match client.send_request(&request).and_then(|res| res.into_result::<MyStruct>()) {
//         Ok(mystruct) => // Ok!
//         Err(e) => // Not so much.
//     }
//
// fn filter(params: serde_json::Value) -> [serde_json::Value] {
//     if (params.as_object().is_empty()) {
//         return [];
//     }
// }

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
        // let params = filter(serde_json::to_value(input));
        // dbg!(&params);
        let params = serde_json::to_value(input)?;
        let map = params.as_object().unwrap();
        let arg = map.iter().map(|(_, param)| param.clone()).collect::<Vec<_>>();
        // if map.is_empty() {
        //     arg = &vec![];
        // } else {
        //     let arg_vec = map.iter().map(|(_, param)| param.clone()).collect::<Vec<_>>();
        //     // let arg_vec = map.iter().as_slice();
        //     arg = &arg_vec;
        // }

        let request = self.client.build_request(method, &arg);

        self.client
            .send_request(&request)
            .and_then(|res| res.into_result::<U>())

        // Ok(resp?.into_result()?)
    }

    /// Show information about this node.
    pub fn getinfo(&mut self) -> Result<responses::GetInfo, Error> {
        self.call("getinfo", requests::GetInfo {})
    }

    pub fn setloglevel(&mut self, level: String) -> Result<(), Error> {
        self.call("setloglevel", requests::SetLogLevel { level })
    }


}

