use crate::utils::logger::{Logger, LogLevel};

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::guild::Guild;

pub struct Bot {
    name: String,
    is_active: bool,
    command_count: i32,
    token: String,
}

impl Bot {
    // 디스코드 봇의 새 인스턴스를 생성
    pub fn new(name: &str, token: &str) -> Self {
        Bot {
            name: name.to_string(),
            is_active: true,
            command_count: 0,
            token: token.to_string(),
        }
    }

    pub async fn start(self) {
        let intents = GatewayIntents::GUILD_MESSAGES // "누가 메시지를 보냈다"라는 이벤트 자체를 받음
            | GatewayIntents::MESSAGE_CONTENT // 메시지의 내용을 읽을 수 있는 권한
            | GatewayIntents::GUILDS; // 봇이 서버에 들어가거나 나가는 것

        let mut client = match Client::builder(&self.token, intents)
            .event_handler(self)
            .await
        {
            Ok(client) => client,
            Err(e) => {
                Logger::print(LogLevel::Error, "bot.rs", "start", &format!("Failed to create client: {:?}", e));
                return;
            }
        };

        if let Err(why) = client.start().await {
            Logger::print(LogLevel::Error, "bot.rs", "start", &format!("Client error: {:?}", why));
        }
    }
}

#[async_trait]
impl EventHandler for Bot {
    // 봇이 로그인됐을 때
    async fn ready(&self, _: Context, ready: Ready) {
        Logger::print(LogLevel::Info, "bot.rs", "ready", &format!("{} connected successfully!", ready.user.name));
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: Option<bool>) {
        if is_new == Some(true) {
            Logger::print(LogLevel::Info, "bot.rs", "guild_create", "Newly invited server");
            if let Some(system_channel) = guild.system_channel_id {
                Logger::print(LogLevel::Info, "bot.rs", "guild_create", &format!("System channel found: {:?}", system_channel));

                let welcome_msg = format!(
                    "안녕하세요. {}입니다.",
                    self.name
                );

                if let Err(e) = system_channel.say(&ctx.http, welcome_msg).await {
                    Logger::print(LogLevel::Error, "bot.rs", "guild_create", &format!("Failed to send welcome message: {:?}", e));
                } else {
                    Logger::print(LogLevel::Info, "bot.rs", "guild_create", "Welcome message sent successfully!");
                }
            } else {
                Logger::print(LogLevel::Warn, "bot.rs", "guild_create", "No system channel found!");
            }
        } else {
            Logger::print(LogLevel::Warn, "bot.rs", "guild_create", &format!("is_new is not Some(true): {:?}", is_new));
        }
    }

}
