use crate::{api::Side, prelude::*};
pub use orderbook::*;
pub use ticker::*;
pub use top_order::*;
pub use trades::*;
mod orderbook;
mod ticker;
mod top_order;
mod trades;
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct ChannelMessage {
    pub ch: String,
    #[serde(flatten)]
    pub data: ChannelVariant,
}
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ChannelVariant {
    Ticker {
        data: Ticker,
    },

    OrderbookUpdate {
        update: HashMap<String, Orderbook>,
    },
    OrderbookSnapshot {
        snapshot: HashMap<String, Orderbook>,
    },

    TopOrder {
        data: TopOrderMap,
    },

    PartialOrderbook {
        data: HashMap<String, Orderbook>,
    },

    TradeSnapshot {
        snapshot: TradeMap,
    },
    TradeUpdate {
        update: TradeMap,
    },
}
