An unofficial SDK for the cryptomrkt API for Rust.
The API currently only supports the websocket cryptomrkt API.
```
# || -> cryptomrkt::Result {async {
use dotenv::*;
use cryptomrkt::{TradingClient, NewOrderBuilder, Buy};
let mut client = TradingClient::new(var("private_key")?, var("public_key")?).await?;
let order = NewOrderBuilder::new()
    .symbol("BTCCLP")
    .side(Buy)
    .quantity(1.0)
    .price(100.0)
    .build();
client.place_order(order).await?;
# Ok(())
# };}
```