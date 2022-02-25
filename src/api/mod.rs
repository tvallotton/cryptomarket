use crate::prelude::*;
mod request;
mod response;
mod rest;

pub use request::*;
pub use response::{Error, *};
pub use rest::*;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Side {
    Buy,
    Sell,
}
impl Display for Side {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                | Side::Buy => "buy",
                | _ => "sell",
            }
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ReportType {
    Status,
    New,
    Canceled,
    Expired,
    Suspended,
    Trade,
    Replaced,
}

impl Display for ReportType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                | ReportType::Status => "status",
                | ReportType::New => "new",
                | ReportType::Canceled => "canceled",
                | ReportType::Expired => "expired",
                | ReportType::Suspended => "suspended",
                | ReportType::Trade => "trade",
                | ReportType::Replaced => "replaced",
            }
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum OrderType {
    Limit,
    Market,
    StopLimit,
    StopMarket,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    New,
    Suspended,
    PartiallyFilled,
    Filled,
    Canceled,
    Expired,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                | Status::New => "new",
                | Status::Suspended => "suspended",
                | Status::PartiallyFilled => "partiallyFilled",
                | Status::Filled => "filled",
                | Status::Canceled => "canceled",
                | Status::Expired => "expired",
            }
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
pub enum TimeInForce {
    GTC,
    IOC,
    FOK,
    Day,
    GTD,
}
#[serde_as]
#[derive(Debug, Clone, Serialize, derive_builder::Builder)]
pub struct NewOrder {
    #[builder(default = "crate::api::request::random_id()")]
    pub client_order_id: String,
    pub symbol: String,
    pub side: Side,
    #[builder(default = "Some(OrderType::Limit)")]
    pub r#type: Option<OrderType>,
    #[builder(default = "None")]
    pub time_in_force: Option<TimeInForce>,
    #[serde_as(as = "DisplayFromStr")]
    pub quantity: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
    #[builder(default = "None")]
    pub stop_price: Option<f64>,
    #[builder(default = "None")]
    pub expire_time: Option<DateTime<Utc>>,
    #[builder(default = "None")]
    pub strict_validate: Option<bool>,
    #[builder(default = "None")]
    pub post_only: Option<bool>,
}

use std::fmt::{self, *};

impl Display for NewOrder {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:15} | {:>4} {:<8} | {:10.10} | {:10.10} | {:10.10} |",
            self.client_order_id, self.side, self.symbol, self.quantity, 0.0, self.price
        )
    }
}

// #[derive(Debug, Clone, Serialize, )]
// struct Trade {

// }
