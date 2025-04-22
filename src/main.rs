use clap::Parser;
use thuai_8_agent_rust::run_agent;
use std::env;

#[derive(Parser)]
#[command(about, long_about = None)]
struct Cli {
    #[arg(long)]
    server: Option<String>,
    #[arg(long)]
    token: Option<String>,
}

const SERVER_DEFAULT: &str = "ws://127.0.0.1:14514";
const TOKEN_DEFAULT: &str = "1919810";

#[tokio::main]
async fn main() {
    let cli = Cli::parse(); // Read Cli Options
    tracing_subscriber::fmt::init();

    // Cli goes first, or get the env.
    let server = cli.server.unwrap_or(
        env::var("SERVER").unwrap_or(SERVER_DEFAULT.to_string())
    );
    let token = cli.token.unwrap_or(
        env::var("TOKEN").unwrap_or(TOKEN_DEFAULT.to_string())
    );

    run_agent(server, token).await;

}
