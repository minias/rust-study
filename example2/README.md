# example2

## Mysql/MariaDB 를 로컬에서 연결

> Macos install MariaDB

```sh
brew install mariadb@11.8
brew services start mariadb@11.8
```

## 루트 패스워드 설정

```sh
sudo mysql
```

```sql
-- root@localhost 비밀번호 설정
ALTER USER 'root'@'localhost' IDENTIFIED BY '새로운비밀번호';

-- 변경 사항 적용
FLUSH PRIVILEGES;
```

## mysql 데이터베이스,유저 설정

```sql
-- 1️⃣ 데이터베이스 생성
CREATE DATABASE IF NOT EXISTS example2
  CHARACTER SET utf8mb4
  COLLATE utf8mb4_general_ci;

-- 2️⃣ 사용자 생성 (localhost에서만 접근 가능)
CREATE USER IF NOT EXISTS 'example2'@'localhost' IDENTIFIED BY 'example2!';

-- 3️⃣ 데이터베이스 권한 부여
GRANT ALL PRIVILEGES ON example2.* TO 'example2'@'localhost';

-- 4️⃣ 권한 적용
FLUSH PRIVILEGES;
```

## env 라이브러리 만들기
