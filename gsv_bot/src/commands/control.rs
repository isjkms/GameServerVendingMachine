use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use serenity::all::{ChannelId, CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, ResolvedOption, ResolvedValue};
use crate::games::ServerInfo;
use crate::games::game_server::GameServer;
use crate::games::minecraft::MinecraftServer;

use crate::utils::logger::{Logger, LogLevel};

// 명령어 등록
pub fn register_start() -> CreateCommand {
    // 서버 시작
    CreateCommand::new("start")
        .description("게임 서버를 시작합니다.")
        // 사용자가 입력이 아닌 선택하도록
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "game_name", "실행할 게임을 선택하세요")
                .required(true)
                .add_string_choice("마인크래프트 (Minecraft)", "minecraft")
                .add_string_choice("팰월드 (Palworld)", "palworld")
        )
}

// 명령어 실행 로직
pub async fn run_start(
    _ctx: &Context,
    command: &CommandInteraction,
    game_servers: Arc<RwLock<HashMap<ChannelId, ServerInfo>>>
) -> String {
    let options = command.data.options();

    // 이미 서버가 실행 중인지 확인
    {
        let map = game_servers.read().await;
        if let Some(server_info) = map.get(&command.channel_id) {
            if server_info.is_running {
                Logger::print(
                    LogLevel::Warn, 
                    "control.rs", 
                    "run_start", 
                    &format!(
                        "Server already running in channel: {}. Requested by: {} ({})", 
                        command.channel_id,
                        command.user.name,
                        command.user.id
                    )
                );

                return format!(
                    "이미 {} 서버가 실행 중입니다! (ID: {})", 
                    server_info.game_name, 
                    server_info.game_server_id
                );
            }
        }
    }

    if let Some(ResolvedOption { value: ResolvedValue::String(game_name), .. }) = options.first() {
        // strategy pattern
        let server: Box<dyn GameServer> = match *game_name {
            "minecraft" => Box::new(MinecraftServer::new()),
            // "palworld" => Box::new(PalworldServer::new()),
            _ => return "지원하지 않는 게임입니다.".to_string(),
        };

        match server.start(&command.user.name) {
            Ok(msg) => {
                // 서버 시작 성공 시 정보 저장
                let mut map = game_servers.write().await;
                // TODO: 포트는 나중에 동적으로 할당받아야 함
                let mut new_info = ServerInfo::new(game_name, 25565);
                let server_id = new_info.game_server_id; // 메시지용으로 ID 저장
                new_info.is_running = true; // 서버 시작
                map.insert(command.channel_id, new_info);
                
                format!("{} (ID: {})", msg, server_id)
            },
            Err(e) => e,
        }

    } else {
        "게임 이름을 선택해주세요.".to_string()
    }
}
