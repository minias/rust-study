use serde::Deserialize;
use std::fs;
use anyhow::{Result, Context};

#[derive(Debug, Deserialize, Clone)]
pub struct AwsConfig {
    pub access_key: String,
    pub secret_key: String,
    pub region: String,
}

impl AwsConfig {
    pub fn from_file(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("환경 파일 읽기 실패: {}", path))?;
        let cfg: AwsConfig = serde_yaml::from_str(&content)
            .with_context(|| format!("YAML 파싱 실패: {}", path))?;
        Ok(cfg)
    }

    pub fn default() -> Self {
        AwsConfig {
            access_key: "YOUR_ACCESS_KEY".into(),
            secret_key: "YOUR_SECRET_KEY".into(),
            region: "ap-northeast-2".into(),
        }
    }
}
