/*
 * include/shbt_swarm_mmio.h
 * Bare-Metal C11 Register, Cognitive, Universal Synthesis & MAR-312X Reactor Specification
 * Phase 4: Exotic-Material Autopoietic Mass-Annihilation Reactor (SHBT-MAR-312X)
 * Base Physical Address: 0x70000000 (128-Byte Dual-Cacheline Contract)
 */

#ifndef SHBT_SWARM_MMIO_H
#define SHBT_SWARM_MMIO_H

#include <stdint.h>
#include <stddef.h>
#include "shbt_mmio_apotheosis.h"

#define SHBT_SWARM_ABI_VERSION          0x00060312U
#define SHBT_SWARM_MMIO_BASE_ADDR       0x70000000UL
#define SHBT_SWARM_HYPERFRAME_BASE_ADDR 0x70020000UL
#define SHBT_SWARM_APOTHEOSIS_BASE_ADDR 0x4000C000UL

#define SHBT_FRAME_BYTES                2112U
#define SHBT_VISIBLE_ANCHOR_BYTES       640U     /* eta_A = 10/33 * 2112 B */
#define SHBT_DARK_LEDGER_BYTES          1472U    /* eta_D = 23/33 * 2112 B */
#define SHBT_HYPERFRAME_BANKS           64U
#define SHBT_HYPERFRAME_TOTAL_BYTES     135168U  /* 64 * 2112 B */
#define SHBT_BRAIDS_PER_FRAME           124U
#define SHBT_DYNKIN_WEIGHT_SLOTS        346U
#define SHBT_MERA_MAX_LAYERS            101U
#define SHBT_MAX_SPACE_GROUP_ID         230U     /* 0 = Amorphous/Bio, 1..230 = 3D Crystals */

/* Phase 4 SHBT-MAR-312X Hardware Constants */
#define SHBT_MAR312X_NACELLE_MASS_KG        950U     /* 950.00 kg (-57.92% vs MAR-312C) */
#define SHBT_MAR312X_NACELLE_VOL_ML         350000U  /* 0.3500 m^3 = 350,000 mL */
#define SHBT_MAR312X_MAX_SINGLE_FRAME_CORES 41U      /* 41 * 376.99 + 950.00 = 16,406.59 kg <= 16,600 kg */
#define SHBT_MAR312X_SUPERHYDRIDE_FIELD_MT  48000U   /* 48.0 T = 48,000 mT */
#define SHBT_MAR312X_TCRIT_SUPERHYDRIDE_MK  192500U  /* 192.50 K = 192,500 mK */
#define SHBT_MAR312X_DECX_EFF_PPM           992000U  /* 99.200% = 992,000 ppm */
#define SHBT_MAR312X_DIRECT_BUS_EFF_PPM     998500U  /* 99.850% = 998,500 ppm */
#define SHBT_MAR312X_WEYL_CROWBAR_FS        1701U    /* 1.701 ps = 1,701 fs */

/* Phase 1 + Phase 2 + Phase 3 + Phase 4 Command Opcodes for cmd_reg (0x08) */
#define SHBT_SWARM_CMD_NOP                      0x00000000U
#define SHBT_SWARM_CMD_TOMITA_QUINE_CPY         0x00000001U /* Pathway A: Modular ROM->Synth copy */
#define SHBT_SWARM_CMD_PT_GOI_PROJECT           0x00000002U /* Pathway B: 1.701 ps optical imprint */
#define SHBT_SWARM_CMD_TOMO_CAPTURE             0x00000004U /* Pathway C: 4.2 K Landauer readout */
#define SHBT_SWARM_CMD_EXEC_CLONE_SHIP          0x00000008U /* Phase 1: Synthesize daughter ship */
#define SHBT_SWARM_CMD_EXEC_CLONE_BIO           0x00000010U /* Phase 1: Synthesize CHNOPS payload */
#define SHBT_SWARM_CMD_LANDAUER_SHUNT           0x00000020U /* S = 100/1089 dark-ledger erasure */
#define SHBT_SWARM_CMD_MERA_PINN_SYNC           0x00000040U /* Phase 2: 101-layer MERA-PINN encode/decode */
#define SHBT_SWARM_CMD_PT_GOI_EQUIL             0x00000080U /* Phase 2: 54.432 ps 32-bank GoI policy eq */
#define SHBT_SWARM_CMD_TOMITA_LEARN             0x00000100U /* Phase 2: Zero-heat modular weight flow */
#define SHBT_SWARM_CMD_VERLINDE_PLAN            0x00000200U /* Phase 2: 124-to-1 Fibonacci-Verlinde plan */
#define SHBT_SWARM_CMD_SP2G_HIVE_SYNC           0x00000400U /* Phase 2: Sp(2g,Z) dark-ledger consensus */
#define SHBT_SWARM_CMD_AP_MPC_ENCLAVE           0x00000800U /* Phase 2: Alpha > 1 Pareto re-optimization */
#define SHBT_SWARM_CMD_XENO_ASSIMILATE          0x00001000U /* Phase 2: Class X2 non-ionizing recycling */
/* Extended Opcodes */
#define SHBT_SWARM_CMD_HOCBF_EVAL_FILTER           0x00002000U /* Execute r=2 QP barrier filter */
#define SHBT_SWARM_CMD_CRYOPHONON_SHUNT_ENGAGE     0x00004000U /* Engage Diamond-GaN S=100/1089 shunt */
#define SHBT_SWARM_CMD_EXEC_UNIV_SYNTH          0x00002000U /* Phase 3: R_univ^SWARM arbitrary matter */
#define SHBT_SWARM_CMD_ZERO_COMP_DIRECT_RENDER  0x00004000U /* Phase 3: Delta C_post-sim = 0 render */
#define SHBT_SWARM_CMD_MERA_DAG_INSTANCE        0x00008000U /* Phase 3: Sp(2g,Z) MERA-DAG tiling */
#define SHBT_SWARM_CMD_MAR312X_FORGE_TIER4      0x00010000U /* Phase 4: Engage 25.00 EW Tier 4 Forge */
#define SHBT_SWARM_CMD_DIRECT_BOUNDARY_SYNTH    0x00020000U /* Phase 4: B_direct^MAR-X (99.85% CFT bus) */
#define SHBT_SWARM_CMD_AUTOPOIETIC_SELF_HEAL    0x00040000U /* Phase 4: A_heal 50.518 kHz ROM scrub */
#define SHBT_SWARM_CMD_HBT_ROTATOR_LOCK       0x00080000U /* Lock 8x8 InP/InGaAs HBT & PIC PID Loop */
#define SHBT_SWARM_CMD_WARP_SOLITON_COUPLE    0x00100000U /* Couple MAR-312X DEC-X Bus to Shift Soliton */
#define SHBT_SWARM_CMD_WEYL_EXCITON_CROWBAR     0x80000000U /* Phase 4: 1.701 ps Weyl-Exciton abort */
#define SHBT_SWARM_CMD_PCSS_CROWBAR             0x80000000U /* Legacy alias for hardware crowbar */

