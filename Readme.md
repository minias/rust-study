# 러스트 학습

## 러스트 설치 방법

## 예제파일

- [입력된 수에서 짝수의 합 계산](./example1/)
- [환경설정 yml 파일 읽기](./example2/env/)
- [로컬 DB 설치 및 연결](./example2/mariaDb/)
- [통합 테스트 모듈 분리](./example3/)
- [단일파일로 러스트 실행 및 테스트](./example4/)
- [기본 restful API](./example6/)
- [https graceful shutdown](./example7/)
- [AWS S3+SecretManager](./example9/)

### 참고 문서 & 사이트

- [easy_rust](https://dhghomon.github.io/easy_rust/)
- [cargo book](https://doc.rust-lang.org/stable/cargo/index.html)
- [rust documents](https://doc.rust-lang.org/stable/)
- [Rust Standard Library](https://doc.rust-lang.org/stable/std/index.html)
  
## Cargo 오류

> error: checksum for `sqlx v0.8.6` changed between lock files

```sh
rm Cargo.lock
cargo clean
cargo fetch
cargo build
```

> warning: virtual workspace defaulting to resolver = "1" despite ... edition 2024 ...

```sh
cat > Cargo.toml

[workspace]
resolver = "3"
members = [
    "example1",
    ...
    "example6",
]
```
