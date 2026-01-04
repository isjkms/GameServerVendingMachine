pub mod control;

use serenity::all::{CommandInteraction, Context, CreateCommand};

// 모든 명령어를 한 번에 등록하기 위한 리스트 반환
pub fn get_all_commands() -> Vec<CreateCommand> {
    vec![
        control::register_start(),
        // control::register_stop(),
    ]
}

// 들어온 명령어를 적절한 함수로 연결
pub async fn run(ctx: &Context, command: &CommandInteraction) -> String {
    match command.data.name.as_str() {
        "start" => control::run_start(ctx, command).await,
        // "stop" => control::run_stop(ctx, command).await,
        _ => "알 수 없는 명령어입니다.".to_string(),
    }
}