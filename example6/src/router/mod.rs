use crate::controllers::hello_controller;
use crate::controllers::default_controller;
use axum::{Router, routing::get};

pub fn create_router() -> Router {
    Router::new()        
        .route("/", get(default_controller::default))
        .route("/hello", get(hello_controller::hello))
}
