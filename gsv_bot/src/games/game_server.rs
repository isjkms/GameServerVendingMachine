use anyhow::Result;

pub trait GameServer {
    // 실행 결과로 성공/실패 메시지를 반환
    fn start(&self) -> Result<String, String>; 
}