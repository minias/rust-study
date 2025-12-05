# example9

## 요약

- AWS security manager 연결
- AWS S3 연결, 버킷생성, 파일 업로드, 파일 다운로드
- [rust aws예제](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/rustv1)

## IA

```sh
rust-study/example9/
├── Cargo.toml
├── env.sample.yml
├── env.sample.yml
└── src/
    ├── lib.rs
    ├── main.rs
    ├── domain/
    │      ├── mod.rs
    │      └── s3.rs
    ├── usecase/
    │      ├── mod.rs
    │      └── s3_service.rs
    ├── infra/
    │      ├── mod.rs
    │      ├── secrets_manager.rs
    │      └── s3_client.rs
    ├── interfaces/
    │      ├── mod.rs
    │      └── cli.rs
    └── env/
            ├── mod.rs
            └── config.rs
```
