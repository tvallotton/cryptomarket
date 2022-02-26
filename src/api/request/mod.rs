#![allow(dead_code)]
use chrono::{DateTime, Utc};

use crate::api::*;
#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub(crate) enum Request<'r> {
    Default {
        method: Method,
        id: i64,
        params: Params<'r>,
    },

    ChannelRequest {
        method: Method,
        ch: &'r str,
        id: i64,
        params: Params<'r>,
    },
}
#[derive(Serialize, Deserialize, Debug, Clone, Hash)]
#[serde(rename_all = "snake_case")]

pub enum Method {
    Subscribe,
    Unsubscribe,
    Subscriptions,
    // Spot requests
    SpotSubscribe,
    SpotGetOrders,
    SpotNewOrder,
    SpotUnsubscribe,
    SpotReplaceOrder,
    SpotCancelOrders,
    SpotCancelOrder,
    SpotBalances,
    SpotFees,
    SpotFee,
    // Wallet management.
    SubscribeTransactions,
    UnsubscribeTransactions,
    SubscribeWalletBalances,
    UnsubscribeWalletBalances,
    WalletBalances,
    WalletBalance,
    GetTransactions,
    // Authentication
    Login,
}

#[serde_as]
#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum Params<'r> {
    Login {
        r#type: &'static str,
        api_key: &'r str,
        timestamp: i64,
        signature: String,
    },
    Empty {},

    Symbol {
        symbol: &'r str,
    },
    Symbols {
        symbols: &'r [&'r str],
    },
    SymbolsLimit {
        symbols: &'r [&'r str],
        limit: i32,
    },
    WalletBalance {
        currency: &'r str,
    },
    NewOrder(crate::NewOrder),
    CancelOrder {
        client_order_id: &'r str,
    },
    ReplaceOrder {
        client_order_id: &'r str,
        new_client_order_id: String,
        #[serde_as(as = "DisplayFromStr")]
        quantity: f64,
        #[serde_as(as = "DisplayFromStr")]
        price: f64,
    },
    GetTransactions {
        from: DateTime<Utc>,
        till: DateTime<Utc>,
        /// Comma-separated transaction types. The list of supported types may
        /// be expanded in future versions. Accepted values: `DEPOSIT`,
        /// `WITHDRAW`, `TRANSFER`, `SWAP`
        types: &'r str,
        /// Comma-separated transaction subtypes. Some subtypes are reserved for
        /// future use and do not purport to provide any functionality on the
        /// platform. The list of supported subtypes may be expanded in future
        /// versions. Accepted values: `UNCLASSIFIED`, `BLOCKCHAIN`,
        /// `AIRDROP`, `AFFILIATE`, `STAKING`, `BUY_CRYPTO`, `OFFCHAIN`, `FIAT`,
        /// `SUB_ACCOUNT`, `WALLET_TO_SPOT`, `SPOT_TO_WALLET`,
        /// `WALLET_TO_DERIVATIVES`, `DERIVATIVES_TO_WALLET`,
        /// `CHAIN_SWITCH_FROM`, `CHAIN_SWITCH_TO`, `INSTANT_EXCHANGE`
        subtypes: &'r str,
        ///Comma-separated transaction statuses.
        /// Accepted values: `CREATED`, `PENDING`, `FAILED`, `SUCCESS`,
        /// `ROLLED_BACK`
        statuses: &'r str,

        offset: i32,
        limit: i32,
        id_from: Option<i32>,
        id_till: Option<i32>,
        /// Sort direction.
        /// Accepted values: `DESC`, `ASC`
        /// Default value: `ﬁDESC`
        sort: &'r str,
    },
}

use Params::*;

impl<'r> Request<'r> {
    pub fn id(&self) -> i64 {
        match self {
            | Self::Default { id, .. } => *id,
            | Self::ChannelRequest { id, .. } => *id,
        }
    }

    pub fn login(private_key: &'r hmac::Key, public_key: &'r str) -> Self {
        let timestamp = chrono::Utc::now().timestamp() * 1000;
        let signature = hmac::sign(private_key, timestamp.to_string().as_bytes());
        let signature = hex::encode(signature.as_ref());
        Request::Default {
            id: Utc::now().timestamp_nanos(),
            method: Method::Login,
            params: Login {
                r#type: "HS256",
                api_key: public_key,
                timestamp,
                signature,
            },
        }
    }

