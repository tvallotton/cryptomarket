use crate::api::{Balance, Fee, Order};
use crate::prelude::*;

use crate::NewOrder;

const WEBSOCKET_URL: &str = "wss://api.exchange.cryptomkt.com/api/3/ws/trading";

#[derive(Clone)]
pub struct TradingClient {
    client: BaseClient,
}

impl TradingClient {
    pub async fn new(priv_key: &str, pub_key: &str) -> Result<Self> {
        let client = BaseClient::new(priv_key, pub_key, WEBSOCKET_URL).await?;
        client.authenticate().await?;
        Ok(Self { client })
    }
    pub async fn is_auth(&self) -> bool {
        self.client.is_auth().await
    }

    pub async fn subscribe_orders(&self) -> Result<Receiver<Order>> {
        let (tx, rx) = channel(8);
        let request = Request::spot_subscribe();
        self.client
            .subscribe("spot_order", tx.clone(), |res| res.as_spot_order())
            .await;
        self.client
            .subscribe_vec("spot_orders", tx, |res| res.as_spot_orders())
            .await;
        self.client.request(&request).await?;

        Ok(rx)
    }
    pub async fn unsubscribe_orders(&self) -> Result<()> {
        let request = Request::spot_unsubscribe();
        let success = self.client.request(&request).await?.success();
        if success {
            self.client.unsubscribe("spot_order").await;
            self.client.unsubscribe("spot_orders").await;
        }
        Ok(())
    }
    /// returns the orders that are currently active.
    pub async fn get_orders(&self) -> Result<Vec<Order>> {
        let request = Request::spot_get_orders();
        self.client
            .request(&request)
            .await?
            .as_spot_orders()
            .map_err(Error::unexpected)
    }
    /// places a new order on the exchange.
    pub async fn place_order(&self, order: NewOrder) -> Result<Order> {
        let request = Request::spot_new_order(order);
        self.client
            .request(&request)
            .await?
            .as_spot_order()
            .map_err(Error::unexpected)
    }
    /// cancels all orders issed by the client.
    pub async fn cancel_orders(&self) -> Result<Vec<Order>> {
        let request = Request::spot_cancel_orders();
        self.client
            .request(&request)
            .await?
            .as_spot_orders()
            .map_err(Error::unexpected)
    }
    /// cancels a specific order using the `client_order_id`.
    pub async fn cancel_order(&self, client_order_id: &str) -> Result<Order> {
        let request = Request::spot_cancel_order(client_order_id);
        self.client
            .request(&request)
            .await?
            .as_spot_order()
            .map_err(Error::unexpected)
    }

    pub async fn get_balance(&self) -> Result<Vec<Balance>> {
        let request = Request::spot_balances();
        self.client
            .request(&request)
            .await?
            .as_balances()
            .or_else(|e| {
                let orders = e.as_spot_orders().map_err(Error::unexpected)?;
                if orders.is_empty() {
                    Ok(vec![])
                } else {
                    Err(Error::UnexpectedResponse)
                }
            })
    }
    /// get the fees for all markets.
    pub async fn get_fees(&self) -> Result<Vec<Fee>> {
        let request = Request::spot_fees();
        self.client
            .request(&request)
            .await?
            .as_spot_fees()
            .map_err(Error::unexpected)
    }
    /// get the fee for a specific market
    /// ```
    /// # let client = TradingClient::new("", "").await?;
    /// let fee = client.get_fee("BTCCLP").await?;
    /// ```
    pub async fn get_fee(&self, symbol: &str) -> Result<Fee> {
        let request = Request::spot_fee(symbol);
        self.client
            .request(&request)
            .await?
            .as_spot_fee()
            .map_err(Error::unexpected)
    }

    pub async fn replace_order(&self, order_id: &str, quantity: f64, price: f64) -> Result<Order> {
        let request = Request::spot_replace_order(order_id, quantity, price);
        self.client
            .request(&request)
            .await?
            .as_spot_order()
            .map_err(Error::unexpected)
    }
}

async fn _auth_client() -> Result<TradingClient> {
    dotenv::dotenv()?;
    use dotenv::var;
    let client = TradingClient::new(&var("private_key")?, &var("public_key")?).await?;
    Ok(client)
}
#[tokio::test]
async fn test() -> Result {
    env_logger::init();
    let client = _auth_client().await?;
    let mut rx = client.subscribe_orders().await?;
    tokio::spawn(async move {
        while let Some(order) = rx.recv().await {
            println!("{:?}", order);
        }
    });
    // let response = client.cancel_orders().await?;
    // println!("Cancelling all orders response: {:?}", response);

    let order = crate::api::NewOrderBuilder::default()
        .symbol("BTCUSDT".into())
        .price(61640.13)
        .quantity(0.00111)
        .side(crate::api::Side::Buy)
        .build()
        .unwrap();
    let client_order_id = order.client_order_id.clone();
    let response = client.place_order(order).await?;
    println!("Placing order response: {:?}", response);
    println!("sleeing for 60 seconds");
    println!("{:?}, {:?}", client_order_id, response.client_order_id);
    tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    println!("waking");
    let response = client.cancel_order(&client_order_id).await?;
    println!("Cancel order response: {:?}", response);
    Ok(())
}
