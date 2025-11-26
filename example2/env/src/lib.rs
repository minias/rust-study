// src/lib.rs
//! 환경설정 로딩 라이브러리 (env)
//! Clean Architecture 기반, 재사용 가능하도록 별도 크레이트로 구성

pub mod loader;
pub mod model;

pub use loader::{load_config, ConfigLoader};
pub use model::AppConfig;