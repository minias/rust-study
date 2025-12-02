use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct S3Credential {
    pub access_key: String,
    pub secret_key: String,
    pub region: String,
}

#[derive(Debug, Clone)]
pub struct UploadRequest {
    pub bucket: String,
    pub key: String,
    pub content: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct DownloadRequest {
    pub bucket: String,
    pub key: String,
    pub output_path: String,
}
