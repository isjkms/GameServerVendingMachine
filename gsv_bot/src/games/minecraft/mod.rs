use crate::games::game_server::GameServer;
use crate::utils::logger::{Logger, LogLevel};

pub struct MinecraftServer;

impl MinecraftServer {
    pub fn new() -> Self {
        Self
    }
}

impl GameServer for MinecraftServer {
    fn start(&self, user_name: &str) -> Result<String, String> {
        // 서버 실행 로직
        Logger::print(
            LogLevel::Info, 
            "minecraft/mod.rs", 
            "start", 
            &format!("Attempting to run Minecraft process... (Requested by: {})", user_name)
        );
        
        let success = true;

        if success {
            Ok("마인크래프트 실행 (준비중)".to_string())
        } else {
            Err("서버 실행 도중 에러가 발생했습니다.".to_string())
        }
    }
}
