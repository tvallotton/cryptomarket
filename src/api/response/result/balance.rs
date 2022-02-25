use crate::prelude::*;

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Balance {
    pub currency: String,
    #[serde_as(as = "DisplayFromStr")]
    pub available: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub reserved: f64,
}
