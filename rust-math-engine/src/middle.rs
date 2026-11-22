//! 중등 과정 — 일차·이차방정식, 연립일차방정식(가우스 소거), 거리·피타고라스, 조합·순열.

use crate::elementary::{gcd_u128, Rational};

/// 일차방정식 `a x + b = 0`의 해. `a == 0`이면 해가 없거나 무수히 많으므로 `None`.
pub fn solve_linear(a: f64, b: f64) -> Option<f64> {
    if a == 0.0 {
        None
    } else {
        Some(-b / a)
    }
}

/// 이차방정식 `a x^2 + b x + c = 0`의 근.
#[derive(Clone, Debug, PartialEq)]
pub enum Roots {
    /// 서로 다른 두 실근(오름차순).
    TwoReal(f64, f64),
    /// 중근.
    Repeated(f64),
    /// 켤레 복소근 `re ± im i`.
    Complex { re: f64, im: f64 },
}

/// 판별식 `b^2 - 4ac`를 이용해 이차방정식을 푼다.
///
/// 두 실근은 상쇄(cancellation)가 생기지 않는 형태로 계산한다. 작은 근을
/// `(-b + sqrt(disc)) / 2a`로 직접 구하면 `b^2`에 비해 `4ac`가 작을 때 거의
/// 같은 두 수를 빼게 되어 유효 자릿수를 잃는다(예: x^2 + 10^8 x + 1).
/// 대신 `q = -0.5 * (b + copysign(sqrt(disc), b))`를 먼저 구하고 두 근을
/// `q/a`와 `c/q`로 얻는다.
pub fn solve_quadratic(a: f64, b: f64, c: f64) -> Roots {
    assert!(a != 0.0, "이차방정식의 최고차 계수가 0입니다");
    let disc = b * b - 4.0 * a * c;
    if disc > 0.0 {
        let s = disc.sqrt();
        let q = -0.5 * (b + s.copysign(b));
        let r1 = normalize_zero(q / a);
        let r2 = normalize_zero(c / q);
        if r1 <= r2 {
            Roots::TwoReal(r1, r2)
        } else {
            Roots::TwoReal(r2, r1)
        }
    } else if disc == 0.0 {
        Roots::Repeated(normalize_zero(-b / (2.0 * a)))
    } else {
        let s = (-disc).sqrt();
        let mut re = -b / (2.0 * a);
        if re == 0.0 {
            re = 0.0; // -0.0 정규화
        }
        Roots::Complex {
            re,
            im: s / (2.0 * a).abs(),
        }
    }
}

/// `-0.0`을 `0.0`으로 되돌려 출력·비교에서 부호가 새지 않게 한다.
fn normalize_zero(v: f64) -> f64 {
    if v == 0.0 {
        0.0
    } else {
        v
    }
}

/// 연립일차방정식 `A x = b`를 부분 피벗팅 가우스 소거로 푼다.
/// 특이행렬(해가 유일하지 않음)이면 `None`.
pub fn solve_linear_system(a: &[Vec<f64>], b: &[f64]) -> Option<Vec<f64>> {
    let n = a.len();
    if n == 0 || b.len() != n || a.iter().any(|row| row.len() != n) {
        return None;
    }
    let mut m: Vec<Vec<f64>> = a
        .iter()
        .enumerate()
        .map(|(i, row)| {
            let mut r = row.clone();
            r.push(b[i]);
            r
        })
        .collect();

    // 피벗 판정은 행렬 자체의 크기에 상대적으로 한다(절대 임계값은 단위 변환으로
    // 계수가 작아진 가역행렬을 특이행렬로 오판한다).
    let scale = a
        .iter()
        .flat_map(|row| row.iter())
        .fold(0.0f64, |m, v| m.max(v.abs()));
    if scale == 0.0 {
        return None;
    }
    let tol = 1e-12 * scale;

    for col in 0..n {
        // 부분 피벗 선택
        let mut pivot = col;
        for r in col + 1..n {
            if m[r][col].abs() > m[pivot][col].abs() {
                pivot = r;
            }
        }
        if m[pivot][col].abs() <= tol {
            return None;
        }
        m.swap(col, pivot);

        // 소거
        let p = m[col][col];
        for r in col + 1..n {
            let f = m[r][col] / p;
            if f != 0.0 {
                for c in col..=n {
                    m[r][c] -= f * m[col][c];
                }
            }
        }
    }

    // 후진 대입
    let mut x = vec![0.0f64; n];
    for i in (0..n).rev() {
        let mut s = m[i][n];
        for j in i + 1..n {
            s -= m[i][j] * x[j];
        }
        x[i] = s / m[i][i];
    }
    Some(x)
}

