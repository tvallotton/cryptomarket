use futures::stream::{SplitSink, SplitStream};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream, *};

pub type Writer = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
pub type Reader = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;
