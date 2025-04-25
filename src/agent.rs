pub mod connection;
pub mod model;
pub mod player_api;

use connection::{AgentClient, ConnectionAPI, PerformMessage};
use model::{
    AvailableBuffs, BuffKind, EnvironmentInfo, GameStatistics, MoveDirection, Players, RequestType,
    SkillKind, TurnDirection,
};
use player_api::PlayerOperate;
use std::error::Error;
use tracing::{debug, error};

pub struct Agent {
    // TODO: fields in Agent
    client: AgentClient,
    token: String,
    players_info: Option<Players>,
    game_statistics: Option<GameStatistics>,
    environment_info: Option<EnvironmentInfo>,
    available_buffs: Option<AvailableBuffs>,
}

impl ConnectionAPI for Agent {
    async fn send_get_available_buffs(&mut self) -> Result<(), Box<dyn Error>> {
        let msg = PerformMessage::GetAvailableBuffs {
            token: self.token.clone(),
        };
        self.client.send(msg).await?;
        Ok(())
    }
    async fn send_get_environment_info(&mut self) -> Result<(), Box<dyn Error>> {
        let msg = PerformMessage::GetEnvironmentInfo {
            token: self.token.clone(),
        };
        self.client.send(msg).await?;
        Ok(())
    }
    async fn send_get_game_statistics(&mut self) -> Result<(), Box<dyn Error>> {
        let msg = PerformMessage::GetGameStatistics {
            token: self.token.clone(),
        };
        self.client.send(msg).await?;
        Ok(())
    }
    async fn send_get_player_info(&mut self) -> Result<(), Box<dyn Error>> {
        let msg = PerformMessage::GetPlayerInfo {
            token: self.token.clone(),
            request: RequestType::Opponent,
        };
        let msg2 = PerformMessage::GetPlayerInfo {
            token: self.token.clone(),
            request: RequestType::TheSelf,
        };
        self.client.send(msg).await?;
        self.client.send(msg2).await?;
        Ok(())
    }
    async fn send_perform_attack(&mut self) -> Result<(), Box<dyn Error>> {
        let msg = PerformMessage::PerformAttack {
            token: self.token.clone(),
        };
        self.client.send(msg).await?;
        Ok(())
    }
    async fn send_perform_move(
        &mut self,
        direction: MoveDirection,
        distance: f64,
    ) -> Result<(), Box<dyn Error>> {
        let msg = PerformMessage::PerformMove {
            token: self.token.clone(),
            direction,
            distance,
        };
        self.client.send(msg).await?;
        Ok(())
    }
    async fn send_perform_select(&mut self, buff_name: BuffKind) -> Result<(), Box<dyn Error>> {
        let msg = PerformMessage::PerformSelect {
            token: self.token.clone(),
            buff_name,
        };
        self.client.send(msg).await?;
        Ok(())
    }
    async fn send_perform_skill(&mut self, skill_name: SkillKind) -> Result<(), Box<dyn Error>> {
        let msg = PerformMessage::PerformSkill {
            token: self.token.clone(),
            skill_name,
        };
        self.client.send(msg).await?;
        Ok(())
    }
    async fn send_perform_turn(
        &mut self,
        direction: TurnDirection,
        angle: u32,
    ) -> Result<(), Box<dyn Error>> {
        let msg = PerformMessage::PerformTurn {
            token: self.token.clone(),
            direction,
            angle,
        };
        self.client.send(msg).await?;
        Ok(())
    }
}

impl PlayerOperate for Agent {
    fn token(&self) -> &str {
        &self.token
    }

    fn players_info(&self) -> Option<&Players> {
        self.players_info.as_ref()
    }

    fn game_statistics(&self) -> Option<&GameStatistics> {
        self.game_statistics.as_ref()
    }

    fn environment_info(&self) -> Option<&EnvironmentInfo> {
        self.environment_info.as_ref()
    }

    fn available_buffs(&self) -> Option<&AvailableBuffs> {
        self.available_buffs.as_ref()
    }

    async fn move_forward(&mut self, distance: f64) {
        debug!("Agent moving forward");
        self.send_perform_move(MoveDirection::Forth, distance)
            .await
            .unwrap_or_else(|err| {
                error!("Sending moving forward message failed: {}", err);
            });
    }

    async fn move_backward(&mut self, distance: f64) {
        debug!("Agent moving backward");
        self.send_perform_move(MoveDirection::Back, distance)
            .await
            .unwrap_or_else(|err| {
                error!("Sending move backward message failed: {}", err);
            })
    }

    async fn turn_clockwise(&mut self, angle: u32) {
        debug!("Agent turning clockwise");
        self.send_perform_turn(TurnDirection::Clockwise, angle)
            .await
            .unwrap_or_else(|err| {
                error!("Sending turning clockwise message failed: {}", err);
            })
    }

    async fn turn_counter_clockwise(&mut self, angle: u32) {
        debug!("Agent turning counter clockwise");
        self.send_perform_turn(TurnDirection::CounterClockwise, angle)
            .await
            .unwrap_or_else(|err| {
                error!("Sending turning counter-clockwise message failed: {}", err);
            })
    }

    async fn attack(&mut self) {
        debug!("Agent attacking");
        self.send_perform_attack().await.unwrap_or_else(|err| {
            error!("Sending attack message failed: {}", err);
        })
    }

    async fn use_skill(&mut self, skill: SkillKind) {
        debug!("Agent using skill {}", skill);
        self.send_perform_skill(skill).await.unwrap_or_else(|err| {
            error!("Sending performing skill {} message failed: {}", skill, err);
        })
    }

    async fn select_buff(&mut self, buff: BuffKind) {
        debug!("Agent selecting buff {}", buff);
        self.send_perform_select(buff).await.unwrap_or_else(|err| {
            error!("Sending selecting buff {} message failed: {}", buff, err);
        })
    }
}
