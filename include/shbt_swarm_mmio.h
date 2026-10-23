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

#define SHBT_SWARM_MMIO_BASE_ADDR       0x70000000UL
#define SHBT_SWARM_HYPERFRAME_BASE_ADDR 0x70020000UL

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
#define SHBT_SWARM_CMD_EXEC_UNIV_SYNTH          0x00002000U /* Phase 3: R_univ^SWARM arbitrary matter */
#define SHBT_SWARM_CMD_ZERO_COMP_DIRECT_RENDER  0x00004000U /* Phase 3: Delta C_post-sim = 0 render */
#define SHBT_SWARM_CMD_MERA_DAG_INSTANCE        0x00008000U /* Phase 3: Sp(2g,Z) MERA-DAG tiling */
#define SHBT_SWARM_CMD_MAR312X_FORGE_TIER4      0x00010000U /* Phase 4: Engage 25.00 EW Tier 4 Forge */
#define SHBT_SWARM_CMD_DIRECT_BOUNDARY_SYNTH    0x00020000U /* Phase 4: B_direct^MAR-X (99.85% CFT bus) */
#define SHBT_SWARM_CMD_AUTOPOIETIC_SELF_HEAL    0x00040000U /* Phase 4: A_heal 50.518 kHz ROM scrub */
#define SHBT_SWARM_CMD_WEYL_EXCITON_CROWBAR     0x80000000U /* Phase 4: 1.701 ps Weyl-Exciton abort */
#define SHBT_SWARM_CMD_PCSS_CROWBAR             0x80000000U /* Legacy alias for hardware crowbar */

/* Phase 1 + Phase 2 + Phase 3 + Phase 4 Fault Bitmasks for fault_mask_reg (0x10) */
#define SHBT_SWARM_FAULT_NONE                           0x00000000U
#define SHBT_SWARM_FAULT_ADM_IMBALANCE                  0x00000001U /* M_feedstock != M_payload */
#define SHBT_SWARM_FAULT_PARTITION_LOCK                 0x00000002U /* eta_A : eta_D != 10 : 23 */
#define SHBT_SWARM_FAULT_FRAMING_WEYL                   0x00000004U /* Delta_fr != 0 or E_munu != 0 */
#define SHBT_SWARM_FAULT_BL_ANOMALY                     0x00000008U /* Delta(B-L)_tot != 0 */
#define SHBT_SWARM_FAULT_NEUTRINO_LEAK                  0x00000010U /* Phi_nu > 0 */
#define SHBT_SWARM_FAULT_SECDED_UNCORR                  0x00000020U /* Double-bit flip in B_3 word */
#define SHBT_SWARM_FAULT_CAYLEY_HASH                    0x00000040U /* Phylogenetic quine / Byzantine mismatch */
#define SHBT_SWARM_FAULT_THERMAL_CLAMP                  0x00000080U /* T_QC > 4.2 K or T_MAR > 192.50 K */
#define SHBT_SWARM_FAULT_CBF_PHYS_VIOL                  0x00000100U /* Phase 2: h_phys(x) < 0 barrier breach */
#define SHBT_SWARM_FAULT_GREY_GOO_CAP                   0x00000200U /* Phase 2: M(t) > M_max population cap */
#define SHBT_SWARM_FAULT_CLF_DIVERGENCE                 0x00000400U /* Phase 2: dV_terra/dt > -kappa * V_terra */
#define SHBT_SWARM_FAULT_X0_PROTECT_LCK                 0x00000800U /* Phase 2: Attempted de-render of Class X0 */
#define SHBT_SWARM_FAULT_X1_SAPIENT_LCK                 0x00001000U /* Phase 2: Attempted de-render of Class X1 */
#define SHBT_SWARM_FAULT_SUGAWARA_CAP_EXCEEDED          0x00002000U /* Phase 3: |Delta h_vis| > 38.196601 nats */
#define SHBT_SWARM_FAULT_DRIPLINE_UNSTABLE              0x00004000U /* Phase 3: Non-integrable / unbound nucleus */
#define SHBT_SWARM_FAULT_SIM_TO_RENDER_HASH_MISMATCH    0x00008000U /* Phase 3: MERA-PINN UV != Synth Frame hash */
#define SHBT_SWARM_FAULT_HABITAT_GRAV_DEVIATION         0x00010000U /* Phase 3: |g_hab - 9.80665 m/s^2| > tol */
#define SHBT_SWARM_FAULT_SUPERHYDRIDE_TETHER_LOSS       0x00020000U /* Phase 4: |delta x| > 10^-4 A or B < 48 T */
#define SHBT_SWARM_FAULT_PURCELL_BANDGAP_DRIFT          0x00040000U /* Phase 4: rho_DOS(w_spont) > 0 or F_P > 0 */
#define SHBT_SWARM_FAULT_DECX_CHERN_BACKSCATTER         0x00080000U /* Phase 4: C_2 == 0 or eta_DEC_X < 99.20% */
#define SHBT_SWARM_FAULT_FORGE_SHEAR_EXCEEDED           0x00100000U /* Phase 4: sigma_max > 4,800.00 MPa */

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

_Static_assert(sizeof(shbt_unified_stinespring_hyperframe_t) == 135168,
               "UnifiedStinespringHyperFrame must be exactly 135,168 bytes");

/* 128-Byte Dual-Cacheline SHBT-MMIO-SWARM v4 Register Layout at 0x70000000 */
#pragma pack(push, 1)
typedef struct __attribute__((packed, gcc_struct, aligned(64))) {
    /* === CACHELINE 0 (Offsets 0x00 .. 0x3F): Control, Quine & Optical GoI === */
    volatile uint32_t magic_id;              /* 0x00: 0x5357524D ('SWRM') */
    volatile uint32_t abi_version;           /* 0x04: 0x00040312 (v4.312 SHBT-MAR-312X Exotic) */
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

#endif /* SHBT_SWARM_MMIO_H */