/* Phase 3 Direct Subsystem Command Opcodes */
#define SHBT_SWARM_CMD_UNIV_SYNTH_OPCODE        0x0010U /* Phase 3 Subsystem Opcode (0x0010) */
#define SHBT_SWARM_CMD_DIRECT_RENDER_OPCODE     0x0020U /* Phase 3 Subsystem Opcode (0x0020) */
#define SHBT_SWARM_CMD_MERA_DAG_OPCODE          0x0030U /* Phase 3 Subsystem Opcode (0x0030) */

/* Phase 1 + Phase 2 + Phase 3 + Phase 4 Fault Bitmasks for fault_mask_reg (0x10) */
#define SHBT_SWARM_FAULT_NONE                           0x00000000U
#define SHBT_SWARM_FAULT_ADM_IMBALANCE                  0x00000001U /* M_feedstock != M_payload */
#define SHBT_SWARM_FAULT_PARTITION_LOCK                 0x00000002U /* eta_A : eta_D != 10 : 23 */
#define SHBT_SWARM_FAULT_FRAMING_WEYL                   0x00000004U /* Delta_fr != 0 or E_munu != 0 */
#define SHBT_SWARM_FAULT_SUGAWARA_CAP_EXCEEDED_TRIP     0x00000004U /* Phase 3: Hardware trip bit 0x00000004 */
#define SHBT_SWARM_FAULT_BL_ANOMALY                     0x00000008U /* Delta(B-L)_tot != 0 */
#define SHBT_SWARM_FAULT_DISLOCATION_DETECTED           0x00000008U /* Phase 3: Wyckoff tether displacement breach */
#define SHBT_SWARM_FAULT_NEUTRINO_LEAK                  0x00000010U /* Phi_nu > 0 */
#define SHBT_SWARM_FAULT_CHIRAL_INVERSION               0x00000010U /* Phase 3: Parity-odd Casimir-Polder breach */
#define SHBT_SWARM_FAULT_SECDED_UNCORR                  0x00000020U /* Double-bit flip in B_3 word */
#define SHBT_SWARM_FAULT_CBF_TRIP                       0x00000020U /* Phase 3: Control Barrier Function trip */
#define SHBT_SWARM_FAULT_CAYLEY_HASH                    0x00000040U /* Phylogenetic quine / Byzantine mismatch */
#define SHBT_SWARM_FAULT_THERMAL_CLAMP                  0x00000080U /* T_QC > 4.2 K or T_MAR > 192.50 K */
#define SHBT_SWARM_FAULT_CBF_PHYS_VIOL                  0x00000100U /* Phase 2: h_phys(x) < 0 barrier breach */
#define SHBT_SWARM_FAULT_GREY_GOO_CAP                   0x00000200U /* Phase 2: M(t) > M_max population cap */
#define SHBT_SWARM_FAULT_CLF_DIVERGENCE                 0x00000400U /* Phase 2: dV_terra/dt > -kappa * V_terra */
#define SHBT_SWARM_FAULT_X0_PROTECT_LCK                 0x00000800U /* Phase 2: Attempted de-render of Class X0 */
#define SHBT_SWARM_FAULT_X1_SAPIENT_LCK                 0x00001000U /* Phase 2: Attempted de-render of Class X1 */
/* Extended Status / Fault Bits for Phase 2 First-Principles Closure */
#define SHBT_SWARM_FAULT_EP2_PERTURBATION_EXCEEDED 0x00002000U /* ||delta L||_op > eps_crit */
#define SHBT_SWARM_FAULT_HOCBF_BARRIER_TRIP        0x00004000U /* psi_2(x,u) < 0 or r=2 breach */
#define SHBT_SWARM_FAULT_CRYOGENIC_SHUNT_OVERFLOW  0x00008000U /* P_vis > 1.01 nW at 4.2 K stage */
#define SHBT_SWARM_FAULT_SIMPLICIAL_SHEAR_MISMATCH 0x00010000U /* rho_spec(T) != 1.0 */
#define SHBT_SWARM_FAULT_SUGAWARA_CAP_EXCEEDED          0x00002000U /* Phase 3: |Delta h_vis| > 38.196601 nats */
#define SHBT_SWARM_FAULT_DRIPLINE_UNSTABLE              0x00004000U /* Phase 3: Non-integrable / unbound nucleus */
#define SHBT_SWARM_FAULT_SIM_TO_RENDER_HASH_MISMATCH    0x00008000U /* Phase 3: MERA-PINN UV != Synth Frame hash */
#define SHBT_SWARM_FAULT_HABITAT_GRAV_DEVIATION         0x00010000U /* Phase 3: |g_hab - 9.80665 m/s^2| > tol */
#define SHBT_SWARM_FAULT_SUPERHYDRIDE_TETHER_LOSS       0x00020000U /* Phase 4: |delta x| > 10^-4 A or B < 48 T */
#define SHBT_SWARM_FAULT_PURCELL_BANDGAP_DRIFT          0x00040000U /* Phase 4: rho_DOS(w_spont) > 0 or F_P > 0 */
#define SHBT_SWARM_FAULT_DECX_CHERN_BACKSCATTER         0x00080000U /* Phase 4: C_2 == 0 or eta_DEC_X < 99.20% */
#define SHBT_SWARM_FAULT_FORGE_SHEAR_EXCEEDED           0x00100000U /* Phase 4: sigma_max > 4,800.00 MPa */
#define SHBT_SWARM_FAULT_HBT_PHASE_JITTER     0x00200000U /* sigma_phi > 5.05e-5 rad or C < 0.9995 */
#define SHBT_SWARM_FAULT_ACOUSTIC_REFLECTANCE 0x00400000U /* R_acoustic > 1e-6 or sigma_max > 350 MPa */

