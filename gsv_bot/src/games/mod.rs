pub mod game_server;
pub mod minecraft;
// pub mod palworld;

use uuid::Uuid;

// #[derive(...)]: 컴파일러에게 기능을 자동으로 넣어달라고 부탁하는 명령어
// Debug: 구조체 내용을 출력해주는 기능 (로그 확인용)
// Clone: 똑같은 데이터를 하나 더 만드는 복사 기능
#[derive(Debug, Clone)]
pub struct ServerInfo {
    pub game_server_id: Uuid, // 고유 ID (겹치지 않게 UUID 사용)
    pub game_name: String,    // 게임 이름 (에: minecraft)
    pub port: u16,            // 접속 포트 (예: 25565)
    pub is_running: bool,     // 서버 실행 여부 확인
}

impl ServerInfo {
    // 새로운 서버 정보를 생성하는 함수
    pub fn new(game_name: &str, port: u16) -> Self {
        Self {
            game_server_id: Uuid::new_v4(), // 랜덤한 고유 ID 생성
            game_name: game_name.to_string(),
            port,
            is_running: false, // 처음 생성 시에는 꺼져있는 상태로 시작
        }
    }

    // 실행 완료되면 상태를 업데이트하는 메서드도 있으면 좋음
    pub fn mark_running(&mut self, port: u16) {
        self.port = port;
        self.is_running = true;
    }
}
