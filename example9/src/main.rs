mod domain;
mod application;
mod infrastructure;

use application::service::AwsService;

#[tokio::main]
async fn main() {
    let svc = AwsService;
    // 기존 process 호출
    if let Err(e) = svc.process().await {
        eprintln!("Error: {:?}", e);
    }
    // test_secret 호출
    if let Err(e) = svc.test_secret().await {
        eprintln!("Secret error: {:?}", e);
    }
}
