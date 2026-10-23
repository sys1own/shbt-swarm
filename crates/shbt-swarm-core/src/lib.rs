//! crates/shbt-swarm-core/src/lib.rs
//! #![no_std] Core Holographic Stinespring Frame, SECDED Hamming(72,64),
//! Burau-Cayley Digest, and WZW (26, 8, 312) Algebraic Primitives for `shbt-swarm`.

#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]

pub mod cayley;
pub mod ecc;
pub mod hyperframe;
pub mod wzw_mpfr;

pub use cayley::{compute_braid_word_cayley_matrix, compute_frame_cayley_digest, BurauMat2x2};
pub use ecc::{decode_hamming72_64, encode_hamming72_64, EccStatus, HAMMING_MIN_DIST};
pub use hyperframe::*;
pub use wzw_mpfr::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wzw_coset_central_charges_and_f123() {
        assert_eq!(C_SO10_312, Rational::new(351, 8));
        assert_eq!(C_SU2_26, Rational::new(39, 14));
        assert_eq!(C_SU3_8, Rational::new(64, 11));
        assert_eq!(C_GKO_COSET, Rational::new(21727, 616));
        assert_eq!(fibonacci_u128(123), 22_698_374_052_006_863_956_975_682u128);
    }

    #[test]
    fn test_hamming72_64_secded_all_bits() {
        let word: u64 = 0xCAFE_BABE_DEAD_BEEF;
        let parity = encode_hamming72_64(word);
        let (w0, p0, st0) = decode_hamming72_64(word, parity);
        assert_eq!(w0, word);
        assert_eq!(p0, parity);
        assert_eq!(st0, EccStatus::Clean);

        for bit in 0..64u8 {
            let flipped = word ^ (1u64 << bit);
            let (w_corr, _, st) = decode_hamming72_64(flipped, parity);
            assert_eq!(w_corr, word);
            assert_eq!(st, EccStatus::SingleBitCorrected { flipped_bit: bit });
        }

        let double_flip = word ^ (1u64 << 3) ^ (1u64 << 55);
        let (_, _, st_double) = decode_hamming72_64(double_flip, parity);
        assert_eq!(st_double, EccStatus::DoubleBitDetected);
    }

    #[test]
    fn test_cayley_digest_non_abelian() {
        let mut braids_a = [0x1122_3344_5566_7788u64; 124];
        let mut braids_b = braids_a;
        braids_b[0] ^= 0x03; // alter first Artin B_3 generator
        let ha = compute_frame_cayley_digest(&braids_a);
        let hb = compute_frame_cayley_digest(&braids_b);
        assert_ne!(ha, hb);
        braids_a[0] ^= 0x03;
        assert_eq!(compute_frame_cayley_digest(&braids_a), hb);
    }
}
