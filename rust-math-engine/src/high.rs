//! 고등 과정 — 복소수, 다항식(기호 미분·부정적분·호너 평가), 등차·등비수열, 수치 미분·적분, 3D 벡터.

use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

// ─────────────────────────────── 복소수 ───────────────────────────────

/// 복소수 `a + b i`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Complex {
        Complex { re, im }
    }

    pub fn zero() -> Complex {
        Complex { re: 0.0, im: 0.0 }
    }

    pub fn from_re(re: f64) -> Complex {
        Complex { re, im: 0.0 }
    }

    /// 켤레복소수.
    pub fn conj(&self) -> Complex {
        Complex::new(self.re, -self.im)
    }

    /// 복소수의 절댓값(모듈러스).
    pub fn abs(&self) -> f64 {
        self.re.hypot(self.im)
    }

    /// 편각(라디안).
    pub fn arg(&self) -> f64 {
        self.im.atan2(self.re)
    }

    /// 정수 거듭제곱.
    pub fn powi(&self, n: i32) -> Complex {
        let mut result = Complex::new(1.0, 0.0);
        if n >= 0 {
            for _ in 0..n {
                result = result * *self;
            }
            result
        } else {
            for _ in 0..(-n) {
                result = result * *self;
            }
            Complex::new(1.0, 0.0) / result
        }
    }
}

impl Add for Complex {
    type Output = Complex;
    fn add(self, rhs: Complex) -> Complex {
        Complex::new(self.re + rhs.re, self.im + rhs.im)
    }
}

impl Sub for Complex {
    type Output = Complex;
    fn sub(self, rhs: Complex) -> Complex {
        Complex::new(self.re - rhs.re, self.im - rhs.im)
    }
}

impl Neg for Complex {
    type Output = Complex;
    fn neg(self) -> Complex {
        Complex::new(-self.re, -self.im)
    }
}

impl Mul for Complex {
    type Output = Complex;
    fn mul(self, rhs: Complex) -> Complex {
        Complex::new(
            self.re * rhs.re - self.im * rhs.im,
            self.re * rhs.im + self.im * rhs.re,
        )
    }
}

impl Div for Complex {
    type Output = Complex;
    fn div(self, rhs: Complex) -> Complex {
        let d = rhs.re * rhs.re + rhs.im * rhs.im;
        Complex::new(
            (self.re * rhs.re + self.im * rhs.im) / d,
            (self.im * rhs.re - self.re * rhs.im) / d,
        )
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.im >= 0.0 {
            write!(f, "{:.4} + {:.4}i", self.re, self.im)
        } else {
            write!(f, "{:.4} - {:.4}i", self.re, -self.im)
        }
    }
}

// ─────────────────────────────── 다항식 ───────────────────────────────

/// 실계수 다항식. `coeffs[i]`는 `x^i`의 계수(낮은 차수부터).
#[derive(Clone, Debug, PartialEq)]
pub struct Polynomial {
    coeffs: Vec<f64>,
}

impl Polynomial {
    /// 계수 벡터로 만든다. 끝의 0 계수는 잘라내며, 영다항식은 `[0.0]`로 표현한다.
    pub fn new(mut coeffs: Vec<f64>) -> Polynomial {
        while coeffs.len() > 1 && coeffs.last().map(|c| *c == 0.0).unwrap_or(false) {
            coeffs.pop();
        }
        if coeffs.is_empty() {
            coeffs.push(0.0);
        }
        Polynomial { coeffs }
    }

    /// `a x^n` 형태의 단항식.
    pub fn monomial(coeff: f64, degree: usize) -> Polynomial {
        let mut c = vec![0.0; degree + 1];
        c[degree] = coeff;
        Polynomial::new(c)
    }

    pub fn zero() -> Polynomial {
        Polynomial::new(vec![0.0])
    }

    pub fn coeffs(&self) -> &[f64] {
        &self.coeffs
    }

