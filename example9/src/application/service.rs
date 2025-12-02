// src/application/service.rs
use crate::infrastructure::aws_secrets;
use crate::infrastructure::aws_s3;

/// AWS 연동 서비스
pub struct AwsService;


impl AwsService {
    pub async fn process(&self) -> Result<(), anyhow::Error> {
        //AWS secret manager 호출먼저
        let secret = aws_secrets::load_secret_json("my/secret/name").await?;
        //Cleartext logging of sensitive information
        //println!("Loaded secret: {:?}", secret); //보안이슈
        println!("DB URL: {}", secret.db_url);
        println!("API Key: {}", secret.api_key);
        
        // 파일 업로드
        aws_s3::upload_file("hello.txt", b"hello world".to_vec()).await?;

        // 파일 다운로드
        let data = aws_s3::download_file("hello.txt").await?;
        println!("다운로드된 내용: {}", String::from_utf8_lossy(&data));

        Ok(())
    }
}

impl AwsService {
    pub async fn test_secret(&self) -> Result<(), anyhow::Error> {
        let secret = crate::infrastructure::aws_secrets::load_secret_json("my/secret/name").await?;
        println!("Loaded secret: {:?}", secret);
        Ok(())
    }
}