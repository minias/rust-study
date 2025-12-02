// src/infrastructure/aws_config.rs

use aws_config::{defaults, BehaviorVersion, SdkConfig};
use aws_credential_types::Credentials;
use crate::domain::config::AwsConfigConst;

/// 상수 기반 AWS Config 생성
pub async fn load_const_config() -> SdkConfig {
    let credentials = Credentials::new(
        AwsConfigConst::ACCESS_KEY.to_string(),
        AwsConfigConst::SECRET_KEY.to_string(),
        None, // session token
        None, // expires_after
        "HardcodedProvider", // provider name
    );

    defaults(BehaviorVersion::latest())
        .credentials_provider(credentials)
        .region(AwsConfigConst::REGION)
        .load()
        .await
}

// 향후 env에서 읽어오도록 변경 예정
// pub async fn load_env_config() -> SdkConfig {
//     aws_config::load_defaults(BehaviorVersion::latest()).await
// }
