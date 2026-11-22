//! 대학원 과정 — 최적화(경사하강 + Armijo, BFGS), 1D 열방정식 유한차분(CFL 검사),
//! 마르코프 체인 정상분포(거듭제곱 반복), 수치 안정성(Kahan 합, log-sum-exp, 조건수).

use crate::undergrad::Matrix;

/// 중심차분으로 기울기를 계산한다.
pub fn numerical_gradient<F: Fn(&[f64]) -> f64>(f: &F, x: &[f64]) -> Vec<f64> {
    let h = 1e-6;
    let mut g = vec![0.0; x.len()];
    for i in 0..x.len() {
        let mut xp = x.to_vec();
        let mut xm = x.to_vec();
        xp[i] += h;
        xm[i] -= h;
        g[i] = (f(&xp) - f(&xm)) / (2.0 * h);
    }
    g
}

fn dot(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

fn norm(a: &[f64]) -> f64 {
    dot(a, a).sqrt()
}

/// Armijo 조건을 만족하는 보폭을 역추적으로 찾는다.
fn backtracking<F: Fn(&[f64]) -> f64>(f: &F, x: &[f64], dir: &[f64], f0: f64, slope: f64) -> f64 {
    let mut alpha = 1.0;
    for _ in 0..60 {
        let cand: Vec<f64> = x.iter().zip(dir.iter()).map(|(xi, di)| xi + alpha * di).collect();
        if f(&cand) <= f0 + 1e-4 * alpha * slope {
            return alpha;
        }
        alpha *= 0.5;
    }
    alpha
}

/// 최적화 결과: 해, 함수값, 반복 횟수.
#[derive(Clone, Debug)]
pub struct OptResult {
    pub x: Vec<f64>,
    pub f: f64,
    pub iterations: usize,
}

/// Armijo 역추적을 쓰는 경사하강법.
pub fn gradient_descent<F: Fn(&[f64]) -> f64>(
    f: F,
    x0: &[f64],
    max_iter: usize,
    tol: f64,
) -> OptResult {
    let mut x = x0.to_vec();
    let mut fx = f(&x);
    let mut iters = 0;
    for it in 0..max_iter {
        iters = it + 1;
        let g = numerical_gradient(&f, &x);
        if norm(&g) < tol {
            break;
        }
        let dir: Vec<f64> = g.iter().map(|v| -v).collect();
        let alpha = backtracking(&f, &x, &dir, fx, -dot(&g, &g));
        for i in 0..x.len() {
            x[i] += alpha * dir[i];
        }
        let new_fx = f(&x);
        if (new_fx - fx).abs() < tol * tol {
            fx = new_fx;
            break;
        }
        fx = new_fx;
    }
    OptResult { x, f: fx, iterations: iters }
}

/// BFGS 준뉴턴법(역헤시안 근사 갱신) + Armijo 역추적.
pub fn bfgs<F: Fn(&[f64]) -> f64>(f: F, x0: &[f64], max_iter: usize, tol: f64) -> OptResult {
    let n = x0.len();
    let mut x = x0.to_vec();
    let mut fx = f(&x);
    let mut g = numerical_gradient(&f, &x);
    let mut h = vec![vec![0.0; n]; n]; // 역헤시안 근사
    for i in 0..n {
        h[i][i] = 1.0;
    }
    let mut iters = 0;
    for it in 0..max_iter {
        iters = it + 1;
        if norm(&g) < tol {
            break;
        }
        // p = -H g
        let mut p = vec![0.0; n];
        for i in 0..n {
            p[i] = -(0..n).map(|j| h[i][j] * g[j]).sum::<f64>();
        }
        if dot(&p, &g) >= 0.0 {
            // 하강 방향이 아니면 최급하강으로 대체
            for i in 0..n {
                p[i] = -g[i];
            }
        }
        let alpha = backtracking(&f, &x, &p, fx, dot(&g, &p));
        let step: Vec<f64> = p.iter().map(|v| alpha * v).collect();
        let x_new: Vec<f64> = x.iter().zip(step.iter()).map(|(a, b)| a + b).collect();
        let g_new = numerical_gradient(&f, &x_new);
        let s = step.clone();
        let y: Vec<f64> = g_new.iter().zip(g.iter()).map(|(a, b)| a - b).collect();
        let sy = dot(&s, &y);
        if sy > 1e-12 {
            // H ← (I - ρ s yᵀ) H (I - ρ y sᵀ) + ρ s sᵀ
            let rho = 1.0 / sy;
            let mut hy = vec![0.0; n];
            for i in 0..n {
                hy[i] = (0..n).map(|j| h[i][j] * y[j]).sum::<f64>();
            }
            let yhy = dot(&y, &hy);
            for i in 0..n {
                for j in 0..n {
                    h[i][j] += (rho * rho * yhy + rho) * s[i] * s[j]
                        - rho * (hy[i] * s[j] + s[i] * hy[j]);
                }
            }
        }
        x = x_new;
        g = g_new;
        let new_fx = f(&x);
        let converged = (new_fx - fx).abs() < tol;
        fx = new_fx;
        if converged {
            break;
        }
    }
    OptResult { x, f: fx, iterations: iters }
}

/// 1D 열방정식 `u_t = alpha u_xx`의 명시적 유한차분 스텝.
///
/// 경계는 디리클레 0(고정), 초기 배열의 양 끝은 경계값으로 취급한다.
/// CFL 조건 `alpha*dt/dx^2 <= 0.5`를 어기면 오류 문자열을 돌려준다.
pub fn heat_1d(
    initial: &[f64],
    alpha: f64,
    dx: f64,
    dt: f64,
    steps: usize,
) -> Result<Vec<f64>, String> {
    if initial.len() < 3 {
        return Err("격자점이 3개 미만입니다".to_string());
    }
    if dx <= 0.0 || dt <= 0.0 || alpha <= 0.0 {
        return Err("dx, dt, alpha는 양수여야 합니다".to_string());
    }
    let r = alpha * dt / (dx * dx);
    if r > 0.5 {
        return Err(format!(
            "CFL 불안정: r = alpha*dt/dx^2 = {r:.6} > 0.5 (dt를 {:.6} 이하로 줄이세요)",
            0.5 * dx * dx / alpha
        ));
    }
    let mut u = initial.to_vec();
    let n = u.len();
    let mut next = u.clone();
    for _ in 0..steps {
        for i in 1..n - 1 {
            next[i] = u[i] + r * (u[i + 1] - 2.0 * u[i] + u[i - 1]);
        }
        std::mem::swap(&mut u, &mut next);
    }
    Ok(u)
}

/// 에너지 소산 검사: 열방정식 해의 최대 절댓값이 시간에 따라 증가하지 않아야 한다.
pub fn heat_is_dissipative(initial: &[f64], alpha: f64, dx: f64, dt: f64, steps: usize) -> bool {
    let u0 = initial.iter().fold(0.0f64, |a, b| a.max(b.abs()));
    match heat_1d(initial, alpha, dx, dt, steps) {
        Ok(u) => {
            let u1 = u.iter().fold(0.0f64, |a, b| a.max(b.abs()));
            u1 <= u0 + 1e-12
        }
        Err(_) => false,
    }
}

/// 마르코프 체인(행 확률행렬)의 정상분포를 거듭제곱 반복으로 구한다.
pub fn markov_stationary(p: &[Vec<f64>], tol: f64, max_iter: usize) -> Option<Vec<f64>> {
    let n = p.len();
    if n == 0 || p.iter().any(|r| r.len() != n) {
        return None;
    }
    for row in p {
        let s: f64 = row.iter().sum();
        if (s - 1.0).abs() > 1e-9 || row.iter().any(|v| *v < 0.0) {
            return None;
        }
    }
    let mut pi = vec![1.0 / n as f64; n];
    for _ in 0..max_iter {
        let mut next = vec![0.0; n];
        for i in 0..n {
            for j in 0..n {
                next[j] += pi[i] * p[i][j];
            }
        }
        let delta = next
            .iter()
            .zip(pi.iter())
            .map(|(a, b)| (a - b).abs())
            .fold(0.0f64, f64::max);
        pi = next;
        if delta < tol {
            let s: f64 = pi.iter().sum();
            for v in pi.iter_mut() {
                *v /= s;
            }
            return Some(pi);
        }
    }
    None
}

/// 카한(Kahan) 보상 합산 — 큰 값과 작은 값이 섞여도 정확도를 지킨다.
pub fn kahan_sum(xs: &[f64]) -> f64 {
    let mut sum = 0.0;
    let mut c = 0.0;
    for &x in xs {
        let y = x - c;
        let t = sum + y;
        c = (t - sum) - y;
        sum = t;
    }
    sum
}

/// log-sum-exp: `ln(Σ exp(x_i))`를 오버플로 없이 계산한다. 빈 입력은 `-∞`.
pub fn log_sum_exp(xs: &[f64]) -> f64 {
    if xs.is_empty() {
        return f64::NEG_INFINITY;
    }
    let m = xs.iter().fold(f64::NEG_INFINITY, |a, b| a.max(*b));
    if !m.is_finite() {
        return m;
    }
    m + xs.iter().map(|x| (x - m).exp()).sum::<f64>().ln()
}

/// 무한대 노름 기준 조건수 추정.
pub fn condition_number(m: &Matrix) -> f64 {
    m.condition_number()
}

/// 소프트맥스 확률분포를 log-sum-exp로 안정하게 계산한다.
pub fn softmax(xs: &[f64]) -> Vec<f64> {
    let lse = log_sum_exp(xs);
    xs.iter().map(|x| (x - lse).exp()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::approx_eq;
    use std::f64::consts::PI;

    fn quadratic(x: &[f64]) -> f64 {
        (x[0] - 1.0).powi(2) + 2.0 * (x[1] + 3.0).powi(2)
    }

    #[test]
    fn gradient_descent_finds_quadratic_minimum() {
        let r = gradient_descent(quadratic, &[5.0, 5.0], 10_000, 1e-10);
        assert!(approx_eq(r.x[0], 1.0, 1e-4), "x = {:?}", r.x);
        assert!(approx_eq(r.x[1], -3.0, 1e-4), "x = {:?}", r.x);
        assert!(r.f < 1e-8);
    }

    #[test]
    fn bfgs_finds_quadratic_minimum_fast() {
        let r = bfgs(quadratic, &[5.0, 5.0], 200, 1e-12);
        assert!(approx_eq(r.x[0], 1.0, 1e-6), "x = {:?}, iters={}", r.x, r.iterations);
        assert!(approx_eq(r.x[1], -3.0, 1e-6), "x = {:?}", r.x);
    }

    #[test]
    fn bfgs_solves_rosenbrock() {
        let rosen = |x: &[f64]| (1.0 - x[0]).powi(2) + 100.0 * (x[1] - x[0] * x[0]).powi(2);
        let r = bfgs(rosen, &[-1.2, 1.0], 5_000, 1e-14);
        assert!(approx_eq(r.x[0], 1.0, 1e-2), "x = {:?}", r.x);
        assert!(approx_eq(r.x[1], 1.0, 1e-2), "x = {:?}", r.x);
    }

    #[test]
    fn heat_equation_matches_analytic_decay() {
        // u(x,0) = sin(pi x), 해: u(x,t) = exp(-pi^2 t) sin(pi x)
        let n = 41;
        let dx = 1.0 / (n as f64 - 1.0);
        let alpha = 1.0;
        let dt = 0.4 * dx * dx;
        let steps = 200;
        let t_end = dt * steps as f64;
        let init: Vec<f64> = (0..n)
            .map(|i| (PI * i as f64 * dx).sin())
            .collect();
        let u = heat_1d(&init, alpha, dx, dt, steps).unwrap();
        let mid = n / 2;
        let analytic = (-PI * PI * t_end).exp() * (PI * 0.5).sin();
        assert!(approx_eq(u[mid], analytic, 5e-3), "u={} analytic={}", u[mid], analytic);
        assert!(approx_eq(u[0], 0.0, 1e-12));
        assert!(approx_eq(u[n - 1], 0.0, 1e-12));
    }

    #[test]
    fn heat_equation_rejects_cfl_violation() {
        let init = vec![0.0, 1.0, 0.0];
        let err = heat_1d(&init, 1.0, 0.1, 0.01, 10).unwrap_err();
        assert!(err.contains("CFL"), "err = {err}");
    }

    #[test]
    fn heat_equation_is_dissipative() {
        let n = 21;
        let dx = 1.0 / (n as f64 - 1.0);
        let init: Vec<f64> = (0..n).map(|i| (PI * i as f64 * dx).sin()).collect();
        assert!(heat_is_dissipative(&init, 1.0, dx, 0.4 * dx * dx, 100));
    }

    #[test]
    fn markov_stationary_distribution() {
        let p = vec![vec![0.9, 0.1], vec![0.2, 0.8]];
        let pi = markov_stationary(&p, 1e-12, 10_000).unwrap();
        assert!(approx_eq(pi[0], 2.0 / 3.0, 1e-9), "pi = {:?}", pi);
        assert!(approx_eq(pi[1], 1.0 / 3.0, 1e-9));
        // 정상분포는 전이 후에도 그대로여야 한다.
        let mut next = vec![0.0; 2];
        for i in 0..2 {
            for j in 0..2 {
                next[j] += pi[i] * p[i][j];
            }
        }
        assert!(approx_eq(next[0], pi[0], 1e-9));
        assert!(approx_eq(next[1], pi[1], 1e-9));
    }

    #[test]
    fn markov_rejects_non_stochastic_matrix() {
        let p = vec![vec![0.5, 0.4], vec![0.2, 0.8]];
        assert!(markov_stationary(&p, 1e-12, 100).is_none());
    }

    #[test]
    fn markov_three_state_chain() {
        let p = vec![
            vec![0.5, 0.5, 0.0],
            vec![0.0, 0.5, 0.5],
            vec![0.5, 0.0, 0.5],
        ];
        let pi = markov_stationary(&p, 1e-13, 50_000).unwrap();
        for v in pi.iter() {
            assert!(approx_eq(*v, 1.0 / 3.0, 1e-8), "pi = {:?}", pi);
        }
    }

    #[test]
    fn kahan_sum_beats_naive_sum() {
        // 1 + 1e-16 - 1: 순진한 합은 작은 항이 흡수되어 0이 되지만,
        // 카한 보상 합산은 잃어버린 항을 회수한다.
        let xs = [1.0, 1e-16, -1.0];
        let naive: f64 = xs.iter().sum();
        let k = kahan_sum(&xs);
        assert!(approx_eq(k, 1e-16, 1e-16), "kahan = {k:e}, naive = {naive:e}");
        assert!(naive == 0.0, "naive = {naive:e}");

        // 0.1을 열 번 더하기
        let tenth = vec![0.1f64; 10];
        let naive10: f64 = tenth.iter().sum();
        let kahan10 = kahan_sum(&tenth);
        assert!((kahan10 - 1.0).abs() <= (naive10 - 1.0).abs());
    }

    #[test]
    fn log_sum_exp_is_overflow_safe() {
        assert!(approx_eq(log_sum_exp(&[1000.0, 1000.0]), 1000.0 + 2f64.ln(), 1e-9));
        assert!(approx_eq(log_sum_exp(&[0.0, 0.0]), 2f64.ln(), 1e-12));
        assert!(approx_eq(log_sum_exp(&[-1000.0, -1000.0]), -1000.0 + 2f64.ln(), 1e-9));
        let s = softmax(&[1000.0, 1000.0]);
        assert!(approx_eq(s[0], 0.5, 1e-12) && approx_eq(s[1], 0.5, 1e-12));
        assert_eq!(log_sum_exp(&[]), f64::NEG_INFINITY);
    }

    #[test]
    fn condition_number_estimate_of_hilbert_matrix() {
        let h = Matrix::from_rows(&[
            vec![1.0, 1.0 / 2.0, 1.0 / 3.0],
            vec![1.0 / 2.0, 1.0 / 3.0, 1.0 / 4.0],
            vec![1.0 / 3.0, 1.0 / 4.0, 1.0 / 5.0],
        ]);
        let cond = condition_number(&h);
        assert!(cond > 700.0 && cond < 800.0, "cond = {cond}");
        assert!(approx_eq(condition_number(&Matrix::identity(3)), 1.0, 1e-9));
    }

    #[test]
    fn numerical_gradient_matches_analytic() {
        let f = |x: &[f64]| x[0] * x[0] * x[1] + x[1].sin();
        let g = numerical_gradient(&f, &[2.0, 3.0]);
        assert!(approx_eq(g[0], 2.0 * 2.0 * 3.0, 1e-5));
        assert!(approx_eq(g[1], 4.0 + 3.0f64.cos(), 1e-5));
    }
}
