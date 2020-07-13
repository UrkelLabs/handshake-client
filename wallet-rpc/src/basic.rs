use crate::client::HandshakeWalletRpcClient;
use crate::Result;
use handshake_client_types::WalletTransaction;
use serde_json::json;

impl HandshakeWalletRpcClient {
    pub async fn select_wallet(&self, id: &str) -> Result<()> {
        self.call("selectwallet", &[json!(id)]).await
    }

    pub async fn get_balance(&self, account: Option<String>) -> Result<f64> {
        self.call("getbalance", &[json!(account)]).await
    }

    pub async fn get_unconfirmed_balance(&self) -> Result<f64> {
        self.call("getunconfirmedbalance", &[]).await
    }

    pub async fn get_address_by_account(&self, account: &str) -> Result<String> {
        self.call("getaccountaddress", &[json!(account)]).await
    }

    ///Send Handshake to a specific address. Important to note that amount
    ///is notated in HNS whole units, not doos. amount = 1 = 1 HNS
    pub async fn send_to_address(&self, address: String, amount: f64) -> Result<String> {
        self.call("sendtoaddress", &[json!(address), json!(amount)])
            .await
    }

    //@todo result of transactions.
    pub async fn history(
        &self,
        account: Option<String>,
        count: Option<u32>,
        from: Option<u32>,
        watch_only: Option<bool>,
    ) -> Result<Vec<WalletTransaction>> {
        self.call(
            "listtransactions",
            &[json!(account), json!(count), json!(from), json!(watch_only)],
        )
        .await
    }
}
