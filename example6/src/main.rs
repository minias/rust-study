mod controllers;
mod models;
mod services;
mod router;

#[tokio::main]
async fn main() {
    // 라우터 초기화
    let app = router::create_router();

    // 서버 실행
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind port");

    println!("Server running at http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
