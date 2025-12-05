use aws_sdk_secretsmanager::Client;
use crate::domain::s3::S3Credential;
use anyhow::{Result, anyhow};
use serde_json;
use aws_config::BehaviorVersion;

/// SecretsManager Adapter
#[derive(Clone)]
pub struct SecretsAdapter {
    client: Client,
}

impl SecretsAdapter {
    /// AWS SecretsManager 클라이언트 초기화
    pub async fn new() -> Self {
        // 최신 SDK: BehaviorVersion 필수 전달
        let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let client = Client::new(&config);
        Self { client }
    }

    /// S3 Credential Secret 불러오기
    pub async fn load_s3_secret(&self, name: &str) -> Result<S3Credential> {
        let resp = self.client.get_secret_value()
            .secret_id(name)
            .send()
            .await?;

        let secret_str = resp.secret_string()
            .ok_or(anyhow!("Secret 문자열 없음"))?;

        let json: S3Credential = serde_json::from_str(secret_str)
            .map_err(|e| anyhow!("Secret JSON 파싱 실패: {}", e))?;

        Ok(json)
    }
}
