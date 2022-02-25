use serde_with::{serde_as, DisplayFromStr};

use super::*;

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ticker(pub HashMap<String, TickerData>);

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct TickerData {
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

    #[serde(rename = "c")]
    #[serde_as(as = "DisplayFromStr")]
    pub close: f64,

    #[serde(rename = "o")]
    #[serde_as(as = "DisplayFromStr")]
    pub open: f64,

    #[serde(rename = "l")]
    #[serde_as(as = "DisplayFromStr")]
    pub low: f64,

    #[serde(rename = "h")]
    #[serde_as(as = "DisplayFromStr")]
    pub high: f64,

    #[serde(rename = "v")]
    #[serde_as(as = "DisplayFromStr")]
    pub volume: f64,

    #[serde(rename = "q")]
    #[serde_as(as = "DisplayFromStr")]
    pub volume_quote: f64,

    #[serde(rename = "L")]
    pub id: i64,

    #[serde(rename = "t")]
    pub timestamp: u64,
}

#[test]
fn deserialize_ticker() {
    let ticker = r#"{
        "ch": "ticker/1s",
        "data": {
            "ETHBTC": {
                "t": 1614815872000,
                "a": "0.031175",
                "A": "0.03329",
                "b": "0.031148",
                "B": "0.10565",
                "c": "0.031210",
                "o": "0.030781",
                "h": "0.031788",
                "l": "0.030733",
                "v": "62.587",
                "q": "1.951420577",
                "p": "0.000429",
                "P": "1.39",
                "L": 1182694927
        }
        }
    }"#;
    serde_json::from_str::<super::ChannelMessage>(ticker).unwrap();
}
