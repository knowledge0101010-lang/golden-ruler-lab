//! 중등 과정 — 일차·이차방정식, 연립일차방정식(가우스 소거), 거리·피타고라스, 조합·순열.

use crate::elementary::Rational;

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
pub fn solve_quadratic(a: f64, b: f64, c: f64) -> Roots {
    assert!(a != 0.0, "이차방정식의 최고차 계수가 0입니다");
    let disc = b * b - 4.0 * a * c;
    if disc > 0.0 {
        let s = disc.sqrt();
        let (r1, r2) = ((-b - s) / (2.0 * a), (-b + s) / (2.0 * a));
        if r1 <= r2 {
            Roots::TwoReal(r1, r2)
        } else {
            Roots::TwoReal(r2, r1)
        }
    } else if disc == 0.0 {
        Roots::Repeated(-b / (2.0 * a))
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

    for col in 0..n {
        // 부분 피벗 선택
        let mut pivot = col;
        for r in col + 1..n {
            if m[r][col].abs() > m[pivot][col].abs() {
                pivot = r;
            }
        }
        if m[pivot][col].abs() < 1e-12 {
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
        result = result.checked_mul(n - i)?;
        result /= i + 1;
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

/// 중등 교과의 비례식 `a : b = c : d`에서 미지의 항 `d`를 구한다(`b`, `c`가 0이 아닐 때).
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
}
