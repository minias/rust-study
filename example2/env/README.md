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
