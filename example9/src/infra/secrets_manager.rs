// infra/secrets_manager.rs
use aws_sdk_secretsmanager::{Client};
use crate::domain::s3::S3Credential;
use anyhow::{Result, anyhow};

#[derive(Clone)]
pub struct SecretsAdapter {
    client: Client,
}

impl SecretsAdapter {
    pub async fn new() -> Self {
        let config = aws_config::load_from_env().await;
        let client = Client::new(&config);
        Self { client }
    }

    /// Secret JSON → Credential 파싱
    pub async fn load_s3_secret(&self, name: &str) -> Result<S3Credential> {
        let resp = self.client.get_secret_value().secret_id(name).send().await?;
        let secret_str = resp.secret_string().ok_or(anyhow!("Secret 문자열 없음"))?;
        let json: S3Credential = serde_json::from_str(secret_str)?;
        Ok(json)
    }
}