/// 평면의 두 점 사이 유클리드 거리.
pub fn distance_2d(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

/// 공간의 두 점 사이 유클리드 거리.
pub fn distance_3d(p: [f64; 3], q: [f64; 3]) -> f64 {
    (0..3).map(|i| (q[i] - p[i]).powi(2)).sum::<f64>().sqrt()
}

/// 피타고라스 정리: 두 직각변으로 빗변.
pub fn hypotenuse(a: f64, b: f64) -> f64 {
    (a * a + b * b).sqrt()
}

/// 조합 `nCr` (정확한 정수). `r > n`이면 0, 계산이 `u128`을 넘으면 `None`.
pub fn combination(n: u128, r: u128) -> Option<u128> {
    if r > n {
        return Some(0);
    }
    let r = r.min(n - r);
    let mut result: u128 = 1;
    for i in 0..r {
        let mut num = n - i;
        let mut den = i + 1;
        // 약분을 곱셈보다 먼저 한다. gcd(num, den) = 1 이면 den은 C(n, i)를
        // 나누어떨어지므로(정수성), 먼저 나눈 뒤 곱하면 중간값이 C(n, i+1) 자체가 되어
        // 표현 가능한 결과가 중간 오버플로로 실패하지 않는다(예: C(128, 64)).
        let g = gcd_u128(num, den);
        num /= g;
        den /= g;
        result /= den;
        result = result.checked_mul(num)?;
    }
    Some(result)
}

/// 순열 `nPr = n! / (n-r)!` (정확한 정수).
pub fn permutation(n: u128, r: u128) -> Option<u128> {
    if r > n {
        return Some(0);
    }
    let mut result: u128 = 1;
    for i in 0..r {
        result = result.checked_mul(n - i)?;
    }
    Some(result)
}

/// 중등 교과의 비례식 `a : b = c : d`에서 미지의 항 `d = b·c/a`를 구한다.
///
/// `a != 0`일 때만 정의된다(`a == 0`이면 `None`). `b`나 `c`가 0이면 `d = 0`이며
/// 이는 정상적인 결과다.
pub fn proportion_fourth(a: Rational, b: Rational, c: Rational) -> Option<Rational> {
    if a.is_zero() {
        return None;
    }
    Some(c * b / a)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::approx_eq;

    #[test]
    fn linear_equation_solves() {
        assert!(approx_eq(solve_linear(2.0, 6.0).unwrap(), -3.0, 1e-12));
        assert!(solve_linear(0.0, 5.0).is_none());
    }

    #[test]
    fn quadratic_two_real_roots_sorted() {
        match solve_quadratic(1.0, -3.0, 2.0) {
            Roots::TwoReal(a, b) => {
                assert!(approx_eq(a, 1.0, 1e-12));
                assert!(approx_eq(b, 2.0, 1e-12));
            }
            other => panic!("실근 두 개를 기대했습니다: {other:?}"),
        }
    }

    #[test]
    fn quadratic_repeated_root() {
        match solve_quadratic(2.0, 4.0, 2.0) {
            Roots::Repeated(r) => assert!(approx_eq(r, -1.0, 1e-12)),
            other => panic!("중근을 기대했습니다: {other:?}"),
        }
    }

    #[test]
    fn quadratic_complex_roots() {
        match solve_quadratic(1.0, 0.0, 1.0) {
            Roots::Complex { re, im } => {
                assert!(approx_eq(re, 0.0, 1e-12));
                assert!(approx_eq(im, 1.0, 1e-12));
            }
            other => panic!("복소근을 기대했습니다: {other:?}"),
        }
        match solve_quadratic(1.0, -2.0, 5.0) {
            Roots::Complex { re, im } => {
                assert!(approx_eq(re, 1.0, 1e-12));
                assert!(approx_eq(im, 2.0, 1e-12));
            }
            other => panic!("복소근을 기대했습니다: {other:?}"),
        }
    }

    #[test]
    fn two_by_two_system_solves() {
        let a = vec![vec![2.0, 3.0], vec![1.0, -1.0]];
        let x = solve_linear_system(&a, &[7.0, 1.0]).unwrap();
        assert!(approx_eq(x[0], 2.0, 1e-12));
        assert!(approx_eq(x[1], 1.0, 1e-12));
    }

    #[test]
    fn three_by_three_system_solves() {
        let a = vec![
            vec![1.0, 1.0, 1.0],
            vec![0.0, 2.0, 5.0],
            vec![2.0, 5.0, -1.0],
        ];
        let x = solve_linear_system(&a, &[6.0, -4.0, 27.0]).unwrap();
        assert!(approx_eq(x[0], 5.0, 1e-10));
        assert!(approx_eq(x[1], 3.0, 1e-10));
        assert!(approx_eq(x[2], -2.0, 1e-10));
    }

    #[test]
    fn singular_system_returns_none() {
        let a = vec![vec![1.0, 2.0], vec![2.0, 4.0]];
        assert!(solve_linear_system(&a, &[3.0, 6.0]).is_none());
    }

    #[test]
    fn distances_and_pythagoras() {
        assert!(approx_eq(distance_2d(0.0, 0.0, 3.0, 4.0), 5.0, 1e-12));
        assert!(approx_eq(distance_3d([1.0, 2.0, 2.0], [0.0, 0.0, 0.0]), 3.0, 1e-12));
        assert!(approx_eq(hypotenuse(5.0, 12.0), 13.0, 1e-12));
    }

    #[test]
    fn combinations_and_permutations_exact() {
        assert_eq!(combination(5, 2), Some(10));
        assert_eq!(combination(10, 0), Some(1));
        assert_eq!(combination(3, 5), Some(0));
        assert_eq!(combination(30, 15), Some(155_117_520));
        assert_eq!(permutation(5, 2), Some(20));
    }

    #[test]
    fn proportion_fourth_term() {
        let d = proportion_fourth(
            Rational::from_int(2),
            Rational::from_int(3),
            Rational::from_int(8),
        )
        .unwrap();
        assert!(d.equals(&Rational::from_int(12)));
    }

    #[test]
    fn proportion_fourth_allows_zero_b_or_c() {
        // 2 : 0 = 8 : d  →  d = 0 (b = 0은 정상 입력)
        let d = proportion_fourth(
            Rational::from_int(2),
            Rational::zero(),
            Rational::from_int(8),
        )
        .unwrap();
        assert!(d.is_zero());
        // a = 0 이면 미정
        assert!(proportion_fourth(
            Rational::zero(),
            Rational::from_int(3),
            Rational::from_int(8)
        )
        .is_none());
    }

    #[test]
    fn combination_returns_values_that_fit_u128() {
        // C(128, 64)은 u128에 들어가지만, 나누기 전 중간 곱은 넘친다.
        assert_eq!(
            combination(128, 64),
            Some(23_951_146_041_928_082_866_135_587_776_380_551_750)
        );
        assert_eq!(combination(34, 2), Some(561));
        assert_eq!(combination(200, 1), Some(200));
        // 진짜로 범위를 넘으면 None
        assert_eq!(combination(200, 100), None);
    }

    #[test]
    fn quadratic_with_widely_separated_roots_keeps_small_root() {
        // x^2 + 10^8 x + 1 = 0 의 작은 근(절댓값 기준)은 약 -1e-8
        match solve_quadratic(1.0, 1e8, 1.0) {
            Roots::TwoReal(lo, hi) => {
                assert!(approx_eq(lo, -1e8, 1e-4), "lo = {lo:e}");
                assert!(approx_eq(hi, -1e-8, 1e-16), "hi = {hi:e}");
                let residual = hi * hi + 1e8 * hi + 1.0;
                assert!(residual.abs() < 1e-12, "잔차 = {residual:e}");
            }
            other => panic!("실근 두 개를 기대했습니다: {other:?}"),
        }
        // c = 0 인 경우(-0.0이 생기지 않아야 한다)
        match solve_quadratic(1.0, 2.0, 0.0) {
            Roots::TwoReal(lo, hi) => {
                assert_eq!(lo, -2.0);
                assert_eq!(hi, 0.0);
                assert!(!hi.is_sign_negative(), "hi = {hi}");
            }
            other => panic!("실근 두 개를 기대했습니다: {other:?}"),
        }
    }

    #[test]
    fn gauss_solves_scaled_down_system() {
        // 단위 변환으로 계수가 작아진 가역계도 풀려야 한다(절대 임계값 오판 방지).
        let s = 1e-15;
        let a = vec![vec![2.0 * s, 0.0], vec![0.0, 4.0 * s]];
        let x = solve_linear_system(&a, &[2.0 * s, 8.0 * s]).unwrap();
        assert!(approx_eq(x[0], 1.0, 1e-12), "x = {x:?}");
        assert!(approx_eq(x[1], 2.0, 1e-12), "x = {x:?}");
        // 영행렬은 여전히 특이행렬이다.
        assert!(solve_linear_system(&vec![vec![0.0, 0.0], vec![0.0, 0.0]], &[1.0, 1.0]).is_none());
    }
}
