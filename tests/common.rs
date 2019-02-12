use handshake_rpc::{HSClient};

pub fn setup() -> HSClient {
    HSClient::new("http://localhost:13037")
}
