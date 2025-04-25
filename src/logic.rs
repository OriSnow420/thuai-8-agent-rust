use crate::agent::{Agent, player_api::PlayerOperate};
pub use crate::agent::{connection, model, player_api};

pub trait Logic: PlayerOperate {
    fn game_loop(agent: &mut Self);

    fn select_buff(agent: &mut Self);
}

impl Logic for Agent {
    fn game_loop(agent: &mut Self) {
        // Your code here...
        // You can use the methods offered by [`PlayerOperate`] trait.
        // agent.move_forward();
    }

    fn select_buff(agent: &mut Self) {
        // Your code here...
        // You can use the methods offered by [`PlayerOperate`] trait.
    }
}
