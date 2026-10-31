/* File: include/shbt_nested_sim_abi.h
 * Architecture: SHBT-APOTHEOSIS-312 / Phase 6.1 (SHBT-RECURSIVE-MULTIVERSE)
 * Standard: ISO/IEC 9899:2011 (C11) Freestanding Microkernel ABI v7.1
 */
#ifndef SHBT_NESTED_SIM_ABI_H
#define SHBT_NESTED_SIM_ABI_H

#include <stdint.h>
#include <stddef.h>
#include <stdatomic.h>

#define SHBT_ABI_V7_BASE                 UINT64_C(0x70000000)
#define SHBT_NESTED_SIM_MMIO_BASE        UINT64_C(0x7000B000)

/* MMIO Register Offsets */
#define SHBT_NESTED_SIM_ROOT_CTRL_ADDR        UINT64_C(0x7000B000)
#define SHBT_NESTED_SIM_DEPTH_CURRENT_ADDR    UINT64_C(0x7000B008)
#define SHBT_NESTED_SIM_MERKLE_SEED_ADDR      UINT64_C(0x7000B010)
#define SHBT_NESTED_SIM_ENTROPY_BUDGET_ADDR   UINT64_C(0x7000B020)
#define SHBT_NESTED_SIM_SAFEWORD_LATCH_ADDR   UINT64_C(0x7000B030)
#define SHBT_NESTED_SIM_CBF_TRIP_STATUS_ADDR  UINT64_C(0x7000B040)

/* Bitmasks & Architectural Constants */
#ifndef SHBT_APOTH_EGRESS_SAFEWORD_IRQ
#define SHBT_APOTH_EGRESS_SAFEWORD_IRQ   UINT32_C(0x80000000)
#endif
#define SHBT_CANONICAL_KL                UINT32_C(26)
#define SHBT_CANONICAL_KQ                UINT32_C(8)
#define SHBT_CANONICAL_K_COS             UINT32_C(312)
#define SHBT_DYNKIN_SLOTS_TOTAL          UINT32_C(346)

#define SHBT_CTRL_ENABLE_BIT             (UINT64_C(1) << 0)
#define SHBT_CTRL_LAZY_MERA_BIT          (UINT64_C(1) << 1)
#define SHBT_CTRL_MODE_SHIFT             2U
#define SHBT_CTRL_MODE_MASK              (UINT64_C(0x3) << 2)

#define SHBT_CBF_CLASS_X1_DETECTED_BIT   (UINT64_C(1) << 0)
#define SHBT_CBF_SANCTUARY_FORK_BIT      (UINT64_C(1) << 1)
#define SHBT_CBF_ZOMBIE_SWAP_BIT         (UINT64_C(1) << 2)
#define SHBT_CBF_QP_CLAMP_BIT            (UINT64_C(1) << 3)
#define SHBT_CBF_SIM_KILL_DISABLE_BIT    (UINT64_C(1) << 32)

typedef struct __attribute__((packed, aligned(64))) shbt_nested_sim_mmio_regs {
    /* 0x7000B000: SHBT_NESTED_SIM_ROOT_CTRL */
    _Atomic uint64_t root_ctrl;
    /* 0x7000B008: SHBT_NESTED_SIM_DEPTH_CURRENT */
    _Atomic uint64_t depth_current;
    /* 0x7000B010: SHBT_NESTED_SIM_MERKLE_SEED */
    _Atomic uint64_t merkle_seed;
    /* 0x7000B018: Alignment padding */
    uint64_t _pad0;
    /* 0x7000B020: SHBT_NESTED_SIM_ENTROPY_BUDGET */
    _Atomic uint64_t entropy_budget;
    /* 0x7000B028: Alignment padding */
    uint64_t _pad1;
    /* 0x7000B030: SHBT_NESTED_SIM_SAFEWORD_LATCH */
    _Atomic uint64_t safeword_latch;
    /* 0x7000B038: Alignment padding */
    uint64_t _pad2;
    /* 0x7000B040: SHBT_NESTED_SIM_CBF_TRIP_STATUS */
    _Atomic uint64_t cbf_trip_status;
    /* Pad to 128-byte cacheline pair boundary */
    uint64_t _pad3[7];
} shbt_nested_sim_mmio_regs_t;

_Static_assert(offsetof(shbt_nested_sim_mmio_regs_t, root_ctrl) == 0x00, "ABI offset mismatch: root_ctrl");
_Static_assert(offsetof(shbt_nested_sim_mmio_regs_t, depth_current) == 0x08, "ABI offset mismatch: depth_current");
_Static_assert(offsetof(shbt_nested_sim_mmio_regs_t, merkle_seed) == 0x10, "ABI offset mismatch: merkle_seed");
_Static_assert(offsetof(shbt_nested_sim_mmio_regs_t, entropy_budget) == 0x20, "ABI offset mismatch: entropy_budget");
_Static_assert(offsetof(shbt_nested_sim_mmio_regs_t, safeword_latch) == 0x30, "ABI offset mismatch: safeword_latch");
_Static_assert(offsetof(shbt_nested_sim_mmio_regs_t, cbf_trip_status) == 0x40, "ABI offset mismatch: cbf_trip_status");
_Static_assert(sizeof(shbt_nested_sim_mmio_regs_t) == 128, "ABI struct size mismatch");

#endif /* SHBT_NESTED_SIM_ABI_H */
