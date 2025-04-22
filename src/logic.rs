use crate::agent::player_api::Agent;

trait PlayerAPI {
    fn game_loop(agent: &mut Self);

    fn select_buff(agent: &mut Self);
}

impl PlayerAPI for Agent {
    fn game_loop(agent: &mut Self) {
        // Your code here...
    }

    fn select_buff(agent: &mut Self) {
        // Your code here...
    }
}
