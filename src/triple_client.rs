use super::api::*;
use super::*;
use crate::api::Subscriptions as Subs;
#[derive(Clone)]
pub struct TripleClient {
    pub trading: TradingClient,
    pub wallet: WalletClient,
    pub public: PublicClient,
}

impl TripleClient {
    pub async fn new(private_key: &str, public_key: &str) -> Result<Self> {
        let trading = TradingClient::new(private_key, public_key).await?;
        let wallet = WalletClient::new(private_key, public_key).await?;
        let public = PublicClient::new(private_key, public_key).await?;
        Ok(Self {
            trading,
            wallet,
            public,
        })
    }

    pub async fn subscriptions(&self, channel: &str) -> Result<api::Response> {
        self.public.subscriptions(channel).await
    }
    pub async fn subscribe_trades(&self, sym: &[&str]) -> Result<(Receiver<TradeMap>, Subs)> {
        self.public.subscribe_trades(sym).await
    }

    pub async fn subscribe_full_orderbook(
        &self,
        symbols: &[&str],
    ) -> Result<(Receiver<HashMap<String, Orderbook>>, Subs)> {
        self.public.subscribe_full_orderbook(symbols).await
    }
    pub async fn subscribe_ticker(
        &self,
        symbols: &[&str],
        speed: &str,
    ) -> Result<(Receiver<Ticker>, Subs)> {
        self.public.subscribe_ticker(symbols, speed).await
    }
    pub async fn subscribe_top_orderbook(
        &self,
        symbols: &[&str],
        speed: &str,
    ) -> Result<(Receiver<TopOrderMap>, Subs)> {
        self.public.subscribe_top_order(symbols, speed).await
    }
    pub async fn subscribe_partial_orderbook(
        &self,
        symbols: &[&str],
        depth: &str,
        speed: &str,
    ) -> Result<(Receiver<HashMap<String, Orderbook>>, Subs)> {
        self.public
            .subscribe_partial_orderbook(symbols, depth, speed)
            .await
    }

    pub async fn subscribe_orders(&self) -> Result<Receiver<Order>> {
        self.trading.subscribe_orders().await
    }

    pub async fn unsubscribe_orders(&self) -> Result<()> {
        self.trading.unsubscribe_orders().await
    }
    /// returns the orders that are currently active.
    pub async fn get_orders(&self) -> Result<Vec<Order>> {
        self.trading.get_orders().await
    }
    /// places a new order on the exchange.
    pub async fn place_order(&self, order: NewOrder) -> Result<Order> {
        self.trading.place_order(order).await
    }
    /// cancels all orders issed by the client.
    pub async fn cancel_orders(&self) -> Result<Vec<Order>> {
        self.trading.cancel_orders().await
    }
    /// cancels a specific order using the `client_order_id`.
    pub async fn cancel_order(&self, client_order_id: &str) -> Result<Order> {
        self.trading.cancel_order(client_order_id).await
    }

    /// prefer WalletClient::wallet_balance()
    pub async fn get_balance(&self) -> Result<Vec<Balance>> {
        self.trading.get_balance().await
    }
    /// get the fees for all markets.
    pub async fn get_fees(&self) -> Result<Vec<Fee>> {
        self.trading.get_fees().await
    }
    /// get the fee for a specific market
    /// ```
    /// # let client = TripleClient::new("", "").await?;
    /// let fee = client.get_fee("BTCCLP").await?;
    /// ```
    pub async fn get_fee(&self, symbol: &str) -> Result<Fee> {
        self.trading.get_fee(symbol).await
    }

    /// Returns the available balance for the specific currency
    /// ```no_run
    /// let balance = client.currency_balance("BTC").await?;
    /// ```
    pub async fn currency_balance(&self, currency: &str) -> Result<Amount> {
        self.wallet.currency_balance(currency).await
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
        self.wallet.wallet_balance().await
    }
    /// Used to subscribe to changes to the wallet balance for all currencies.
    /// ```no_run
    /// let rx = client.wallet_balance_subscribe();
    /// let balance = rx.recv().await?;
    /// ```
    pub async fn subscribe_wallet_balances(&self) -> Result<Receiver<Balance>> {
        self.wallet.subscribe_wallet_balances().await
    }

    /// Used to subsctibe to transactions.
    pub async fn subscribe_transactions(&self) -> Result<Receiver<Transaction>> {
        self.wallet.subscribe_transactions().await
    }

    pub async fn replace_order(&self, order_id: &str, quantity: f64, price: f64) -> Result<Order> {
        self.trading.replace_order(order_id, quantity, price).await
    }

    /// Used to unsubscribe to changes to the wallet balance for all currencies.
    /// ```no_run
    /// # || async {
    /// # let client = TripleClient::new("", "").await?;
    /// let success = client.unsubscribe_wallet_balance().await?;
    /// # Ok(())
    /// # };
    /// ```
    pub async fn unsubscribe_wallet_balances(&self) -> Result<bool> {
        self.wallet.unsubscribe_wallet_balances().await
    }
}
