// src/infrastructure/aws_s3.rs

use aws_sdk_s3::Client;
use crate::infrastructure::aws_config::load_const_config;
use crate::domain::config::AwsConfigConst;

/// S3 업로드 (현재 bucket 상수 사용)
pub async fn upload_file(key: &str, content: Vec<u8>) -> Result<(), anyhow::Error> {
    let config = load_const_config().await;
    let client = Client::new(&config);

    client
        .put_object()
        .bucket(AwsConfigConst::BUCKET_NAME)
        .key(key)
        .body(content.into())
        .send()
        .await?;

    Ok(())
}

/// S3 다운로드
pub async fn download_file(key: &str) -> Result<Vec<u8>, anyhow::Error> {
    let config = load_const_config().await;
    let client = Client::new(&config);

    let resp = client.get_object().bucket(AwsConfigConst::BUCKET_NAME).key(key).send().await?;
    let data = resp.body.collect().await?;

    Ok(data.into_bytes().to_vec())
}
