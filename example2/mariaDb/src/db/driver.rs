// src/db/driver.rs
/// 공용 Database Driver Trait
///
/// 클린 아키텍처를 위해 모든 DB 드라이버는 이 Trait을 구현해야 한다.

use async_trait::async_trait;
use sqlx::Error;

/// 단일 Row 타입 Alias - DB 타입 변경 시 여기만 수정하면 된다.
pub type DbRow = sqlx::mysql::MySqlRow;

/// 여러 Row 타입 Alias
pub type DbRows = Vec<DbRow>;

#[async_trait]
#[allow(dead_code)]
pub trait DatabaseDriver: Send + Sync {
    /// 데이터베이스 풀을 초기화한다.
    async fn init_pool(&self) -> Result<(), Error>;

    /// INSERT / UPDATE / DELETE 와 같은 실행 쿼리
    async fn execute(&self, sql: &str) -> Result<u64, Error>;

    /// 단일 레코드 조회
    async fn fetch_one(&self, sql: &str) -> Result<DbRow, Error>;

    /// 다중 레코드 조회
    async fn fetch_all(&self, sql: &str) -> Result<DbRows, Error>;
}
