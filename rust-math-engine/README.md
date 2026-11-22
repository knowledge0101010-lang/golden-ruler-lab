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
cargo test --release        # 단위 79 + 통합 13 = 92 테스트
cargo run --release         # 5단계 대표 문제 실제 계산 결과 출력
```

## 검증 방식

- `#[cfg(test)]` 단위 테스트 79개 + `tests/tier_integration.rs` 통합 테스트 13개.
- 모든 검증은 실제 계산값과 해석해(또는 정확 유리수)를 비교한다. 예: `A = V Λ Vᵀ` 재구성, `P A = L U` 재구성, `Qᵀ Q = I`, 열방정식 해와 `e^{-π²t} sin(πx)` 비교, `cond∞(H₃) = 748`.
- 위 명령은 로컬에서 직접 실행해 확인한다. **이 저장소에는 아직 GitHub Actions 워크플로가 커밋되어 있지 않다**(`.github/` 없음) — 머지 전 상태 점검에서 확인한 사실이며, 자동 CI를 붙일지는 별도 결정 사항이다.
- 외부 크레이트 0개·Python 파일 0개는 `tests/tier_integration.rs::no_external_crate_dependency_in_manifest`가 검사한다.

## 제약

- 외부 크레이트 의존성 없음(`[dependencies]` 비어 있음) — 오프라인에서도 빌드된다.
- Python 코드·Python 출력 없음. 모든 수치는 Rust가 계산한 값이다.
