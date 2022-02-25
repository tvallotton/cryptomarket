use crate::api::*;
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpotTrade {
    pub id: i64,
    pub client_order_id: String,
    pub symbol: String,
    pub side: Side,
    pub status: Status,
    pub r#type: OrderType,
    pub time_in_force: TimeInForce,
    #[serde_as(as = "DisplayFromStr")]
    pub quantity: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub quantity_cumulative: f64,
    pub post_only: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde_as(as = "DisplayFromStr")]
    pub trade_id: i64,
    #[serde_as(as = "DisplayFromStr")]
    pub trade_quantity: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub trade_price: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub trade_fee: f64,
    pub trade_taker: bool,
    pub report_type: ReportType,
}
