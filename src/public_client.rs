use super::base_client::BaseClient;
use crate::api::{Orderbook, Request, Subscriptions as Subs, Ticker, TopOrderMap, TradeMap};
use crate::prelude::*;
use tokio::sync::mpsc::{channel, Receiver};

const WEBSOCKET_URL: &str = "wss://api.exchange.cryptomkt.com/api/3/ws/public";

#[derive(Clone)]
pub struct PublicClient {
    client: BaseClient,
}

impl PublicClient {
    pub async fn new(private_key: &str, public_key: &str) -> Result<PublicClient> {
        let client = BaseClient::new(private_key, public_key, WEBSOCKET_URL).await?;

        Ok(Self { client })
    }

    pub async fn authenticate(&self) -> Result<()> {
        self.client.authenticate().await
    }
    async fn _is_auth(&self) -> bool {
        self.client.is_auth().await
    }

    pub async fn subscriptions(&self, channel: &str) -> Result<Response> {
        let request = Request::subscriptions(channel);
        self.client.request(&request).await
    }
    /// registers a subscription to the trades channel.
    /// ```no_run
    /// let (tx, res) = client.subscribe_trades().await?;
    /// println!("current subscriptions: {:?}", res);
    /// while Some(trade) = tx.recv().await {
    ///     println!("{:?}", trade);
    /// }
    /// ```
    pub async fn subscribe_trades(&self, sym: &[&str]) -> Result<(Receiver<TradeMap>, Subs)> {
        let request = Request::subscribe("trades", sym);
        let (tx, rx) = channel(2);
        let res = self
            .client
            .request(&request)
            .await?
            .as_subscriptions()
            .map_err(Error::unexpected)?;

        self.client
            .subscribe("trades", tx.clone(), |res| res.as_trades())
            .await;
        Ok((rx, res))
    }

    pub async fn subscribe_full_orderbook(
        &self,
        symbols: &[&str],
    ) -> Result<(Receiver<HashMap<String, Orderbook>>, Subs)> {
        let request = Request::subscribe("orderbook/full", symbols);

        let (tx, rx) = channel(2);
        self.client
            .subscribe("orderbook/full", tx.clone(), |res| res.as_orderbook())
            .await;
        let res = self
            .client
            .request(&request)
            .await?
            .as_subscriptions()
            .map_err(Error::unexpected)?;

        Ok((rx, res))
    }

    /// The only valid speeds are "100ms", "500ms" and "1000ms".
    /// The valid depths are top, D5, D10, D20
    pub async fn subscribe_partial_orderbook(
        &self,
        symbols: &[&str],
        depth: &str,
        speed: &str,
    ) -> Result<(Receiver<HashMap<String, Orderbook>>, Subs)> {
        let (tx, rx) = channel(2);
        let channel = format!("orderbook/{}/{}", depth, speed);
        let request = Request::subscribe(&channel, symbols);
        self.client
            .subscribe(&channel, tx, |res| res.as_orderbook())
            .await;
        let res = self
            .client
            .request(&request)
            .await?
            .as_subscriptions()
            .map_err(Error::unexpected)?;
        Ok((rx, res))
    }

    /// valid channels are 1s and 3s
    pub async fn subscribe_ticker(
        &self,
        symbols: &[&str],
        speed: &str,
    ) -> Result<(Receiver<Ticker>, Subs)> {
        let (tx, rx) = channel(2);
        let channel = format!("ticker/{}", speed);
        let request = Request::subscribe(&channel, symbols);
        self.client
            .subscribe(&channel, tx, |res| res.as_ticker())
            .await;
        let res = self
            .client
            .request(&request)
            .await?
            .as_subscriptions()
            .map_err(Error::unexpected)?;
        Ok((rx, res))
    }

    pub async fn subscribe_top_order(
        &self,
        symbols: &[&str],
        speed: &str,
    ) -> Result<(Receiver<TopOrderMap>, Subs)> {
        let (tx, rx) = channel(2);
        let channel = format!("orderbook/top/{}", speed);
        let request = Request::subscribe(&channel, symbols);
        self.client
            .subscribe(&channel, tx, |res| res.as_top_order())
            .await;
        let res = self
            .client
            .request(&request)
            .await?
            .as_subscriptions()
            .map_err(Error::unexpected)?;
        Ok((rx, res))
    }
}

async fn _auth_client() -> Result<PublicClient> {
    dotenv::dotenv()?;
    use dotenv::var;
    let client = PublicClient::new(&var("private_key")?, &var("public_key")?).await?;
    Ok(client)
}

#[tokio::test]
async fn public_client() -> Result {
    env_logger::init();
    Ok(())
}