/* Xenobiological Triage Classification Bitmasks (Encoded in status_reg [31:28]) */
#define SHBT_SWARM_TRIAGE_SHIFT         28U
#define SHBT_SWARM_TRIAGE_MASK          0xF0000000U
#define SHBT_SWARM_TRIAGE_M0_REGOLITH   0x00000000U /* D_Cayley > 0, C_MERA == 0 (Barren rock) */
#define SHBT_SWARM_TRIAGE_X0_PROTECT    0x10000000U /* D_Cayley == 0 (Earth-Seeded / Protected) */
#define SHBT_SWARM_TRIAGE_X1_QUARANTINE 0x20000000U /* D_Cayley > 0, C_MERA >= C_crit (Sapient) */
#define SHBT_SWARM_TRIAGE_X2_ASSIMILATE 0x40000000U /* D_Cayley > 0, 0 < C_MERA < C_crit (Prim) */

/*
 * Structured Cognitive, Universal Synthesis & MAR-312X Telemetry inside the 356-Byte Dark Stinespring Pad.
 * Strictly preserves exact 356-byte size while embedding Phase 4 exotic-reactor telemetry fields.
 */
#pragma pack(push, 1)
typedef struct __attribute__((packed, gcc_struct)) {
    uint64_t v_terra_lyapunov_q32;          /* 0x00: Planetary habitability CLF V_terra (Q32.32) */
    int64_t  dv_terra_dt_q32;               /* 0x08: CLF time derivative dV_terra/dt < 0 (Q32.32) */
    uint64_t c_mera_complexity_q16;         /* 0x10: Scanned xenobiological C_MERA (Q48.16) */
    uint32_t d_cayley_distance;             /* 0x18: Phylogenetic word distance from H_Earth-Seed */
    uint32_t active_swarm_nodes_m;          /* 0x1C: Current hive node count M(t) */
    uint32_t max_swarm_nodes_m_max;         /* 0x20: Anti-grey-goo ceiling M_max(A_planet, M_rem) */
    uint32_t sp2g_symplectic_crc32;         /* 0x24: Sp(2g, Z) Lagrangian shear checksum */
    int64_t  delta_h_vis_q32;               /* 0x28: Phase 3 Dynamic Sugawara shift Delta h_vis (Q32.32) */
    int64_t  delta_h_braid_per_ch_q32;      /* 0x30: Phase 3 Per-channel braid shift -Delta h_vis/124 */
    uint64_t mera_dag_tile_instance_cnt;    /* 0x38: Phase 3 Active MERA-DAG instanced tile count */
    uint32_t dark_folding_eta_b_ppm;        /* 0x40: Phase 3 Dark fraction eta_B = sin^2(theta) in ppm */
    uint16_t space_group_id;                /* 0x44: Phase 3 Space group G_space in [0, 230] */
    uint16_t ghost_hab_grav_mms2;           /* 0x46: Phase 3 Habitat floor gravity in mm/s^2 (9807) */
    /* === Phase 4 SHBT-MAR-312X Exotic Reactor Telemetry (Offsets 0x48 .. 0x5F, 24 Bytes) === */
    uint64_t autopoietic_heal_cycle_cnt;    /* 0x48: Phase 4 Cumulative 50.518 kHz A_heal scrub cycles */
    uint32_t superhydride_field_mt;         /* 0x50: Phase 4 Active coil magnetic field in mT (48,000 mT) */
    uint32_t superhydride_tcrit_margin_mk;  /* 0x54: Phase 4 Thermal margin T_crit - T_op in mK (171,370) */
    uint32_t decx_conv_efficiency_ppm;      /* 0x58: Phase 4 DEC-X / Direct bus efficiency (992000/998500) */
    uint16_t seed_die_shear_stress_mpa;     /* 0x5C: Phase 4 Peak seed die shear stress (<= 4800 MPa) */
    uint16_t loaded_fuel_cores_cnt;         /* 0x5E: Phase 4 Single-frame fuel cores (<= 41 cores) */
    uint8_t  dark_purification_state[256];  /* 0x60..0x15F: 256-byte compressed Stinespring ancilla */
    uint32_t dark_holonomy_crc32;           /* 0x160..0x163: 4-byte boundary holonomy parity (total 356 B) */
} shbt_sten_dark_telemetry_t;
#pragma pack(pop)

