# rust-math-engine

순수 Rust(**std만 사용**, 외부 크레이트 0개, Python 0줄)로 작성한 5단계 수학 계산 엔진.

| 단계 | 모듈 | 내용 |
|------|------|------|
| 초등 | `src/elementary.rs` | 정확한 유리수 `Rational`(i128 기약분수, 사칙·비교·거듭제곱), `gcd`/`lcm`, 소인수분해, 기약 비율, 백분율 |
| 중등 | `src/middle.rs` | 일차방정식, 이차방정식(판별식 → 실근/중근/복소근), 연립일차방정식(부분 피벗 가우스 소거), 평면·공간 거리, 피타고라스, `nCr`/`nPr`(정확한 u128) |
| 고등 | `src/high.rs` | 복소수 `Complex`, 다항식 `Polynomial`(사칙·기호 미분·기호 부정적분·호너 평가·정적분), 등차·등비수열, 중앙차분 미분, 심프슨 적분, 3D 벡터 내적·외적 |
| 대학 | `src/undergrad.rs` | `Matrix`(곱·전치·행렬식·가우스-조던 역행렬·LU·QR·해 구하기·무한대 노름·조건수), 대칭행렬 야코비 고유값/고유벡터, 야코비안, 다변수 뉴턴법, RK4 ODE, 정규분포 PDF/CDF, 최소제곱 선형회귀 |
| 대학원 | `src/grad.rs` | Armijo 역추적 경사하강, BFGS, 1D 열방정식 FDM(CFL 안정성 검사), 마르코프 체인 정상분포(거듭제곱 반복), Kahan 합산, log-sum-exp, softmax, 조건수 추정 |

## 실행

```bash
cd rust-math-engine
cargo test --verbose      # 단위 59 + 통합 13 = 72 테스트
cargo run --release       # 5단계 대표 문제 실제 계산 결과 출력
```

## 검증 방식

- `#[cfg(test)]` 단위 테스트 59개 + `tests/tier_integration.rs` 통합 테스트 13개.
- 모든 검증은 실제 계산값과 해석해(또는 정확 유리수)를 비교한다. 예: `A = V Λ Vᵀ` 재구성, `P A = L U` 재구성, `Qᵀ Q = I`, 열방정식 해와 `e^{-π²t} sin(πx)` 비교, `cond∞(H₃) = 748`.
- CI(`.github/workflows/rust-math-engine.yml`)가 `ubuntu-latest`에서 `cargo build` → `cargo test` → `cargo run --release`를 실제로 수행하고, 외부 크레이트 0개·Python 파일 0개를 강제 검사한다.

## 제약

- 외부 크레이트 의존성 없음(`[dependencies]` 비어 있음) — 오프라인에서도 빌드된다.
- Python 코드·Python 출력 없음. 모든 수치는 Rust가 계산한 값이다.
