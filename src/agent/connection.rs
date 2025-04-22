/*! Contains struct and method to handle the connection to the server. */
use std::time::Duration;

use futures::stream::{SplitSink, SplitStream};
use futures::StreamExt;
use tokio::{net::TcpStream, time::sleep};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

type Connection = WebSocketStream<MaybeTlsStream<TcpStream>>;
type WriteConnection = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
type ReadConnection = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

use tokio_tungstenite::connect_async;
use tracing::{info, error, debug};

const TRY_TIME: u32 = 3;
const CONNECT_SLEEP_SEC: u64 = 3;

/// Hold the connection to the server.
/// 
/// Should be created with [`AgentClient::new`].
pub struct AgentClient {
    // ws_stream: Connection,
    write: WriteConnection,
    read: ReadConnection,
    token: String
}

impl AgentClient {
    async fn try_connect(server: &String, mut try_count: u32) -> Option<Connection> {
        while try_count > 0 {
            debug!("Trying to connect to {server}");
            if let Ok((ws_stream, _)) = 
            connect_async(server).await {
                return Some(ws_stream);
            }
            debug!("Connect failed! Sleeping...");
            sleep(Duration::from_secs(CONNECT_SLEEP_SEC)).await;
            try_count -= 1;
        }
        debug!("Connection failed too many times!");
        None
    }

    /// Create a new [`AgentClient`] connecting to `server` for agent with `token`.
    /// 
    /// If connect fails, it will sleep and then retry for some times before panic.
    /// 
    /// # Panics
    /// 
    /// Panics if connecting to server always fail.
    pub async fn new(server: String, token: String) -> AgentClient {
        info!("Connecting to {server} with token {token}");
        let ws_stream = 
            Self::try_connect(&server, TRY_TIME)
            .await
            .unwrap_or_else(
            || {
                error!("Cannot connect to {server}!");
                panic!("Connection Error!");
            }
        );
        info!("Connected to {server} successfully!");
        let (write, read) = ws_stream.split();
        AgentClient { write, read, token }
    }

    async fn on_message(&self, msg: impl MsgSend) {
        unimplemented!();
    }

    async fn send(&self, msg: impl MsgSend) {
        unimplemented!()
    }
}

/// If you believe the struct can be sent as a message to server,
/// then impl the Message trait. 
/// 
/// Should implement [Display][`std::fmt::Display`] trait first.
trait MsgSend : ToString {}

// TODO: definition of messages
