//! 초등 과정 — 정확한 유리수, 최대공약수·최소공배수, 소인수분해, 비율·백분율.
//!
//! 모든 유리수 연산은 부동소수점을 거치지 않고 `i128` 정수쌍으로 정확하게 계산한다.

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// `i128` 두 수의 최대공약수(절댓값 기준).
fn gcd_i128(mut a: i128, mut b: i128) -> i128 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    if a == 0 {
        0
    } else {
        a
    }
}

/// 음이 아닌 정수 `a`, `b`의 최대공약수. `gcd(0, 0) == 0`.
pub fn gcd(a: u64, b: u64) -> u64 {
    gcd_i128(a as i128, b as i128) as u64
}

/// 음이 아닌 정수 `a`, `b`의 최소공배수. 둘 중 하나가 0이면 0.
pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }
    a / gcd(a, b) * b
}

/// 기약분수 형태의 정확한 유리수. `den > 0`을 항상 유지한다.
#[derive(Clone, Copy, Debug)]
pub struct Rational {
    num: i128,
    den: i128,
}

impl Rational {
    /// 분자·분모로 유리수를 만든다. 기약분수로 약분하고 부호를 분자로 옮긴다.
    ///
    /// # Panics
    /// `den == 0`이면 패닉한다(0으로 나누기는 정의되지 않음).
    pub fn new(num: i128, den: i128) -> Rational {
        assert!(den != 0, "0으로 나눌 수 없습니다: {num}/0");
        let g = gcd_i128(num, den);
        let (mut n, mut d) = if g == 0 {
            (num, den)
        } else {
            (num / g, den / g)
        };
        if d < 0 {
            n = -n;
            d = -d;
        }
        Rational { num: n, den: d }
    }

    /// 정수를 유리수로 승격한다.
    pub fn from_int(n: i128) -> Rational {
        Rational { num: n, den: 1 }
    }

    pub fn num(&self) -> i128 {
        self.num
    }

    pub fn den(&self) -> i128 {
        self.den
    }

    pub fn is_integer(&self) -> bool {
        self.den == 1
    }

    pub fn is_zero(&self) -> bool {
        self.num == 0
    }

    pub fn abs(&self) -> Rational {
        Rational {
            num: self.num.abs(),
            den: self.den,
        }
    }

    pub fn recip(&self) -> Rational {
        Rational::new(self.den, self.num)
    }

    /// 정수 거듭제곱. 음수 지수는 역수를 취한다(0의 음수 거듭제곱은 패닉).
    pub fn pow(&self, e: i32) -> Rational {
        if e >= 0 {
            Rational::new(
                self.num.checked_pow(e as u32).expect("분자 거듭제곱 오버플로"),
                self.den.checked_pow(e as u32).expect("분모 거듭제곱 오버플로"),
            )
        } else {
            self.recip().pow(-e)
        }
    }

    pub fn to_f64(&self) -> f64 {
        self.num as f64 / self.den as f64
    }

    /// 유리수 값이 정확히 같은지(기약분수이므로 필드 비교로 충분).
    pub fn equals(&self, other: &Rational) -> bool {
        self.num == other.num && self.den == other.den
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.den == 1 {
            write!(f, "{}", self.num)
        } else {
            write!(f, "{}/{}", self.num, self.den)
        }
    }
}

impl PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.den == other.den
    }
}

impl Eq for Rational {}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rational {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.num * other.den).cmp(&(other.num * self.den))
    }
}

impl Add for Rational {
    type Output = Rational;
    fn add(self, rhs: Rational) -> Rational {
        Rational::new(
            self.num * rhs.den + rhs.num * self.den,
            self.den * rhs.den,
        )
    }
}

impl Sub for Rational {
    type Output = Rational;
    fn sub(self, rhs: Rational) -> Rational {
        Rational::new(
            self.num * rhs.den - rhs.num * self.den,
            self.den * rhs.den,
        )
    }
}

impl Mul for Rational {
    type Output = Rational;
    fn mul(self, rhs: Rational) -> Rational {
        Rational::new(self.num * rhs.num, self.den * rhs.den)
    }
}

impl Div for Rational {
    type Output = Rational;
    fn div(self, rhs: Rational) -> Rational {
        assert!(!rhs.is_zero(), "유리수를 0으로 나눌 수 없습니다");
        Rational::new(self.num * rhs.den, self.den * rhs.num)
    }
}

impl Neg for Rational {
    type Output = Rational;
    fn neg(self) -> Rational {
        Rational {
            num: -self.num,
            den: self.den,
        }
    }
}

/// 자연수 `n`의 소인수분해 `[(소수, 지수)]`를 오름차순으로 돌려준다. `1`은 빈 벡터.
pub fn prime_factorization(mut n: u64) -> Vec<(u64, u32)> {
    let mut out = Vec::new();
    let mut p = 2u64;
    while p * p <= n {
        if n % p == 0 {
            let mut e = 0u32;
            while n % p == 0 {
                n /= p;
                e += 1;
            }
            out.push((p, e));
        }
        p += if p == 2 { 1 } else { 2 };
    }
    if n > 1 {
        out.push((n, 1));
    }
    out
}

