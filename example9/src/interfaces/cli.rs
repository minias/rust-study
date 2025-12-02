use crate::usecase::s3_service::S3Service;
use crate::domain::s3::{UploadRequest, DownloadRequest};

pub async fn run_cli(service: S3Service) {
    let secret_name = "my_s3_secret";
    let bucket_name = "msa-example9-bucket";
    let upload_key = "hello.txt";
    let download_path = "downloaded.txt";
    let file_content = b"Hello from MSA Rust!";

    let cred = service.load_credential(secret_name).await.unwrap();
    let s3_adapter = crate::infra::s3_client::S3Adapter::new(cred).await;
    let service = S3Service::new(service.secrets().clone(), s3_adapter);

    service.create_bucket(bucket_name).await.unwrap();
    service.upload(UploadRequest {
        bucket: bucket_name.into(),
        key: upload_key.into(),
        content: file_content.to_vec(),
    }).await.unwrap();
    service.download(DownloadRequest {
        bucket: bucket_name.into(),
        key: upload_key.into(),
        output_path: download_path.into(),
    }).await.unwrap();
}
