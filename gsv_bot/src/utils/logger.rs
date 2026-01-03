use chrono::Local; // 현재 시간을 가져오기 위해 필요

pub enum LogLevel {
    Info,
    Warn,
    Error,
}

pub struct Logger;

impl Logger {
    pub fn print(level: LogLevel, file_name: &str, fn_name: &str, message: &str) {
        // 현재 시간 가져오기
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        // 레벨에 따라 색상을 다르게 하거나 텍스트를 다르게 출력
        let level_str = match level {
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        };

        // 실제 출력
        println!("[{}][{}][{}][{}] : {}", now, level_str, file_name, fn_name, message);
    }
}