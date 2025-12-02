// src/domain/config.rs

/// 현재는 상수 기반 AWS 설정
pub struct AwsConfigConst;

impl AwsConfigConst {
    pub const ACCESS_KEY: &'static str = "AKIAxxxxxxxxxxxx";
    pub const SECRET_KEY: &'static str = "xxxxxxxxxxxxxxxxxxxxxxxxxxxx";
    pub const REGION: &'static str = "ap-northeast-2";
    pub const BUCKET_NAME: &'static str = "my-bucket-example";
}
/// SecretsManager에서 로드할 시크릿 구조체
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AwsSecretConfig {
    pub db_url: String,
    pub api_key: String,
}