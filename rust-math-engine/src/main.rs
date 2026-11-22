//! 5단계(초·중·고·대·대학원) 대표 문제를 실제로 계산해 출력하는 데모.

use rust_math_engine::{elementary, grad, high, middle, undergrad};

fn stage(title: &str, items: Vec<(&str, String)>) {
    println!("\n{title}");
    println!("{}", "-".repeat(title.chars().count().max(40)));
    for (label, value) in items {
        println!("  {label:<34} {value}");
    }
}

fn main() {
    println!("rust-math-engine v{}", env!("CARGO_PKG_VERSION"));
    println!("순수 Rust(std만) 5단계 수학 엔진 — 실제 계산 결과");

    // ── 1. 초등 ─────────────────────────────────────────────
    let half = elementary::Rational::new(1, 2);
    let third = elementary::Rational::new(1, 3);
    let ratio = elementary::simplify_ratio(12, 18);
    let pct = elementary::percent(
        elementary::Rational::from_int(45),
        elementary::Rational::from_int(60),
    );
    stage(
        "[1] 초등 과정",
        vec![
            ("1/2 + 1/3", format!("{}", half + third)),
            ("7/4 x (-2/3)", format!("{}", elementary::Rational::new(7, 4) * elementary::Rational::new(-2, 3))),
            ("(2/3)^3", format!("{}", elementary::Rational::new(2, 3).pow(3))),
            ("gcd(48, 180)", format!("{}", elementary::gcd(48, 180))),
            ("lcm(21, 6)", format!("{}", elementary::lcm(21, 6))),
            ("360 소인수분해", elementary::factorization_string(360)),
            ("12 : 18 기약", format!("{} : {}", ratio.0, ratio.1)),
            ("45 / 60 백분율", format!("{pct}%")),
        ],
    );

    // ── 2. 중등 ─────────────────────────────────────────────
    let a = vec![vec![2.0, 3.0], vec![1.0, -1.0]];
    let sol = middle::solve_linear_system(&a, &[7.0, 1.0]).unwrap();
    let roots = middle::solve_quadratic(1.0, -3.0, 2.0);
    let cplx = middle::solve_quadratic(1.0, 0.0, 1.0);
    stage(
        "[2] 중등 과정",
        vec![
            ("2x + 6 = 0", format!("x = {}", middle::solve_linear(2.0, 6.0).unwrap())),
            ("x^2 - 3x + 2 = 0", format!("{roots:?}")),
            ("x^2 + 1 = 0", format!("{cplx:?}")),
            ("2x+3y=7, x-y=1", format!("x = {:.4}, y = {:.4}", sol[0], sol[1])),
            ("(0,0)-(3,4) 거리", format!("{}", middle::distance_2d(0.0, 0.0, 3.0, 4.0))),
            ("5, 12 직각삼각형 빗변", format!("{}", middle::hypotenuse(5.0, 12.0))),
            ("C(30, 15)", format!("{}", middle::combination(30, 15).unwrap())),
            ("P(10, 3)", format!("{}", middle::permutation(10, 3).unwrap())),
        ],
    );

    // ── 3. 고등 ─────────────────────────────────────────────
    let z = high::Complex::new(1.0, 2.0);
    let w = high::Complex::new(3.0, -4.0);
    let poly = high::Polynomial::new(vec![-5.0, 2.0, -3.0, 0.0, 1.0]); // x^4 - 3x^2 + 2x - 5
    let deriv = poly.derivative();
    let integ = high::Polynomial::new(vec![0.0, 0.0, 3.0]).integrate(); // ∫3x^2 dx
    stage(
        "[3] 고등 과정",
        vec![
            ("(1+2i)(3-4i)", format!("{}", z * w)),
            ("(1+2i)/(3-4i)", format!("{}", z / w)),
            ("|3+4i|", format!("{}", high::Complex::new(3.0, 4.0).abs())),
            ("p(x) = x^4-3x^2+2x-5", format!("{poly}")),
            ("p'(x)", format!("{deriv}")),
            ("p'(2) (호너 평가)", format!("{}", deriv.eval(2.0))),
            ("∫3x^2 dx", format!("{integ}")),
            ("∫₀³ 3x^2 dx", format!("{}", high::Polynomial::new(vec![0.0, 0.0, 3.0]).integrate_definite(0.0, 3.0))),
            ("등차 a1=3,d=4, S10", format!("{}", high::arithmetic_sum(3.0, 4.0, 10))),
            ("등비 a1=1,r=2, S10", format!("{}", high::geometric_sum(1.0, 2.0, 10))),
            ("f(x)=x^3, f'(2) 중앙차분", format!("{:.9}", high::numerical_derivative(|x| x * x * x, 2.0, 1e-6))),
            ("∫₀^π sin x dx (심프슨)", format!("{:.9}", high::simpson(|x: f64| x.sin(), 0.0, std::f64::consts::PI, 200))),
            ("(1,2,3)·(4,-5,6)", format!("{}", high::Vec3::new(1.0, 2.0, 3.0).dot(&high::Vec3::new(4.0, -5.0, 6.0)))),
            ("(1,0,0)×(0,1,0)", format!("{}", high::Vec3::new(1.0, 0.0, 0.0).cross(&high::Vec3::new(0.0, 1.0, 0.0)))),
        ],
    );

    // ── 4. 대학 ─────────────────────────────────────────────
    let m3 = undergrad::Matrix::from_rows(&[
        vec![1.0, 2.0, 3.0],
        vec![0.0, 1.0, 4.0],
        vec![5.0, 6.0, 0.0],
    ]);
    let inv2 = undergrad::Matrix::from_rows(&[vec![4.0, 7.0], vec![2.0, 6.0]])
        .inverse()
        .unwrap();
    let sym = undergrad::Matrix::from_rows(&[vec![2.0, 1.0], vec![1.0, 2.0]]);
    let (eigvals, _) = undergrad::jacobi_eigen(&sym, 1e-12, 200).unwrap();
    let (q, r) = undergrad::Matrix::from_rows(&[
        vec![12.0, -51.0],
        vec![6.0, 167.0],
        vec![-4.0, 24.0],
    ])
    .qr()
    .unwrap();
    let rk = undergrad::rk4(|_t, y| y, 0.0, 1.0, 1.0, 100);
    let (t_end, y_end) = *rk.last().unwrap();
    let (a0, b0, r2) = undergrad::least_squares_line(&[1.0, 2.0, 3.0, 4.0], &[5.0, 7.0, 9.0, 11.0]).unwrap();
    stage(
        "[4] 대학 과정",
        vec![
            ("det[[1,2,3],[0,1,4],[5,6,0]]", format!("{:.6}", m3.determinant())),
            ("inv[[4,7],[2,6]] 첫 행", format!("[{:.4}, {:.4}]", inv2.get(0, 0), inv2.get(0, 1))),
            ("LU(n=3) 재구성 오차", {
                let (l, u, perm) = m3.lu().unwrap();
                let lu = l.mul(&u);
                let mut err = 0.0f64;
                for i in 0..3 {
                    for j in 0..3 {
                        err = err.max((lu.get(i, j) - m3.get(perm[i], j)).abs());
                    }
                }
                format!("{err:.3e}")
            }),
            ("QR: ||Q^T Q - I||∞", {
                let qtq = q.transpose().mul(&q);
                let mut err = 0.0f64;
                for i in 0..2 {
                    for j in 0..2 {
                        let e = if i == j { 1.0 } else { 0.0 };
                        err = err.max((qtq.get(i, j) - e).abs());
                    }
                }
                let _ = &r;
                format!("{err:.3e}")
            }),
            ("고유값 [[2,1],[1,2]]", format!("{:.6}, {:.6}", eigvals[0], eigvals[1])),
            ("야코비안 f=(x²y, 5x+sin y) @(2,3)", {
                let j = undergrad::jacobian(|v: &[f64]| vec![v[0] * v[0] * v[1], 5.0 * v[0] + v[1].sin()], &[2.0, 3.0], 1e-6);
                format!("[{:.4}, {:.4}; {:.4}, {:.4}]", j.get(0, 0), j.get(0, 1), j.get(1, 0), j.get(1, 1))
            }),
            ("뉴턴법 x²+y²=4, x=y", {
                let s = undergrad::newton_system(|v: &[f64]| vec![v[0] * v[0] + v[1] * v[1] - 4.0, v[0] - v[1]], &[1.0, 1.0], 1e-10, 50).unwrap();
                format!("({:.9}, {:.9})", s[0], s[1])
            }),
            ("RK4 y'=y, y(0)=1 → y(1)", format!("{y_end:.9} (t={t_end:.1})")),
            ("N(0,1) pdf(0), cdf(1.96)", format!("{:.9}, {:.9}", undergrad::normal_pdf(0.0, 0.0, 1.0), undergrad::normal_cdf(1.96, 0.0, 1.0))),
            ("회귀 y = a + bx", format!("a = {a0:.4}, b = {b0:.4}, R² = {r2:.6}")),
        ],
    );

    // ── 5. 대학원 ─────────────────────────────────────────────
    let quad = |x: &[f64]| (x[0] - 1.0).powi(2) + 2.0 * (x[1] + 3.0).powi(2);
    let gd = grad::gradient_descent(quad, &[5.0, 5.0], 10_000, 1e-10);
    let bf = grad::bfgs(quad, &[5.0, 5.0], 200, 1e-12);
    let rosen = |x: &[f64]| (1.0 - x[0]).powi(2) + 100.0 * (x[1] - x[0] * x[0]).powi(2);
    let rb = grad::bfgs(rosen, &[-1.2, 1.0], 5_000, 1e-14);
    let heat_n = 41usize;
    let heat_dx = 1.0 / (heat_n as f64 - 1.0);
    let heat_dt = 0.4 * heat_dx * heat_dx;
    let heat_steps = 200usize;
    let heat_init: Vec<f64> = (0..heat_n)
        .map(|i| (std::f64::consts::PI * i as f64 * heat_dx).sin())
        .collect();
    let heat_u = grad::heat_1d(&heat_init, 1.0, heat_dx, heat_dt, heat_steps).unwrap();
    let heat_analytic = (-std::f64::consts::PI.powi(2) * heat_dt * heat_steps as f64).exp();
    let pi2 = grad::markov_stationary(&[vec![0.9, 0.1], vec![0.2, 0.8]], 1e-12, 10_000).unwrap();
    let hilbert = undergrad::Matrix::from_rows(&[
        vec![1.0, 1.0 / 2.0, 1.0 / 3.0],
        vec![1.0 / 2.0, 1.0 / 3.0, 1.0 / 4.0],
        vec![1.0 / 3.0, 1.0 / 4.0, 1.0 / 5.0],
    ]);
    stage(
        "[5] 대학원 과정",
        vec![
            ("경사하강(armijo) 최소점", format!("({:.6}, {:.6}), f={:.3e}, {}회", gd.x[0], gd.x[1], gd.f, gd.iterations)),
            ("BFGS 최소점", format!("({:.6}, {:.6}), f={:.3e}, {}회", bf.x[0], bf.x[1], bf.f, bf.iterations)),
            ("BFGS 로젠브록 최소점", format!("({:.6}, {:.6}), f={:.3e}", rb.x[0], rb.x[1], rb.f)),
            ("열방정식 u(0.5,T) vs 해석해", format!("{:.9} vs {:.9}", heat_u[heat_n / 2], heat_analytic)),
            ("열방정식 CFL 위반 검출", {
                let e = grad::heat_1d(&[0.0, 1.0, 0.0], 1.0, 0.1, 0.01, 10).unwrap_err();
                if e.contains("CFL") { "차단됨".to_string() } else { e }
            }),
            ("마르코프 정상분포", format!("[{:.9}, {:.9}]", pi2[0], pi2[1])),
            ("Kahan [1, 1e-16, -1]", format!("{:e} (naive {:e})", grad::kahan_sum(&[1.0, 1e-16, -1.0]), [1.0, 1e-16, -1.0].iter().sum::<f64>())),
            ("log-sum-exp [1000, 1000]", format!("{:.9}", grad::log_sum_exp(&[1000.0, 1000.0]))),
            ("softmax [1000, 1000]", format!("{:?}", grad::softmax(&[1000.0, 1000.0]))),
            ("cond∞(Hilbert 3x3)", format!("{:.3}", grad::condition_number(&hilbert))),
        ],
    );

    println!("\n모든 값은 기호·정확 산술(유리수) 또는 실측 부동소수 연산 결과입니다.");
}
