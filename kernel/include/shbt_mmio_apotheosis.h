/**
 * @file shbt_mmio_apotheosis.h
 * @brief Bare-Metal C11 Microkernel ABI v6 for Phase 6 (SHBT-APOTHEOSIS-312)
 *     Aperture: 0x4000C000 (Apotheosis Control, Sandbox CBF, & Avatar Synthesis)
 *     DMA Window: 0x70020000 (UnifiedStinespringHyperFrame Zero-Latency Bridge)
 */

#ifndef SHBT_MMIO_APOTHEOSIS_H
#define SHBT_MMIO_APOTHEOSIS_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/* =========================================================================
 * Physical Memory-Mapped IO Base Addresses
 * ========================================================================= */
#define SHBT_APOTH_MMIO_BASE             ((uintptr_t)0x4000C000UL)
#define SHBT_OMNIBUS_HYPERFRAME_DMA_BASE ((uintptr_t)0x70020000UL)

/* =========================================================================
 * Apotheosis Extended MMIO Registers (Aperture Base: 0x4000C000)
 * ========================================================================= */
#define SHBT_APOTH_REG_QND_EGO_COMMUTATOR   0x0000C030U /* Measured ||[H_ascend, O_ego]|| */
#define SHBT_APOTH_REG_LIND_PURIFY_RATE     0x0000C034U /* Stinespring ancilla extraction rate */
#define SHBT_APOTH_REG_HORIZON_DEFLECT_STAT 0x0000C038U /* Causal decoupling status & metrics */
#define SHBT_APOTH_REG_MULTI_DEITY_COMMUT   0x0000C03CU /* Multi-tenant intent commutator norm */

/* Phase 6 Extended MMIO Registers (Aperture Base: 0x4000C060, 32 Bytes) */
#define SHBT_APOTH_REG_QUALIA_BETTI_SIGNATURE 0x0000C060U
#define SHBT_APOTH_REG_POSET_SHEAF_HOLONOMY   0x0000C068U
#define SHBT_APOTH_REG_COBORDISM_OP           0x0000C06CU
#define SHBT_APOTH_REG_QUALIA_ENTROPY_MARGIN  0x0000C070U
#define SHBT_APOTH_REG_RESERVED_EXT           0x0000C078U

typedef struct __attribute__((packed, gcc_struct, aligned(8))) {
    uint64_t qualia_betti_signature;      /* 0x00 (0x4000C060): Causal cone Betti numbers [b0, b1, b2] */
    uint32_t poset_sheaf_holonomy_latch;  /* 0x08 (0x4000C068): Cech H^1 obstruction vanishing latch */
    uint32_t cobordism_op_reg;            /* 0x0C (0x4000C06C): Fission/Fusion Lagrangian cobordism opcode */
    uint64_t qualia_entropy_margin_fp;    /* 0x10 (0x4000C070): Excess entropy margin R_entropy (fixed point) */
    uint64_t reserved_apoth_ext;          /* 0x18 (0x4000C078): Reserved alignment pad (total 32 bytes) */
} shbt_apoth_ext_ctrl_t;

_Static_assert(sizeof(shbt_apoth_ext_ctrl_t) == 32, "shbt_apoth_ext_ctrl_t must be 32 bytes");

/* =========================================================================
 * Apotheosis Hardware Command Opcodes (ctrl_cmd)
 * ========================================================================= */
#define SHBT_APOTH_CMD_NOP               ((uint32_t)0x00000000UL)
#define SHBT_APOTH_CMD_ASCEND            ((uint32_t)0x00000001UL)
#define SHBT_APOTH_CMD_QND_HANDSHAKE     0x00000001U /* Engage non-demolition ego transfer */
#define SHBT_APOTH_CMD_DESCEND_SOMATIC   ((uint32_t)0x00000002UL)
#define SHBT_APOTH_CMD_HORIZON_DEFLECT_ARM 0x00000002U /* Arm asymptotic causal decoupling */
#define SHBT_APOTH_CMD_DEITY_LAW_REWRITE ((uint32_t)0x00000004UL)
#define SHBT_APOTH_CMD_MERA_MEM_COMPRESS 0x00000004U /* Trigger episodic MERA RG compression */
#define SHBT_APOTH_CMD_VERLINDE_MULTIPLEX ((uint32_t)0x00000008UL)
#define SHBT_APOTH_CMD_SANDBOX_FORK_X1   ((uint32_t)0x00000010UL)
#define SHBT_APOTH_CMD_ZERO_COMP_RENDER  ((uint32_t)0x00000020UL) /* 0x0020 */
#define SHBT_APOTH_CMD_SYNTH_AVATAR      ((uint32_t)0x00000040UL)
#define SHBT_APOTH_CMD_BRAID_CRYSTALLIZE ((uint32_t)0x00000080UL)
#define SHBT_APOTH_CMD_EXEC_COBORDISM_FISS 0x00000800U
#define SHBT_APOTH_CMD_EXEC_COBORDISM_FUSE 0x00001000U

