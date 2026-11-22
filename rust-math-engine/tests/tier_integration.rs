//! 5단계 통합 테스트 — 모듈 경계를 넘나드는 계산을 공개 API로만 검증한다.

use rust_math_engine::elementary::Rational;
use rust_math_engine::grad;
use rust_math_engine::high;
use rust_math_engine::high::{Complex, Polynomial, Vec3};
use rust_math_engine::middle;
use rust_math_engine::undergrad::{self, Matrix};

const TOL: f64 = 1e-9;

#[test]
fn tier1_exact_rational_arithmetic_crosses_zero() {
    // 1/2 + 1/3 - 5/6 = 0 (정확히 0이어야 한다)
    let sum = Rational::new(1, 2) + Rational::new(1, 3) - Rational::new(5, 6);
    assert!(sum.is_zero());
    assert!(sum.is_integer());
    // 1/2 * 2 = 1
    let one = Rational::new(1, 2) * Rational::from_int(2);
    assert!(one.equals(&Rational::from_int(1)));
}

#[test]
fn tier2_quadratic_roots_satisfy_the_equation() {
    let (a, b, c) = (2.0, -7.0, 3.0);
    match middle::solve_quadratic(a, b, c) {
        middle::Roots::TwoReal(r1, r2) => {
            for r in [r1, r2] {
                let v = a * r * r + b * r + c;
                assert!(v.abs() < TOL, "r = {r} 잔차 = {v}");
            }
        }
        other => panic!("실근 두 개를 기대했습니다: {other:?}"),
    }
}

#[test]
fn tier2_system_matches_matrix_solver() {
    let a = [vec![3.0, 2.0, -1.0], vec![2.0, -2.0, 4.0], vec![-1.0, 0.5, -1.0]];
    let b = [1.0, -2.0, 0.0];
    let gauss = middle::solve_linear_system(&a, &b).unwrap();
    let lu = Matrix::from_rows(&a).solve(&b).unwrap();
    for i in 0..3 {
        assert!((gauss[i] - lu[i]).abs() < 1e-9, "i={i}: {} vs {}", gauss[i], lu[i]);
    }
    // 잔차 0
    let m = Matrix::from_rows(&a);
    let r = m.matvec(&gauss);
    for i in 0..3 {
        assert!((r[i] - b[i]).abs() < 1e-9);
    }
}

#[test]
fn tier3_complex_roots_of_quadratic_are_conjugate() {
    let roots = middle::solve_quadratic(1.0, 2.0, 10.0);
    match roots {
        middle::Roots::Complex { re, im } => {
            let z1 = Complex::new(re, im);
            let z2 = Complex::new(re, -im);
            // (z - z1)(z - z2) = z^2 + 2z + 10
            let prod = z1 * z2;
            let p = Polynomial::new(vec![prod.re, -(z1.re + z2.re), 1.0]);
            assert!((p.coeff(0) - 10.0).abs() < TOL, "상수항 {}", p.coeff(0));
            assert!((prod.im).abs() < TOL, "켤레끼리 곱은 실수");
            assert!((p.coeff(1) - 2.0).abs() < TOL);
            assert!((z1.abs() - z2.abs()).abs() < TOL);
        }
        other => panic!("복소근을 기대했습니다: {other:?}"),
    }
}

#[test]
fn tier3_symbolic_and_numeric_derivative_agree() {
    let p = Polynomial::new(vec![1.0, 0.0, -2.0, 0.0, 0.5]); // 1 - 2x^2 + 0.5x^4
    let d = p.derivative();
    for x in [-1.5, -0.5, 0.0, 0.75, 2.0] {
        let numeric = high::numerical_derivative(|t| p.eval(t), x, 1e-6);
        assert!(
            (d.eval(x) - numeric).abs() < 1e-5,
            "x = {x}: 기호 {} vs 수치 {}",
            d.eval(x),
            numeric
        );
    }
}

#[test]
fn tier3_symbolic_integral_matches_simpson() {
    let p = Polynomial::new(vec![2.0, -1.0, 0.25]); // 2 - x + 0.25x^2
    let symbolic = p.integrate_definite(0.0, 4.0);
    let numeric = high::simpson(|x| p.eval(x), 0.0, 4.0, 1000);
    assert!((symbolic - numeric).abs() < 1e-9, "{symbolic} vs {numeric}");
    // [2x - x^2/2 + x^3/12]_0^4 = 8 - 8 + 64/12
    assert!((symbolic - 64.0 / 12.0).abs() < 1e-12);
}

#[test]
fn tier3_vector_triple_product_identity() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(-2.0, 4.0, 1.0);
    let c = Vec3::new(5.0, -1.0, 2.0);
    // a·(b×c) = b·(c×a) = c·(a×b) (스칼라 삼중적)
    let v1 = a.dot(&b.cross(&c));
    let v2 = b.dot(&c.cross(&a));
    let v3 = c.dot(&a.cross(&b));
    assert!((v1 - v2).abs() < TOL && (v2 - v3).abs() < TOL, "{v1} {v2} {v3}");
}

