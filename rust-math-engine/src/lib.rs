//! `rust-math-engine` — 순수 Rust(std만 사용) 수학 계산 엔진.
//!
//! 단계별 모듈:
//! * [`elementary`] 초등: 정확한 유리수, GCD/LCM, 소인수분해, 비율·백분율
//! * [`middle`]     중등: 일·이차방정식, 연립일차방정식(가우스 소거), 거리, 조합·순열
//! * [`high`]       고등: 복소수, 다항식(기호 미분·적분), 수열, 수치 미분·적분, 3D 벡터
//! * [`undergrad`]  대학: 행렬(LU/QR/역행렬/행렬식), 고유값, 야코비안·뉴턴, RK4, 정규분포, 회귀
//! * [`grad`]       대학원: 경사하강·BFGS, 열방정식 FDM, 마르코프 정상분포, 수치 안정성

pub mod elementary;
pub mod grad;
pub mod high;
pub mod middle;
pub mod undergrad;

/// 두 `f64` 값이 주어진 절대 오차 안에서 같은지 확인한다.
pub fn approx_eq(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() <= tol
}

/// 슬라이스 안의 모든 값을 절대 오차 `tol`로 비교한다.
pub fn approx_eq_slice(a: &[f64], b: &[f64], tol: f64) -> bool {
    a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| approx_eq(*x, *y, tol))
}