    /// `x^i`의 계수. 범위를 벗어나면 0.
    pub fn coeff(&self, i: usize) -> f64 {
        *self.coeffs.get(i).unwrap_or(&0.0)
    }

    /// 영다항식은 차수 0으로 본다.
    pub fn degree(&self) -> usize {
        self.coeffs.len() - 1
    }

    pub fn is_zero(&self) -> bool {
        self.coeffs.iter().all(|c| *c == 0.0)
    }

    /// 호너 방법으로 `x`에서의 값을 계산한다.
    pub fn eval(&self, x: f64) -> f64 {
        let mut acc = 0.0;
        for c in self.coeffs.iter().rev() {
            acc = acc * x + c;
        }
        acc
    }

    /// 기호 미분.
    pub fn derivative(&self) -> Polynomial {
        if self.coeffs.len() <= 1 {
            return Polynomial::zero();
        }
        let d: Vec<f64> = self
            .coeffs
            .iter()
            .enumerate()
            .skip(1)
            .map(|(i, c)| *c * i as f64)
            .collect();
        Polynomial::new(d)
    }

    /// 기호 부정적분(적분상수 0).
    pub fn integrate(&self) -> Polynomial {
        let mut out = vec![0.0];
        for (i, c) in self.coeffs.iter().enumerate() {
            out.push(*c / (i as f64 + 1.0));
        }
        Polynomial::new(out)
    }

    /// 정적분 `∫_a^b p(x) dx` — 기호 적분 결과를 정확히 평가한다.
    pub fn integrate_definite(&self, a: f64, b: f64) -> f64 {
        let f = self.integrate();
        f.eval(b) - f.eval(a)
    }

    pub fn add(&self, other: &Polynomial) -> Polynomial {
        let n = self.coeffs.len().max(other.coeffs.len());
        let mut c = vec![0.0; n];
        for i in 0..n {
            c[i] = self.coeff(i) + other.coeff(i);
        }
        Polynomial::new(c)
    }

    pub fn sub(&self, other: &Polynomial) -> Polynomial {
        let n = self.coeffs.len().max(other.coeffs.len());
        let mut c = vec![0.0; n];
        for i in 0..n {
            c[i] = self.coeff(i) - other.coeff(i);
        }
        Polynomial::new(c)
    }

    pub fn mul(&self, other: &Polynomial) -> Polynomial {
        let mut c = vec![0.0; self.coeffs.len() + other.coeffs.len() - 1];
        for (i, a) in self.coeffs.iter().enumerate() {
            for (j, b) in other.coeffs.iter().enumerate() {
                c[i + j] += a * b;
            }
        }
        Polynomial::new(c)
    }
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_zero() {
            return write!(f, "0");
        }
        let mut first = true;
        for i in (0..self.coeffs.len()).rev() {
            let c = self.coeffs[i];
            if c == 0.0 {
                continue;
            }
            let sign = if c < 0.0 {
                if first {
                    "-"
                } else {
                    " - "
                }
            } else if first {
                ""
            } else {
                " + "
            };
            let mag = c.abs();
            let body = if mag == 1.0 && i >= 1 {
                if i == 1 {
                    String::from("x")
                } else {
                    format!("x^{i}")
                }
            } else if i == 0 {
                format!("{mag}")
            } else if i == 1 {
                format!("{mag}x")
            } else {
                format!("{mag}x^{i}")
            };
            write!(f, "{sign}{body}")?;
            first = false;
        }
        Ok(())
    }
}

// ─────────────────────────────── 수열 ───────────────────────────────

/// 등차수열의 제 `n` 항(1부터 셈).
pub fn arithmetic_nth(a1: f64, d: f64, n: usize) -> f64 {
    assert!(n >= 1, "항 번호는 1부터 시작합니다");
    a1 + (n as f64 - 1.0) * d
}

