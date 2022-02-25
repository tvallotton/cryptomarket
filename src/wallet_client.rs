use crate::api::Balance;
use crate::{
    api::{Amount, Request, Transaction},
    base_client::BaseClient,
    prelude::*,
};

use tokio::sync::mpsc;

const WEBSOCKET_URL: &str = "wss://api.exchange.cryptomkt.com/api/3/ws/wallet";
#[derive(Clone)]
pub struct WalletClient {
    client: BaseClient,
}

impl WalletClient {
    pub async fn new(priv_key: &str, pub_key: &str) -> Result<Self> {
        let client = BaseClient::new(priv_key, pub_key, WEBSOCKET_URL).await?;
        client.authenticate().await?;
        Ok(Self { client })
    }
    pub async fn is_auth(&self) -> bool {
        self.client.is_auth().await
    }

    /// Returns the available balance for the specific currency
    /// ```no_run
    /// let balance = client.currency_balance("BTC").await?;
    /// ```
    pub async fn currency_balance(&self, currency: &str) -> Result<Amount> {
        let request = Request::wallet_balance(currency);
        let amount = self.client.request(&request).await?;

        amount.as_amount().map_err(Error::unexpected)
    }
    /// Used to get the current wallet balance for all currencies.
    /// ```no_run
    /// let balance = client.wallet_balance().await?;
    /// let available = vec![];
    /// for currency in balance {
    ///    available.push(currency.available);
    /// }
    /// ```
    pub async fn wallet_balance(&self) -> Result<Vec<Balance>> {
        let request = Request::wallet_balances();
        let balance = self.client.request(&request).await?;
        balance.as_balances().map_err(Error::unexpected)
    }
    /// Used to subscribe to changes to the wallet balance for all currencies.
    /// ```no_run
    /// let rx = client.wallet_balance_subscribe();
    /// let balance = rx.recv().await?;
    /// ```
    pub async fn subscribe_wallet_balances(&self) -> Result<Receiver<Balance>> {
        let request = Request::subscribe_wallet_balances();
        let (tx, rx) = mpsc::channel(1);
        let success = self.client.request(&request).await?.success();
        if success {
            self.client
                .subscribe_vec("wallet_balances", tx.clone(), |res| res.as_balances())
                .await;
            self.client
                .subscribe("wallet_balance_update", tx, |res| res.as_balance())
                .await;
        }
        Ok(rx)
    }

    /// Used to subsctibe to transactions.
    pub async fn subscribe_transactions(&self) -> Result<Receiver<Transaction>> {
        let request = Request::subscribe_transactions();
        let (tx, rx) = mpsc::channel(1);
        let success = self.client.request(&request).await?.success();
        if success {
            self.client
                .subscribe("transaction_update", tx, |res| res.as_transaction())
                .await;
        }
        Ok(rx)
    }

    /// Used to unsubscribe to changes to the wallet balance for all currencies.
    /// ```no_run
    /// # || async {
    /// let client = WalletClient::new("", "").await?;
    /// let success = client.unsubscribe_wallet_balance().await?;
    /// # Ok(())
    /// # };
    /// ```
    pub async fn unsubscribe_wallet_balances(&self) -> Result<bool> {
        let request = Request::unsubscribe_wallet_balances();
        let res = self.client.request(&request).await?;
        self.client.unsubscribe("wallet_balances").await;
        self.client.unsubscribe("wallet_balance_update").await;
        Ok(res.success())
    }
}

async fn _auth_client() -> Result<WalletClient> {
    dotenv::dotenv()?;
    use dotenv::var;

    let client = WalletClient::new(&var("private_key")?, &var("public_key")?).await?;

    Ok(client)
}

#[tokio::test]
async fn wallet_balance() -> Result {
    env_logger::init();
    let client = _auth_client().await?;
    print!("subscribe_wallet_balances: ");
    let mut rx = client.subscribe_wallet_balances().await?;

    while let Some(res) = rx.recv().await {
        println!("{:?}", res);
    }

    println!("is_auth: {:?}", client.is_auth().await);
    // while let Some(response) = rx.recv().await {
    //     if let Response::Unknown(json) = response {
    //         info!("Unknown: {}", json);
    //     } else {
    //         info!("{:?}", response);
    //     }
    // }
    Ok(())
}
