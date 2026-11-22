//! 대학 과정 — 행렬(곱·역행렬·행렬식·LU·QR), 대칭행렬 고유값, 다변수 야코비안·뉴턴법,
//! 상미분방정식 RK4, 정규분포, 최소제곱 선형회귀.

use std::fmt;

use crate::middle::solve_linear_system;

/// 조밀 행렬(행 우선 저장).
#[derive(Clone, Debug, PartialEq)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

impl Matrix {
    /// 행 벡터들로 행렬을 만든다. 모든 행의 길이가 같아야 한다.
    pub fn from_rows(rows_in: &[Vec<f64>]) -> Matrix {
        assert!(!rows_in.is_empty(), "빈 행렬은 만들 수 없습니다");
        let cols = rows_in[0].len();
        assert!(cols > 0, "열이 없는 행렬은 만들 수 없습니다");
        assert!(
            rows_in.iter().all(|r| r.len() == cols),
            "행마다 길이가 다릅니다"
        );
        Matrix {
            rows: rows_in.len(),
            cols,
            data: rows_in.iter().flat_map(|r| r.iter().copied()).collect(),
        }
    }

    pub fn zeros(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    pub fn identity(n: usize) -> Matrix {
        let mut m = Matrix::zeros(n, n);
        for i in 0..n {
            m.set(i, i, 1.0);
        }
        m
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn get(&self, i: usize, j: usize) -> f64 {
        self.data[i * self.cols + j]
    }

    pub fn set(&mut self, i: usize, j: usize, v: f64) {
        self.data[i * self.cols + j] = v;
    }

    pub fn row(&self, i: usize) -> Vec<f64> {
        (0..self.cols).map(|j| self.get(i, j)).collect()
    }

    pub fn transpose(&self) -> Matrix {
        let mut m = Matrix::zeros(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                m.set(j, i, self.get(i, j));
            }
        }
        m
    }

    /// 행렬곱. 차원이 맞지 않으면 패닉.
    pub fn mul(&self, other: &Matrix) -> Matrix {
        assert!(
            self.cols == other.rows,
            "행렬곱 차원 불일치: {}x{} * {}x{}",
            self.rows,
            self.cols,
            other.rows,
            other.cols
        );
        let mut m = Matrix::zeros(self.rows, other.cols);
        for i in 0..self.rows {
            for k in 0..self.cols {
                let a = self.get(i, k);
                if a == 0.0 {
                    continue;
                }
                for j in 0..other.cols {
                    let v = m.get(i, j) + a * other.get(k, j);
                    m.set(i, j, v);
                }
            }
        }
        m
    }

    /// 행렬-벡터곱.
    pub fn matvec(&self, v: &[f64]) -> Vec<f64> {
        assert_eq!(v.len(), self.cols, "행렬-벡터 차원 불일치");
        (0..self.rows)
            .map(|i| (0..self.cols).map(|j| self.get(i, j) * v[j]).sum())
            .collect()
    }

    pub fn add(&self, other: &Matrix) -> Matrix {
        assert!(self.rows == other.rows && self.cols == other.cols, "차원 불일치");
        let mut m = self.clone();
        for i in 0..self.rows {
            for j in 0..self.cols {
                m.set(i, j, self.get(i, j) + other.get(i, j));
            }
        }
        m
    }

    pub fn sub(&self, other: &Matrix) -> Matrix {
        assert!(self.rows == other.rows && self.cols == other.cols, "차원 불일치");
        let mut m = self.clone();
        for i in 0..self.rows {
            for j in 0..self.cols {
                m.set(i, j, self.get(i, j) - other.get(i, j));
            }
        }
        m
    }

    pub fn scale(&self, k: f64) -> Matrix {
        let mut m = self.clone();
        for v in m.data.iter_mut() {
            *v *= k;
        }
        m
    }

    /// 부분 피벗 LU 분해로 행렬식 계산(특이행렬이면 0).
    pub fn determinant(&self) -> f64 {
        assert_eq!(self.rows, self.cols, "정방행렬이 아닙니다");
        let n = self.rows;
        let mut a = self.data.clone();
        let mut det = 1.0;
        for col in 0..n {
            let mut pivot = col;
            for r in col + 1..n {
                if a[r * n + col].abs() > a[pivot * n + col].abs() {
                    pivot = r;
                }
            }
            if a[pivot * n + col].abs() < 1e-14 {
                return 0.0;
            }
            if pivot != col {
                for j in 0..n {
                    a.swap(col * n + j, pivot * n + j);
                }
                det = -det;
            }
            det *= a[col * n + col];
            for r in col + 1..n {
                let f = a[r * n + col] / a[col * n + col];
                for j in col..n {
                    a[r * n + j] -= f * a[col * n + j];
                }
            }
        }
        det
    }

    /// 가우스-조던 소거로 역행렬. 특이행렬이면 `None`.
    pub fn inverse(&self) -> Option<Matrix> {
        if self.rows != self.cols {
            return None;
        }
        let n = self.rows;
        let mut aug = Matrix::zeros(n, 2 * n);
        for i in 0..n {
            for j in 0..n {
                aug.set(i, j, self.get(i, j));
            }
            aug.set(i, n + i, 1.0);
        }
        for col in 0..n {
            let mut pivot = col;
            for r in col + 1..n {
                if aug.get(r, col).abs() > aug.get(pivot, col).abs() {
                    pivot = r;
                }
            }
            if aug.get(pivot, col).abs() < 1e-12 {
                return None;
            }
            if pivot != col {
                for j in 0..2 * n {
                    let tmp = aug.get(col, j);
                    aug.set(col, j, aug.get(pivot, j));
                    aug.set(pivot, j, tmp);
                }
            }
            let p = aug.get(col, col);
            for j in 0..2 * n {
                aug.set(col, j, aug.get(col, j) / p);
            }
            for r in 0..n {
                if r == col {
                    continue;
                }
                let f = aug.get(r, col);
                if f == 0.0 {
                    continue;
                }
                for j in 0..2 * n {
                    let v = aug.get(r, j) - f * aug.get(col, j);
                    aug.set(r, j, v);
                }
            }
        }
        let mut inv = Matrix::zeros(n, n);
        for i in 0..n {
            for j in 0..n {
                inv.set(i, j, aug.get(i, n + j));
            }
        }
        Some(inv)
    }

    /// 부분 피벗 LU 분해. `P A = L U`가 되도록 `(L, U, perm)`을 돌려준다.
    /// `perm[i]`는 결과 `i`번째 행이 원래 `perm[i]`번째 행이라는 뜻.
    pub fn lu(&self) -> Option<(Matrix, Matrix, Vec<usize>)> {
        if self.rows != self.cols {
            return None;
        }
        let n = self.rows;
        let mut a = self.clone();
        let mut perm: Vec<usize> = (0..n).collect();
        let mut l = Matrix::identity(n);
        for col in 0..n {
            let mut pivot = col;
            for r in col + 1..n {
                if a.get(r, col).abs() > a.get(pivot, col).abs() {
                    pivot = r;
                }
            }
            if a.get(pivot, col).abs() < 1e-12 {
                return None;
            }
            if pivot != col {
                for j in 0..n {
                    let tmp = a.get(col, j);
                    a.set(col, j, a.get(pivot, j));
                    a.set(pivot, j, tmp);
                }
                for j in 0..col {
                    let tmp = l.get(col, j);
                    l.set(col, j, l.get(pivot, j));
                    l.set(pivot, j, tmp);
                }
                perm.swap(col, pivot);
            }
            for r in col + 1..n {
                let f = a.get(r, col) / a.get(col, col);
                l.set(r, col, f);
                for j in col..n {
                    let v = a.get(r, j) - f * a.get(col, j);
                    a.set(r, j, v);
                }
            }
        }
        let mut u = Matrix::zeros(n, n);
        for i in 0..n {
            for j in i..n {
                u.set(i, j, a.get(i, j));
            }
        }
        Some((l, u, perm))
    }

    /// LU 분해로 `A x = b`를 푼다.
    pub fn solve(&self, b: &[f64]) -> Option<Vec<f64>> {
        let (l, u, perm) = self.lu()?;
        let n = self.rows;
        if b.len() != n {
            return None;
        }
        let pb: Vec<f64> = perm.iter().map(|i| b[*i]).collect();
        let mut y = vec![0.0; n];
        for i in 0..n {
            let mut s = pb[i];
            for j in 0..i {
                s -= l.get(i, j) * y[j];
            }
            y[i] = s;
        }
        let mut x = vec![0.0; n];
        for i in (0..n).rev() {
            let mut s = y[i];
            for j in i + 1..n {
                s -= u.get(i, j) * x[j];
            }
            if u.get(i, i).abs() < 1e-14 {
                return None;
            }
            x[i] = s / u.get(i, i);
        }
        Some(x)
    }

    /// 수정 그람-슈미트 QR 분해 `A = Q R` (Q 정규직교, R 상삼각).
    /// 열이 선형종속이면 `None`.
    pub fn qr(&self) -> Option<(Matrix, Matrix)> {
        let (m, n) = (self.rows, self.cols);
        if m < n {
            return None;
        }
        let mut q = Matrix::zeros(m, n);
        let mut r = Matrix::zeros(n, n);
        for j in 0..n {
            let mut v: Vec<f64> = (0..m).map(|i| self.get(i, j)).collect();
            for i in 0..j {
                let qi: Vec<f64> = (0..m).map(|k| q.get(k, i)).collect();
                let dot: f64 = qi.iter().zip(v.iter()).map(|(a, b)| a * b).sum();
                r.set(i, j, dot);
                for k in 0..m {
                    v[k] -= dot * qi[k];
                }
            }
            let norm = v.iter().map(|x| x * x).sum::<f64>().sqrt();
            if norm < 1e-12 {
                return None;
            }
            r.set(j, j, norm);
            for k in 0..m {
                q.set(k, j, v[k] / norm);
            }
        }
        Some((q, r))
    }

    /// 무한대 노름(행 합 최댓값).
    pub fn norm_inf(&self) -> f64 {
        (0..self.rows)
            .map(|i| (0..self.cols).map(|j| self.get(i, j).abs()).sum::<f64>())
            .fold(0.0, f64::max)
    }

    /// 무한대 노름 기준 조건수 추정 `||A||_inf * ||A^-1||_inf`. 특이행렬이면 `f64::INFINITY`.
    pub fn condition_number(&self) -> f64 {
        match self.inverse() {
            Some(inv) => self.norm_inf() * inv.norm_inf(),
            None => f64::INFINITY,
        }
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.rows {
            write!(f, "[")?;
            for j in 0..self.cols {
                if j > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:9.4}", self.get(i, j))?;
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}

/// 대칭행렬의 야코비 회전법. 고유값(오름차순)과 그에 대응하는 고유벡터(열)를 돌려준다.
pub fn jacobi_eigen(a: &Matrix, tol: f64, max_iter: usize) -> Option<(Vec<f64>, Matrix)> {
    if a.rows() != a.cols() {
        return None;
    }
    let n = a.rows();
    for i in 0..n {
        for j in 0..n {
            if (a.get(i, j) - a.get(j, i)).abs() > 1e-9 {
                return None;
            }
        }
    }
    let mut m = a.clone();
    let mut v = Matrix::identity(n);
    for _ in 0..max_iter {
        // 가장 큰 비대각 원소
        let (mut p, mut q, mut best) = (0usize, 0usize, 0.0f64);
        for i in 0..n {
            for j in i + 1..n {
                if m.get(i, j).abs() > best {
                    best = m.get(i, j).abs();
                    p = i;
                    q = j;
                }
            }
        }
        if best < tol {
            break;
        }
        let app = m.get(p, p);
        let aqq = m.get(q, q);
        let apq = m.get(p, q);
        let theta = 0.5 * (aqq - app) / apq;
        let t = if theta >= 0.0 {
            1.0 / (theta + (1.0 + theta * theta).sqrt())
        } else {
            -1.0 / (-theta + (1.0 + theta * theta).sqrt())
        };
        let c = 1.0 / (1.0 + t * t).sqrt();
        let s = t * c;
        // 비대각 항을 0으로 만드는 회전: A' = J^T A J
        for k in 0..n {
            if k == p || k == q {
                continue;
            }
            let mkp = m.get(k, p);
            let mkq = m.get(k, q);
            let np = c * mkp - s * mkq;
            let nq = s * mkp + c * mkq;
            m.set(k, p, np);
            m.set(p, k, np);
            m.set(k, q, nq);
            m.set(q, k, nq);
        }
        m.set(p, p, c * c * app - 2.0 * s * c * apq + s * s * aqq);
        m.set(q, q, s * s * app + 2.0 * s * c * apq + c * c * aqq);
        m.set(p, q, 0.0);
        m.set(q, p, 0.0);
        for k in 0..n {
            let vkp = v.get(k, p);
            let vkq = v.get(k, q);
            v.set(k, p, c * vkp - s * vkq);
            v.set(k, q, s * vkp + c * vkq);
        }
    }
    let mut pairs: Vec<(f64, Vec<f64>)> = (0..n)
        .map(|i| (m.get(i, i), (0..n).map(|k| v.get(k, i)).collect::<Vec<f64>>()))
        .collect();
    pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    let eigvals: Vec<f64> = pairs.iter().map(|(l, _)| *l).collect();
    let mut eigvecs = Matrix::zeros(n, n);
    for (j, (_, vec)) in pairs.iter().enumerate() {
        for i in 0..n {
            eigvecs.set(i, j, vec[i]);
        }
    }
    Some((eigvals, eigvecs))
}

/// 다변수 함수 `f: R^n -> R^m`의 야코비 행렬을 중앙차분으로 근사한다.
pub fn jacobian<F: Fn(&[f64]) -> Vec<f64>>(f: F, x: &[f64], h: f64) -> Matrix {
    let n = x.len();
    let f0 = f(x);
    let m = f0.len();
    let mut j = Matrix::zeros(m, n);
    for k in 0..n {
        let mut xp = x.to_vec();
        let mut xm = x.to_vec();
        xp[k] += h;
        xm[k] -= h;
        let fp = f(&xp);
        let fm = f(&xm);
        for i in 0..m {
            j.set(i, k, (fp[i] - fm[i]) / (2.0 * h));
        }
    }
    j
}

/// 다변수 뉴턴-랩슨법으로 `f(x) = 0`을 푼다.
pub fn newton_system<F: Fn(&[f64]) -> Vec<f64>>(
    f: F,
    x0: &[f64],
    tol: f64,
    max_iter: usize,
) -> Option<Vec<f64>> {
    let mut x = x0.to_vec();
    for _ in 0..max_iter {
        let fx = f(&x);
        if fx.iter().all(|v| v.abs() < tol) {
            return Some(x);
        }
        let j = jacobian(&f, &x, 1e-7);
        let step = j.solve(&fx)?;
        for i in 0..x.len() {
            x[i] -= step[i];
        }
    }
    let fx = f(&x);
    if fx.iter().all(|v| v.abs() < tol * 10.0) {
        Some(x)
    } else {
        None
    }
}

/// 고전 4차 룽게-쿠타법으로 `y' = f(t, y)`를 적분한다. `(t, y)` 점 목록을 돌려준다.
pub fn rk4<F: Fn(f64, f64) -> f64>(
    f: F,
    t0: f64,
    y0: f64,
    t_end: f64,
    steps: usize,
) -> Vec<(f64, f64)> {
    assert!(steps > 0, "스텝 수는 1 이상이어야 합니다");
    let h = (t_end - t0) / steps as f64;
    let mut t = t0;
    let mut y = y0;
    let mut out = vec![(t, y)];
    for _ in 0..steps {
        let k1 = f(t, y);
        let k2 = f(t + h / 2.0, y + h * k1 / 2.0);
        let k3 = f(t + h / 2.0, y + h * k2 / 2.0);
        let k4 = f(t + h, y + h * k3);
        y += h / 6.0 * (k1 + 2.0 * k2 + 2.0 * k3 + k4);
        t += h;
        out.push((t, y));
    }
    out
}

/// 오차함수 근사(Abramowitz–Stegun 7.1.26, 최대 오차 ~1.5e-7).
pub fn erf(x: f64) -> f64 {
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();
    let t = 1.0 / (1.0 + 0.3275911 * x);
    let poly = ((((1.061405429 * t - 1.453152027) * t + 1.421413741) * t - 0.284496736) * t
        + 0.254829592)
        * t;
    sign * (1.0 - poly * (-x * x).exp())
}

/// 정규분포 확률밀도함수.
pub fn normal_pdf(x: f64, mu: f64, sigma: f64) -> f64 {
    let z = (x - mu) / sigma;
    (-0.5 * z * z).exp() / (sigma * (2.0 * std::f64::consts::PI).sqrt())
}

/// 정규분포 누적분포함수.
pub fn normal_cdf(x: f64, mu: f64, sigma: f64) -> f64 {
    0.5 * (1.0 + erf((x - mu) / (sigma * std::f64::consts::SQRT_2)))
}

/// 최소제곱 단순 선형회귀 `y = a + b x`. `(a, b)`와 결정계수 `R^2`를 돌려준다.
pub fn least_squares_line(xs: &[f64], ys: &[f64]) -> Option<(f64, f64, f64)> {
    if xs.len() != ys.len() || xs.len() < 2 {
        return None;
    }
    let n = xs.len() as f64;
    let sx: f64 = xs.iter().sum();
    let sy: f64 = ys.iter().sum();
    let sxx: f64 = xs.iter().map(|x| x * x).sum();
    let sxy: f64 = xs.iter().zip(ys.iter()).map(|(x, y)| x * y).sum();
    let denom = n * sxx - sx * sx;
    if denom.abs() < 1e-15 {
        return None;
    }
    let b = (n * sxy - sx * sy) / denom;
    let a = (sy - b * sx) / n;
    let mean_y = sy / n;
    let ss_tot: f64 = ys.iter().map(|y| (y - mean_y).powi(2)).sum();
    let ss_res: f64 = xs
        .iter()
        .zip(ys.iter())
        .map(|(x, y)| (y - (a + b * x)).powi(2))
        .sum();
    let r2 = if ss_tot == 0.0 { 1.0 } else { 1.0 - ss_res / ss_tot };
    Some((a, b, r2))
}

/// 연립일차방정식을 `middle::solve_linear_system`으로 푸는 편의 함수(대학 과정 교차 검증용).
pub fn solve_with_gauss(a: &[Vec<f64>], b: &[f64]) -> Option<Vec<f64>> {
    solve_linear_system(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::approx_eq;

    fn m(rows: &[&[f64]]) -> Matrix {
        Matrix::from_rows(&rows.iter().map(|r| r.to_vec()).collect::<Vec<_>>())
    }

    #[test]
    fn matrix_multiplication_is_correct() {
        let a = m(&[&[1.0, 2.0], &[3.0, 4.0]]);
        let b = m(&[&[5.0, 6.0], &[7.0, 8.0]]);
        let c = a.mul(&b);
        assert_eq!(c.get(0, 0), 19.0);
        assert_eq!(c.get(0, 1), 22.0);
        assert_eq!(c.get(1, 0), 43.0);
        assert_eq!(c.get(1, 1), 50.0);
    }

    #[test]
    fn transpose_and_identity() {
        let a = m(&[&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]]);
        let t = a.transpose();
        assert_eq!((t.rows(), t.cols()), (3, 2));
        assert_eq!(t.get(2, 1), 6.0);
        let i = Matrix::identity(3);
        let p = a.mul(&i);
        assert_eq!(p, a);
    }

    #[test]
    fn determinant_of_three_by_three() {
        let a = m(&[&[1.0, 2.0, 3.0], &[0.0, 1.0, 4.0], &[5.0, 6.0, 0.0]]);
        assert!(approx_eq(a.determinant(), 1.0, 1e-9));
        let s = m(&[&[1.0, 2.0], &[2.0, 4.0]]);
        assert!(approx_eq(s.determinant(), 0.0, 1e-12));
    }

    #[test]
    fn inverse_times_matrix_is_identity() {
        let a = m(&[&[4.0, 7.0], &[2.0, 6.0]]);
        let inv = a.inverse().unwrap();
        assert!(approx_eq(inv.get(0, 0), 0.6, 1e-12));
        assert!(approx_eq(inv.get(0, 1), -0.7, 1e-12));
        let i = a.mul(&inv);
        for r in 0..2 {
            for c in 0..2 {
                let expect = if r == c { 1.0 } else { 0.0 };
                assert!(approx_eq(i.get(r, c), expect, 1e-10));
            }
        }
    }

    #[test]
    fn singular_matrix_has_no_inverse() {
        let s = m(&[&[1.0, 2.0], &[2.0, 4.0]]);
        assert!(s.inverse().is_none());
        assert!(s.lu().is_none());
    }

    #[test]
    fn lu_reconstructs_permuted_matrix() {
        let a = m(&[&[0.0, 5.0, 5.0], &[2.0, 9.0, 3.0], &[1.0, 3.0, 9.0]]);
        let (l, u, perm) = a.lu().unwrap();
        let lu = l.mul(&u);
        for i in 0..3 {
            for j in 0..3 {
                assert!(approx_eq(lu.get(i, j), a.get(perm[i], j), 1e-9));
            }
        }
    }

    #[test]
    fn lu_solve_matches_gauss_elimination() {
        let a = m(&[&[2.0, 3.0], &[1.0, -1.0]]);
        let x = a.solve(&[7.0, 1.0]).unwrap();
        assert!(approx_eq(x[0], 2.0, 1e-12));
        assert!(approx_eq(x[1], 1.0, 1e-12));
        let g = solve_with_gauss(
            &[vec![2.0, 3.0], vec![1.0, -1.0]],
            &[7.0, 1.0],
        )
        .unwrap();
        assert!(approx_eq(g[0], x[0], 1e-12));
    }

    #[test]
    fn qr_decomposition_is_orthonormal_and_reconstructs() {
        let a = m(&[&[12.0, -51.0], &[6.0, 167.0], &[-4.0, 24.0]]);
        let (q, r) = a.qr().unwrap();
        let qtq = q.transpose().mul(&q);
        for i in 0..2 {
            for j in 0..2 {
                let expect = if i == j { 1.0 } else { 0.0 };
                assert!(approx_eq(qtq.get(i, j), expect, 1e-9));
            }
        }
        let recon = q.mul(&r);
        for i in 0..3 {
            for j in 0..2 {
                assert!(approx_eq(recon.get(i, j), a.get(i, j), 1e-9));
            }
        }
    }

    #[test]
    fn jacobi_eigenvalues_of_symmetric_two_by_two() {
        let a = m(&[&[2.0, 1.0], &[1.0, 2.0]]);
        let (vals, vecs) = jacobi_eigen(&a, 1e-12, 200).unwrap();
        assert!(approx_eq(vals[0], 1.0, 1e-9));
        assert!(approx_eq(vals[1], 3.0, 1e-9));
        // A v = λ v 검증
        for k in 0..2 {
            let v: Vec<f64> = (0..2).map(|i| vecs.get(i, k)).collect();
            let av = a.matvec(&v);
            for i in 0..2 {
                assert!(approx_eq(av[i], vals[k] * v[i], 1e-9));
            }
        }
    }

    #[test]
    fn jacobi_rejects_nonsymmetric_matrix() {
        let a = m(&[&[1.0, 2.0], &[3.0, 4.0]]);
        assert!(jacobi_eigen(&a, 1e-12, 100).is_none());
    }

    #[test]
    fn jacobian_of_multivariable_function() {
        // f(x, y) = [x^2 y, 5x + sin y]  →  J = [[2xy, x^2], [5, cos y]]
        let f = |v: &[f64]| vec![v[0] * v[0] * v[1], 5.0 * v[0] + v[1].sin()];
        let j = jacobian(f, &[2.0, 3.0], 1e-6);
        assert!(approx_eq(j.get(0, 0), 12.0, 1e-5));
        assert!(approx_eq(j.get(0, 1), 4.0, 1e-5));
        assert!(approx_eq(j.get(1, 0), 5.0, 1e-5));
        assert!(approx_eq(j.get(1, 1), 3.0f64.cos(), 1e-5));
    }

    #[test]
    fn newton_system_finds_circle_line_intersection() {
        // x^2 + y^2 = 4, x = y  →  (√2, √2)
        let f = |v: &[f64]| vec![v[0] * v[0] + v[1] * v[1] - 4.0, v[0] - v[1]];
        let s = newton_system(f, &[1.0, 1.0], 1e-10, 50).unwrap();
        assert!(approx_eq(s[0], 2f64.sqrt(), 1e-8));
        assert!(approx_eq(s[1], 2f64.sqrt(), 1e-8));
    }

    #[test]
    fn rk4_solves_exponential_growth() {
        let pts = rk4(|_t, y| y, 0.0, 1.0, 1.0, 100);
        let (t, y) = *pts.last().unwrap();
        assert!(approx_eq(t, 1.0, 1e-12));
        assert!(approx_eq(y, std::f64::consts::E, 1e-6));
        assert_eq!(pts.len(), 101);
    }

    #[test]
    fn normal_distribution_pdf_and_cdf() {
        assert!(approx_eq(normal_pdf(0.0, 0.0, 1.0), 0.398_942_280_401_432_7, 1e-12));
        assert!(approx_eq(normal_cdf(0.0, 0.0, 1.0), 0.5, 1e-7));
        assert!(approx_eq(normal_cdf(1.96, 0.0, 1.0), 0.975, 1e-3));
        assert!(approx_eq(normal_cdf(10.0, 10.0, 2.0), 0.5, 1e-7));
    }

    #[test]
    fn least_squares_recovers_exact_line() {
        let xs = [1.0, 2.0, 3.0, 4.0];
        let ys = [5.0, 7.0, 9.0, 11.0]; // y = 3 + 2x
        let (a, b, r2) = least_squares_line(&xs, &ys).unwrap();
        assert!(approx_eq(a, 3.0, 1e-10));
        assert!(approx_eq(b, 2.0, 1e-10));
        assert!(approx_eq(r2, 1.0, 1e-10));
    }

    #[test]
    fn condition_number_of_identity_is_one() {
        let i = Matrix::identity(4);
        assert!(approx_eq(i.condition_number(), 1.0, 1e-9));
    }
}
