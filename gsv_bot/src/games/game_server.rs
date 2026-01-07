use anyhow::Result;

// Send: 타입이 다른 스레드로 안전하게 이동될 수 있음
// Sync: 여러 스레드에서 동시에 참조해도 안전함
pub trait GameServer: Send + Sync {
    // 실행 결과로 성공/실패 메시지를 반환
    fn start(&self, user_name: &str) -> Result<String, String>; 
}