use crate::prelude::*;
pub use amount::*;
pub use balance::*;
pub use fee::*;

pub use subscription::*;

mod amount;
mod balance;
mod fee;
mod subscription;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct APIResult {
    pub id: i64,
    pub result: ResultVariant,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum ResultVariant {
    Success(bool),
    Subscriptions(Subscriptions),
    Amount(Amount),
    SpotOrder(super::Order),
    SpotOrders(Vec<super::Order>),
    SpotTrade(super::SpotTrade),
    Balances(Vec<Balance>),
    Balance(Balance),
    Fees(Vec<Fee>),
    Fee(Fee),
}
