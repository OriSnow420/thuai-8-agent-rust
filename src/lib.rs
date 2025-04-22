#[macro_use]
extern crate strum;

mod agent;

use tracing::info;

pub async fn run_agent(server: String, token: String) {
    info!("Connecting to {server} with token {token}");
}
