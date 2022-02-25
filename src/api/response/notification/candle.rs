use super::*;
use async_tungstenite::tungstenite::handshake::client::Response;
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ticker(HashMap<String, TickerData>);

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct TickerData {
    #[serde(rename = "a")]
    #[serde_as(as = "DisplayFromStr")]
    pub ask_price: f32,

    #[serde(rename = "A")]
    #[serde_as(as = "DisplayFromStr")]
    pub ask_quantity: f32,

    #[serde(rename = "b")]
    #[serde_as(as = "DisplayFromStr")]
    pub bid_price: f32,

    #[serde(rename = "B")]
    #[serde_as(as = "DisplayFromStr")]
    pub bid_quantity: f32,

    #[serde(rename = "c")]
    #[serde_as(as = "DisplayFromStr")]
    pub close: f32,

    #[serde(rename = "o")]
    #[serde_as(as = "DisplayFromStr")]
    pub open: f32,

    #[serde(rename = "l")]
    #[serde_as(as = "DisplayFromStr")]
    pub low: f32,

    #[serde(rename = "h")]
    #[serde_as(as = "DisplayFromStr")]
    pub high: f32,

    #[serde(rename = "v")]
    #[serde_as(as = "DisplayFromStr")]
    pub volume: f32,

    #[serde(rename = "q")]
    #[serde_as(as = "DisplayFromStr")]
    pub volume_quote: f32,

    #[serde(rename = "L")]
    pub id: i64,

    #[serde(rename = "t")]
    #[serde(with = "ts_seconds")]
    pub timestamp: chrono::DateTime<chrono::Utc>,
}