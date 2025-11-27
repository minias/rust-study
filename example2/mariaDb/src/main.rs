mod db;

use db::driver::DatabaseDriver;
use db::mariadb::MariaDbDriver;

use env::load_config;

// MySqlRow의 try_get 메서드를 사용하려면 Row trait을 import 해야 함
use sqlx::Row;

#[tokio::main]
async fn main() {
    let config = load_config().expect("환경설정 로딩 실패");

    let driver = MariaDbDriver::new(&config);

    driver.init_pool().await.expect("DB 초기화 실패");

    println!("MariaDB 연결 성공!");

    let rows = driver.fetch_all("SELECT 1 AS value").await.unwrap();

    for row in rows {
        // "value" 컬럼을 i32 타입으로 가져오기
        let v: i32 = row.try_get("value").unwrap();
        println!("값 = {}", v);
    }
}
