/*
 * kernel/include/shbt_ecc.h
 * Systematic Hamming(72,64) SECDED Codec & 50.518 kHz Microkernel Tick Interface
 */

#ifndef SHBT_ECC_H
#define SHBT_ECC_H

#include <stdint.h>
#include <stddef.h>
#include "shbt_swarm_mmio.h"

#define SHBT_HAMMING_MIN_DIST   4U
#define SHBT_HAMMING_DATA_BITS  64U
#define SHBT_HAMMING_CODE_BITS  72U

typedef enum {
    SHBT_ECC_OK              = 0,
    SHBT_ECC_SINGLE_CORRECTED = 1,
    SHBT_ECC_DOUBLE_DETECTED = 2
} shbt_ecc_status_t;

/* Compute 8-bit systematic SECDED Hamming(72,64) parity byte for a 64-bit Artin B_3 word */
uint8_t shbt_ecc_encode_hamming72_64(uint64_t word);

/* Verify and correct a 64-bit word using its 8-bit SECDED parity byte */
shbt_ecc_status_t shbt_ecc_decode_hamming72_64(uint64_t *word, uint8_t *parity);

/* Freestanding 50.518 kHz (19.795 us) microkernel tick handler enforcing hardware fault interlocks */
void shbt_swarm_kernel_tick(shbt_swarm_mmio_t *mmio, shbt_unified_stinespring_hyperframe_t *hyperframe);

#endif /* SHBT_ECC_H */
