# example9

## 요약

- AWS security manager 연결
- AWS S3 연결, 버킷생성, 파일 업로드, 파일 다운로드

## IA

```sh
rust-study/example9/
├── Cargo.toml
├── README.md
└── src/
    ├── domain/
    │      ├── mod.rs
    │      └── s3.rs              # Domain 모델(Struct, Entity)
    │
    ├── usecase/
    │      ├── mod.rs
    │      └── s3_service.rs      # S3 워크플로우 유즈케이스
    │
    ├── infra/
    │      ├── mod.rs
    │      ├── secrets_manager.rs # AWS Secret Manager 어댑터
    │      └── s3_client.rs       # AWS S3 Client 어댑터
    │
    ├── interfaces/
    │      ├── mod.rs
    │      └── cli.rs             # CLI 기반 API (REST로 변경 가능)
    │
    ├── app/
    │      └── main.rs            # App Entrypoint
    │
    └── lib.rs
```