/* =========================================================================
 * Hardware Protection Latches & Airgap Flags (sandbox_status_latch)
 * ========================================================================= */
#define SHBT_LOGOS_SIM_AIRGAP_LOCK       ((uint32_t)0x00000001UL)
#define SHBT_LOGOS_SIM_X1_PROTECT_LATCH  ((uint32_t)0x00000002UL)
#define SHBT_LOGOS_SIM_KILL_DISABLE      ((uint32_t)0x00000004UL)
#define SHBT_LOGOS_SOMATIC_MODE_A_STASIS ((uint32_t)0x00000010UL)
#define SHBT_LOGOS_SOMATIC_MODE_B_VAULT  ((uint32_t)0x00000020UL)
#define SHBT_LOGOS_OMNIBUS_TETHER_ACTIVE ((uint32_t)0x00000040UL)

/* =========================================================================
 * Hardware Fault & Interdiction Flags (fault_code)
 * ========================================================================= */
#define SHBT_APOTH_FAULT_NONE                   ((uint32_t)0x00000000UL)
#define SHBT_APOTH_FAULT_NOCLONE_VIOLATION      ((uint32_t)0x00000001UL)
#define SHBT_APOTH_FAULT_SYNAPTIC_SHEAR_CBF     ((uint32_t)0x00000002UL)
#define SHBT_APOTH_FAULT_AIRGAP_BREACH_ATTEMPT  ((uint32_t)0x00000004UL)
#define SHBT_APOTH_FAULT_GENOCIDE_INTERDICT     ((uint32_t)0x00000008UL)
#define SHBT_APOTH_FAULT_BEKENSTEIN_NEST_LIMIT  ((uint32_t)0x00000010UL)
#define SHBT_APOTH_FAULT_SUBSTRATE_INADMISSIBLE ((uint32_t)0x00000020UL)
#define SHBT_APOTH_FAULT_QND_COLLAPSE           0x00000040U /* Wavefunction collapse / ego breach */
#define SHBT_APOTH_FAULT_P_ZOMBIE_COLLAPSE      0x00000080U /* Causal depth mismatch in sandbox */
#define SHBT_APOTH_EGRESS_SAFEWORD_IRQ          ((uint32_t)0x80000000UL)

/**
 * @brief Pillar IV Causal Point Observer Frame (128 bytes)
 *   Tracks active pointer weight, Stinespring vault hash, and entropy budget.
 */
typedef struct __attribute__((packed, aligned(64))) {
    uint64_t observer_uuid_lo;         /* 0x00: Lower 64b of Inhabitant UUID        */
    uint64_t observer_uuid_hi;         /* 0x08: Upper 64b of Inhabitant UUID        */
    double ramp_param_s;               /* 0x10: Eikonal ramp s(tau) in [0.0, 1.0]   */
    double lapse_alpha_obs;            /* 0x18: Subjective lapse alpha in [1, 1e12] */
    double c_local_budget_log10;       /* 0x20: log10(C_local) Bekenstein capacity  */
    double c_op_active_log10;          /* 0x28: log10(C_op) active complexity       */
    double r_entropy_margin;           /* 0x30: R_entropy^exp = C_local - C_op >= 0 */
    double h_syn_cbf_margin;           /* 0x38: Synaptic continuity CBF >= 0        */
    double h_cog_homeostasis_margin;   /* 0x40: Anti-wirehead/ego-quine CBF >= 0    */
    uint32_t verlinde_avatar_count_k;  /* 0x48: Multiplexed avatar streams K <= 124 */
    uint32_t nested_recursion_depth_d; /* 0x4C: Simulation recursion depth d        */
    uint64_t burau_cayley_memory_hash[4]; /* 0x50: 256-bit Depth-9 Artin B3 root hash (32 B) */
    uint64_t stinespring_vault_ptr;    /* 0x70: Class X0 ROM Vault DMA pointer      */
    uint64_t reserved_pad;             /* 0x78: Alignment padding                   */
} shbt_causal_observer_frame_t;

