use super::{
    connection::ConnectionAPI,
    model::{AvailableBuffs, BuffKind, EnvironmentInfo, GameStatistics, Players, SkillKind},
};

pub trait PlayerOperate: ConnectionAPI {
    fn token(&self) -> &str;
    fn players_info(&self) -> Option<&Players>;
    fn game_statistics(&self) -> Option<&GameStatistics>;
    fn environment_info(&self) -> Option<&EnvironmentInfo>;
    fn available_buffs(&self) -> Option<&AvailableBuffs>;
    fn move_forward(&mut self, distance: f64) -> impl std::future::Future<Output = ()> + Send;
    fn move_backward(&mut self, distance: f64) -> impl std::future::Future<Output = ()> + Send;
    fn turn_clockwise(&mut self, angle: u32) -> impl std::future::Future<Output = ()> + Send;
    fn turn_counter_clockwise(&mut self, angle: u32) -> impl std::future::Future<Output = ()> + Send;
    fn attack(&mut self) -> impl std::future::Future<Output = ()> + Send;
    fn use_skill(&mut self, skill: SkillKind) -> impl std::future::Future<Output = ()> + Send;
    fn select_buff(&mut self, buff: BuffKind) -> impl std::future::Future<Output = ()> + Send;
}
