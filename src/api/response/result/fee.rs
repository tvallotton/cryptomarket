use crate::prelude::*;

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fee {
    symbol: String,
    #[serde_as(as = "DisplayFromStr")]
    take_rate: f64,
    #[serde_as(as = "DisplayFromStr")]
    make_rate: f64,
}
