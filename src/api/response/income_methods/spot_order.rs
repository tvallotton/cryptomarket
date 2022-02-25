use crate::api::*;
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    /// Order unique identifier as assigned by exchange.
    pub id: i64,
    /// Order unique identifier as assigned by trader.
    pub client_order_id: String,
    /// Symbol code.
    pub symbol: String,
    /// Trade side. Accepted values: `sell`, `buy`
    pub side: Side,
    /// Order state. Possible values: `new`, `suspended`, `partiallyFilled`,
    /// `filled`, `canceled`, `expired`
    pub status: Status,
    /// Order type.Possible values: `limit`, `market`, `stopLimit`, `stopMarket`
    pub r#type: OrderType,
    ///  Time in Force is a special instruction used when placing an order to
    /// indicate how long it will remain active before it is executed or
    /// expired. GTC — "Good-Till-Cancelled" order won't be closed until it
    /// is filled. IOC — "Immediate-Or-Cancel" order must be executed
    /// immediately. Any part of an IOC order that cannot be filled immediately
    /// will be cancelled. FOK — "Fill-Or-Kill" order must be executed
    /// immediately and completely or not executed at all. Day — keeps the
    /// order active until the end of the trading day (UTC).
    /// GTD — "Good-Till-Date" order may remain active until the end of the day
    /// specified in expire_time.
    pub time_in_force: TimeInForce,
    /// Order quantity.
    #[serde_as(as = "DisplayFromStr")]
    pub quantity: f64,

    #[serde_as(as = "DisplayFromStr")]
    /// Cumulative executed quantity.
    pub quantity_cumulative: f64,

    /// Order price.
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
    /// A post-only order is an order that does not remove liquidity. If your
    /// post-only order causes a match with a pre-existing order as a taker,
    /// then the order will be cancelled.
    pub post_only: bool,
    /// Report creation date.
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Date of the report's last update.
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub report_type: ReportType,
}

use std::{fmt, fmt::Display};
impl Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:15} | {:4} {:8} | {:10.10} | {:10.10} | {:10.10} | {:>8?} | {:>8?} | {:?} ",
            self.client_order_id,
            self.side,
            self.symbol,
            self.quantity,
            self.quantity_cumulative,
            self.price,
            self.report_type,
            self.r#type,
            self.time_in_force
        )
    }
}

#[test]
fn order() {
    #[derive(Debug)]
    struct FOO;
    println!("|{:width$?}|", FOO, width = 10);
}
