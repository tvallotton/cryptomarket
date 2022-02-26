//! An unofficial SDK for the cryptomrkt API for Rust.
//! The API currently only supports the websocket cryptomrkt API.
//!
//! ```
//! # || -> cryptomkt::Result {async {
//! use dotenv::*;
//! use cryptomkt::{TradingClient, NewOrderBuilder, Buy};
//!
//! let mut client = TradingClient::new(var("private_key")?, var("public_key")?).await?;
//! let order = NewOrderBuilder::new()
//!     .symbol("BTCCLP")
//!     .side(Buy)
//!     .quantity(1.0)
//!     .price(100.0)
//!     .build();
//! client.place_order(order).await?;
//! # Ok(())
//! # };}
//! ```

#![warn(unused_crate_dependencies)]

use crate::prelude::*;
use api::*;
pub use error::Error;
pub mod api;

mod base_client;
mod client_pool;
mod error;
mod prelude;
mod public_client;
#[cfg(feature = "rest-client")]
mod rest_client;
mod trading_client;
mod triple_client;
mod type_alias;
mod wallet_client;

const TIMEMOUT: Duration = Duration::from_secs(5);
pub use api::{Order, Side, Side::*};
pub use client_pool::ClientPool;
pub use public_client::PublicClient;
#[cfg(feature = "rest-client")]
pub use rest_client::RestClient;
pub use trading_client::TradingClient;
pub use triple_client::TripleClient;
pub use wallet_client::WalletClient;