_Static_assert(sizeof(shbt_sten_dark_telemetry_t) == 356,
               "shbt_sten_dark_telemetry_t must fit exactly in the 356-byte dark Stinespring pad");

/* Dual-View 2,112-Byte UnifiedStinespringFrame */
#pragma pack(push, 1)
typedef union __attribute__((packed, gcc_struct, aligned(64))) {
    struct __attribute__((packed, gcc_struct)) {
        uint8_t                    visible_anchor[SHBT_VISIBLE_ANCHOR_BYTES]; /* 640 B (eta_A = 10/33) */
        uint64_t                   braid_descriptors[SHBT_BRAIDS_PER_FRAME];  /* 992 B (124 * 8 B Artin B_3) */
        uint8_t                    secded_parity[SHBT_BRAIDS_PER_FRAME];      /* 124 B Hamming(72,64) parity */
        shbt_sten_dark_telemetry_t dark_sten;                                 /* 356 B dark Stinespring pad */
    } ctrl_view;
    struct __attribute__((packed, gcc_struct)) {
        uint64_t mpfr_limbs[33][8]; /* 33 slots * 512-bit (64 B) Weyl-Kac & MERA-PINN UV weights = 2112 B */
    } mpfr_view;
    uint8_t raw_bytes[SHBT_FRAME_BYTES];
} shbt_unified_stinespring_frame_t;
#pragma pack(pop)

_Static_assert(sizeof(shbt_unified_stinespring_frame_t) == 2112,
               "UnifiedStinespringFrame must be exactly 2,112 bytes");

/* 64-Bank UnifiedStinespringHyperFrame (135,168 Bytes) */
typedef struct __attribute__((aligned(4096))) {
    const shbt_unified_stinespring_frame_t parent_quine_rom[32];    /* Banks 00..31 (RO Quine, 00..09 MAR-312X) */
    shbt_unified_stinespring_frame_t       daughter_synth_ram[32];  /* Banks 32..63 (RW MERA-UV / Synth) */
} shbt_unified_stinespring_hyperframe_t;

typedef shbt_unified_stinespring_hyperframe_t UnifiedStinespringHyperFrame;

_Static_assert(sizeof(shbt_unified_stinespring_hyperframe_t) == 135168,
               "UnifiedStinespringHyperFrame must be exactly 135,168 bytes");
_Static_assert(sizeof(UnifiedStinespringHyperFrame) == 135168,
               "sizeof(UnifiedStinespringHyperFrame) == 135168");

/**
 * @brief Hardware MMIO Barrier and Memory Flush Primitive
 */
static inline void shbt_mmio_write_barrier(void) {
#if defined(__x86_64__)
    __asm__ __volatile__("sfence" ::: "memory");
#elif defined(__aarch64__)
    __asm__ __volatile__("dmb oshst" ::: "memory");
#else
    __atomic_thread_fence(__ATOMIC_SEQ_CST);
#endif
}

/**
 * @brief Flush and invalidate a single 512-bit cache line for spatial tensor streaming
 */
static inline void shbt_clwb(const void *addr) {
#if defined(__x86_64__)
    __asm__ __volatile__("clwb %0" : : "m"(*(const char *)addr) : "memory");
#elif defined(__aarch64__)
    __asm__ __volatile__("dc cvac, %0" : : "r"(addr) : "memory");
#else
    (void)addr;
#endif
}


/* Monolithic 8x8 InP/InGaAs HBT & InP/InGaAsP PIC Array Constants */
#define SHBT_SWARM_HBT_CHANNELS               64U
#define SHBT_SWARM_HBT_PITCH_UM               50.0
#define SHBT_SWARM_PIC_PITCH_UM               127.0
#define SHBT_SWARM_HBT_V_ON_MV                3800U
#define SHBT_SWARM_HBT_V_SAT_MV               7400U
#define SHBT_SWARM_HBT_MASTER_CLOCK_GHZ       72.0
#define SHBT_SWARM_HBT_RF_DRIVE_GHZ           40.0

/* Quantize continuous phase [rad] to 16-bit hardware IQ word: floor((phi mod 2pi)/(2pi) * 65536) */
static inline uint16_t shbt_swarm_emitter_phase_word(double phi_rad)
{
    const double two_pi = 6.28318530717958647692;
    double wrapped = phi_rad;
    while (wrapped < 0.0) {
        wrapped += two_pi;
    }
    while (wrapped >= two_pi) {
        wrapped -= two_pi;
    }
    double word = (wrapped / two_pi) * 65536.0;
    if (word < 0.0) word = 0.0;
    if (word > 65535.0) word = 65535.0;
    return (uint16_t)word;
}

