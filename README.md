An unofficial SDK for the [cryptomarket](https://api.exchange.cryptomkt.com/) API for Rust.
It is primarily focused on the websocket API. It also supports some 
useful Rest API queries. 
```rust
use dotenv::*;
use cryptomrkt::{TradingClient, NewOrderBuilder, Buy, Order};
let mut client = TradingClient::new(var("private_key")?, var("public_key")?).await?;
let order = NewOrderBuilder::new()
    .symbol("BTCCLP")
    .side(Buy)
    .quantity(1.0)
    .price(100.0)
    .build();
let order: Order = client.place_order(order).await?;
```
