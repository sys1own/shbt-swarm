//! crates/shbt-swarm-core/src/ecc.rs
//! Systematic Hamming(72,64) SECDED Codec (d_min = 4, R = 64/72 = 8/9).

pub const HAMMING_MIN_DIST: u8 = 4;
pub const HAMMING_DATA_BITS: u8 = 64;
pub const HAMMING_CODE_BITS: u8 = 72;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum EccStatus {
    Clean,
    SingleBitCorrected { flipped_bit: u8 },
    DoubleBitDetected,
}

#[inline]
const fn data_bit_col_mask(bit_idx: u8) -> u8 {
    let mut col: u8 = 3;
    let mut k: u8 = 0;
    while k < bit_idx {
        col += 1;
        if (col & (col - 1)) != 0 {
            k += 1;
        }
    }
    col
}

/// Computes the 8-bit systematic Hamming(72,64) SECDED parity byte for a 64-bit Artin B_3 word.
#[inline]
pub const fn encode_hamming72_64(word: u64) -> u8 {
    let mut syn: u8 = 0;
    let mut i: u8 = 0;
    while i < 64 {
        if ((word >> i) & 1) != 0 {
            syn ^= data_bit_col_mask(i);
        }
        i += 1;
    }
    let total_ones = (word.count_ones() + (syn.count_ones())) as u8;
    let p7 = (total_ones & 1) << 7;
    syn | p7
}

/// Decodes and scrubs a 64-bit Artin B_3 word and its 8-bit SECDED parity byte.
#[inline]
pub const fn decode_hamming72_64(word: u64, parity: u8) -> (u64, u8, EccStatus) {
    let expected_p = encode_hamming72_64(word);
    let syn7 = (parity ^ expected_p) & 0x7F;
    let overall_err = ((word.count_ones() + parity.count_ones()) & 1) as u8;

    if syn7 == 0 && overall_err == 0 {
        return (word, parity, EccStatus::Clean);
    }

    if overall_err == 1 {
        if syn7 == 0 || (syn7 & (syn7 - 1)) == 0 {
            let fixed_p = parity ^ (if syn7 == 0 { 0x80 } else { syn7 });
            return (word, fixed_p, EccStatus::SingleBitCorrected { flipped_bit: 64 });
        }
        let mut i: u8 = 0;
        while i < 64 {
            if data_bit_col_mask(i) == syn7 {
                let fixed_w = word ^ (1u64 << i);
                return (fixed_w, parity, EccStatus::SingleBitCorrected { flipped_bit: i });
            }
            i += 1;
        }
    }

    (word, parity, EccStatus::DoubleBitDetected)
}
