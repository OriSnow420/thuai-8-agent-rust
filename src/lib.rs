#[macro_use]
extern crate strum;

pub mod agent;
pub mod logic;

use std::time::Duration;

use agent::connection::AgentClient;
use tokio::time::sleep;

// use agent;

pub async fn run_agent(server: String, token: String) {
    let agent = AgentClient::new(server, token).await;
    sleep(Duration::from_secs(10)).await;
    // TODO: finish the function
}
