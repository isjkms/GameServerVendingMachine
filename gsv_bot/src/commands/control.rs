use serenity::all::{CommandInteraction, Context, ResolvedOption, ResolvedValue, CreateCommand, CreateCommandOption, CommandOptionType};
use crate::games::game_server::GameServer;
use crate::games::minecraft::MinecraftServer;

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
pub async fn run_start(_ctx: &Context, command: &CommandInteraction) -> String {
    let options = command.data.options();

    if let Some(ResolvedOption { value: ResolvedValue::String(game_name), .. }) = options.first() {
        // strategy pattern
        let server: Box<dyn GameServer> = match *game_name {
            "minecraft" => Box::new(MinecraftServer::new()),
            // "palworld" => Box::new(PalworldServer::new()),
            _ => return "지원하지 않는 게임입니다.".to_string(),
        };

        match server.start() {
            Ok(msg) => msg,
            Err(e) => e,
        }

    } else {
        "게임 이름을 선택해주세요.".to_string()
    }
}