/// 등차수열 첫 `n` 항의 합.
pub fn arithmetic_sum(a1: f64, d: f64, n: usize) -> f64 {
    n as f64 / 2.0 * (2.0 * a1 + (n as f64 - 1.0) * d)
}

/// 등비수열의 제 `n` 항.
pub fn geometric_nth(a1: f64, r: f64, n: usize) -> f64 {
    assert!(n >= 1, "항 번호는 1부터 시작합니다");
    a1 * r.powi(n as i32 - 1)
}

/// 등비수열 첫 `n` 항의 합.
pub fn geometric_sum(a1: f64, r: f64, n: usize) -> f64 {
    if (r - 1.0).abs() < 1e-15 {
        a1 * n as f64
    } else {
        a1 * (r.powi(n as i32) - 1.0) / (r - 1.0)
    }
}

// ───────────────────────── 수치 미분·적분 ─────────────────────────

/// 중앙차분으로 `f'(x)`를 근사한다(오차 O(h^2)).
pub fn numerical_derivative<F: Fn(f64) -> f64>(f: F, x: f64, h: f64) -> f64 {
    (f(x + h) - f(x - h)) / (2.0 * h)
}

/// 심프슨 1/3 법칙으로 `∫_a^b f dx`를 근사한다. 구간 수는 짝수로 보정된다.
pub fn simpson<F: Fn(f64) -> f64>(f: F, a: f64, b: f64, n: usize) -> f64 {
    let n = if n < 2 { 2 } else { n + (n % 2) };
    let h = (b - a) / n as f64;
    let mut s = f(a) + f(b);
    for i in 1..n {
        let x = a + h * i as f64;
        s += if i % 2 == 0 { 2.0 * f(x) } else { 4.0 * f(x) };
    }
    s * h / 3.0
}

// ─────────────────────────────── 3D 벡터 ───────────────────────────────

