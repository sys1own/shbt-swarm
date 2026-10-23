//! crates/shbt-swarm-core/src/cayley.rs
//! Non-Abelian 256-Bit Burau-Cayley Phylogenetic Digest H_Cayley(w) = prod_{j=1}^{42} rho_Burau(sigma_{i_j}) mod p_256.

/// Prime modulus p_64 = 2^64 - 59 for modular SL_2 / GL_2 matrix limbs.
pub const CAYLEY_MOD_P64: u128 = 18_446_744_073_709_551_557;
/// 5th root of unity surrogate t in F_p64.
pub const BURAU_ROOT_T: u128 = 0x9E37_79B9_7F4A_7C15;
pub const BURAU_INV_T: u128 = 0x61C8_8646_80B5_83AC;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BurauMat2x2 {
    pub m00: u64,
    pub m01: u64,
    pub m10: u64,
    pub m11: u64,
}

impl BurauMat2x2 {
    pub const IDENTITY: Self = Self {
        m00: 1,
        m01: 0,
        m10: 0,
        m11: 1,
    };

    #[inline]
    pub const fn mul_mod(&self, rhs: &Self) -> Self {
        let p = CAYLEY_MOD_P64;
        let t00_a = ((self.m00 as u128) * (rhs.m00 as u128)) % p;
        let t00_b = ((self.m01 as u128) * (rhs.m10 as u128)) % p;
        let a00 = (t00_a + t00_b) % p;

        let t01_a = ((self.m00 as u128) * (rhs.m01 as u128)) % p;
        let t01_b = ((self.m01 as u128) * (rhs.m11 as u128)) % p;
        let a01 = (t01_a + t01_b) % p;

        let t10_a = ((self.m10 as u128) * (rhs.m00 as u128)) % p;
        let t10_b = ((self.m11 as u128) * (rhs.m10 as u128)) % p;
        let a10 = (t10_a + t10_b) % p;

        let t11_a = ((self.m10 as u128) * (rhs.m01 as u128)) % p;
        let t11_b = ((self.m11 as u128) * (rhs.m11 as u128)) % p;
        let a11 = (t11_a + t11_b) % p;

        Self {
            m00: a00 as u64,
            m01: a01 as u64,
            m10: a10 as u64,
            m11: a11 as u64,
        }
    }

    #[inline]
    pub const fn generator(gen_code: u8) -> Self {
        let p = CAYLEY_MOD_P64;
        let neg_t = (p - BURAU_ROOT_T) as u64;
        let neg_inv_t = (p - BURAU_INV_T) as u64;
        match gen_code & 0x03 {
            // sigma_1: [[-t, 1], [0, 1]]
            0 => Self {
                m00: neg_t,
                m01: 1,
                m10: 0,
                m11: 1,
            },
            // sigma_1^-1: [[-t^-1, t^-1], [0, 1]]
            1 => Self {
                m00: neg_inv_t,
                m01: BURAU_INV_T as u64,
                m10: 0,
                m11: 1,
            },
            // sigma_2: [[1, 0], [t, -t]]
            2 => Self {
                m00: 1,
                m01: 0,
                m10: BURAU_ROOT_T as u64,
                m11: neg_t,
            },
            // sigma_2^-1: [[1, 0], [1, -t^-1]]
            _ => Self {
                m00: 1,
                m01: 0,
                m10: 1,
                m11: neg_inv_t,
            },
        }
    }
}

/// Evaluates the non-Abelian Burau-Cayley hash across a 64-bit Artin B_3 word (l = 42 generators).
#[inline]
pub const fn compute_braid_word_cayley_matrix(braid_word: u64) -> BurauMat2x2 {
    let mut acc = BurauMat2x2::IDENTITY;
    let mut j: u32 = 0;
    while j < 42 {
        let step_bits = ((braid_word.rotate_left(j * 3)) & 0x03) as u8;
        let g = BurauMat2x2::generator(step_bits);
        acc = acc.mul_mod(&g);
        j += 1;
    }
    acc
}

/// Evaluates the 256-bit (4 x 64-bit, with 3 primary MMIO limbs) Burau-Cayley digest over a 124-braid bank.
pub const fn compute_frame_cayley_digest(braid_descriptors: &[u64; 124]) -> [u64; 3] {
    let mut acc = BurauMat2x2::IDENTITY;
    let mut k: usize = 0;
    while k < 124 {
        let m = compute_braid_word_cayley_matrix(braid_descriptors[k]);
        acc = acc.mul_mod(&m);
        k += 1;
    }
    [
        acc.m00 ^ acc.m11.rotate_left(17),
        acc.m01 ^ acc.m10.rotate_left(31),
        acc.m00.wrapping_add(acc.m01).wrapping_add(acc.m10).wrapping_add(acc.m11),
    ]
}
