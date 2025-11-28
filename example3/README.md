# example3

## 문제

```text
요일은 3자리 문자열로 표현됩니다.
요일을 나타내는 문자열 S와 정수 K(0~500 사이)가 주어지면 K일 후의 요일을 반환합니다. 
예를 들어, S = "Wed"이고 K = 2인 경우 함수는 "Fri"를 반환해야 합니다. 
S = "Sat"이고 K = 23인 경우 함수는 "Mon"을 반환해야 합니다.
```

## 학습 내용

1. 모듈 내부 테스트(Inline tests)

    일반적으로 라이브러리 코드 파일(src/lib.rs) 안에 #[cfg(test)] mod tests 형태로 작성하는 방식입니다.

    ✔ 장점

    - 테스트할 코드와 가까워 흐름 파악이 빠름
    - 작은 유닛 테스트 작성에 적합
    - 대부분의 Rust 크레이트가 이 방식을 기본으로 사용
2. 통합 테스트(Integration tests)

    tests/ 디렉토리에 별도 파일로 분리하는 방식입니다.

    ```sh
    example3/
    ├── Cargo.toml
    ├── src/
    │    ├── lib.rs
    │    └── main.rs
    └── tests/
        └── day_after_test.rs
    ```

    ✔ 장점
    - 실제 크레이트를 외부에서 사용하듯 테스트할 수 있음
    - 규모가 커질수록 테스트 파일을 분리해 관리하기에 용이
    - 여러 테스트를 파일 단위로 정리할 수 있어 구조적 설계에 적합
3. 언제 어떤 방식을 쓰면 좋은가?

    | 상황                                    | 권장 방식                      |
    | --------------------------------------- | ------------------------------ |
    | 간단한 함수, 유닛 테스트                | **같은 파일 내부 inline test** |
    | 모듈이 커지고 테스트가 많아짐           | **tests/ 디렉토리로 분리**     |
    | 외부 사용 관점에서 테스트하고 싶음      | **integration test**           |
    | 클린 아키텍처, 계층적 구조, 도메인 분리 | **tests/ 디렉토리 권장**       |

4. 공통 상수 구조체 생성 및 재사용.

## cargo run 오류

> 이 경고는 Rust 컴파일러(cargo)에서 증분 컴파일 캐시(incremental compilation cache)를 생성할 때 하드링크를 만들지 못했다는 의미입니다.
>
> MacOs 에서 기본디스크에서 사용시 발생하지 않는 문제입니다.

```sh
warning: hard linking files in the incremental compilation cache failed. copying files instead. consider moving the cache directory to a file system which supports hard linking in session dir `/rust-study/example3/target/debug/incremental/example3-1xbmtutnrlx6q/s-hde8ft4uxr-1aiw3s1-working`
```

### 원인

> MacOS에서 외장 SSD, 네트워크 드라이브, 혹은 일부 파일 시스템(HFS+, FAT 등)은 하드링크를 지원하지 않거나 제한적입니다.
>
> Cargo는 증분 컴파일 성능 향상을 위해 하드링크를 사용하려고 시도하지만, 실패하면 자동으로 파일 복사 방식으로 대신 처리합니다.
>
> 경고만 뜨며 컴파일은 정상적으로 진행됩니다.

### 해결방법

1. Cargo 증분 캐시 위치 변경
   > target 위치가 기본 폴더에 저장 (빌드파일 찾기가 힘듬.)
   > .cargo/config.toml 또는 환경변수로 증분 디렉터리를 다른 경로로 지정:

   ```sh
    # .cargo/config.toml
    [build]
    incremental = true
    target-dir = "/Users/username/dev/cargo-target"
   ```

2. 하드링크 가능한 로컬 디스크 사용
   > 프로젝트를 MacOS 내부 SSD(예: /Users/username/dev/...)로 옮기면 대부분 해결됩니다.
3. 경고 무시
   > 기능상 문제는 없고 성능만 약간 느려질 수 있음.
