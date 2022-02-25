use crate::prelude::*;
use std::collections::HashMap;
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Result {
    Subscription {
        ch: String,
        subscriptions: Vec<String>,
    },
    Snapshot {
        ch: String,
        snapshot: HashMap<String, Vec<TradeData>>,
    },

    Update {
        ch: String,
        update: HashMap<String, Vec<TradeData>>,
    },
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]

pub struct TradeData {
    #[serde(rename = "t")]
    timestamp: u64,
    #[serde(rename = "i")]
    id: u64,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "p")]
    price: f64,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "q")]
    quantity: f64,
    #[serde(rename = "s")]
    s: Side,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Side {
    Buy,
    Sell,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CandleData {
    #[serde(rename = "t")]
    timestamp: u64,
    #[serde(rename = "o")]
    #[serde_as(as = "DisplayFromStr")]
    open_price: f64,
    #[serde(rename = "c")]
    #[serde_as(as = "DisplayFromStr")]
    close_price: f64,
    #[serde(rename = "h")]
    #[serde_as(as = "DisplayFromStr")]
    high_price: f64,
    #[serde(rename = "l")]
    #[serde_as(as = "DisplayFromStr")]
    low_price: f64,
    #[serde(rename = "v")]
    #[serde_as(as = "DisplayFromStr")]
    volume: f64,
    #[serde(rename = "q")]
    #[serde_as(as = "DisplayFromStr")]
    quote_volume: f64,
}


// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub enum Channel {
//     // Trades
//     #[serde(rename = "trades")]
//     Trades,

//     // MiniTikers
//     #[serde(rename = "ticker/price/1s")]
//     TickerPrice1S,
//     #[serde(rename = "ticker/price/3s")]
//     TickerPrice3S,

//     // MiniTickers Batches
//     #[serde(rename = "ticker/price/1s/batch")]
//     TickerPrice1SBatch,
//     #[serde(rename = "ticker/price/3s/batch")]
//     TickerPrice3SBatch,
//     // Tickers
//     #[serde(rename = "ticker/1s")]
//     Ticker1s,
//     #[serde(rename = "ticker/3s")]
//     Ticker3s,
//     #[serde(rename = "ticker/1s/batch")]
//     Ticker1sBatch,
//     #[serde(rename = "ticker/3s/batch")]
//     Ticker3sBatch,
//     // Orderbooks
//     #[serde(rename = "orderbook/full")]
//     OrderbookFull,
//     // Candels
//     #[serde(rename = "candles/M1")]
//     CandlesM1,
//     #[serde(rename = "candles/M3")]
//     CandlesM3,
//     #[serde(rename = "candles/M15")]
//     CandlesM15,
//     #[serde(rename = "candles/M30")]
//     CandlesM30,
//     #[serde(rename = "candles/H1")]
//     CandlesH1,
//     #[serde(rename = "candles/H4")]
//     CandlesH4,
//     #[serde(rename = "candles/D1")]
//     CandlesD1,
//     #[serde(rename = "candles/D7")]
//     CandlesD7,
//     #[serde(rename = "candles/1M")]
//     Candles1M,
// }
