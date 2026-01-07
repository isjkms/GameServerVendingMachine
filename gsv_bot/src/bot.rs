use crate::commands;
use crate::utils::logger::{Logger, LogLevel};
use crate::games::ServerInfo;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use serenity::all::{
    ChannelId, Command, Context, CreateInteractionResponse,
    CreateInteractionResponseMessage, EventHandler, GatewayIntents, Guild, Interaction, Ready,
};
use serenity::async_trait;
use serenity::Client;

pub struct Bot {
    name: String,
    token: String,
    is_active: bool,
    game_servers: Arc<RwLock<HashMap<ChannelId, ServerInfo>>>,
}

impl Bot {
    // 디스코드 봇의 새 인스턴스를 생성
    pub fn new(name: &str, token: &str) -> Self {
        Bot {
            name: name.to_string(),
            token: token.to_string(),
            is_active: true,
            game_servers: Arc::new(RwLock::new(HashMap::new())),
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

    pub fn get_status_message(&self) -> &'static str {
        if self.is_active {
            "봇이 현재 실행 중입니다."
        } else {
            "봇이 쉬고 있습니다."
        }
    }
}

#[async_trait]
impl EventHandler for Bot {
    // 봇이 로그인됐을 때
    async fn ready(&self, ctx: Context, ready: Ready) {
        Logger::print(LogLevel::Info, "bot.rs", "ready", &format!("{} connected successfully!", ready.user.name));

        // 명령어 등록
        let commands = commands::get_all_commands();

        if let Err(e) = Command::set_global_commands(&ctx.http, commands).await {
            Logger::print(LogLevel::Error, "bot.rs", "ready", &format!("Failed to register commands: {:?}", e));
        } else {
            Logger::print(LogLevel::Info, "bot.rs", "ready", "Global commands registered successfully.");
        }
    }

    // 사용자로부터 입력 받은 명령어 처리
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            // 명령어 실행 및 결과 획득
            let content = match command.data.name.as_str() {
                "status" => self.get_status_message().to_string(),
                _ => commands::run(&ctx, &command, self.game_servers.clone()).await,
            };

            // 결과 전송
            let data = CreateInteractionResponseMessage::new().content(content);
            let builder = CreateInteractionResponse::Message(data);

            if let Err(why) = command.create_response(&ctx.http, builder).await {
                Logger::print(LogLevel::Error, "bot.rs", "interaction", &format!("Cannot respond to command: {:?}", why));
            }
        }
    }

    // 초대되었을 때 환영 메시지 전송
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
