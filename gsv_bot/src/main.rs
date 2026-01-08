mod bot;
mod utils;
mod commands;
pub mod games;

use std::env;
use utils::logger::{Logger, LogLevel};
use bot::Bot;

#[tokio::main]
async fn main() {
    if let Err(e) = dotenv::dotenv() {
        Logger::print(LogLevel::Error, "main.rs", "main", &format!("Failed to load .env file: {:?}", e));
        return;
    }

    let discord_bot_token = match env::var("DISCORD_BOT_TOKEN") {
        Ok(token) => token,
        Err(e) => {
            Logger::print(LogLevel::Error, "main.rs", "main", &format!("DISCORD_BOT_TOKEN not set: {:?}", e));
            return;
        }
    };

    Bot::new("GSV Bot", &discord_bot_token).start().await;
}
