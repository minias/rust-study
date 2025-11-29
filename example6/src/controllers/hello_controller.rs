use crate::models::hello_model::HelloResponse;
use crate::services::hello_service;
use axum::Json;

/// GET /hello
pub async fn hello() -> Json<HelloResponse> {
    // 서비스 호출
    let message = hello_service::get_message();

    Json(HelloResponse { message })
}
