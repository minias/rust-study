// src/db/mariadb.rs
//! MariaDB Driver Implementation
//! - OnceLock + Arc 를 활용한 전역 Pool 초기화
//! - 환경설정 기반 DSN 생성
//! - sqlx::mysql 전용 Pool 사용
//! - 클린아키텍처 기반: DriverTrait 구현부 분리

use std::sync::{Arc, OnceLock};

use async_trait::async_trait;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions, MySqlRow};
use sqlx::Error;

use env::model::AppConfig as Config;

use super::driver::DatabaseDriver;

// 전역 풀 저장소 (Arc + OnceLock)
// OnceLock<Arc<Pool>> 사용으로 전역 Pool 1회 초기화 보장
static POOL: OnceLock<Arc<MySqlPool>> = OnceLock::new();

/// MariaDB Driver
/// - &'a Config 를 받아 DSN 생성
/// - 풀은 OnceLock 에 의해 최초 1회만 생성됨
pub struct MariaDbDriver<'a> {
    config: &'a Config,
}

impl<'a> MariaDbDriver<'a> {
    /// 새 드라이버 생성
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    /// DSN 생성
    #[inline]
    fn dsn(&self) -> String {
        let db = &self.config.database;

        format!(
            "mysql://{}:{}@{}:{}/{}",
            db.user, db.password, db.host, db.port, db.database
        )
    }

    /// 풀 생성 (OnceLock이 async를 지원 안 하므로 분리)
    async fn create_pool(&self) -> Result<MySqlPool, Error> {
        MySqlPoolOptions::new()
            .max_connections(10)
            .connect(&self.dsn())
            .await
    }

    /// 풀 가져오기 (초기화 포함)
    async fn get_pool(&self) -> Result<Arc<MySqlPool>, Error> {
        // 이미 초기화되어있으면 그대로 반환
        if let Some(pool) = POOL.get() {
            return Ok(pool.clone());
        }

        // 최초 초기화 수행
        let pool = self.create_pool().await?;
        let arc = Arc::new(pool);

        // OnceLock 에 등록
        POOL.set(arc.clone()).unwrap();

        Ok(arc)
    }
}

#[async_trait]
impl<'a> DatabaseDriver for MariaDbDriver<'a> {
    /// 풀 초기화
    async fn init_pool(&self) -> Result<(), Error> {
        self.get_pool().await?;
        Ok(())
    }

    /// INSERT / UPDATE / DELETE 실행
    async fn execute(&self, sql: &str) -> Result<u64, Error> {
        let pool = self.get_pool().await?;

        let result = sqlx::query(sql)
            .execute(&*pool)
            .await?;

        Ok(result.rows_affected())
    }

    /// 단일 Row 조회
    async fn fetch_one(&self, sql: &str) -> Result<MySqlRow, Error> {
        let pool = self.get_pool().await?;

        sqlx::query(sql)
            .fetch_one(&*pool)
            .await
    }

    /// 여러 Row 조회
    async fn fetch_all(&self, sql: &str) -> Result<Vec<MySqlRow>, Error> {
        let pool = self.get_pool().await?;

        sqlx::query(sql)
            .fetch_all(&*pool)
            .await
    }
}