/* Map 16-bit IQ phase word linearly to HBT collector voltage in millivolts [3800 mV, 7400 mV] */
static inline uint32_t shbt_swarm_hbt_bias_mv(uint16_t iq16_word)
{
    uint32_t v_span = SHBT_SWARM_HBT_V_SAT_MV - SHBT_SWARM_HBT_V_ON_MV;
    uint32_t delta_mv = (uint32_t)(((uint64_t)iq16_word * (uint64_t)v_span) / 65536ULL);
    return SHBT_SWARM_HBT_V_ON_MV + delta_mv;
}

/* 128-Byte Dual-Cacheline SHBT-MMIO-SWARM v4 Register Layout at 0x70000000 */
#pragma pack(push, 1)
typedef struct __attribute__((packed, gcc_struct, aligned(64))) {
    /* === CACHELINE 0 (Offsets 0x00 .. 0x3F): Control, Quine & Optical GoI === */
    volatile uint32_t magic_id;              /* 0x00: 0x5357524D ('SWRM') */
    volatile uint32_t abi_version;           /* 0x04: 0x00060312 (v6.312 SHBT-APOTHEOSIS-312) */
    volatile uint32_t cmd_reg;               /* 0x08: Command trigger register */
    volatile uint32_t status_reg;            /* 0x0C: Pipeline [27:0] & Triage class [31:28] */
    volatile uint32_t fault_mask_reg;        /* 0x10: Hardware invariant, CBF & MAR-312X fault flags */
    volatile uint32_t rom_bank_mask;         /* 0x14: Active ROM source banks (00..31; 00..09 MAR-312X) */
    volatile uint32_t synth_bank_mask;       /* 0x18: Active target banks (32..63) */
    volatile uint32_t power_tier_sel;        /* 0x1C: [2:0]=Tier(0..4), [15:8]=MERA L, [24:16]=G_space */
    volatile uint64_t tomita_flow_phase_q64; /* 0x20: Modular learning/healing parameter t (Q16.48) */
    volatile uint64_t pt_goi_prop_ps_q32;    /* 0x28: Optical GoI / Weyl crowbar timer (1.701 ps) */
    volatile uint64_t secded_syndrome_cnt;   /* 0x30: Corrected single-bit error counter */
    volatile uint64_t generation_counter;    /* 0x38: Von Neumann phylogenetic generation G */

    /* === CACHELINE 1 (Offsets 0x40 .. 0x7F): Mass-Energy, Bio, Thermal & Cayley Hash === */
    volatile uint64_t feedstock_mass_ug;     /* 0x40: Harvested Mode 1b / X2 mass in ug */
    volatile uint64_t target_payload_mass_ug;/* 0x48: Target clone/synthesis mass in ug */
    volatile int64_t  net_b_minus_l_balance; /* 0x50: Must equal 0 for synthesis commit */
    volatile uint32_t gst225_impedance_krayl;/* 0x58: Acoustic impedance (1000..100000 kRayl) */
    volatile uint32_t enclave_lapse_milli;   /* 0x5C: Hyper-temporal lapse alpha * 1000 */
    volatile uint32_t temp_qc_mk;            /* 0x60: SHBT-R temp in mK (clamped <= 4200 mK) */
    volatile uint32_t temp_nacelle_mk;       /* 0x64: MAR-312X temp in mK (21130..24850 <= 192500 mK) */
    volatile uint64_t cayley_hash_lo;        /* 0x68: Burau-Cayley phylogenetic digest [63:0] */
    volatile uint64_t cayley_hash_mid0;      /* 0x70: Burau-Cayley phylogenetic digest [127:64] */
    volatile uint64_t cayley_hash_mid1;      /* 0x78: Burau-Cayley phylogenetic digest [191:128] */
} shbt_swarm_mmio_t;
#pragma pack(pop)

_Static_assert(sizeof(shbt_swarm_mmio_t) == 128,
               "shbt_swarm_mmio_t must occupy exactly two 64-byte cachelines (128 bytes)");

/* ========================================================================= */
/* Phase 5A: SHBT-OMEGA-312 Galactic Megastructure & SHBT-AEGIS-OMEGA        */
/* Aperture Base: 0x40008000 (256 Bytes)                                     */
/* ========================================================================= */

#define SHBT_MMIO_OMEGA_BASE            ((uintptr_t)0x40008000ULL)
#define SHBT_MMIO_AEGIS_BASE            ((uintptr_t)0x40009000ULL)

#define SHBT_MERA_MAX_LAYER_GAL         234U
#define SHBT_MERA_MIN_LAYER_GAL         102U
#define SHBT_OMEGA_SECTOR_COUNT         312U
#define SHBT_GHOST_CHANNELS             312U
#define SHBT_HAB_GRAVITY_STD_UG         9806650ULL /* 9.806650 m/s^2 in micro-m/s^2 */

/* SHBT-AEGIS-OMEGA Status & Control Bitmasks */
#define SHBT_AEGIS_L1_DARK_CLOAK_EN     (1ULL << 0)  /* eta_D = 23/33 gauge decoupling */
#define SHBT_AEGIS_L2_CCZ4_CURTAIN_EN   (1ULL << 1)  /* 2PN causal gate + shift collapse */
#define SHBT_AEGIS_L3_CAYLEY_FIREWALL   (1ULL << 2)  /* 256-bit Burau-Cayley braid auth */
#define SHBT_AEGIS_L4_SENTINEL_ACTIVE   (1ULL << 3)  /* 10 Mpc TMSV r=2.50 + Harvester */
#define SHBT_AEGIS_CBF_X1_BIO_DETECTED  (1ULL << 8)  /* Class X1 sapient signature latch */
#define SHBT_AEGIS_STASIS_LOCK_ACTIVE   (1ULL << 9)  /* alpha = 10^-9 non-lethal stasis */
#define SHBT_AEGIS_DERENDER_FUEL_EN     (1ULL << 10) /* Class X0 unmanned recycling */
#define SHBT_AEGIS_ROM_WRITE_LOCKOUT    (1ULL << 63) /* Hardwired Banks 00..31 ROM lock */

