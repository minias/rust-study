# env package

## 핵심

| 항목             | 설명                                   |
| ---------------- | -------------------------------------- |
| **패키지 명**    | `env` (요구사항 충족)                  |
| **구조**         | Clean Architecture: model, loader 분리 |
| **파일 포맷**    | `.yml` (serde_yaml 사용)               |
| **환경 선택 방식** | `APP_ENV=local/dev/prod` 택1         |
| **캐싱 방식**    | `once_cell::sync::OnceCell`            |
| **오류 처리**    | `thiserror` 기반 Custom Error          |
| **확장성**       | 다른 서비스에서도 그대로 재사용 가능   |

## RUST_PROFILE 설정

```sh
echo 'export RUST_PROFILE=local' >> ~/.zshrc
source ~/.zshrc
echo $RUST_PROFILE
```

## .env.sample.yml

```yml
database:
  host: "127.0.0.1"      # 로컬 DB 주소
  port: 3306             # MariaDB 기본 포트
  user: "user"           # 로컬 개발 계정
  password: "password"   # MacOS brew 설치 시 기본은 비밀번호 없음
  database: "database"   # 로컬 개발 DB 이름 (직접 생성 필요)
```

## Make 명령어

- [example1/README.md 참조](../../example1/Readme.md)

## Debug 실행

> .env.local.yml 가 있는 위치에서 실행해야 실행이됩니다.

```sh
./target/debug/env
AppConfig {
    database: DatabaseConfig {
        host: "127.0.0.1",
        port: 3306,
        user: "exmaple2",
        password: "exmaple2!",
        database: "exmaple2",
    },
}
```
