use crate::prelude::*;

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Subscriptions {
    ch: String,
    subscriptions: Vec<String>,
}