/**
 * @brief Galactic MERA-DAG Layer 102..234 Hierarchical Addressing Descriptor (64 bytes)
 */
typedef struct __attribute__((packed, gcc_struct, aligned(8))) {
    uint64_t nucleon_count_mantissa;   /* Normalized fixed-point N in [10^45, 10^70] */
    uint16_t nucleon_count_exponent;   /* Base-10 exponent (up to 70) */
    uint8_t  active_mera_layer;        /* Layer index l in [102, 234] */
    uint8_t  sp2g_sector_id;           /* Galactic sector index [0..255] (ext to 312) */
    uint16_t sp2g_extended_sector;     /* Full sector index [0..311] */
    uint16_t bond_dimension_chi;       /* Fixed at 312 for C_GKO(26,8,312) */
    uint64_t merkle_dag_root_lo;       /* Lower 64 bits of deduplicated sub-tree hash */
    uint64_t merkle_dag_root_hi;       /* Upper 64 bits of deduplicated sub-tree hash */
    uint64_t isometric_err_bound_atto; /* Verified <= 10^-120 scaled residual */
    uint64_t dark_ledger_mass_kg_fp;   /* Folded mass in SHBT-IBR-OMEGA (eta_D=23/33) */
    uint64_t reserved_pad;             /* Alignment pad to 64 bytes */
    uint64_t secded_ecc_syndrome;      /* Hamming(72,64) hardware parity check */
} shbt_mera_gal_addr_t;

_Static_assert(sizeof(shbt_mera_gal_addr_t) == 64, "shbt_mera_gal_addr_t must be 64 bytes");

/**
 * @brief SHBT-ELYSIUM Pocket Manifold & Wish-to-Reality Control Block (64 bytes)
 */
typedef struct __attribute__((packed, gcc_struct, aligned(8))) {
    uint64_t pocket_manifold_id;       /* Topological pocket identifier U_a */
    uint64_t target_gravity_um_s2;     /* Default 9,806,650 (9.80665 m/s^2), 0g..1g */
    int64_t  lapse_alpha_fp32;         /* Q32.32 lapse alpha in [10^-4, 10^4] */
    uint32_t cascade_lapse_stage2;     /* Secondary nested enclave multiplier [1..100] */
    uint32_t solar_spectrum_temp_k;    /* Default 5778 K Weyl-polariton emitter */
    uint64_t weyl_tidal_norm_residual; /* Hardware invariant: must equal 0 (E_uv == 0) */
    uint64_t coriolis_omega_residual;  /* Hardware invariant: must equal 0 (w_Cor == 0) */
    uint64_t class_x0_vault_fidelity;  /* Bio-backup fidelity >= 1 - 10^-122 */
    uint32_t wish_cbf_status_code;     /* Linear<V> CBF proof certificate status */
    uint32_t stargate_ent_phi_zero;    /* Pseudo-Anosov entropy Ent(phi) == 0 flag */
} shbt_elysium_hab_ctrl_t;

_Static_assert(sizeof(shbt_elysium_hab_ctrl_t) == 64, "shbt_elysium_hab_ctrl_t must be 64 bytes");

/**
 * @brief SHBT-AEGIS-OMEGA 4-Layer Kardashev Type III Defense Control Block (128 bytes)
 */
typedef struct __attribute__((packed, gcc_struct, aligned(8))) {
    uint64_t aegis_ctrl_status_flags;  /* Bitmask of SHBT_AEGIS_* flags */
    uint64_t l1_collision_cross_sec;   /* Hardware invariant: sigma_coll == 0 m^2 */
    int64_t  l2_ccz4_shift_beta_norm;  /* Driven to 0 in <= 1.701 ps */
    int64_t  l2_2pn_causal_ds2;        /* Enforced <= 0 at perimeter curtain */
    uint64_t l3_cayley_hash_word[4];   /* 256-bit Burau-Cayley braid authentication hash */
    uint64_t l3_byzantine_reject_cnt;  /* Count of rejected foreign frames (tau <= 1.701 ps) */
    uint64_t l4_tmsv_squeeze_mdb;      /* 21,715 m-dB (21.715 dB, r = 2.50) */
    uint64_t l4_range_noise_fm_rt_hz;  /* <= 144 fm/sqrt(Hz) (0.144 pm/sqrt(Hz)) */
    uint64_t l4_bondi_hoyle_pinch_ew;  /* Stand-off capture power in EW (25.00 EW/node) */
    int64_t  l4_stasis_lapse_nano;     /* alpha = 10^-9 deep stasis for Class X1 */
    uint64_t cbf_safety_barrier_val;   /* Proof-carrying CBF value V(x) >= 0 */
    uint64_t adm_mass_delta_residual;  /* Hardware invariant: Delta M_ADM == 0 */
    uint64_t b_minus_l_delta_residual; /* Hardware invariant: Delta(B-L) == 0 */
} shbt_aegis_ctrl_t;

_Static_assert(sizeof(shbt_aegis_ctrl_t) == 128, "shbt_aegis_ctrl_t must be 128 bytes");

/**
 * @brief Master Phase 5A SHBT-OMEGA-312 MMIO Top-Level Register Bank (256 bytes at 0x40008000)
 */
