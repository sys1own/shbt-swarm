/*
 * kernel/src/shbt_ecc.c
 * Freestanding C11 Systematic Hamming(72,64) SECDED Implementation (d_min = 4, R = 8/9)
 */

#include "../include/shbt_ecc.h"

static inline uint8_t popcount64(uint64_t x) {
    uint8_t count = 0U;
    while (x != 0ULL) {
        x &= (x - 1ULL);
        count++;
    }
    return count;
}

/*
 * Systematic Hsiao/Hamming(72,64) SECDED generator masks.
 * Each data bit i in [0..63] is mapped to a unique 7-bit non-zero column (index i + 1,
 * skipping powers of 2 so every data bit participates in >= 2 parity equations),
 * and bit 7 of the parity byte is the overall even parity across all 64 data + 7 Hamming bits.
 */
static inline uint8_t data_bit_col_mask(uint8_t bit_idx) {
    /* Map bit_idx (0..63) to 7-bit integer in 1..127 that is not a power of 2 */
    uint8_t col = 3U;
    uint8_t k = 0U;
    while (k < bit_idx) {
        col++;
        if ((col & (col - 1U)) != 0U) {
            k++;
        }
    }
    return col; /* 7-bit mask in [3..71] */
}

uint8_t shbt_ecc_encode_hamming72_64(uint64_t word) {
    uint8_t syn = 0U;
    for (uint8_t i = 0U; i < 64U; i++) {
        if (((word >> i) & 1ULL) != 0ULL) {
            syn ^= data_bit_col_mask(i);
        }
    }
    /* Overall parity in bit 7 across 64 data bits and 7 syndrome bits */
    uint8_t total_ones = (uint8_t)(popcount64(word) + popcount64((uint64_t)syn));
    uint8_t p7 = (uint8_t)((total_ones & 1U) << 7U);
    return (uint8_t)(syn | p7);
}

shbt_ecc_status_t shbt_ecc_decode_hamming72_64(uint64_t *word, uint8_t *parity) {
    uint64_t w = *word;
    uint8_t p = *parity;
    uint8_t expected_p = shbt_ecc_encode_hamming72_64(w);
    uint8_t syn7 = (uint8_t)((p ^ expected_p) & 0x7FU);
    uint8_t overall_err = (uint8_t)((popcount64(w) + popcount64((uint64_t)p)) & 1U);

    if (syn7 == 0U && overall_err == 0U) {
        return SHBT_ECC_OK;
    }

    if (overall_err == 1U) {
        /* Single-bit error: either in parity byte (if syn7 is power of 2 or 0) or in data word */
        if (syn7 == 0U || (syn7 & (syn7 - 1U)) == 0U) {
            *parity = (uint8_t)(p ^ (syn7 == 0U ? 0x80U : syn7));
            return SHBT_ECC_SINGLE_CORRECTED;
        }
        for (uint8_t i = 0U; i < 64U; i++) {
            if (data_bit_col_mask(i) == syn7) {
                *word = w ^ (1ULL << i);
                return SHBT_ECC_SINGLE_CORRECTED;
            }
        }
    }

    /* Double-bit (or even-weight) error detected */
    return SHBT_ECC_DOUBLE_DETECTED;
}