/// 3차원 벡터.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn dot(&self, o: &Vec3) -> f64 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }

    pub fn cross(&self, o: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * o.z - self.z * o.y,
            self.z * o.x - self.x * o.z,
            self.x * o.y - self.y * o.x,
        )
    }

    pub fn norm(&self) -> f64 {
        self.dot(self).sqrt()
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.4}, {:.4}, {:.4})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::approx_eq;
    use std::f64::consts::PI;

    #[test]
    fn complex_multiplication_and_division() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(3.0, -4.0);
        let p = a * b;
        assert!(approx_eq(p.re, 11.0, 1e-12) && approx_eq(p.im, 2.0, 1e-12));
        let q = a / b;
        assert!(approx_eq(q.re, -0.2, 1e-12) && approx_eq(q.im, 0.4, 1e-12));
        let m = a * a.conj();
        assert!(approx_eq(m.re, 5.0, 1e-12) && approx_eq(m.im, 0.0, 1e-12));
    }

    #[test]
    fn complex_modulus_argument_and_powers() {
        let z = Complex::new(3.0, 4.0);
        assert!(approx_eq(z.abs(), 5.0, 1e-12));
        assert!(approx_eq(Complex::new(0.0, 1.0).arg(), PI / 2.0, 1e-12));
        let i = Complex::new(0.0, 1.0);
        let i2 = i.powi(2);
        assert!(approx_eq(i2.re, -1.0, 1e-12) && approx_eq(i2.im, 0.0, 1e-12));
        // (1+i)^8 = 16
        let p = Complex::new(1.0, 1.0).powi(8);
        assert!(approx_eq(p.re, 16.0, 1e-9) && approx_eq(p.im, 0.0, 1e-9));
    }

    #[test]
    fn polynomial_horner_evaluation() {
        // 1 + 2x + 3x^2 at x = 3  →  34
        let p = Polynomial::new(vec![1.0, 2.0, 3.0]);
        assert!(approx_eq(p.eval(3.0), 34.0, 1e-12));
        assert_eq!(p.degree(), 2);
    }

    #[test]
    fn polynomial_symbolic_derivative_matches_known_result() {
        // p(x) = x^4 - 3x^2 + 2x - 5   →  p'(x) = 4x^3 - 6x + 2
        let p = Polynomial::new(vec![-5.0, 2.0, -3.0, 0.0, 1.0]);
        let d = p.derivative();
        assert_eq!(d.coeffs(), &[2.0, -6.0, 0.0, 4.0]);
        assert!(approx_eq(d.eval(2.0), 22.0, 1e-12));
        assert!(approx_eq(d.eval(-1.0), 4.0, 1e-12));
    }

    #[test]
    fn polynomial_symbolic_integral_and_definite_integral() {
        // ∫ 3x^2 dx = x^3
        let p = Polynomial::new(vec![0.0, 0.0, 3.0]);
        let f = p.integrate();
        assert_eq!(f.coeffs(), &[0.0, 0.0, 0.0, 1.0]);
        assert!(approx_eq(f.eval(2.0), 8.0, 1e-12));
        // ∫_0^3 3x^2 dx = 27
        assert!(approx_eq(p.integrate_definite(0.0, 3.0), 27.0, 1e-12));
    }

    #[test]
    fn polynomial_multiplication_and_degree() {
        let a = Polynomial::new(vec![1.0, 1.0]); // x + 1
        let b = Polynomial::new(vec![-1.0, 1.0]); // x - 1
        let c = a.mul(&b); // x^2 - 1
        assert_eq!(c.coeffs(), &[-1.0, 0.0, 1.0]);
        assert_eq!(c.degree(), 2);
        let d = a.add(&b);
        assert_eq!(d.coeffs(), &[0.0, 2.0]);
        assert_eq!(a.sub(&b).coeffs(), &[2.0]);
    }

    #[test]
    fn sequences_have_closed_form_values() {
        assert!(approx_eq(arithmetic_nth(3.0, 4.0, 10), 39.0, 1e-12));
        assert!(approx_eq(arithmetic_sum(3.0, 4.0, 10), 210.0, 1e-12));
        assert!(approx_eq(geometric_nth(1.0, 2.0, 10), 512.0, 1e-12));
        assert!(approx_eq(geometric_sum(1.0, 2.0, 10), 1023.0, 1e-12));
        // r = 1 인 경우 합 = a1 * n
        assert!(approx_eq(geometric_sum(5.0, 1.0, 4), 20.0, 1e-12));
    }

    #[test]
    fn numerical_derivative_of_cubic() {
        let f = |x: f64| x * x * x;
        assert!(approx_eq(numerical_derivative(f, 2.0, 1e-6), 12.0, 1e-6));
    }

    #[test]
    fn simpson_integrates_polynomial_and_sine() {
        let g = |x: f64| x * x;
        assert!(approx_eq(simpson(g, 0.0, 3.0, 100), 9.0, 1e-9));
        let s = |x: f64| x.sin();
        assert!(approx_eq(simpson(s, 0.0, PI, 200), 2.0, 1e-6));
    }

    #[test]
    fn vector_dot_cross_and_norm() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, -5.0, 6.0);
        assert!(approx_eq(a.dot(&b), 12.0, 1e-12));
        let c = Vec3::new(1.0, 0.0, 0.0).cross(&Vec3::new(0.0, 1.0, 0.0));
        assert!(approx_eq(c.z, 1.0, 1e-12) && approx_eq(c.x, 0.0, 1e-12));
        assert!(approx_eq(Vec3::new(3.0, 4.0, 0.0).norm(), 5.0, 1e-12));
        // 직교 벡터의 내적은 0
        let p = Vec3::new(1.0, 2.0, 3.0);
        let q = p.cross(&Vec3::new(0.0, 1.0, 0.0));
        assert!(approx_eq(p.dot(&q), 0.0, 1e-12));
    }
}
