// src/infrastructure/aws_secrets.rs

use aws_sdk_secretsmanager::Client;
use crate::domain::config::AwsSecretConfig;
use anyhow::Result;
use aws_config::BehaviorVersion;
use serde_json;

/// SecretsManager에서 JSON 시크릿을 파싱하는 어댑터
pub async fn load_secret_json(secret_name: &str) -> Result<AwsSecretConfig> {
    // 최신 AWS SDK 방식으로 config 로딩
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&config);

    // SecretsManager에서 시크릿 가져오기
    let resp = client
        .get_secret_value()
        .secret_id(secret_name)
        .send()
        .await?;

    let secret_str = resp
        .secret_string()
        .ok_or_else(|| anyhow::anyhow!("SecretString is empty"))?;

    // JSON → AwsSecretConfig 구조체 변환
    let data: AwsSecretConfig = serde_json::from_str(&secret_str)?;

    Ok(data)
}
