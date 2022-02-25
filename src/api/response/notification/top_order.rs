use crate::api::*;

pub type TopOrderMap = HashMap<String, TopOrder>;
#[serde_as]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TopOrder {
    #[serde(rename = "t")]
    pub timestamp: u64,
    #[serde(rename = "a")]
    #[serde_as(as = "DisplayFromStr")]
    pub ask_price: f64,
    #[serde(rename = "A")]
    #[serde_as(as = "DisplayFromStr")]
    pub ask_quantity: f64,
    #[serde(rename = "b")]
    #[serde_as(as = "DisplayFromStr")]
    pub bid_price: f64,
    #[serde(rename = "B")]
    #[serde_as(as = "DisplayFromStr")]
    pub bid_quantity: f64,
}
