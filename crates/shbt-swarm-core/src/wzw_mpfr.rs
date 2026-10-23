//! crates/shbt-swarm-core/src/wzw_mpfr.rs
//! Exact rational WZW (26, 8, 312) central charges, Fibonacci quantum dimensions,
//! and F_123 = 22,698,374,052,006,863,956,975,682 Hilbert space dimension.

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Rational {
    pub num: i128,
    pub den: i128,
}

impl Rational {
    #[inline]
    pub const fn new(num: i128, den: i128) -> Self {
        let g = Self::gcd(if num < 0 { -num } else { num }, if den < 0 { -den } else { den });
        let sign = if den < 0 { -1 } else { 1 };
        Self {
            num: sign * (num / g),
            den: sign * (den / g),
        }
    }

    #[inline]
    const fn gcd(mut a: i128, mut b: i128) -> i128 {
        while b != 0 {
            let r = a % b;
            a = b;
            b = r;
        }
        if a == 0 { 1 } else { a }
    }

    #[inline]
    pub const fn add(self, rhs: Self) -> Self {
        Self::new(self.num * rhs.den + rhs.num * self.den, self.den * rhs.den)
    }

    #[inline]
    pub const fn sub(self, rhs: Self) -> Self {
        Self::new(self.num * rhs.den - rhs.num * self.den, self.den * rhs.den)
    }
}

/// Sugawara central charge c(g_k) = (k * dim(g)) / (k + g^v).
#[inline]
pub const fn sugawara_central_charge(level_k: i128, dim_g: i128, dual_coxeter_gv: i128) -> Rational {
    Rational::new(level_k * dim_g, level_k + dual_coxeter_gv)
}

pub const C_SO10_312: Rational = sugawara_central_charge(312, 45, 8); // 351 / 8 = 43.875
pub const C_SU2_26: Rational   = sugawara_central_charge(26, 3, 2);   // 39 / 14
pub const C_SU3_8: Rational    = sugawara_central_charge(8, 8, 3);    // 64 / 11
pub const C_GKO_COSET: Rational = C_SO10_312.sub(C_SU2_26.add(C_SU3_8)); // 21727 / 616

const _: () = assert!(C_SO10_312.num == 351 && C_SO10_312.den == 8);
const _: () = assert!(C_SU2_26.num == 39 && C_SU2_26.den == 14);
const _: () = assert!(C_SU3_8.num == 64 && C_SU3_8.den == 11);
const _: () = assert!(C_GKO_COSET.num == 21727 && C_GKO_COSET.den == 616);

/// Golden ratio d_tau = phi = (1 + sqrt(5)) / 2 and total quantum dimension D_Fib = sqrt(2 + phi).
#[allow(clippy::approx_constant)]
pub const FIB_QUANTUM_DIM_PHI: f64 = 1.618_033_988_749_895;
pub const FIB_TOTAL_QUANTUM_DIM: f64 = 1.902_113_032_590_307;

/// Exact 124-channel SU(2)_3 Fibonacci fusion space dimension: dim H_124^(tau) = F_123.
pub const FIB_DIM_H124_F123: u128 = 22_698_374_052_006_863_956_975_682;

/// Computes the n-th Fibonacci number F_n using exact u128 integer arithmetic.
#[inline]
pub const fn fibonacci_u128(n: u32) -> u128 {
    if n == 0 {
        return 0;
    }
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    let mut i: u32 = 1;
    while i < n {
        let next = a + b;
        a = b;
        b = next;
        i += 1;
    }
    b
}

const _: () = assert!(fibonacci_u128(123) == FIB_DIM_H124_F123);
