use crate::domain::s3::{DownloadRequest, S3Credential, UploadRequest};
use anyhow::Result;
use tokio::io::AsyncWriteExt;

// AWS SDK v1.x
use aws_sdk_s3::{Client, config::Region};
use aws_sdk_s3::config::Credentials;
use aws_config::BehaviorVersion;

#[derive(Clone)]
pub struct S3Adapter {
    client: Client,
}

impl S3Adapter {
    /// S3 클라이언트를 초기화한다.
    /// - 사용자 정의 Credentials 사용
    /// - Region, BehaviorVersion 설정
    pub async fn new(cred: S3Credential) -> Self {
        // 사용자 정의 Credentials 생성
        let custom_credentials = Credentials::new(
            cred.access_key,
            cred.secret_key,
            None,     // session token
            None,     // expiry
            "custom", // provider name
        );

        // AWS SDK Config 생성
        // NOTE: from_env()는 deprecated → defaults(BehaviorVersion::latest()) 사용
        let shared_config = aws_config::defaults(BehaviorVersion::latest())
            .credentials_provider(custom_credentials)
            .region(Region::new(cred.region.clone()))
            .load()
            .await;

        // S3 클라이언트 생성
        let client = Client::new(&shared_config);

        Self { client }
    }

    /// 버킷 생성
    pub async fn create_bucket(&self, name: &str) -> Result<()> {
        self.client.create_bucket().bucket(name).send().await?;
        Ok(())
    }

    /// 파일 업로드
    pub async fn upload(&self, req: UploadRequest) -> Result<()> {
        self.client
            .put_object()
            .bucket(req.bucket)
            .key(req.key)
            .body(req.content.into())
            .send()
            .await?;
        Ok(())
    }

    /// 파일 다운로드 후 로컬 파일로 저장
    pub async fn download(&self, req: DownloadRequest) -> Result<()> {
        let obj = self
            .client
            .get_object()
            .bucket(req.bucket)
            .key(req.key)
            .send()
            .await?;

        let bytes = obj.body.collect().await?.into_bytes();
        let mut file = tokio::fs::File::create(req.output_path).await?;
        file.write_all(&bytes).await?;
        Ok(())
    }
}