    /// Creates a subscription request for the channel provided
    /// and the symbols given
    ///```rust
    /// # use cryptomkt::api::request::*;
    /// Request::subscribe("ticker".into(), vec!["ETHBTC".into()]);
    /// ```
    pub fn subscribe(channel: &'r str, symbols: &'r [&'r str]) -> Self {
        Request::ChannelRequest {
            method: Method::Subscribe,
            ch: channel,
            id: Utc::now().timestamp_nanos(),
            params: Symbols { symbols },
        }
    }

    pub fn subscribe_limit(channel: &'r str, symbols: &'r [&'r str], limit: i32) -> Self {
        Request::ChannelRequest {
            method: Method::Subscribe,
            ch: channel,
            id: Utc::now().timestamp_nanos(),
            params: SymbolsLimit { symbols, limit },
        }
    }

    /// Returns the list of all active subscriptions on a channel.
    /// In case of a successful subscriptions, the server will send:
    /// for `ticker/price/{speed}`, `ticker/{speed}`,
    /// `orderbook/{depth}/{speed}`, `orderbook/top/{speed}`: data notifications
    /// (data) with a specified rate. {speed}  — the period of updating data
    /// which embraces the changes that have occurred if any; for `trades`,
    /// `orderbook/full`, `candles/{period}`: snapshot (snapshot) and update
    /// (update) notifications.

    pub fn subscriptions(channel: &'r str) -> Self {
        Request::ChannelRequest {
            method: Method::Subscriptions,
            ch: channel,
            params: Empty {},
            id: Utc::now().timestamp_nanos(),
        }
    }

    pub fn spot_subscribe() -> Self {
        Request::Default {
            method: Method::SpotSubscribe,
            id: Utc::now().timestamp_nanos(),
            params: Empty {},
        }
    }
    pub fn spot_cancel_order(client_order_id: &'r str) -> Self {
        Request::Default {
            method: Method::SpotCancelOrder,
            id: Utc::now().timestamp_nanos(),
            params: CancelOrder { client_order_id },
        }
    }

    pub fn spot_unsubscribe() -> Self {
        Request::Default {
            method: Method::SpotUnsubscribe,
            id: Utc::now().timestamp_nanos(),
            params: Empty {},
        }
    }

    pub fn spot_new_order(order: crate::NewOrder) -> Self {
        Request::Default {
            method: Method::SpotNewOrder,
            params: NewOrder(order),
            id: Utc::now().timestamp_nanos(),
        }
    }

    pub fn spot_replace_order(client_order_id: &'r str, quantity: f64, price: f64) -> Self {
        Request::Default {
            method: Method::SpotReplaceOrder,
            params: ReplaceOrder {
                client_order_id,
                new_client_order_id: random_id(),
                quantity,
                price,
            },
            id: Utc::now().timestamp_nanos(),
        }
    }

    pub fn spot_cancel_orders() -> Self {
        Request::Default {
            method: Method::SpotCancelOrders,
            params: Empty {},
            id: Utc::now().timestamp_nanos(),
        }
    }

    pub fn spot_balances() -> Self {
        Request::Default {
            method: Method::SpotBalances,
            params: Empty {},
            id: Utc::now().timestamp_nanos(),
        }
    }

    pub fn spot_fees() -> Self {
        Request::Default {
            method: Method::SpotFees,
            params: Empty {},
            id: Utc::now().timestamp_nanos(),
        }
    }

    pub fn spot_get_orders() -> Self {
        Request::Default {
            method: Method::SpotGetOrders,
            params: Empty {},
            id: Utc::now().timestamp_nanos(),
        }
    }

    pub fn spot_fee(symbol: &'r str) -> Self {
        Request::Default {
            method: Method::SpotFee,
            params: Symbol { symbol },
            id: Utc::now().timestamp_nanos(),
        }
    }

    pub fn subscribe_transactions() -> Self {
        Request::Default {
            method: Method::SubscribeTransactions,
            params: Params::Empty {},
            id: Utc::now().timestamp_nanos(),
        }
    }
    pub fn unsubscribe_transactions() -> Self {
        Request::Default {
            method: Method::UnsubscribeTransactions,
            params: Params::Empty {},
            id: Utc::now().timestamp_nanos(),
        }
    }
    pub fn subscribe_wallet_balances() -> Self {
        Request::Default {
            method: Method::SubscribeWalletBalances,
            params: Params::Empty {},
            id: Utc::now().timestamp_nanos(),
        }
    }
    pub fn unsubscribe_wallet_balances() -> Self {
        Request::Default {
            method: Method::UnsubscribeWalletBalances,
            params: Params::Empty {},
            id: Utc::now().timestamp_nanos(),
        }
    }

    pub fn wallet_balances() -> Self {
        Request::Default {
            method: Method::WalletBalances,
            params: Params::Empty {},
            id: Utc::now().timestamp_nanos(),
        }
    }

    pub fn wallet_balance(currency: &'r str) -> Self {
        Request::Default {
            method: Method::WalletBalance,
            params: Params::WalletBalance { currency },
            id: Utc::now().timestamp_nanos(),
        }
    }
}

pub(crate) fn random_id() -> String {
    (0..)
        .into_iter()
        .map(|_| rand::random())
        .filter(|c: &char| c.is_ascii_alphanumeric())
        .take(15)
        .collect()
}
