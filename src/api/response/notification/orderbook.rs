use crate::prelude::*;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct BookOrder(
    #[serde_as(as = "DisplayFromStr")] f64,
    #[serde_as(as = "DisplayFromStr")] f64,
);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Orderbook {
    t: i64,
    s: i64,
    a: Vec<BookOrder>,
    b: Vec<BookOrder>,
}

#[test]
fn deserialize_orderbook() {
    let ticker = r#"{
    "ch": "orderbook/full",                 
    "snapshot": {
        "ETHBTC": {
            "t": 1626866578796,             
            "s": 27617207,                  
            "a": [                          
                ["0.060506", "0"],
                ["0.060549", "12.6431"],
                ["0.060570", "0"],
                ["0.060612", "0"]
            ],
            "b": [                          
                ["0.060439", "4.4095"],
                ["0.060414", "0"],
                ["0.060407", "7.3349"],
                ["0.060390", "0"]
            ]
        }
    }
}
"#;
    serde_json::from_str::<super::ChannelMessage>(ticker).unwrap();
}
