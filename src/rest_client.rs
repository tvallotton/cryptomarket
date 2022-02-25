//! # Rest Client
//! The Rest client exists to support two queries that are not supported in
//! by the websocket clients. these are `symbols_info` and `get_trades`.

use crate::api::{RestTrades, SymbolsInfo};
use crate::prelude::*;
use reqwest::Client;

const SYMBOL: &str = "https://api.exchange.cryptomkt.com/api/3/public/symbol";
const TRADES: &str = "https://api.exchange.cryptomkt.com/api/3/public/trades";
#[derive(Clone)]
pub struct RestClient {
    client: Client,
}

impl Default for RestClient {
    fn default() -> Self {
        Self::new()
    }
}

impl RestClient {
    pub fn new() -> RestClient {
        RestClient {
            client: Client::new(),
        }
    }

    pub async fn symbols_info(&self) -> Result<SymbolsInfo> {
        let json = self.client.get(SYMBOL).send().await?.json().await?;
        Ok(json)
    }

    pub async fn get_trades(&self) -> Result<RestTrades> {
        Ok(self.client.get(TRADES).send().await?.json().await?)
    }
}

#[tokio::test]
async fn test_symbols_info() {
    let client = RestClient::new();
    let symbols_info = client.get_trades().await.unwrap();
    println!("{:?}", symbols_info);
}
