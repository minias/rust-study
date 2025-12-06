# example_sss

## 요약

> [Shamir secret sharing in Rust](https://github.com/dsprenkels/sss-rs) 코드를 이용한 샤미르 쉐어 알고리즘을 사용.
> 해당 코드에서는 DATA_SIZE = 64 제약 명시. (함수 시그니처와 반환 설명)
> 추가하고 싶다면 sss-rs를 포크하고, DATA_SIZE 타입을 변경하여 사용해야함.

## 실행 결과

```sh
$ cargo run

==================================================
🟦 [원본 암호키]
11199f0ef89786ed73fc2389fb808b26280f871b8bc12b3a08cc361d289f92723bf5f2a3a4ad2db615ae528907a2efc3e28f1d649ec8e181de9ffa50ed340c44
==================================================

🟩 [생성된 Shares - 총 3개]
  share[0] (len=113): 01c2aa456b090379b6de4fae78146a125e821396ec437c1137f32e7f86f882367270b25950b493e2224c6587513cd2d564fb496b267b6fe35a21a4922ad66eca5ca24ef61d81c2426b1fc6dcf1ead9dbcba427d3fa8f5502e49efbffbce1550ea9b46c42c98d2287a312f3a5dddcf7d662
  share[1] (len=113): 0221c50657b32acf2bfa5541bcae239c53a169d905673a400601f21a27621119b370b25950b493e2224c6587513cd2d564fb496b267b6fe35a21a4922ad66eca5ca24ef61d81c2426b1fc6dcf1ead9dbcba427d3fa8f5502e49efbffbce1550ea9b46c42c98d2287a312f3a5dddcf7d662
  share[2] (len=113): 0389e0ce432cc454a9e6aaed0931ede6a149b615ab7bf186e0a64f39b11460f50570b25950b493e2224c6587513cd2d564fb496b267b6fe35a21a4922ad66eca5ca24ef61d81c2426b1fc6dcf1ead9dbcba427d3fa8f5502e49efbffbce1550ea9b46c42c98d2287a312f3a5dddcf7d662
==================================================

🟨 [복호화에 사용할 Share 2개]
  share[0]: 01c2aa456b090379b6de4fae78146a125e821396ec437c1137f32e7f86f882367270b25950b493e2224c6587513cd2d564fb496b267b6fe35a21a4922ad66eca5ca24ef61d81c2426b1fc6dcf1ead9dbcba427d3fa8f5502e49efbffbce1550ea9b46c42c98d2287a312f3a5dddcf7d662
  share[1]: 0221c50657b32acf2bfa5541bcae239c53a169d905673a400601f21a27621119b370b25950b493e2224c6587513cd2d564fb496b267b6fe35a21a4922ad66eca5ca24ef61d81c2426b1fc6dcf1ead9dbcba427d3fa8f5502e49efbffbce1550ea9b46c42c98d2287a312f3a5dddcf7d662
  share[2]: 0389e0ce432cc454a9e6aaed0931ede6a149b615ab7bf186e0a64f39b11460f50570b25950b493e2224c6587513cd2d564fb496b267b6fe35a21a4922ad66eca5ca24ef61d81c2426b1fc6dcf1ead9dbcba427d3fa8f5502e49efbffbce1550ea9b46c42c98d2287a312f3a5dddcf7d662
==================================================

🟪 [복호화된 시크릿]
11199f0ef89786ed73fc2389fb808b26280f871b8bc12b3a08cc361d289f92723bf5f2a3a4ad2db615ae528907a2efc3e28f1d649ec8e181de9ffa50ed340c44
11199f0ef89786ed73fc2389fb808b26280f871b8bc12b3a08cc361d289f92723bf5f2a3a4ad2db615ae528907a2efc3e28f1d649ec8e181de9ffa50ed340c44
11199f0ef89786ed73fc2389fb808b26280f871b8bc12b3a08cc361d289f92723bf5f2a3a4ad2db615ae528907a2efc3e28f1d649ec8e181de9ffa50ed340c44
==================================================
```
