use example9::{
    usecase::s3_service::S3Service,
    infra::{secrets_manager::SecretsAdapter, s3_client::S3Adapter},
    interfaces::cli::run_cli,
    domain::s3::S3Credential,
    env::config::AwsConfig,
};

#[tokio::main]
async fn main() {

    let secrets = SecretsAdapter::new().await;
    let aws_cfg = AwsConfig::default();
    let s3_cred = S3Credential {
        access_key: aws_cfg.access_key,
        secret_key: aws_cfg.secret_key,
        region: aws_cfg.region,
    };
    let s3 = S3Adapter::new(s3_cred).await;
    let service = S3Service::new(secrets, s3);
    run_cli(service).await;
}
