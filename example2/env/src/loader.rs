//! YAML 환경설정 파일 로더
//! - .env.{RUST_PROFILE}.yml 파일 로드
//! - once_cell 로 Lazy static 캐싱

use crate::model::AppConfig;
use once_cell::sync::OnceCell;
use serde_yaml;
use std::{env, fs};
use thiserror::Error;

static CONFIG: OnceCell<AppConfig> = OnceCell::new();

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("환경설정 파일을 읽을 수 없습니다: {0}")]
    FileReadError(String),

    #[error("환경설정을 파싱할 수 없습니다: {0}")]
    ParseError(String),
}

/// 환경설정 파일 선택
fn select_env_file() -> String {
    let profile = env::var("RUST_PROFILE").unwrap_or_else(|_| "local".to_string());
    format!(".env.{profile}.yml")
}

/// 설정 로드 (최초 1회 캐싱)
pub fn load_config() -> Result<&'static AppConfig, ConfigError> {
    CONFIG.get_or_try_init(|| {
        let file = select_env_file();

        let content = fs::read_to_string(&file)
            .map_err(|e| ConfigError::FileReadError(e.to_string()))?;

        serde_yaml::from_str(&content)
            .map_err(|e| ConfigError::ParseError(e.to_string()))
    })
}

/// 직접 호출 가능한 로더 구조체
pub struct ConfigLoader;

impl ConfigLoader {
    pub fn load() -> Result<&'static AppConfig, ConfigError> {
        load_config()
    }
}
