use super::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TradeMap(HashMap<String, Vec<Trade>>);

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trade {
    pub t: i64,
    pub i: i64,
    #[serde_as(as = "DisplayFromStr")]
    pub p: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub q: f64,
    pub s: Side,
}

#[test]
fn serialize_trade() {
    let _: super::ChannelMessage = serde_json::from_str(
        r#"{"ch":"trades","update":{"BTCCLP":[{"i":1635188320828,"p":"51711712","q":"0.01150","s":"sell","t":1635188320828}]}}"#,
    )
    .unwrap();
}
