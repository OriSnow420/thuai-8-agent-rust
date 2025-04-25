use clap::Parser;
use std::env;
use thuai_8_agent_rust::run_agent;
use tracing::{Level, error};
use tracing_subscriber::fmt::time::OffsetTime;

#[derive(Parser)]
#[command(about, long_about = None)]
struct Cli {
    #[arg(long)]
    server: Option<String>,
    #[arg(long)]
    token: Option<String>,
    #[arg(long)]
    logging_level: Option<String>,
}

const SERVER_DEFAULT: &str = "ws://127.0.0.1:14514";
const TOKEN_DEFAULT: &str = "1919810";

#[tokio::main]
async fn run(cli: Cli) {
    // Cli goes first, or get the env.
    let server = cli
        .server
        .unwrap_or(env::var("SERVER").unwrap_or(SERVER_DEFAULT.to_string()));
    let token = cli
        .token
        .unwrap_or(env::var("TOKEN").unwrap_or(TOKEN_DEFAULT.to_string()));

    run_agent(server, token).await;
}

fn main() {
    let cli = Cli::parse(); // Read Cli Options

    let logging_level: Level = cli
        .logging_level
        .clone()
        .unwrap_or(env::var("RUST_LOG").unwrap_or("INFO".to_string()))
        .parse()
        .unwrap();

    tracing_subscriber::fmt()
        .with_max_level(logging_level)
        .with_timer(OffsetTime::local_rfc_3339().unwrap_or_else(|_| {
            error!("Could not get local offset!");
            panic!("Could not get local offset!");
        }))
        .init();

    run(cli);
}
