#[macro_use]
extern crate strum;

mod agent;

use agent::model::Position;
use tracing::info;
// use agent;

pub async fn run_agent(server: String, token: String) {
    info!("Connecting to {server} with token {token}");
    // let x: Position<int> = model::Position::new(2, 3, 3.0);
    let x: Position<i32> = Position::new(2, 3, 1.0);
}

