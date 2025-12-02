use anyhow::Result;
use crate::domain::s3::{S3Credential, UploadRequest, DownloadRequest};
use tokio::io::AsyncWriteExt;

#[derive(Clone)]
pub struct S3Adapter {
    client: aws_sdk_s3::Client,
}

impl S3Adapter {
 
    pub async fn new(cred: S3Credential) -> Self {
        let sdk_config = aws_config::from_env()
            .credentials_provider(aws_sdk_s3::Credentials::new(
                cred.access_key,
                cred.secret_key,
                None,
                None,
                "custom",
            ))
            .region(aws_sdk_s3::Region::new(cred.region))
            .load()
            .await;

        let client = aws_sdk_s3::Client::new(&sdk_config);
        Self { client }
    }
 
    pub async fn create_bucket(&self, name: &str) -> Result<()> {
        self.client.create_bucket().bucket(name).send().await?;
        Ok(())
    }

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

    pub async fn download(&self, req: DownloadRequest) -> Result<()> {
        let obj = self.client
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
