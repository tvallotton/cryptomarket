pub use self::{error::Error, notification::*, result::*};
use crate::prelude::*;
pub use income_methods::*;
pub use result::Balance;

mod error;
mod income_methods;
mod notification;
mod result;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Response {
    Channel(ChannelMessage),
    IncomeMethods(IncomeMethods),
    Error(Error),
    Result(APIResult),
    Unknown(serde_json::Value),
}

impl Response {
    pub fn id(&self) -> Option<i64> {
        match self {
            | Response::Error(Error { id, .. }) => *id,
            | Response::IncomeMethods(_) => None,
            | Response::Channel(_) => None,
            | Response::Result(result) => Some(result.id),
            | Response::Unknown(obj) => obj["id"].as_i64(),
        }
    }

    pub fn method(&self) -> Option<String> {
        match self {
            | Response::IncomeMethods(method) => Some(method.method().into()),
            | Response::Channel(channel) => Some(channel.ch.clone()),
            | _ => None,
        }
    }
    pub fn as_subscriptions(self) -> Result<Subscriptions, Self> {
        match self {
            | Response::Result(APIResult {
                result: ResultVariant::Subscriptions(subs),
                ..
            }) => Ok(subs),
            | res => Err(res),
        }
    }
    pub fn as_top_order(self) -> Result<TopOrderMap, Self> {
        match self {
            | Response::Channel(ChannelMessage {
                data: ChannelVariant::TopOrder { data },
                ..
            }) => Ok(data),
            | res => Err(res),
        }
    }
    pub fn as_orderbook(self) -> Result<HashMap<String, Orderbook>, Self> {
        match self {
            | Response::Channel(ChannelMessage {
                data: ChannelVariant::OrderbookUpdate { update },
                ..
            }) => Ok(update),
            | Response::Channel(ChannelMessage {
                data: ChannelVariant::OrderbookSnapshot { snapshot },
                ..
            }) => Ok(snapshot),
            | Response::Channel(ChannelMessage {
                data: ChannelVariant::PartialOrderbook { data },
                ..
            }) => Ok(data),
            | res => Err(res),
        }
    }
    pub fn as_spot_fees(self) -> Result<Vec<Fee>, Self> {
        match self {
            | Response::Result(APIResult {
                result: ResultVariant::Fees(fees),
                ..
            }) => Ok(fees),
            | res => Err(res),
        }
    }

    pub fn as_spot_fee(self) -> Result<Fee, Self> {
        match self {
            | Response::Result(APIResult {
                result: ResultVariant::Fee(fee),
                ..
            }) => Ok(fee),
            | res => Err(res),
        }
    }
    pub fn as_ticker(self) -> Result<Ticker, Self> {
        match self {
            | Response::Channel(ChannelMessage {
                data: ChannelVariant::Ticker { data },
                ..
            }) => Ok(data),
            | res => Err(res),
        }
    }
    pub fn as_amount(self) -> Result<Amount, Self> {
        use Response::*;
        match self {
            | Result(APIResult {
                result: ResultVariant::Amount(amount),
                ..
            }) => Ok(amount),
            | r => Err(r),
        }
    }

    pub fn as_balance(self) -> Result<Balance, Self> {
        match self {
            | Response::Result(APIResult {
                result: ResultVariant::Balance(balance),
                ..
            }) => Ok(balance),
            | Response::IncomeMethods(IncomeMethods::WalletBalanceUpdate { params }) => Ok(params),
            | res => Err(res),
        }
    }

    pub fn as_balances(self) -> Result<Vec<Balance>, Self> {
        match self {
            | Response::Result(APIResult {
                result: ResultVariant::Balances(balance),
                ..
            }) => Ok(balance),
            | Response::IncomeMethods(IncomeMethods::WalletBalances { params }) => Ok(params),
            | res => Err(res),
        }
    }
    pub fn as_transaction(self) -> Result<Transaction, Self> {
        match self {
            | Response::IncomeMethods(IncomeMethods::TransactionUpdate { params }) => Ok(params),
            | res => Err(res),
        }
    }
    pub fn as_result(self) -> Result<APIResult, Self> {
        match self {
            | Self::Result(result) => Ok(result),
            | res => Err(res),
        }
    }
    pub fn as_spot_order(self) -> Result<Order, Self> {
        match self {
            | Response::Result(APIResult {
                result: ResultVariant::SpotOrder(spot_order),
                ..
            }) => Ok(spot_order),
            | Response::IncomeMethods(IncomeMethods::SpotOrder { params }) => Ok(params),
            | res => Err(res),
        }
    }

    pub fn as_trades(self) -> Result<TradeMap, Self> {
        match self {
            | Response::Channel(ChannelMessage {
                data: ChannelVariant::TradeSnapshot { snapshot },
                ..
            }) => Ok(snapshot),

            | Response::Channel(ChannelMessage {
                data: ChannelVariant::TradeUpdate { update },
                ..
            }) => Ok(update),

            | res => Err(res),
        }
    }
    pub fn as_spot_orders(self) -> Result<Vec<Order>, Self> {
        match self {
            | Response::Result(APIResult {
                result: ResultVariant::SpotOrders(spot_orders),
                ..
            }) => Ok(spot_orders),
            | Response::IncomeMethods(IncomeMethods::SpotOrders { params }) => Ok(params),
            | res => panic!("{:?}", res),
        }
    }
    pub fn success(self) -> bool {
        self.as_result()
            .map(|r| match r.result {
                | ResultVariant::Success(bool) => bool,
                | _ => false,
            })
            .unwrap_or_default()
    }
}
