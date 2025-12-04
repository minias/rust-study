# example7

## 요약

- 3번에 걸려서 tls지원 (https)
- http 리다이렉트 http
- https graceful 지원(샘플 자체)
- 패키지 이슈로 결국 axum 0.7버전 샘플 코드 [tls-graceful-shutdown](https://github.com/tokio-rs/axum/tree/main/examples/tls-graceful-shutdown)를 참고

## 러스트 패키지 버전 안정성

- 네트워크와 보안 관련해서는 패키지가 자주 바뀜
- 코드 형태가 자주 바뀜으로 안정성에 문제가 생김
- 가급적 최신 버전 보다 안정화 또는 많이 사용하는 코드를 사용하길 권장
- 러스트 패키지 생태계 완숙도가 높지 않음 (2025.12.04 현재 auxum 0.7)
- 암호로직, 네트워크 로직에서 보안취약점이 빈번하게 발생중.
- 타언어처럼 패키지를 믿으면, ChatGPT를 믿는것과 같은 꼴남.
