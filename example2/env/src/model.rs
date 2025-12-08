// src/model.rs
/// 환경설정 모델 정의
use serde::Deserialize;

/// 공통 앱 설정 구조체
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    /// 데이터베이스 설정
    pub database: DatabaseConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}
