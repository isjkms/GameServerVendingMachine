mod bot;

use std::env;
use bot::Bot;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    let discord_bot_token = env::var("DISCORD_BOT_TOKEN").expect("DISCORD_BOT_TOKEN not set");

    Bot::new("GSV Bot", &discord_bot_token).start().await;
}
