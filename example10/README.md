# example10

## 요약

- Rust WorkSpace 이해
- 하나의 시드로 ETH / TRX / BTC 주소 생성
- MSA 형태로 완전 파일 분리
- 클린 아키텍처 기반 구조
- 주석 풍부 / 일관된 코드 스타일
- 니모닉 → 시드 모듈 또한 분리
- 니모닉 코드로 마스터시드를 만든다 (BIP-32)
- 만들어진 마스터 시드로 각 코인별 마스터키를 만든다 (BIP-44)
- 만들어진 마스터 키로 각 코인의 HD 파생지갑을 만든다

## Workspace 핵심

Cargo는 프로젝트 최상위의 디렉토리 단위로 crate를 인식하며,
src/ 내부에 있는 crate는 Cargo가 workspace member로 등록할 수 없습니다.

### 내부 폴더의 workspace member로 등록 못하는 원인

- Cargo는 crate 구조를 다음 규칙으로 단일화합니다:
- crate = 디렉토리 + Cargo.toml
- crate는 반드시 workspace 루트 기준으로 별도 디렉토리로 존재해야 함
- src/는 단일 binary 또는 library crate만 허용됨
- src/ 내부에 또 다른 crate를 중첩시킬 수 없음

> 출처 : [cargo-workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)

## 아키텍처 원칙

- src/main.rs = 단일 binary crate
- 각 코인 모듈은 완전 독립 crate
- Seed/BIP39/RFC8032/BTC/EVM/Tron 키 계산은 common crate에 위치
- 코인별 crate는 address derivation 만 담당
- 클린 아키텍처 원칙: domain → application → infrastructure 계층 분리(경량 적용)

## IA

```sh
example10/
├── rust-toolchain.toml
├── README.md
├── Cargo.toml              # workspace 루트 + Main Package
└── src/
│   └── main.rs             # Application 엔트리포인트
├── common/                 # common Package
│   ├── Cargo.toml          # workspace common Package
│   └── src/lib.rs
└── wallet/                 # wallet 그룹
    ├── bitcoin/            # bitcoin Package
    │   ├── Cargo.toml      # workspace bitcoin Package
    │   └── src/lib.rs
    ├── ethereum/           # ethereum Package
    │   ├── Cargo.toml      # workspace ethereum Package
    │   └── src/lib.rs
    └── tron/               # tron Package
        ├── Cargo.toml      # workspace tron Package
        └── src/lib.rs
```

## rust-toolchain.toml 설명

1. channel
   - "stable": 최신 안정화 버전을 사용
   - "beta" / "nightly": 각 채널 버전 지정 가능
   - "1.72.0" 처럼 특정 버전 고정 가능
   - 이 프로젝트에서는 안정성을 위해 stable 사용
2. profile
   1. Rust 설치 시 설치 구성 옵션을 결정
   2. 선택 가능한 profile:
      - default (표준): compiler + cargo + rust-std 설치
      - minimal: 최소 Rust 설치 (기본 tool만 설치)
      - complete: compiler + cargo + rust-std + rustfmt, clippy, docs 등 전체 설치
   3. 여기서는 default로 두고, 필요한 component를 따로 설치
3. components
   1. Rust 설치 시 자동으로 포함할 추가 구성 요소
   2. 예제:
      - "rustfmt": 코드 자동 포맷
      - "clippy": lint 검사
   3. 다른 가능 component: "rls", "rust-src" 등
4. targets
   1. cross compilation 가능 target 지정
   2. 예제:
      - "wasm32-unknown-unknown" → WebAssembly
      - "x86_64-pc-windows-gnu" → Windows cross compile
   3. 빈 리스트 ([]) 면 현재 시스템 기본 target만 설치

## 빌드 및 실행

```sh
cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
cargo run
  Compiling example10 v0.1.0 (/rust/rust-study/example10)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.90s
     Running `target/debug/example10`
Mnemonic: mammal practice capital pull entire punch runway exchange drama copper leg cup kick fork metal news lamp cry grunt device remain fringe spirit emerge
BTC: 007839e11c6598d7d426eadc1b75762fb02e5a8d368de8767c
ETH: 0xa2c47e04d1a25e3484fe508676b6e1929cb3ee17
TRON: 41a2c47e04d1a25e3484fe508676b6e1929cb3ee17
```
