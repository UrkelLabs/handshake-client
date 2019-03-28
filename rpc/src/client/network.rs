use crate::responses;

use crate::client::RPCClient;
use crate::error::Error;

use serde_json;
use serde_json::json;

impl RPCClient {
    pub fn get_connection_count(&self) -> Result<responses::ConnectionCount, Error> {
        self.call("getconnectioncount", &[])
    }

    pub fn ping(&self) -> Result<(), Error> {
        self.call("ping", &[])
    }

    // not sure if will work but we'll see
    pub fn get_peer_info(&self) -> Result<Vec<responses::PeerInfo>, Error> {
        self.call("getpeerinfo", &[])
    }

    //Consider breaking this into multiple functions since we only have 3 acceptable cmds
    //"add", "onetry", "remove"
    pub fn add_node(&self, addr: &str, cmd: &str) -> Result<(), Error> {
        self.call("addnode", &[json!(addr), json!(cmd)])
    }

    pub fn disconnect_node(&self, addr: &str) -> Result<(), Error> {
        self.call("disconnectnode", &[json!(addr)])
    }

    pub fn get_added_node_info(&self, addr: &str) -> Result<responses::AddedNodeInfo, Error> {
        self.call("getaddednodeinfo", &[json!(addr)])
    }

    pub fn get_net_totals(&self) -> Result<responses::NetTotals, Error> {
        self.call("getnettotals", &[])
    }

    pub fn get_network_info(&self) -> Result<responses::NetworkInfo, Error> {
        self.call("getnetworkinfo", &[])
    }

    //AS with add_node this command has a set amount of cmds so consider breaking this up
    pub fn set_ban(&self, addr: &str, cmd: &str) -> Result<(), Error> {
        self.call("setban", &[json!(addr), json!(cmd)])
    }

    pub fn list_banned(&self) -> Result<Vec<responses::BannedNode>, Error> {
        self.call("listbanned", &[])
    }

    pub fn clear_banned(&self) -> Result<(), Error> {
        self.call("clearbanned", &[])
    }
}
