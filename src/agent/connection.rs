/*! Contains struct and method to handle the connection to the server. */
use core::error::Error;
use std::time::Duration;

use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use serde::Serialize;
use tokio::{net::TcpStream, time::sleep};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

type Connection = WebSocketStream<MaybeTlsStream<TcpStream>>;
type WriteConnection = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
type ReadConnection = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

use tokio_tungstenite::connect_async;
use tracing::{debug, error, info};

use super::model::{BuffKind, MoveDirection, RequestType, SkillKind, TurnDirection};

const TRY_TIME: u32 = 3;
const CONNECT_SLEEP_SEC: u64 = 3;

/// Hold the connection to the server.
///
/// Should be created with [`AgentClient::new`].
pub struct AgentClient {
    // ws_stream: Connection,
    write: WriteConnection,
    read: ReadConnection,
    token: String,
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

    /// Create a new [`AgentClient`] connecting to `server` for agent with `token`.
    ///
    /// If connect fails, it will sleep and then retry for some times before panic.
    ///
    /// # Panics
    ///
    /// Panics if connecting to server always fail.
    pub async fn new(server: String, token: String) -> AgentClient {
        info!("Connecting to {server} with token {token}");
        let ws_stream = Self::try_connect(&server, TRY_TIME)
            .await
            .unwrap_or_else(|| {
                error!("Cannot connect to {server}!");
                panic!("Connection Error!");
            });
        info!("Connected to {server} successfully!");
        let (write, read) = ws_stream.split();
        AgentClient { write, read, token }
    }

    async fn on_message(&self, msg: String) -> Option<AgentMessage> {
        debug!("Received Message: {}", msg);
        unimplemented!()
    }

    pub async fn send(&mut self, msg: impl Serialize) -> Result<(), Box<dyn Error>> {
        let to_send = serde_json::to_string(&msg)?;
        debug!("Sending Message: {}", to_send);
        self.write.send(to_send.into()).await?;
        Ok(())
    }
}

pub trait ConnectionAPI {
    async fn send_perform_turn(
        &mut self,
        direction: TurnDirection,
        angle: u32,
    ) -> Result<(), Box<dyn Error>>;
    async fn send_perform_move(
        &mut self,
        direction: MoveDirection,
        distance: f64,
    ) -> Result<(), Box<dyn Error>>;
    async fn send_perform_attack(&mut self) -> Result<(), Box<dyn Error>>;
    async fn send_perform_skill(&mut self, skill_name: SkillKind) -> Result<(), Box<dyn Error>>;
    async fn send_perform_select(&mut self, buff_name: BuffKind) -> Result<(), Box<dyn Error>>;
    async fn send_get_player_info(&mut self) -> Result<(), Box<dyn Error>>;
    async fn send_get_environment_info(&mut self) -> Result<(), Box<dyn Error>>;
    async fn send_get_game_statistics(&mut self) -> Result<(), Box<dyn Error>>;
    async fn send_get_available_buffs(&mut self) -> Result<(), Box<dyn Error>>;
}

// TODO: definition of messages

enum AgentMessage {}

#[derive(Debug, Serialize)]
struct PlayerPerform {
    #[serde(rename = "messageType")]
    message_type: &'static str,
    perform: PerformMessage,
}

#[derive(Debug, Serialize)]
#[serde(tag = "messageType")]
pub enum PerformMessage {
    #[serde(rename = "PERFORM_MOVE")]
    PerformMove {
        token: String,
        direction: MoveDirection,
        distance: f64,
    },
    #[serde(rename = "PERFORM_TURN")]
    PerformTurn {
        token: String,
        direction: TurnDirection,
        angle: u32, // TODO: confirm, is it really integer?
    },
    #[serde(rename = "PERFORM_ATTACK")]
    PerformAttack { token: String },
    #[serde(rename = "PERFORM_SKILL")]
    PerformSkill {
        token: String,
        #[serde(rename = "skillName")]
        skill_name: SkillKind,
    },
    #[serde(rename = "PERFORM_SELECT")]
    PerformSelect {
        token: String,
        #[serde(rename = "buffName")]
        buff_name: BuffKind,
    },
    #[serde(rename = "GET_PLAYER_INFO")]
    GetPlayerInfo { token: String, request: RequestType },
    #[serde(rename = "GET_ENVIRONMENT_INFO")]
    GetEnvironmentInfo { token: String },
    #[serde(rename = "GET_GAME_STATISTICS")]
    GetGameStatistics { token: String },
    #[serde(rename = "GET_AVAILABLE_BUFFS")]
    GetAvailableBuffs { token: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perform_skill_serialize() {
        let msg = PerformMessage::PerformSkill {
            token: "1919810".to_string(),
            skill_name: SkillKind::Flash,
        };

        let serialized = serde_json::to_string(&msg).unwrap();

        assert_eq!(
            serialized,
            r#"{"messageType":"PERFORM_SKILL","token":"1919810","skillName":"FLASH"}"#
        );
    }

    #[test]
    fn get_player_info_serialize() {
        let msg = PerformMessage::GetPlayerInfo {
            token: "1919810".to_string(),
            request: RequestType::TheSelf,
        };

        let serialized = serde_json::to_string(&msg).unwrap();

        assert_eq!(
            serialized,
            r#"{"messageType":"GET_PLAYER_INFO","token":"1919810","request":"SELF"}"#
        )
    }
}
