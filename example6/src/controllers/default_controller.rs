use crate::models::default_model::DefaultResponse;
use crate::services::default_service;
use axum::Json;

/// GET /
pub async fn default() -> Json<DefaultResponse> {
    // 서비스 호출
    let message = default_service::get_message();

    Json(DefaultResponse { message })
}