/// 소인수분해 결과를 사람이 읽는 문자열로 만든다: `2^3 * 3^2 * 5`.
pub fn factorization_string(n: u64) -> String {
    let f = prime_factorization(n);
    if f.is_empty() {
        return String::from("1");
    }
    f.iter()
        .map(|(p, e)| if *e == 1 { format!("{p}") } else { format!("{p}^{e}") })
        .collect::<Vec<_>>()
        .join(" * ")
}

/// 정수 비율 `a : b`를 기약 정수쌍으로 만든다. 부호는 `a` 쪽에 모은다.
pub fn simplify_ratio(a: i128, b: i128) -> (i128, i128) {
    assert!(b != 0, "비율의 두 번째 항이 0입니다: {a}:0");
    let g = gcd_i128(a, b);
    if g == 0 {
        return (a, b);
    }
    let (mut x, mut y) = (a / g, b / g);
    if y < 0 {
        x = -x;
        y = -y;
    }
    (x, y)
}

/// `part`가 `whole`의 몇 퍼센트인지 정확한 유리수로 계산한다. `whole == 0`이면 패닉.
pub fn percent(part: Rational, whole: Rational) -> Rational {
    assert!(!whole.is_zero(), "전체가 0이면 백분율을 정의할 수 없습니다");
    part / whole * Rational::from_int(100)
}

/// `whole`의 `pct`퍼센트를 정확한 유리수로 계산한다.
pub fn apply_percent(whole: Rational, pct: Rational) -> Rational {
    whole * pct / Rational::from_int(100)
}

/// 분수 표기를 소수 전개로 바꾼다(순환 여부도 알려준다).
pub fn is_terminating_decimal(r: Rational) -> bool {
    let mut d = r.den();
    while d % 2 == 0 {
        d /= 2;
    }
    while d % 5 == 0 {
        d /= 5;
    }
    d == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rational_adds_and_reduces_to_lowest_terms() {
        let s = Rational::new(1, 2) + Rational::new(1, 3);
        assert!(s.equals(&Rational::new(5, 6)), "1/2 + 1/3 = {s}");
    }

    #[test]
    fn rational_normalizes_sign_and_gcd() {
        let r = Rational::new(3, -6);
        assert_eq!(r.num(), -1);
        assert_eq!(r.den(), 2);
        assert_eq!(r.to_string(), "-1/2");
    }

    #[test]
    fn rational_four_operations_are_exact() {
        let a = Rational::new(7, 4);
        let b = Rational::new(-2, 3);
        assert!(a.equals(&Rational::new(7, 4)));
        assert!((a - b).equals(&Rational::new(29, 12)));
        assert!((a * b).equals(&Rational::new(-7, 6)));
        assert!((a / b).equals(&Rational::new(-21, 8)));
        assert!((b.neg()).equals(&Rational::new(2, 3)));
    }

    #[test]
    fn rational_ordering_is_exact() {
        assert!(Rational::new(1, 3) < Rational::new(1, 2));
        assert!(Rational::new(-1, 2) < Rational::new(1, 100));
        assert_eq!(Rational::new(3, 4).cmp(&Rational::new(6, 8)), Ordering::Equal);
    }

    #[test]
    fn rational_power_and_recip() {
        assert!((Rational::new(2, 3).pow(3)).equals(&Rational::new(8, 27)));
        assert!((Rational::new(2, 3).pow(-2)).equals(&Rational::new(9, 4)));
        assert!((Rational::new(5, 7).recip()).equals(&Rational::new(7, 5)));
    }

    #[test]
    fn gcd_and_lcm_of_elementary_numbers() {
        assert_eq!(gcd(48, 180), 12);
        assert_eq!(gcd(0, 7), 7);
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(21, 6), 42);
        assert_eq!(lcm(0, 5), 0);
    }

    #[test]
    fn prime_factorization_of_360() {
        assert_eq!(prime_factorization(360), vec![(2, 3), (3, 2), (5, 1)]);
        assert_eq!(factorization_string(360), "2^3 * 3^2 * 5");
        assert!(prime_factorization(1).is_empty());
        assert_eq!(prime_factorization(97), vec![(97, 1)]);
    }

    #[test]
    fn ratio_simplification() {
        assert_eq!(simplify_ratio(12, 18), (2, 3));
        assert_eq!(simplify_ratio(-9, 6), (-3, 2));
        assert_eq!(simplify_ratio(5, -10), (-1, 2));
    }

    #[test]
    fn percent_of_45_out_of_60_is_75() {
        let p = percent(Rational::from_int(45), Rational::from_int(60));
        assert!(p.equals(&Rational::from_int(75)));
        let v = apply_percent(Rational::from_int(240), Rational::new(15, 2));
        assert!(v.equals(&Rational::from_int(18)));
    }

    #[test]
    fn terminating_decimal_detection() {
        assert!(is_terminating_decimal(Rational::new(3, 8)));
        assert!(!is_terminating_decimal(Rational::new(1, 3)));
    }
}
