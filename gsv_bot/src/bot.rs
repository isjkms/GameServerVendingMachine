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

        let mut client = Client::builder(&self.token, intents)
            .event_handler(self)
            .await
            .expect("Failed to create client");

        if let Err(why) = client.start().await {
            println!("Client error: {:?}", why);
        }
    }
}

#[async_trait]
impl EventHandler for Bot {
    // 봇이 로그인됐을 때
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} connected successfully!", ready.user.name);
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: Option<bool>) {
        if is_new == Some(true) {
            println!("Newly invited server"); // 새로 초대된 서버
            if let Some(system_channel) = guild.system_channel_id {
                println!("System channel found: {:?}", system_channel);

                let welcome_msg = format!(
                    "안녕하세요. {}입니다.",
                    self.name
                );

                if let Err(e) = system_channel.say(&ctx.http, welcome_msg).await {
                    println!("Failed to send welcome message: {:?}", e);
                } else {
                    println!("Welcome message sent successfully!");
                }
            } else {
                println!("No system channel found!");
            }
        } else {
            println!("is_new is not Some(true): {:?}", is_new);
        }
    }

}
