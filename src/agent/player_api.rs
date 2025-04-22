use super::model::
    {AvailableBuffs, BuffKind, EnvironmentInfo, GameStatistics, Players, SkillKind};


pub struct Agent {
    // TODO: fields in Agent
}

impl Agent {
    pub fn token(&self) -> String {
        unimplemented!()
    }

    pub fn players_info(&self) -> Option<Players> {
        unimplemented!()
    }
    
    pub fn game_statistics(&self) -> Option<GameStatistics> {
        unimplemented!()
    }
    
    pub fn environment_info(&self) -> Option<EnvironmentInfo> {
        unimplemented!()
    }

    pub fn available_buffs(&self) -> Option<AvailableBuffs> {
        unimplemented!()
    }

    pub fn move_forward(&self) {
        unimplemented!()
    }

    pub fn move_backward(&self) {
        unimplemented!()
    }

    pub fn turn_clockwise(&self) {
        unimplemented!()
    }

    pub fn turn_counter_clockwise(&self) {
        unimplemented!()
    }

    pub fn attack(&self) {
        unimplemented!()
    }

    pub fn use_skill(&self, skill: SkillKind) {
        unimplemented!()
    }
    
    pub fn select_buff(&self, buff: BuffKind) {
        unimplemented!()
    }

}
