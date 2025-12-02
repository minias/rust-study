// usecase/s3_service.rs
use anyhow::Result;
use crate::domain::s3::{S3Credential, UploadRequest, DownloadRequest};
use crate::infra::{secrets_manager::SecretsAdapter, s3_client::S3Adapter};

/// S3 유즈케이스 서비스
///
/// Domain 규칙 수행 → Infra Adapter 사용
pub struct S3Service {
    pub(crate) secrets: SecretsAdapter,
    s3: S3Adapter,
}

impl S3Service {
    /// 생성자
    pub fn new(secrets: SecretsAdapter, s3: S3Adapter) -> Self {
        Self { secrets, s3 }
    }

    /// Getter: SecretsAdapter 참조 반환
    pub fn secrets(&self) -> &SecretsAdapter {
        &self.secrets
    }

    /// Getter: S3Adapter 참조 반환
    pub fn s3(&self) -> &S3Adapter {
        &self.s3
    }

    /// 1) Secret 로드 후 반환
    pub async fn load_credential(&self, secret_name: &str) -> Result<S3Credential> {
        self.secrets.load_s3_secret(secret_name).await
    }

    /// 2) 버킷 생성
    pub async fn create_bucket(&self, name: &str) -> Result<()> {
        self.s3.create_bucket(name).await
    }

    /// 3) 파일 업로드
    pub async fn upload(&self, req: UploadRequest) -> Result<()> {
        self.s3.upload(req).await
    }

    /// 4) 파일 다운로드
    pub async fn download(&self, req: DownloadRequest) -> Result<()> {
        self.s3.download(req).await
    }
}