#[test]
fn tier4_eigen_decomposition_reconstructs_matrix() {
    let a = Matrix::from_rows(&[
        vec![4.0, 1.0, 2.0],
        vec![1.0, 3.0, 0.5],
        vec![2.0, 0.5, 5.0],
    ]);
    let (vals, vecs) = undergrad::jacobi_eigen(&a, 1e-14, 500).unwrap();
    // A = V Λ V^T
    let mut lambda = Matrix::zeros(3, 3);
    for i in 0..3 {
        lambda.set(i, i, vals[i]);
    }
    let recon = vecs.mul(&lambda).mul(&vecs.transpose());
    for i in 0..3 {
        for j in 0..3 {
            assert!((recon.get(i, j) - a.get(i, j)).abs() < 1e-9, "({i},{j})");
        }
    }
}

#[test]
fn tier4_inverse_and_determinant_are_consistent() {
    let a = Matrix::from_rows(&[
        vec![2.0, 1.0, 0.0],
        vec![1.0, 3.0, 1.0],
        vec![0.0, 1.0, 4.0],
    ]);
    let det = a.determinant();
    let inv = a.inverse().unwrap();
    let det_inv = inv.determinant();
    assert!((det * det_inv - 1.0).abs() < 1e-9, "det = {det}, det(inv) = {det_inv}");
}

#[test]
fn tier4_rk4_solves_trigonometric_ode() {
    // y' = cos t, y(0) = 0  →  y(t) = sin t, y(π/2) = 1
    let y = undergrad::rk4(|t, _v| t.cos(), 0.0, 0.0, std::f64::consts::PI / 2.0, 200);
    let last = y.last().unwrap().1;
    assert!((last - 1.0).abs() < 1e-9, "y(π/2) = {last}");
}

#[test]
fn tier5_optimizer_beats_starting_point_and_log_sum_exp_softmax_sums_to_one() {
    let quad = |x: &[f64]| (x[0] - 2.0).powi(2) + (x[1] - 5.0).powi(2);
    let start = [10.0, -4.0];
    let f_start = quad(&start);
    let r = grad::bfgs(quad, &start, 300, 1e-14);
    assert!(r.f < f_start, "{} < {}", r.f, f_start);
    assert!((r.x[0] - 2.0).abs() < 1e-6 && (r.x[1] - 5.0).abs() < 1e-6, "{:?}", r.x);

    let p = grad::softmax(&[1.0, 2.0, 3.0, 4.0]);
    let s: f64 = p.iter().sum();
    assert!((s - 1.0).abs() < 1e-12, "softmax 합 = {s}");
    assert!(p[3] > p[2] && p[2] > p[1] && p[1] > p[0]);
}

#[test]
fn tier5_heat_solution_stays_bounded_and_markov_stationary_is_fixed_point() {
    let n = 31;
    let dx = 1.0 / (n as f64 - 1.0);
    let dt = 0.25 * dx * dx;
    let init: Vec<f64> = (0..n)
        .map(|i| {
            let x = i as f64 * dx;
            4.0 * x * (1.0 - x) // 경계 0, 포물선 초기값
        })
        .collect();
    let u = grad::heat_1d(&init, 0.5, dx, dt, 500).unwrap();
    let peak = u.iter().fold(0.0f64, |a, b| a.max(b.abs()));
    assert!(peak <= 1.0 + 1e-9, "peak = {peak}");
    assert!(peak > 0.0, "해가 완전히 사라지면 안 됩니다");

    let p = vec![vec![0.7, 0.3], vec![0.4, 0.6]];
    let pi = grad::markov_stationary(&p, 1e-14, 100_000).unwrap();
    let mut next = vec![0.0; 2];
    for i in 0..2 {
        for j in 0..2 {
            next[j] += pi[i] * p[i][j];
        }
    }
    assert!((next[0] - pi[0]).abs() < 1e-9 && (next[1] - pi[1]).abs() < 1e-9);
    // 이론값: p0 = 0.4 / (0.3 + 0.4) = 4/7
    assert!((pi[0] - 4.0 / 7.0).abs() < 1e-9, "pi = {:?}", pi);
}

#[test]
fn no_external_crate_dependency_in_manifest() {
    // 순수 std 구현 확인: 매니페스트에 [dependencies] 항목이 비어 있어야 한다.
    let manifest = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml")).unwrap();
    let deps_section = manifest
        .split("[dependencies]")
        .nth(1)
        .expect("[dependencies] 섹션이 없습니다");
    let body = deps_section
        .split("\n[")
        .next()
        .unwrap()
        .lines()
        .filter(|l| !l.trim().is_empty() && !l.trim().starts_with('#'))
        .count();
    assert_eq!(body, 0, "외부 크레이트 의존성이 있습니다: {deps_section}");
}
