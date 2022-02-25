use crate::prelude::*;
pub use spot_order::*;
pub use spot_trade::*;
pub use transaction::*;

mod spot_order;
mod spot_trade;
mod transaction;
#[non_exhaustive]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "method")]
#[serde(rename_all = "snake_case")]

pub enum IncomeMethods {
    SpotOrder { params: Order },
    SpotOrders { params: Vec<Order> },
    TransactionUpdate { params: Transaction },
    WalletBalances { params: Vec<super::Balance> },
    WalletBalanceUpdate { params: super::Balance },
}

use IncomeMethods::*;
impl IncomeMethods {
    pub fn method(&self) -> &'static str {
        match self {
            | SpotOrders { .. } => "spot_orders",
            | SpotOrder { .. } => "spot_order",
            | TransactionUpdate { .. } => "transaction_update",
            | WalletBalances { .. } => "wallet_balances",
            | WalletBalanceUpdate { .. } => "wallet_balance_update",
        }
    }
}
