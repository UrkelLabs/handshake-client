// use std::error;
use std::fmt;

use rpc_json_client;
use serde_json;

#[derive(Debug)]
pub enum Error {
    JsonRpc(rpc_json_client::Error),
    Json(serde_json::error::Error),
}

impl From<rpc_json_client::Error> for Error {
    fn from(e: rpc_json_client::Error) -> Error {
        Error::JsonRpc(e)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(e: serde_json::error::Error) -> Error {
        Error::Json(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::JsonRpc(ref e) => write!(f, "JSON-RPC error: {}", e),
            Error::Json(ref e) => write!(f, "JSON error: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::JsonRpc(ref e) => Some(e),
            Error::Json(ref e) => Some(e),
        }
    }
}
