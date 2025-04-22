use std::time::Duration;

use tokio::{net::TcpStream, time::sleep};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

type Connection = WebSocketStream<MaybeTlsStream<TcpStream>>;

use tokio_tungstenite::connect_async;
use tracing::{info, error, debug};

const TRY_TIME: u32 = 3;
const CONNECT_SLEEP_SEC: u64 = 3;

pub struct AgentClient {
    ws_stream: Connection,
    token: String
}

impl AgentClient {
    async fn try_connect(server: &String, mut try_count: u32) -> Option<Connection> {
        while try_count > 0 {
            debug!("Trying to connect to {server}");
            if let Ok((ws_stream, _)) = connect_async(server).await {
                return Some(ws_stream);
            }
            debug!("Connect failed! Sleeping...");
            sleep(Duration::from_secs(CONNECT_SLEEP_SEC)).await;
            try_count -= 1;
        }
        debug!("Connection failed too many times!");
        None
    }

    pub async fn new(server: String, token: String) -> AgentClient {
        info!("Connecting to {server} with token {token}");
        let ws_stream = Self::try_connect(&server, TRY_TIME).await.unwrap_or_else(
            || {
                error!("Cannot connect to {server}!");
                panic!("Connection Error!");
            }
        );
        info!("Connected to {server} successfully!");
        AgentClient { ws_stream, token }
    }

    async fn on_message() {
        unimplemented!()
    }

    async fn send() {
        unimplemented!()
    }
}

// TODO: definition of messages
