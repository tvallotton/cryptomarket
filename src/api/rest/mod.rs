use crate::api::*;
pub type SymbolsInfo = HashMap<String, SymbolInfo>;
pub type RestTrades = HashMap<String, Vec<RestTrade>>;

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SymbolInfo {
    pub r#type: String,
    pub base_currency: String,
    pub quote_currency: String,
    #[serde_as(as = "DisplayFromStr")]
    pub quantity_increment: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub tick_size: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub take_rate: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub make_rate: f64,
    pub fee_currency: String,
}

// {"id": 1443054611, "price": "0.99901", "qty": "0.05", "side": "sell", "timestamp": "2021-10-30T15:54:13.088Z"}
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RestTrade {
    pub id: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub qty: f64,
    pub side: Side,
    pub timestamp: chrono::DateTime<Utc>,
}

use std::fmt::{self, *};
impl Display for SymbolInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{} - {} - {}",
            self.base_currency, self.quote_currency, self.quantity_increment, self.tick_size
        )
    }
}