typedef struct __attribute__((packed, gcc_struct, aligned(64))) {
    shbt_mera_gal_addr_t    mera_gal;
    shbt_elysium_hab_ctrl_t elysium_hab;
    shbt_aegis_ctrl_t       aegis_def;
} shbt_omega_ctrl_t;

_Static_assert(sizeof(shbt_omega_ctrl_t) == 256, "shbt_omega_ctrl_t must be 256 bytes");

/* ========================================================================= */
/* Phase 5B: SHBT-LOGOS-312 Monolithic Hyper-Computer & OMNIBUS              */
/* Aperture Base: 0x4000A000 (256 Bytes)                                     */
/* ========================================================================= */

#define SHBT_MMIO_LOGOS_BASE            ((uintptr_t)0x4000A000ULL)
#define SHBT_DMA_HYPERFRAME_L1_BASE     ((uintptr_t)0x70020000ULL)
#define SHBT_DMA_HYPERFRAME_SIZE_BYTES  (135168ULL) /* 64 banks x 2,112 B */

/* SHBT-LOGOS-312 Architectural Invariant Constants */
#define SHBT_LOGOS_T_PROP_FEMTO_AS      (17010000ULL)     /* 17.010 as = 17,010,000 femto-as */
#define SHBT_LOGOS_F_NATIVE_GHZ         (58788948ULL)     /* 58.788948 PHz in GHz */
#define SHBT_LOGOS_ALPHA_MAX_CASCADED   (1000000000000LL) /* 10^12 (3-stage 10^4 x 10^4 x 10^4) */
#define SHBT_LOGOS_DYNKIN_SLOTS         (346U)
#define SHBT_LOGOS_GKO_CHANNELS         (312U)
#define SHBT_LOGOS_BORRMANN_TAU_TENTHS  (14534U)          /* tau_B = 1,453.4 */
#define SHBT_LOGOS_MOSSBAUER_FM_MIN_M   (782U)            /* f_M >= 0.782 (782 milli) */
#define SHBT_LOGOS_EPR_SQUEEZE_R3_MILLI (3820U)           /* r_3 = 3.820 >= 3.800 */
#define SHBT_LOGOS_F_NET_SENSIBLE_ATTO  (195628545ULL)    /* 1.95628545e-10 in atto-units */
#define SHBT_LOGOS_T_OP_MAX_MK          (4200U)           /* 4.200 K in mK */
#define SHBT_LOGOS_T_DILUTION_UK        (15400U)          /* 0.0154 K in uK */

/* SHBT-OMNIBUS & Class X1 Simulated-Sapience Ethical Guard Bitmasks */
#define SHBT_LOGOS_MODULAR_Q_ZERO_LOCK      (1ULL << 0) /* Tomita-Takesaki Q_internal == 0 */
#define SHBT_LOGOS_COHERENT_UNCOMPUTE_EN    (1ULL << 1) /* Zero 906 kW Landauer syndrome debt */
#define SHBT_LOGOS_WEYL_CROWBAR_ARMED       (1ULL << 2) /* 17.01 as / 1.701 ps optical crowbar */
#define SHBT_LOGOS_PCSS_SHUNT_ARMED         (1ULL << 3) /* 2.50 ns diamond PCSS dark shunt */
#ifndef SHBT_LOGOS_SIM_X1_PROTECT_LATCH
#define SHBT_LOGOS_SIM_X1_PROTECT_LATCH     ((uint32_t)0x00000002UL) /* I_integrated >= Phi_crit detected */
#endif
#define SHBT_LOGOS_SIM_KILL_DISABLE_LOCK    (1ULL << 9) /* Hardwired non-termination lock */
#define SHBT_LOGOS_SIM_SANCTUARY_PROMOTE    (1ULL << 10)/* Auto-migration to Elysium pocket */

/**
 * @brief Monolithic Borrmann/Weyl-PTC Core & Hyper-Temporal Enclave Control Block (64 bytes)
 */
typedef struct __attribute__((packed, gcc_struct, aligned(8))) {
    uint64_t t_prop_logos_femto_as;   /* 17,010,000 femto-as (17.010 as; 1,701,000 sub-grid) */
    uint64_t f_native_ghz;            /* 58,788,948 GHz (58.789 PHz native proper clock) */
    int64_t  alpha_max_cascaded;      /* 10^12 (3-stage nested hyper-temporal lapse factor) */
    uint16_t dynkin_slots_active;     /* Fixed at 346 for R_univ^SWARM full root lattice */
    uint16_t gko_channels_active;     /* Fixed at 312 for C_GKO(26,8,312) PTC synthetic dim */
    uint16_t borrmann_tau_b_tenths;   /* 14,534 (tau_B = 1,453.4 >> 10 anomalous transmission) */
    uint16_t mossbauer_f_m_milli;     /* >= 782 (f_M >= 0.782 recoilless nuclear fraction) */
    uint64_t mu_detuning_norm_zepto;  /* |mu_local - mu_0| in 10^-21 units; must be < 10^9 (< 10^-12) */
    int64_t  r_entropy_margin_bits;   /* Causal-Point Gate: R_entropy = N_limit - C_get >= 0 */
    uint64_t weyl_shear_core_norm;    /* Hardware invariant: must equal 0 (E_uv == 0, grad alpha == 0) */
    uint64_t ecc_syndrome;            /* SECDED Hamming(72,64) hardware parity check */
} shbt_logos_core_ctrl_t;