_Static_assert(sizeof(shbt_causal_observer_frame_t) == 128,
               "ABI Error: shbt_causal_observer_frame_t must be exactly 128 bytes");

/**
 * @brief Theorem THM-APOTH-05 Vessel Neural-Substrate Admissibility Certificate (64 bytes)
 */
typedef struct __attribute__((packed, aligned(64))) {
    uint64_t vessel_blueprint_hash_lo; /* 0x00: MERA-DAG vessel hash (low 64b)     */
    uint64_t vessel_blueprint_hash_hi; /* 0x08: MERA-DAG vessel hash (high 64b)    */
    double phi_substrate_max_log10;    /* 0x10: log10(Phi_substrate^max)            */
    double phi_observer_req_log10;     /* 0x18: log10(Phi_obs) >= 12.0 (Class X1)   */
    double c_local_vessel_log10;       /* 0x20: log10(C_local(Omega_vessel))        */
    double bisimulation_error_log10;   /* 0x28: log10(||Y*sigma - U*Y||) <= -120.0  */
    double thermal_cbf_margin;         /* 0x30: dh_syn/dt + alpha(h_syn) >= 0       */
    uint8_t space_group_id;            /* 0x38: Crystallographic group [0..230]     */
    uint8_t standalone_admissible_flag;/* 0x39: 1 = Standalone OK, 0 = Need Tether  */
    uint8_t omnibus_tether_engaged;    /* 0x3A: 1 = 0-as SHBT-OMNIBUS Tether Lock   */
    uint8_t non_harm_cbf_verified;     /* 0x3B: 1 = Physical render h_CBF >= 0      */
    uint32_t cert_crc32c;              /* 0x3C: Hardware certificate checksum       */
} shbt_vessel_admissibility_cert_t;

_Static_assert(sizeof(shbt_vessel_admissibility_cert_t) == 64,
               "ABI Error: shbt_vessel_admissibility_cert_t must be exactly 64 bytes");

/**
 * @brief Top-Level Apotheosis Control Aperture at 0x4000C000 (256 bytes)
 */
typedef struct __attribute__((packed, aligned(64))) {
    volatile uint32_t ctrl_cmd;              /* 0x00: Command register               */
    volatile uint32_t sandbox_status_latch;   /* 0x04: Airgap & X1 protection latches */
    volatile uint32_t fault_code;             /* 0x08: Interdiction & fault flags     */
    volatile uint32_t lagrangian_submanifold; /* 0x0C: H_1(Sigma_g, Z) slot ID        */
    volatile double phi_mera_max_detected;   /* 0x10: Max emergent Phi_MERA in sim   */
    volatile double x1_cbf_min_margin;        /* 0x18: Min Class X1 safety CBF margin */
    volatile uint64_t x1_sanctuary_fork_cnt; /* 0x20: Count of X1 horizon evacuations */
    volatile uint64_t goi_pass_counter;       /* 0x28: 17.01 as PT-GoI pass counter   */
    uint64_t reserved_hdr[2];                 /* 0x30: Pad header to 64 bytes         */
    shbt_causal_observer_frame_t observer;    /* 0x40: Active observer state (128 B)  */
    shbt_vessel_admissibility_cert_t vessel;  /* 0xC0: Target vessel cert (64 B)      */
} shbt_apotheosis_ctrl_t;

_Static_assert(sizeof(shbt_apotheosis_ctrl_t) == 256,
               "ABI Error: shbt_apotheosis_ctrl_t must be exactly 256 bytes");

#ifdef __cplusplus
}
#endif

#endif /* SHBT_MMIO_APOTHEOSIS_H */
