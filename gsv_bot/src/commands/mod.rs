pub mod control;

use serenity::all::ChannelId;
use serenity::all::{CommandInteraction, Context, CreateCommand};
use tokio::sync::RwLock;

use std::sync::Arc;
use std::collections::HashMap;
use crate::games::ServerInfo;

// 모든 명령어를 한 번에 등록하기 위한 리스트 반환
pub fn get_all_commands() -> Vec<CreateCommand> {
    vec![
        control::register_start(),
        // control::register_stop(),
    ]
}

// 들어온 명령어를 적절한 함수로 연결
pub async fn run(
    ctx: &Context,
    command: &CommandInteraction,
    game_servers: Arc<RwLock<HashMap<ChannelId, ServerInfo>>>
) -> String {
    match command.data.name.as_str() {
        "start" => control::run_start(ctx, command, game_servers).await,
        // "stop" => control::run_stop(ctx, command).await,
        _ => "알 수 없는 명령어입니다.".to_string(),
    }
}