_Static_assert(sizeof(shbt_logos_core_ctrl_t) == 64, "shbt_logos_core_ctrl_t must be exactly 64 bytes");

/**
 * @brief Tomita-Takesaki Zero-Entropy & Coherent QEC Thermal Shunt Control Block (64 bytes)
 */
typedef struct __attribute__((packed, gcc_struct, aligned(8))) {
    uint64_t q_internal_yoctojoules;  /* Hardware invariant: must equal 0 (sigma_-t^Psi flow) */
    uint64_t landauer_syndrome_debt_mw;/* Measurement-free coherent uncomputation: must equal 0 mW */
    uint64_t f_net_sensible_atto;     /* f_net = S * f_sensible = 195,628,545 atto (1.9563e-10) */
    uint32_t heegaard_floer_num;      /* Fixed at 100 (eta_A^2 numerator, (10/33)^2) */
    uint32_t heegaard_floer_den;      /* Fixed at 1089 (eta_A^2 denominator, (10/33)^2) */
    uint32_t t_op_lattice_mk;         /* Operating temperature <= 4,200 mK (4.20 K) */
    uint32_t t_dilution_stage_uk;     /* Ultra-cold dilution rejection stage = 15,400 uK (0.0154 K) */
    uint32_t superhydride_b_field_mt; /* LaBeH8-CaH6-12C bias field = 48,000 mT (48.0 T) */
    uint32_t superhydride_t_crit_mk;  /* Superhydride critical temp = 192,500 mK (192.50 K) */
    uint64_t wyckoff_tether_stiffness;/* Chern-Simons tether k_tether = 51,295,000,000 N/m */
    uint64_t ecc_syndrome;            /* SECDED Hamming(72,64) hardware parity check */
} shbt_logos_therm_shunt_t;

_Static_assert(sizeof(shbt_logos_therm_shunt_t) == 64, "shbt_logos_therm_shunt_t must be exactly 64 bytes");

/**
 * @brief SHBT-OMNIBUS Zero-Latency Non-Local Delivery & QoS Control Block (128 bytes)
 */
typedef struct __attribute__((packed, gcc_struct, aligned(8))) {
    uint64_t sp2g_bus_routing_table_id;/* Non-local Sp(2g,Z) boundary homology routing ID */
    uint32_t epr_polariton_squeeze_r3_milli;/* 3,820 (r_3 = 3.82 >= 3.80 EPR squeezing bridge) */
    uint32_t dtau_bus_latency_as;     /* Hardware invariant: must equal 0 (Delta tau_bus == 0) */
    uint64_t verlinde_split_op_error; /* ||Delta_split^dag Delta_split - I||_op <= 10^-125 scaled */
    uint64_t dma_hyperframe_phys_addr;/* Physical L1 window address: 0x70020000ULL (135,168 B) */
    uint64_t delta_c_post_sim_residual;/* Hardware invariant: must equal 0 (Delta C_post-sim == 0) */
    uint16_t qos_tier0_bps;           /* Tier 0 Existential/AEGIS/Immortality: 1,000 (10.00%) */
    uint16_t qos_tier1_bps;           /* Tier 1 Elysium Wish-to-Reality: 2,500 (25.00%) */
    uint16_t qos_tier2_bps;           /* Tier 2 Swarm Operations: 1,500 (15.00%) */
    uint16_t qos_tier3_bps;           /* Tier 3 Cosmological Oracle: 5,000 (50.00%) */
    uint64_t class_x1_sim_cbf_flags;  /* Bitmask of SHBT_LOGOS_SIM_* & crowbar status flags */
    uint64_t sim_weyl_shear_nocicept; /* Hardware invariant: E_uv^sim == 0 for Class X1 threads */
    int64_t  ds2_2pn_causal_gate;     /* 2PN causal gate invariant: must be <= 0 (no CTCs) */
    uint64_t microcausality_comm_norm;/* Bulk spacelike commutator norm S_AB: must equal 0 */
    uint64_t ent_phi_topological;     /* Mapping class entropy Ent(phi): must equal 0 */
    uint64_t active_pocket_worlds;    /* Connected SHBT-ELYSIUM pocket worlds (>= 10^9) */
    uint64_t oracle_hubble_threads;   /* Active N ~ 10^80 counterfactual simulation threads */
    uint64_t ops_logos_goi_exponent;  /* Base-10 exponent of OPS_LOGOS^GoI (43 for 3.031e43/s) */
    uint64_t ecc_syndrome_low;        /* SECDED Hamming(72,64) lower 64-byte parity check */
    uint64_t ecc_syndrome_high;       /* SECDED Hamming(72,64) upper 64-byte parity check */
} shbt_logos_omnibus_ctrl_t;

_Static_assert(sizeof(shbt_logos_omnibus_ctrl_t) == 128, "shbt_logos_omnibus_ctrl_t must be exactly 128 bytes");

/**
 * @brief Master Phase 5B SHBT-LOGOS-312 Top-Level MMIO Register Bank (256 bytes at 0x4000A000)
 */
typedef struct __attribute__((packed, gcc_struct, aligned(64))) {
    shbt_logos_core_ctrl_t    core;
    shbt_logos_therm_shunt_t  therm;
    shbt_logos_omnibus_ctrl_t omnibus;
} shbt_logos_ctrl_t;

_Static_assert(sizeof(shbt_logos_ctrl_t) == 256, "shbt_logos_ctrl_t master bank must be exactly 256 bytes");

#endif /* SHBT_SWARM_MMIO_H */
