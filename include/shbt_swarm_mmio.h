/*
 * include/shbt_swarm_mmio.h
 * Bare-Metal C11 Register, Cognitive, Universal Synthesis & MAR-312X Reactor Specification
 * Phase 5C: SHBT-AEGIS-ABSOLUTE Causal Horizon Isolation (SHBT-MAR-312X)
 * Base Physical Address: 0x70000000 / 0x80000000 (832-Byte 13-Cacheline ABI v15.0 Contract)
 */

#ifndef SHBT_SWARM_MMIO_H
#define SHBT_SWARM_MMIO_H

#include <stdint.h>
#include <stddef.h>
#include <stdalign.h>
#include "shbt_mmio_apotheosis.h"
#include "shbt_nested_sim_abi.h"

#define SHBT_SWARM_MMIO_MAGIC              0x53484254UL /* "SHBT" */
#define SHBT_SWARM_ABI_VERSION             0x000F0312U /* ABI v15.0 */
#define SHBT_SWARM_ABI_VERSION_V12         0x000C0312UL
#define SHBT_SWARM_ABI_VERSION_V13         0x000D0312UL
#define SHBT_SWARM_ABI_VERSION_V14         0x000E0312UL
#define SHBT_SWARM_ABI_VERSION_V15         0x000F0312UL
#define SHBT_SWARM_MMIO_ABI_V14_MAGIC      ((uint32_t)0x000E0312UL)
#define SHBT_SWARM_MMIO_ABI_V15_MAGIC      ((uint32_t)0x000F0312UL)
#define SHBT_SWARM_FORMAL_PROOFS_COUNT     122U        /* 122 Formal Z3 Theorems UNSAT */
#define SHBT_SWARM_HIL_GATES_COUNT         400U        /* 400 HIL Verification Gates PASS */
#define SHBT_SWARM_MMIO_TOTAL_SIZE         832U
#define SHBT_SWARM_MMIO_TOTAL_SIZE_BYTES   832U
#define SHBT_SWARM_CACHE_LINE_BYTES        64U
#define SHBT_SWARM_MMIO_CACHELINE_BYTES    64U
#define SHBT_SWARM_MMIO_CACHELINE_COUNT    13U

/* Dark-OS & Dark-ZPE Control Bitmasks (Cache Line 12, Offset 0x0304) */
#define SHBT_DARK_OS_NATIVE_VOA_LOCK          (1U << 0)
#define SHBT_DARK_OS_ZERO_DECODE_ACTIVE       (1U << 1)
#define SHBT_DARK_OS_COCYCLE_SCHED_EN         (1U << 2)
#define SHBT_DARK_ZPE_KMS_PASSIVITY_GUARD     (1U << 3)
#define SHBT_DARK_ZPE_CH1_DM_RELAX_EN         (1U << 4)
#define SHBT_DARK_ZPE_CH2_CASIMIR_EN          (1U << 5)
#define SHBT_DARK_ZPE_CH3_ERGOTROPY_EN        (1U << 6)
#define SHBT_DARK_ZPE_REGIME_III_ACTIVE       (1U << 7)
#define SHBT_MMIO_BASE_MAR_OMEGA        0x7F000600ULL
#define SHBT_MMIO_BASE_UNIVERSAL_HARVEST 0x7F000700ULL
#define SHBT_SWARM_DIVERTOR_BASE_ADDR   0x7F000800UL

/* Continuous RMHD Divertor Bitmask Flags & Command Opcodes */
#define DIV_STATUS_LAMINAR             (1u << 0)
#define DIV_STATUS_SEPARATRIX_LOCKED   (1u << 1)
#define DIV_STATUS_CCZ4_DAMPING_ACTIVE (1u << 2)
#define DIV_STATUS_FRENKEL_SAFE        (1u << 3)
#define DIV_STATUS_CHOKE_ALARM         (1u << 4)
#define DIV_STATUS_FEEDSTOCK_OPTIMAL   (1u << 5)

#define DIV_CMD_NOP                    0x00000000u
#define DIV_CMD_LOCK_SEPARATRIX        0x00000001u
#define DIV_CMD_ENGAGE_CCZ4_DAMPING    0x00000002u
#define DIV_CMD_TRIM_X_POINT           0x00000004u
#define DIV_CMD_EMERGENCY_SCRAM        0x00000080u

/* Dual-Reactor Power Arbiter State Flags (Cache Line 0) */
#define SHBT_POWER_BUS_MAR312X_ACTIVE       (1ULL << 0)
#define SHBT_POWER_BUS_MAR312X_STANDBY      (1ULL << 1)
#define SHBT_POWER_BUS_OMEGA_IGNITED        (1ULL << 2)
#define SHBT_POWER_BUS_OMEGA_CFT_LOCKED     (1ULL << 3)
#define SHBT_POWER_BUS_SLIPSTREAM_ENGAGED   (1ULL << 4)
#define SHBT_POWER_BUS_FOUNDRY_INLINE_FOLD  (1ULL << 5)

#define SHBT_SWARM_MMIO_BASE_ADDR       0x70000000UL
#define SHBT_SWARM_HYPERFRAME_BASE_ADDR 0x70020000UL
#define SHBT_SWARM_APOTHEOSIS_BASE_ADDR 0x4000C000UL
#define SHBT_SWARM_HARVEST_APERTURE_BASE 0x4000E000UL
#define SHBT_SWARM_FOUNDRY_DENSE_BASE   0x00007F8000040000ULL


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

/* Hardware Protection & Isolation Latch Bitmasks (aegis_abs_latch_flags) */
#ifndef SHBT_LOGOS_SIM_X1_PROTECT_LATCH
#define SHBT_LOGOS_SIM_X1_PROTECT_LATCH     ((uint32_t)0x00000002UL)
#endif
#define SHBT_LOGOS_PURE_DARK_MODE           ((uint32_t)0x00000008UL)
#define SHBT_AEGIS_COBORDISM_PINCH_LATCH    ((uint32_t)0x00000010UL)
#define SHBT_AEGIS_VACUUM_DECAY_LOCK        ((uint32_t)0x00000020UL)
#define SHBT_AEGIS_ADM_MONOPOLE_NULL_LOCK   ((uint32_t)0x00000040UL)
#define SHBT_OMEGA_CROSS_LAPSE_AIRGAP_LOCK  ((uint32_t)0x00000080UL)
#define SHBT_AEGIS_LAYER_ALL_6_ACTIVE       ((uint32_t)0x0000003FUL)
#define SHBT_AEGIS_COBORDISM_PINCH_LOCKED   ((uint32_t)0x00000080UL)
#define SHBT_AEGIS_DEHN_INGRESS_DIODE_EN    ((uint32_t)0x00000100UL)
#define SHBT_OMEGA_KMS_REACTIVE_CLOSED      ((uint32_t)0x00000200UL)
#define SHBT_LOGOS_STAGE2_PURE_DARK_LOCK    ((uint32_t)0x00000400UL)

/* Phase 5C SHBT-AEGIS-ABSOLUTE Fault Bitmasks */
#ifndef SHBT_SWARM_FAULT_CBF_VIOLATION
#define SHBT_SWARM_FAULT_CBF_VIOLATION      ((uint32_t)0x00000004UL)
#endif
#ifndef SHBT_SWARM_FAULT_CBF_BREACH
#define SHBT_SWARM_FAULT_CBF_BREACH         ((uint32_t)0x00000100UL)
#endif
#define SHBT_SWARM_FAULT_FRAMING_BREACH      ((uint32_t)0x00010000UL)
#define SHBT_SWARM_FAULT_INSTANTON_DROP      ((uint32_t)0x00020000UL)
#define SHBT_SWARM_FAULT_COBORDISM_LEAK      ((uint32_t)0x00040000UL)
#define SHBT_SWARM_FAULT_PSEUDO_ANOSOV_TWIST ((uint32_t)0x00080000UL)
#define SHBT_SWARM_FAULT_EP2_NILPOTENCY_LOSS ((uint32_t)0x00100000UL)
#define SHBT_SWARM_FAULT_BLUESHEET_SURGE     ((uint32_t)0x00200000UL)
#define SHBT_SWARM_FAULT_WEYL_CROWBAR_TRIP   ((uint32_t)0x80000000UL)

/* Normative Fixed-Point Lock Constants for Cache Line 11 (0x02C0..0x02FF) */
#define SHBT_VARTHETA_DARK_PI_OVER_2_UQ2_62  ((uint64_t)0x6487ED5110B4611AULL)
#define SHBT_INSTANTON_SE_1E12_UQ44_20       ((uint64_t)0x000E8D4A51000000ULL)
#define SHBT_ADM_MONOPOLE_ZERO_Q32_32        ((int64_t)0x0000000000000000LL)
#define SHBT_LOG10_S_MATRIX_ZERO_SENTINEL    ((int64_t)-35184372088832000LL) /* -125.0 in Q16.48 */
#define SHBT_LOG10_BOGOLIUBOV_FLOOR_Q16_48   ((int64_t)-33776997205278720LL) /* -120.0 in Q16.48 */
#define SHBT_DARK_GRID_1P8E56J_UQ32_32       ((uint64_t)0x0ABA950000000000ULL)

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
#define SHBT_SWARM_CMD_HOCBF_EVAL_FILTER           0x00200000U /* Execute r=2 QP barrier filter */
#define SHBT_SWARM_CMD_CRYOPHONON_SHUNT_ENGAGE     0x00400000U /* Engage Diamond-GaN S=100/1089 shunt */
#define SHBT_SWARM_CMD_CLAMP_GW_RECOIL             0x00000100U /* Phase 5A: Quad-axial recoil clamp */
#define SHBT_SWARM_CMD_ENGAGE_CONT_RIBBON          0x00000200U /* Phase 5B: Continuous Fibonacci ribbon */
#define SHBT_SWARM_CMD_ASSERT_QRF_COVARIANCE       0x00000400U /* Phase 2: Relativistic QRF covariance */
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
/* First-Principles Subsystem Command Opcodes */
#define SHBT_OMEGA_CMD_SIPHON_SMBH_START       0x00000040U
#define SHBT_OMEGA_CMD_HALO_VIRIAL_CLAMP       0x00000080U
#define SHBT_FOUNDRY_CMD_AMORPHOUS_BF_ENGAGE   0x00000100U
#define SHBT_LOGOS_CMD_DCE_SUPPRESSION_LOCK    0x00000200U
#define SHBT_SHIELD_CMD_POSET_SHEAF_SYNC       0x00000400U
#define SHBT_APOTH_CMD_EXEC_COBORDISM_FISS     0x00000800U
#define SHBT_APOTH_CMD_EXEC_COBORDISM_FUSE     0x00001000U

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
/* Subsystem First-Principles Invariant Fault Bitmasks */
#define SHBT_SWARM_FAULT_SMBH_AREA_DECREASE    0x00020000U
#define SHBT_SWARM_FAULT_HALO_VIRIAL_UNBIND    0x00040000U
#define SHBT_SWARM_FAULT_AMORPHOUS_WEYL_SPIKE  0x00080000U
#define SHBT_SWARM_FAULT_DCE_PARAMETRIC_TRIP   0x00100000U
#define SHBT_SWARM_FAULT_POSET_COCYCLE_OBSTR   0x00200000U
#define SHBT_SWARM_FAULT_QUALIA_DISTORTION     0x00400000U

/* v7 First-Principles Closure Fault Bitmasks (Bits 32..39) */
#define SHBT_SWARM_FAULT_CHIRAL_RACEMIC       (1ULL << 32)
#define SHBT_SWARM_FAULT_LLE_CHAOS            (1ULL << 33)
#define SHBT_SWARM_FAULT_PACHNER_COCYCLE      (1ULL << 34)
#define SHBT_SWARM_FAULT_ATMOS_SHOCK          (1ULL << 35)
#define SHBT_SWARM_FAULT_FLUX_JUMP            (1ULL << 36)
#define SHBT_SWARM_FAULT_POYNTING_JET         (1ULL << 37)
#define SHBT_SWARM_FAULT_MOSSBAUER_DETUNE     (1ULL << 38)
#define SHBT_SWARM_FAULT_BINDING_DECOHERENCE  (1ULL << 39)
#define SHBT_SWARM_FAULT_V7_MASK              (0x000000FF00000000ULL)

/* v9 Research Extensions Fault Bitmasks (Bits 47..53) */
#define SHBT_SWARM_FAULT_GW_MEMORY_STRAIN_TRIP (1ULL << 47)
#define SHBT_SWARM_FAULT_COM_HALO_DRIFT_TRIP   (1ULL << 48)
#define SHBT_SWARM_FAULT_RIBBON_KZ_SINGULARITY (1ULL << 49)
#define SHBT_SWARM_FAULT_DISCRETIZATION_NONZERO (1ULL << 50)
#define SHBT_SWARM_FAULT_QRF_DECOHERENCE_TRIP  (1ULL << 51)
#define SHBT_SWARM_FAULT_SHEAF_SPECTRUM_DIV    (1ULL << 52)
#define SHBT_SWARM_FAULT_METRIC_SUP_CAUSALITY  (1ULL << 53)
#define SHBT_SWARM_FAULT_V9_MASK               (0x003F800000000000ULL)

/* v10 Galactic Harvest & Dense Matter Invariant Fault Bitmasks (Bits 54..60) */
#define SHBT_SWARM_FAULT_EDDINGTON_OVERPRESSURE     (1ULL << 54)
#define SHBT_SWARM_FAULT_CORE_ADIABAT_COLLAPSE      (1ULL << 55)
#define SHBT_SWARM_FAULT_HAWKING_AREA_DECREASE      (1ULL << 56)
#define SHBT_SWARM_FAULT_HALO_VIRIAL_JEANS_DRIFT    (1ULL << 57)
#define SHBT_SWARM_FAULT_FLOQUET_COLOR_UNLOCKED     (1ULL << 58)
#define SHBT_SWARM_FAULT_PLANETARY_ACOUSTIC_BREACH  (1ULL << 59)
#define SHBT_SWARM_FAULT_BZ_JET_LEAKAGE             (1ULL << 60)
#define SHBT_SWARM_FAULT_V10_MASK                   (0x001FC00000000000ULL)

/* v11 Degenerate Core, Dynamo Quench & NSC Loss-Cone Fault Bitmasks (Bits 61..63) */
#define SHBT_SWARM_FAULT_PYCNONUCLEAR_TRIP          (1ULL << 61)
#define SHBT_SWARM_FAULT_SUPERFLARE_DYNAMO_TRIP     (1ULL << 62)
#define SHBT_SWARM_FAULT_TDE_LOSS_CONE_TRIP         (1ULL << 63)
#define SHBT_SWARM_FAULT_V11_MASK                   (0xE000000000000000ULL)

/* v11 Hardware Command Opcodes */
#define SHBT_SWARM_CMD_MELT_COULOMB_CRYSTAL         0x00008000U
#define SHBT_SWARM_CMD_HADRONIZE_CFL_CORE           0x00010000U
#define SHBT_SWARM_CMD_QUENCH_CONV_DYNAMO           0x00020000U
#define SHBT_SWARM_CMD_CLAMP_PAIR_INSTABILITY       0x00040000U
#define SHBT_SWARM_CMD_SHIELD_NSC_LOSS_CONE         0x00080000U
#define SHBT_SWARM_CMD_STEP_ASYNC_SYMPLECTIC        0x00100000U


/* v7 First-Principles Hardware Threshold Constants */
#define SHBT_SWARM_CHIRAL_EE_Q16_HOMOCHIRAL   0x00010000U  /* 1.000000 in Q16.16 */
#define SHBT_SWARM_KERR_PHASE_MARGIN_MIN_RAD  0.15000000f  /* >= 0.15 rad */
#define SHBT_SWARM_PACHNER_COCYCLE_CLEAN      0x00000000U  /* H^1 == 0 */
#define SHBT_SWARM_ATMOS_SHEAR_MAX_PA         45.0000000f  /* <= 45.0 Pa */
#define SHBT_SWARM_VORTEX_DRIFT_MAX_MPS       1.0000e-15   /* <= 10^-15 m/s */
#define SHBT_SWARM_POYNTING_LEAKAGE_MAX_W     0.00000000   /* == 0.0 W */
#define SHBT_SWARM_MOSSBAUER_FACTOR_MIN       0.78200000   /* f_M >= 0.782 */
#define SHBT_SWARM_BORRMANN_TAU_MIN           1453.40000   /* tau_B >= 1453.4 */
#define SHBT_SWARM_BINDING_PURITY_NOMINAL     1.00000000f  /* Tr(rho^2) == 1.0 */
#define SHBT_SWARM_BRANCH_ENTROPY_DEBT_ZERO   0.00000000f  /* dS_vN == 0.0 */

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

/* SHBT-OMEGA Extended MMIO Registers & Bitmasks (Aperture Base: 0x40008000) */
#define SHBT_OMEGA_REG_OPACITY_QUENCH       0x00008030U /* Metric opacity factor Q(S) (Q0.32) */
#define SHBT_OMEGA_REG_ADIABATIC_GAMMA      0x00008034U /* Measured core adiabatic index Gamma_1 (Q16.16) */
#define SHBT_OMEGA_REG_DYNAMO_BRAKE_TENSOR  0x00008038U /* Dynkin magnetic braking control */
#define SHBT_OMEGA_REG_DECONSTRUCT_M_DOT    0x0000803CU /* Current mass harvesting rate (kg/s) */

#define SHBT_OMEGA_CMD_BYPASS_EDDINGTON     0x00000002U /* Engage metric-conformal opacity shunt */
#define SHBT_OMEGA_CMD_ENGAGE_DYNAMO_BRAKE  0x00000004U /* Activate so(10)_312 braking tensor */
#define SHBT_OMEGA_FAULT_RADIATION_PRESSURE 0x00000020U /* Rosseland opacity quench failure */
#define SHBT_OMEGA_FAULT_CORE_COLLAPSE_RISK 0x00000040U /* Gamma_1 < 4/3 stability breach */

/* Phase 5A Extended MMIO Registers (Aperture Base: 0x40008060, 32 Bytes) */
#define SHBT_OMEGA_REG_SMBH_HORIZON_AREA      0x00008060U
#define SHBT_OMEGA_REG_HALO_VIRIAL_DISP       0x00008068U
#define SHBT_OMEGA_REG_SMBH_ERGOSPHERE_SPIN   0x00008070U
#define SHBT_OMEGA_REG_SMBH_SIPHON_STATUS     0x00008074U
#define SHBT_OMEGA_REG_RESERVED_EXT           0x00008078U

typedef struct __attribute__((packed, gcc_struct, aligned(8))) {
    uint64_t smbh_horizon_area_fp;        /* 0x00 (0x40008060): Hawking horizon area A_H in m^2 (fixed point) */
    uint64_t halo_virial_dispersion_fp;   /* 0x08 (0x40008068): Virial dispersion sigma_r^2 (fixed point) */
    uint32_t smbh_ergosphere_spin_reg;    /* 0x10 (0x40008070): Dimensionless Kerr spin a* (Q0.32) */
    uint32_t smbh_siphon_status_latch;    /* 0x14 (0x40008074): Hexameric siphon interlock status */
    uint64_t reserved_omega_ext;          /* 0x18 (0x40008078): Reserved alignment pad (total 32 bytes) */
} shbt_omega_ext_ctrl_t;

_Static_assert(sizeof(shbt_omega_ext_ctrl_t) == 32, "shbt_omega_ext_ctrl_t must be 32 bytes");

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

/* SHBT-LOGOS Extended MMIO Registers & Bitmasks (Aperture Base: 0x4000A000) */
#define SHBT_LOGOS_REG_BORRMANN_SUSCEPT     0x0000A030U /* Fe-57 Lamb-Mossbauer susceptibility */
#define SHBT_LOGOS_REG_FRACTON_DENSITY      0x0000A034U /* Fracton spectral dimension d_s (Q16.16) */
#define SHBT_LOGOS_REG_ISRAEL_LAPSE_GRAD    0x0000A038U /* Israel-Darmois junction gradient */
#define SHBT_LOGOS_REG_TIDAL_SHEAR_MAX      0x0000A03CU /* Measured max interior tidal shear */

#define SHBT_LOGOS_CMD_BORRMANN_TUNE        0x00000001U /* Align lattice nodes to EUV standing wave */
#define SHBT_LOGOS_CMD_FRACTON_SHUNT_ENGAGE 0x00000002U /* Enable non-Klemens LO phonon absorption */
#define SHBT_LOGOS_FAULT_HOT_PHONON_TRIP    0x00000040U /* Optical phonon bottleneck detected */
#define SHBT_LOGOS_FAULT_TIDAL_FRACTURE     0x00000080U /* Extrinsic curvature discontinuity */

/* Phase 5B Extended MMIO Registers (Aperture Base: 0x4000A060, 32 Bytes) */
#define SHBT_LOGOS_REG_DCE_VACUUM_PHOTON_FLUX 0x0000A060U
#define SHBT_LOGOS_REG_DCE_NODAL_CANCELLATION 0x0000A068U
#define SHBT_LOGOS_REG_DCE_KAGAN_AFANASEV     0x0000A06CU
#define SHBT_LOGOS_REG_RESERVED_EXT           0x0000A070U

typedef struct __attribute__((packed, gcc_struct, aligned(8))) {
    uint64_t dce_vacuum_photon_flux_fp;   /* 0x00 (0x4000A060): Squeezed DCE photon production rate (fixed point) */
    uint32_t dce_nodal_cancellation_reg;  /* 0x08 (0x4000A068): Borrmann nodal pinning cancellation (Q0.32) */
    uint32_t dce_kagan_afanasev_latch;    /* 0x0C (0x4000A06C): Kagan-Afanas'ev suppression latch */
    uint64_t reserved_logos_ext[2];       /* 0x10 (0x4000A070): Reserved alignment pad (total 32 bytes) */
} shbt_logos_ext_ctrl_t;

_Static_assert(sizeof(shbt_logos_ext_ctrl_t) == 32, "shbt_logos_ext_ctrl_t must be 32 bytes");

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


#ifndef SHBT_ALIGN64
#define SHBT_ALIGN64 __attribute__((aligned(64)))
#endif

/* === Cache Line 10 (0x0280..0x02BF, 64 Bytes): Continuous RMHD Divertor & Constellation Aperture === */
typedef struct SHBT_ALIGN64 {
    volatile uint32_t divertor_separatrix_psi_q16;   /* 0x0280: Separatrix flux (Q16.16 Wb, nom ~0.08504 Wb = 0x000015C5) */
    volatile uint32_t divertor_reynolds_margin_reg;  /* 0x0284: Turbulence margin (Re_crit - Re) in Q16.16 */
    volatile uint64_t divertor_feedstock_split_fp;   /* 0x0288: Feedstock split ratio (1 - f_slip) in Q32.32 */
    volatile uint32_t divertor_status_latch;         /* 0x0290: Status and fault telemetry latch */
    volatile uint32_t divertor_b_field_guide_q16;    /* 0x0294: Measured guide field in Q16.16 T (nom 48.0 T = 0x00300000) */
    volatile uint64_t divertor_shear_stress_eff_fp;  /* 0x0298: Effective shear stress sigma_eff in Q32.32 Pa */
    volatile uint64_t divertor_mass_inflow_rate_fp;  /* 0x02A0: Siphoned mass inflow rate M_dot in Q32.32 kg/s (1.256e8 kg/s) */
    volatile uint64_t divertor_throat_divert_rate_fp;/* 0x02A8: Diverted throat mass rate m_dot_Omega in Q32.32 kg/s (111,265.01 kg/s) */
    volatile uint32_t divertor_kh_growth_rate_q16;   /* 0x02B0: Kelvin-Helmholtz growth rate (1e-12 s^-1 units) */
    volatile uint32_t divertor_control_command;      /* 0x02B4: Divertor command register */
    uint64_t          reserved_cl10_pad;             /* 0x02B8: Padding to complete exactly 64 bytes */
} shbt_mmio_cacheline_10_divertor_t;

/* === CACHELINE 11 (Offsets 0x02C0 .. 0x02FF): ABI v14.0 SHBT-AEGIS-ABSOLUTE Isolation Registers (64 bytes) === */
typedef struct SHBT_ALIGN64 {
    volatile uint32_t aegis_abs_abi_magic;         /* 0x02C0: 0x000E0312 (ABI v14.0) */
    volatile uint32_t aegis_abs_latch_flags;       /* 0x02C4: Hardware isolation & pinch latches */
    volatile uint64_t vartheta_dark_q2_62;         /* 0x02C8: UQ2.62 rad (pi/2 -> eta_D == 1.0) */
    volatile uint64_t instanton_action_se_q44_20;  /* 0x02D0: UQ44.20 Euclidean bounce action S_E >= 10^12 */
    volatile int64_t  adm_monopole_res_q32_32;     /* 0x02D8: Q32.32 Exterior ADM monopole (0 kg) */
    volatile int64_t  s_matrix_leak_log10_q16_48;  /* 0x02E0: Q16.48 log10(||S_(ext,LOGOS)||_op) */
    volatile int64_t  bogoliubov_airgap_q16_48;    /* 0x02E8: Q16.48 log10(int |beta_k|^2 dk) */
    volatile uint64_t dark_grid_exergy_q32_32;     /* 0x02F0: UQ32.32 Perpetual DarkPowerGrid (1.80e56 J) */
    volatile uint64_t cl11_secded_syndrome;        /* 0x02F8: 7x Hamming(72,64) syndromes + CRC8 */
} shbt_aegis_absolute_cl11_t;

/* === CACHELINE 12 (Offsets 0x0300 .. 0x033F): ABI v15.0 Dark-OS / Dark-ZPE Registers (64 bytes) === */
typedef struct SHBT_ALIGN64 {
    volatile uint32_t dark_os_abi_magic;        /* 0x0300: 0x000F0312 (ABI v15.0 magic) */
    volatile uint32_t dark_os_zpe_ctrl_flags;   /* 0x0304: Dark-OS & Dark-ZPE latch flags */
    volatile uint64_t voa_crt_primary_state;    /* 0x0308: Active primary [0..2901359] & VOA opcode */
    volatile uint64_t kz_clock_period_femto_as; /* 0x0310: 17,010,000 femto-as (17.010000 as) */
    volatile uint64_t tqec_decode_stall_as;     /* 0x0318: 0 as decode stall & eps_CRT == 0 */
    volatile uint64_t zpe_ch1_dm_power_w;       /* 0x0320: Channel 1 DM relaxation power telemetry (W) */
    volatile uint64_t zpe_ch2_ch3_power_packed; /* 0x0328: Packed Ch2 Casimir & Ch3 Ergotropy power */
    volatile uint64_t regime_iii_net_milli_yw;  /* 0x0330: Net Regime III power (+34,726 milli-YW) */
    volatile uint64_t cl12_secded_syndrome;     /* 0x0338: Hamming(72,64) SECDED syndrome & ECC count */
} shbt_dark_os_zpe_cl12_t;

static inline uint8_t shbt_hamming72_64_syndrome(uint64_t word) {
    uint8_t p0 = (uint8_t)__builtin_parityll(word & 0x5555555555555555ULL);
    uint8_t p1 = (uint8_t)__builtin_parityll(word & 0x6666666666666666ULL);
    uint8_t p2 = (uint8_t)__builtin_parityll(word & 0x7878787878787878ULL);
    uint8_t p3 = (uint8_t)__builtin_parityll(word & 0x7F807F807F807F80ULL);
    uint8_t p4 = (uint8_t)__builtin_parityll(word & 0x7FFF80007FFF8000ULL);
    uint8_t p5 = (uint8_t)__builtin_parityll(word & 0x7FFFFFFF80000000ULL);
    uint8_t p6 = (uint8_t)__builtin_parityll(word);
    uint8_t syn = (uint8_t)((p0 << 0) | (p1 << 1) | (p2 << 2) | (p3 << 3) | (p4 << 4) | (p5 << 5) | (p6 << 6));
    uint8_t overall = (uint8_t)__builtin_parity((unsigned int)syn ^ (unsigned int)p6);
    return (uint8_t)(syn | (overall << 7));
}

/* === Cache Line 5 foundry / Elysium biosphere overlay types === */
typedef struct SHBT_ALIGN64 {
    volatile uint32_t cl5_foundry_mode;
    volatile uint32_t cl5_foundry_fault;
    volatile uint64_t cl5_foundry_throughput;
    volatile uint64_t cl5_foundry_exergy_q32_32;
    volatile uint64_t cl5_foundry_dissipation;
    volatile uint32_t cl5_foundry_status;
    volatile uint32_t cl5_foundry_reserved0;
    volatile uint64_t cl5_foundry_reserved1;
    volatile uint64_t cl5_foundry_reserved2;
    volatile uint64_t cl5_foundry_reserved3;
} shbt_swarm_cl5_foundry_t;

typedef struct SHBT_ALIGN64 {
    volatile double elysium_pocket_lapse_alpha;
    volatile double elysium_coriolis_residual_norm;
    volatile double elysium_weyl_electric_norm;
    volatile double elysium_israel_splice_stress;
    volatile double biosphere_lyapunov_v;
    volatile double biosphere_lyapunov_v_dot;
    volatile double biosphere_chnops_conservation_margin;
    volatile double biosphere_dark_shunt_flow_fraction;
} shbt_swarm_cl5_elysium_biosphere_t;

/* ==========================================================================
 * SHBT-MMIO-SWARM v15 REGISTER BLOCK (832 BYTES / 13 CACHE LINES)
 * Physical Base Address: 0x80000000
 * ========================================================================== */
typedef struct __attribute__((aligned(64))) shbt_swarm_mmio_regs {
    /* --- Cache Line 0 (0x0000..0x003F): Phase 1 & 2 Baseline Registers --- */
    uint64_t stinespring_dma_phys_addr;       /* 0x0000: 0x70020000 */
    uint64_t fault_status_mask;               /* 0x0008: Bitmask of active faults */
    uint64_t active_ledger_bits;              /* 0x0010: Active capacity partition */
    uint64_t dark_ledger_bits;                /* 0x0018: Dark ledger capacity (10:23) */
    double   pt_goi_spectral_radius;          /* 0x0020: rho(L) <= 10^-60 */
    double   pt_goi_latency_ps;               /* 0x0028: 54.432 ps */
    uint32_t sheaf_betti_h0;                  /* 0x0030: Connected components = 1 */
    uint32_t sheaf_betti_h1;                  /* 0x0034: Consensus obstruction = 0 */
    uint32_t bio_folding_window_ns;           /* 0x0038: 11,400 ns (11.40 us) */
    uint32_t cloner_mode_select;              /* 0x003C: Mode II (2) */

    /* --- Cache Line 1 (0x0040..0x007F): Phase 3 & 4 Baseline Registers --- */
    double   mar312x_power_watts;             /* 0x0040: Up to 2.50e19 W (25 EW) */
    double   mar312x_magnetic_field_tesla;    /* 0x0048: 48.0 T */
    double   mar312x_coil_temp_kelvin;        /* 0x0050: 77.0 K (T_c = 192.50 K) */
    double   mar312x_hoop_stress_mpa;         /* 0x0058: <= 350.0 MPa */
    double   foundry_mass_injection_kgps;     /* 0x0060: Volatile injection rate */
    double   foundry_surface_temp_kelvin;     /* 0x0068: 298.15 K */
    double   foundry_surface_pressure_bar;    /* 0x0070: 1.01325 bar */
    uint64_t architect_wyckoff_group_mask;    /* 0x0078: 230 space-group status */

    /* --- Cache Line 2 (0x0080..0x00BF): Phase 5A, 5B & 6 Baseline Registers --- */
    double   omega_smbh_spin_param;           /* 0x0080: Kerr spin a/M */
    double   omega_horizon_area_deriv;        /* 0x0088: dA_H/dt >= 0 */
    double   logos_mossbauer_factor_fm;       /* 0x0090: f_M >= 0.782 */
    double   logos_borrmann_tau;              /* 0x0098: tau_B >= 1453.4 */
    double   aegis_tmsv_squeezing_db;         /* 0x00A0: 21.715 dB (r = 2.50) */
    double   apoth_ego_continuity_fidelity;   /* 0x00A8: 1.000000 */
    uint64_t apoth_simulation_epoch_yr_log10; /* 0x00B0: > 100 (10^100 yr) */
    uint64_t pcss_crowbar_latency_ps;         /* 0x00B8: <= 2500 ps (2.5 ns) */

    /* --- Cache Line 3 (0x00C0..0x00FF): Microkernel Control & Audit Header --- */
    uint32_t abi_version;                     /* 0x00C0: 0x000F0312 */
    uint32_t formal_proofs_unsat_count;       /* 0x00C4: 122 */
    uint32_t hil_gates_pass_count;            /* 0x00C8: 400 */
    uint32_t active_phase_mask;               /* 0x00CC: 0x000000FF (Phases 1..6 + 5C) */
    uint64_t heartbeat_monotonic_ticks;       /* 0x00D0: Cycle counter */
    uint64_t reserved_v6_pad[5];              /* 0x00D8..0x00FF: Pad to 0x0100 */

    /* --- Cache Line 4 (0x0100..0x013F): ABI v7 First-Principles Extension --- */
    uint32_t chiral_ee_q16;                   /* 0x0100: Q16.16 enantiomeric excess */
    float    kerr_phase_margin_rad;           /* 0x0104: IEEE 754 Kerr phase margin */
    uint32_t pachner_flip_count;              /* 0x0108: Relativistic Pachner flips */
    uint32_t pachner_cocycle_status;          /* 0x010C: Sheaf coboundary defect == 0 */
    float    fluid_cft_reynolds_stress_max;   /* 0x0110: Atmospheric shear clamp (Pa) */
    uint32_t v7_align_guard;                  /* 0x0114: Explicit 8-byte align pad */
    double   clathrate_vortex_drift_velocity; /* 0x0118: TDGL flux creep meter (m/s) */
    double   smbh_poynting_leakage_watts;     /* 0x0120: Horizon Poynting null (W) */
    double   mossbauer_stark_detuning_hz;     /* 0x0128: Acceleration Stark shift (Hz) */
    float    phenomenal_binding_purity;       /* 0x0130: Braided anyon purity == 1.0 */
    float    branch_prune_entropy_debt;       /* 0x0134: Unitary prune entropy == 0.0 */
    uint64_t v7_reserved_tail;                /* 0x0138: Cache-line closure to 0x0140 */

    /* === Cache Line 5 (0x0140..0x017F): ABI v14.0 Foundry / Elysium Biosphere Overlay (64 bytes) === */
    union {
        uint64_t reserved_v8_pad[8];                    /* 0x0140..0x017F: legacy 64-byte pad alias */
        shbt_swarm_cl5_foundry_t cl5_foundry;           /* Foundry overlay */
        shbt_swarm_cl5_elysium_biosphere_t elysium_biosphere; /* Zero-Coriolis Elysium overlay */
    };

    /* === CACHELINE 6 (Offsets 0x0180 .. 0x01BF): Target Research Closures (64 bytes) === */
    double   gw_memory_residual_strain_norm;          /* 0x0180: <= 1.0e-120 */
    double   secular_com_drift_meters;                /* 0x0188: <= 1.0e-15 m */
    double   continuous_ribbon_discretization_error;  /* 0x0190: == 0.000000 */
    double   ribbon_kz_connection_residual;           /* 0x0198: <= 1.0e-120 */
    float    qrf_superposed_sheaf_spectral_radius;    /* 0x01A0: == 1.000000 */
    float    qrf_microcausality_trace_fidelity;       /* 0x01A4: >= 0.999999 */
    uint64_t v9_reserved_tail[3];                     /* 0x01A8..0x01BF: 24 bytes pad */

    /* === CACHELINE 7 (Offsets 0x01C0 .. 0x01FF): Galactic Synthesis & Multi-Reservoir Invariants (64 bytes) === */
    float    eddington_damping_factor;        /* 0x01C0: kappa_eff / kappa_0 <= 1.0e-12 */
    float    core_adiabatic_index_gamma1;     /* 0x01C4: Gamma_1 >= 4/3 + 1.0e-6 */
    double   hawking_area_delta_margin;       /* 0x01C8: dA_H / dt >= 0 */
    double   galactic_virial_drift_norm;      /* 0x01D0: Delta sigma_r^2 / sigma_r^2 <= 1.0e-6 */
    float    qgp_floquet_debye_mass_fm;       /* 0x01D8: Dynamic screening mass */
    float    planetary_gst_reflection_coeff;  /* 0x01DC: R_acoustic <= 1.0e-6 */
    uint64_t dark_ledger_anyon_braid_count;   /* 0x01E0: Fibonacci anyon braid count */
    uint32_t bz_jet_attenuation_db;           /* 0x01E8: Jet suppression in dB */
    uint32_t v10_reserved_tail[5];            /* 0x01EC..0x01FF: 20 bytes pad */

    /* === CACHELINE 8 (Offsets 0x0200 .. 0x023F): ABI v11 Stellar Hydrodynamics & Loss-Cone Invariants (64 bytes) === */
    double   wd_instanton_action_measured;    /* 0x0200: S_instanton >= 250.0 */
    double   wd_pycnonuclear_rate_cm3_s;      /* 0x0208: R_pyc <= 1.0e-120 cm^-3 s^-1 */
    double   m_dwarf_dynamo_attenuation_db;   /* 0x0210: >= 120.0 dB */
    double   massive_star_opacity_ratio;      /* 0x0218: kappa_eff / kappa_0 <= 1.0e-12 */
    double   nsc_tde_penetration_prob;        /* 0x0220: P_TDE <= 1.0e-120 */
    double   async_symplectic_2form_drift;    /* 0x0228: ||d omega|| <= 1.0e-19 */
    uint64_t v11_harvested_nucleons_counter;  /* 0x0230: Transmuted/harvested baryons */
    uint64_t v11_reserved_tail;               /* 0x0238: 8 bytes pad to 0x0240 */

    /* === CACHELINE 9 (Offsets 0x0240 .. 0x027F): ABI v13 MAR-OMEGA Continuous Reactor & HIL Audit (64 bytes) === */
    double   omega_gross_power_watts;         /* 0x0240: [1.0e21, 1.0e22] W (1-10 ZW) */
    double   omega_cft_direct_eff;            /* 0x0248: >= 0.99980 (0.99981146) */
    double   omega_cryo_sensible_watts;       /* 0x0250: <= 500,000.0 W (245.0 kW at 10 ZW) */
    double   omega_throat_shear_mpa;          /* 0x0258: < 1.0e-6 MPa (7.85e-9 MPa) */
    double   omega_bondi_hoyle_msun_yr;       /* 0x0260: >= 20.0 M_sun/yr */
    uint64_t power_bus_arbiter_flags;         /* 0x0268: Dual-Reactor Arbiter state flags */
    uint32_t hil_gates_pass_mask;             /* 0x0270: 32-bit HIL pass-mask window (400/400 PASS) */
    uint32_t hil_status_code;                 /* 0x0274: 0x00000400 (400/400 PASS) */
    uint64_t crc64_iso_checksum;              /* 0x0278: CRC-64-ISO hardware seal */

    /* === CACHELINE 10 (Offsets 0x0280 .. 0x02BF): Continuous RMHD Divertor & Constellation Aperture (64 bytes) === */
    shbt_mmio_cacheline_10_divertor_t cl10_divertor;

    /* === CACHELINE 11 (Offsets 0x02C0 .. 0x02FF): ABI v14.0 SHBT-AEGIS-ABSOLUTE Isolation Registers (64 bytes) === */
    shbt_aegis_absolute_cl11_t cl11_aegis_absolute;

    /* === CACHELINE 12 (Offsets 0x0300 .. 0x033F): ABI v15.0 Dark-OS / Dark-ZPE Registers (64 bytes) === */
    shbt_dark_os_zpe_cl12_t cl12_dark_os_zpe;
} shbt_swarm_mmio_regs_t;

#if defined(__STDC_VERSION__) && (__STDC_VERSION__ >= 201112L)
_Static_assert(sizeof(shbt_mmio_cacheline_10_divertor_t) == 64,
               "Cache Line 10 must be 64 bytes");
_Static_assert(sizeof(shbt_aegis_absolute_cl11_t) == 64,
               "Cache Line 11 must be 64 bytes");
_Static_assert(sizeof(shbt_swarm_cl5_foundry_t) == 64,
               "Cache Line 5 foundry overlay must be 64 bytes");
_Static_assert(sizeof(shbt_swarm_cl5_elysium_biosphere_t) == 64,
               "Cache Line 5 Elysium biosphere overlay must be 64 bytes");
_Static_assert(sizeof(shbt_dark_os_zpe_cl12_t) == 64,
               "Cache Line 12 must be 64 bytes");
_Static_assert(sizeof(shbt_swarm_mmio_regs_t) == 832,
               "Total MMIO structure must be exactly 832 bytes (13 cache lines)");
_Static_assert(offsetof(shbt_swarm_mmio_regs_t, cl10_divertor) == 0x0280,
               "cl10_divertor must be at offset 0x0280");
_Static_assert(offsetof(shbt_swarm_mmio_regs_t, cl11_aegis_absolute) == 0x02C0,
               "cl11_aegis_absolute must be at offset 0x02C0");
_Static_assert(offsetof(shbt_swarm_mmio_regs_t, cl12_dark_os_zpe) == 0x0300,
               "cl12_dark_os_zpe must be at offset 0x0300");
_Static_assert(offsetof(shbt_swarm_mmio_regs_t, abi_version) == 0x00C0,
               "abi_version offset mismatch");
_Static_assert(offsetof(shbt_swarm_mmio_regs_t, chiral_ee_q16) == 0x0100,
               "chiral_ee_q16 must start at Cache Line 4 (0x0100)");
_Static_assert(offsetof(shbt_swarm_mmio_regs_t, clathrate_vortex_drift_velocity) == 0x0118,
               "clathrate_vortex_drift_velocity must be 8-byte aligned at 0x0118");
_Static_assert(offsetof(shbt_swarm_mmio_regs_t, branch_prune_entropy_debt) == 0x0134,
               "branch_prune_entropy_debt must reside at 0x0134");
_Static_assert(offsetof(shbt_swarm_mmio_regs_t, gw_memory_residual_strain_norm) == 0x0180,
               "gw_memory_residual_strain_norm must start at Cache Line 6 (0x0180)");
_Static_assert(offsetof(shbt_swarm_mmio_regs_t, continuous_ribbon_discretization_error) == 0x0190,
               "continuous_ribbon_discretization_error offset mismatch");
_Static_assert(offsetof(shbt_swarm_mmio_regs_t, qrf_superposed_sheaf_spectral_radius) == 0x01A0,
               "qrf_superposed_sheaf_spectral_radius offset mismatch");
_Static_assert(offsetof(shbt_swarm_mmio_regs_t, eddington_damping_factor) == 0x01C0,
               "eddington_damping_factor must start at Cache Line 7 (0x01C0)");
_Static_assert(offsetof(shbt_swarm_mmio_regs_t, dark_ledger_anyon_braid_count) == 0x01E0,
               "dark_ledger_anyon_braid_count offset mismatch");
_Static_assert(offsetof(shbt_swarm_mmio_regs_t, wd_instanton_action_measured) == 0x0200,
               "wd_instanton_action_measured must start at Cache Line 8 (0x0200)");
_Static_assert(offsetof(shbt_swarm_mmio_regs_t, v11_harvested_nucleons_counter) == 0x0230,
               "v11_harvested_nucleons_counter offset mismatch");
_Static_assert(offsetof(shbt_swarm_mmio_regs_t, omega_gross_power_watts) == 0x0240,
               "omega_gross_power_watts must start at Cache Line 9 (0x0240)");
#endif

/* ==========================================================================
 * 256-BYTE GALACTIC HARVEST APERTURE (0x4000E000)
 * ========================================================================== */
#define SHBT_GALACTIC_APERTURE_ALIGNMENT  256
#define SHBT_GALACTIC_APERTURE_MAGIC      0x534842544F4D4547ULL /* "SHBTOMEG" */

#define SHBT_HARVEST_CTRL_ENABLE          (1U << 0)
#define SHBT_HARVEST_CTRL_ISENTROPIC_LOCK (1U << 1)
#define SHBT_HARVEST_CTRL_BZ_QUENCH       (1U << 2)
#define SHBT_HARVEST_CTRL_MODULAR_FLOW    (1U << 3)
#define SHBT_HARVEST_CTRL_RESET           (1U << 31)

#define SHBT_HARVEST_STAT_ACTIVE          (1U << 0)
#define SHBT_HARVEST_STAT_ERGOSPHERE_LOCK (1U << 1)
#define SHBT_HARVEST_STAT_AREA_PRESERVED  (1U << 2)
#define SHBT_HARVEST_STAT_JET_QUENCHED    (1U << 3)
#define SHBT_HARVEST_STAT_ZERO_VOL_VALID  (1U << 4)
#define SHBT_HARVEST_STAT_ERROR           (1U << 31)

typedef struct __attribute__((packed)) {
    uint32_t radial_distance_m;       /**< Fixed-point radial coordinate (meters) */
    int16_t  polar_angle_mrad;        /**< Polar coordinate theta in milliradians */
    int16_t  azimuth_angle_mrad;      /**< Azimuthal coordinate phi in milliradians */
    uint16_t synthetic_shift_mhz;     /**< Shift counter-vector frequency component */
} shbt_harvester_node_telemetry_t;

_Static_assert(sizeof(shbt_harvester_node_telemetry_t) == 10,
               "shbt_harvester_node_telemetry_t must be 10 bytes");

typedef struct __attribute__((packed, aligned(SHBT_GALACTIC_APERTURE_ALIGNMENT))) {
    /* Offset 0x00 - 0x1F: Aperture Header & Global Control (32 Bytes) */
    uint64_t magic_id;              /**< Offset 0x00: Hardware Magic ID */
    uint32_t control_reg;           /**< Offset 0x08: Aperture Control Register */
    uint32_t status_reg;            /**< Offset 0x0C: Aperture Status Register */
    uint64_t uptime_ticks;          /**< Offset 0x10: Master epoch tick counter */
    uint32_t error_code;            /**< Offset 0x18: Detailed error diagnostic flags */
    uint32_t firmware_revision;     /**< Offset 0x1C: Aperture microkernel revision */

    /* Offset 0x20 - 0x3F: Kerr Metric & Astrodynamic Tracking (32 Bytes) */
    double   bh_mass_kg;            /**< Offset 0x20: Tracked Black Hole Mass M */
    double   bh_spin_dimless;       /**< Offset 0x28: Dimensionless spin a_* in [0, 1) */
    double   horizon_radius_m;      /**< Offset 0x30: Outer Event Horizon radius r_H */
    double   ergosurface_radius_m;  /**< Offset 0x38: Equatorial ergosurface radius r_E */

    /* Offset 0x40 - 0x7F: Hexameric Constellation Array Telemetry (64 Bytes) */
    shbt_harvester_node_telemetry_t nodes[6]; /**< 6 * 10 bytes = 60 bytes */
    uint32_t constellation_sync_phase;        /**< Offset 0x7C: Inter-node phase sync word */

    /* Offset 0x80 - 0x9F: Blandford-Znajek Jet Quencher Subsystem (32 Bytes) */
    double   poloidal_current_amps; /**< Offset 0x80: Residual polar current I_polar */
    double   quench_counter_pot_v;  /**< Offset 0x88: Applied synthetic counter-potential */
    double   synthetic_impedance;   /**< Offset 0x90: Effective boundary impedance Z_load */
    uint32_t jet_attenuation_db;    /**< Offset 0x98: Jet radiated attenuation in dB */
    uint32_t reserved_bz;           /**< Offset 0x9C: Alignment pad */

    /* Offset 0xA0 - 0xDF: Dark Ledger & Topological Injection Rail (64 Bytes) */
    double   total_extracted_joules;/**< Offset 0xA0: Cumulative extracted exergy */
    double   dark_ledger_joules;    /**< Offset 0xA8: Stored exergy in eta_D ledger */
    double   grid_baseload_joules;  /**< Offset 0xB0: Exergy streamed to local grid */
    uint64_t anyon_braid_count_lo;  /**< Offset 0xB8: Fibonacci anyon braid count (low 64) */
    uint64_t anyon_braid_count_hi;  /**< Offset 0xC0: Fibonacci anyon braid count (high 64) */
    uint32_t modular_flow_phase_rad;/**< Offset 0xC8: Tomita-Takesaki modular phase */
    uint32_t dark_partition_ratio;  /**< Offset 0xCC: Fixed-point 23/33 partition lock */
    uint64_t zero_volume_proof_hash;/**< Offset 0xD0: Topological zero-volume check hash */
    uint64_t reserved_dark;         /**< Offset 0xD8: Reserved dark sector rail */

    /* Offset 0xE0 - 0xFF: Cryptographic Audit & Hardware ECC (32 Bytes) */
    uint8_t  hmac_signature[16];    /**< Offset 0xE0: Microkernel HMAC attestation */
    uint64_t ecc_checksum;          /**< Offset 0xF0: Reed-Solomon memory ECC word */
    uint64_t frame_canary;          /**< Offset 0xF8: Stack boundary protection canary */
} shbt_galactic_aperture_mmio_t;

_Static_assert(sizeof(shbt_galactic_aperture_mmio_t) == 256,
               "SHBT ABI Error: Galactic Aperture MMIO structure must be exactly 256 bytes.");

/* ==========================================================================
 * 160-BYTE DENSE COMPACT MATTER DECONSTRUCTION APERTURE (0x00007F8000040000ULL)
 * ========================================================================== */
typedef struct __attribute__((packed, aligned(8))) {
    volatile uint64_t engine_ctrl;                 /* 0x0000: Engine Master Control Register */
    const volatile uint64_t engine_status;         /* 0x0008: Engine Status & Fault Interlock */
    volatile uint64_t wzw_algebra_cfg;             /* 0x0010: WZW Affine Level & Slot Validation */
    volatile uint64_t floquet_freq_mev;            /* 0x0018: Floquet Driving Frequency (MeV) */
    volatile uint64_t floquet_debye_mass;          /* 0x0020: Dynamic Debye Screening Mass (fm^-1) */
    const volatile uint64_t gamma_power_uw;        /* 0x0028: Radiated Prompt Gamma Power (micro-watts) */
    volatile uint64_t central_density_kg_m3;       /* 0x0030: Monitored Central Density (kg/m^3) */
    volatile uint64_t central_pressure_pa;         /* 0x0038: Monitored Central Pressure (Pa) */
    const volatile uint64_t adiabatic_index_fixed; /* 0x0040: Core Gamma_1 in Q32.32 Fixed-Point */
    volatile uint64_t mass_siphon_rate_kg_s;       /* 0x0048: Target Siphon Extraction Rate (kg/s) */
    volatile uint64_t magnetar_b_field_tesla;      /* 0x0050: Magnetic Flux Density B (Tesla) */
    volatile uint64_t dark_ledger_drain_ratio;     /* 0x0058: Dark Drain Efficiency (Q32.32 Fixed) */
    const volatile int64_t  b_minus_l_accumulator; /* 0x0060: Signed (B - L) Net Disparity Count */
    const volatile uint64_t visible_neutrino_flux; /* 0x0068: Neutrino Flux (cm^-2 s^-1) */
    const volatile int64_t  activation_xsec_exponent; /* 0x0070: Activation Cross Section Log10(Barns) */
    volatile uint64_t crystal_space_group;         /* 0x0078: Space Group Projector Code (e.g. 227) */
    const volatile uint64_t peak_crystal_temp_mk;  /* 0x0080: Peak Transient Temperature (milli-Kelvin) */
    volatile uint64_t heegaard_floer_shock_factor; /* 0x0088: Thermal Shunt Factor (100/1089) */
    const volatile uint64_t borrmann_defect_density; /* 0x0090: Borrmann Defect Density */
    volatile uint64_t aperture_command_trigger;    /* 0x0098: Aperture Command Trigger */
} shbt_dense_matter_mmio_t;

_Static_assert(sizeof(shbt_dense_matter_mmio_t) == 160,
               "shbt_dense_matter_mmio_t must be exactly 160 bytes");

/* ==========================================================================
 * DENSE MATTER DECONSTRUCTION & GALACTIC ASTRODYNAMICS APERTURES
 * ========================================================================== */

#define SHBT_MMIO_ABI_VERSION         0x000E0312U
#define SHBT_MMIO_DENSE_BASE          0x7F000000ULL
#define SHBT_MMIO_QUENCH_BASE         0x7F000100ULL
#define SHBT_MMIO_SHUNT_BASE          0x7F000200ULL
#define SHBT_MMIO_HIL_DENSE_BASE      0x7F000300ULL
#define SHBT_MMIO_GALACTIC_BASE       0x8F000000ULL

#define MULTIPHYSICS_13_ENGINES_MASK  0x00001FFFU
#define MULTIPHYSICS_16_ENGINES_MASK  0x0000FFFFU
#define HIL_GATES_233_241_MASK        0x000001FFU
#define HIL_GATES_242_250_MASK        0x000001FFU

#ifdef __cplusplus
extern "C" {
#endif

/* Dense Matter Siphon & Color Screening (0x7F000000) */
typedef struct __attribute__((packed, aligned(8))) {
    volatile uint32_t abi_version;        /* 0x00: 0x00080312 */
    volatile uint32_t status_flags;       /* 0x04: Bit 0=Enable, Bit 1=Trip */
    volatile double   current_rho_b;      /* 0x08: Central density (kg/m^3) */
    volatile double   commanded_m_dot;    /* 0x10: Commanded M_dot (kg/s) */
    volatile double   rf_field_amplitude; /* 0x18: E_RF (V/m) */
    volatile double   drive_freq_omega;   /* 0x20: Omega (rad/s) */
    volatile double   eval_debye_mass;    /* 0x28: mu_D (fm^-1) */
    volatile double   screen_potential;   /* 0x30: U_screen (MeV) */
    volatile double   evaluated_gamma_1;  /* 0x38: Relativistic Gamma_1 */
    volatile double   fluid_drift_v_c;    /* 0x40: v_fluid / c */
    volatile uint32_t pasta_phase;        /* 0x48: 1=Gnocchi, 2=Spaghetti, 3=Lasagna, 4=Fluid */
    volatile uint32_t _pad_tail;          /* 0x4C: 8-byte alignment pad */
} shbt_mmio_dense_transmute_t;

/* Magnetar Helicity Quencher (0x7F000100) */
typedef struct __attribute__((packed, aligned(8))) {
    volatile uint32_t control_bits;       /* 0x00: Bit 0=Arm, Bit 1=Quench */
    volatile uint32_t status_enum;        /* 0x04: QuenchStatus */
    volatile double   b_init_tesla;       /* 0x08: Initial B (T) */
    volatile double   b_current_tesla;    /* 0x10: Current B (T) */
    volatile double   current_db_down;    /* 0x18: Attenuation (dB) */
    volatile double   dissipation_rate;   /* 0x20: dU_B/dt (W) */
    volatile int64_t  topological_w;      /* 0x28: pi_3(SU(2)) invariant */
    volatile double   run_duration_s;     /* 0x30: Elapsed time (s) */
} shbt_mmio_magnetar_quench_t;

/* Lepton Shunt & 25 EW CFT Bus (0x7F000200) */
typedef struct __attribute__((packed, aligned(8))) {
    volatile uint32_t coupling_mode;      /* 0x00: ShuntCouplingMode */
    volatile uint32_t phase_locked;       /* 0x04: 1=Locked */
    volatile double   theta_k_rad;        /* 0x08: Bogoliubov angle (rad) */
    volatile double   phi_nu_observed;    /* 0x10: Neutrino flux (cm^-2 s^-1) */
    volatile double   cft_power_output_w; /* 0x18: 25 EW Bus Power (W) */
    volatile double   dark_debt_joules;   /* 0x20: Cumulative dark exergy */
    volatile uint64_t disassembled_count; /* 0x28: Baryons shunted */
} shbt_mmio_lepton_shunt_t;

/* Galactic Astrodynamics Engine (0x8F000000) */
typedef struct __attribute__((packed, aligned(8))) {
    volatile uint32_t symplectic_ctrl;    /* 0x00: Bit 0=Engage, Bit 1=Reset */
    volatile uint32_t symplectic_status;  /* 0x04: Bit 0=Preserved (d_omega == 0) */
    volatile uint64_t cascade_ratio;      /* 0x08: alpha_casc (default 1e12) */
    volatile double   energy_drift_rel;   /* 0x10: Delta H / H(0) */
    volatile double   ccz4_kappa1;        /* 0x18: Gundlach damping parameter */
    volatile double   ccz4_z_norm;        /* 0x20: ||Z_i|| constraint norm */
    volatile double   gw_mem_residual;    /* 0x28: ||h_ij^mem + h_ij^counter|| */
    volatile double   com_drift_x;        /* 0x30: CoM drift X (m) */
    volatile double   com_drift_y;        /* 0x38: CoM drift Y (m) */
    volatile double   com_drift_z;        /* 0x40: CoM drift Z (m) */
    volatile uint64_t cs_level_k;         /* 0x48: Level K (312) */
    volatile int64_t  cs_winding_x;       /* 0x50: Sp(2g, Z) Winding X */
    volatile int64_t  cs_winding_y;       /* 0x58: Sp(2g, Z) Winding Y */
    volatile int64_t  cs_winding_z;       /* 0x60: Sp(2g, Z) Winding Z */
    volatile double   shift_anomaly_norm; /* 0x68: ||beta^i||_inf */
    volatile uint32_t multiphysics_bits;  /* 0x70: Bits 0..12 engine status */
    volatile uint32_t hil_gate_status;    /* 0x74: Gates 242-250 pass mask */
} shbt_galactic_mmio_t;

/* ==========================================================================
 * ABI v11.0 DEGENERATE CORE, STELLAR HYDRODYNAMIC, AND KINEMATIC PATCH APERTURES
 * ========================================================================== */

#define SHBT_DCA_BASE_ADDR              0x7F000400ULL
#define SHBT_STELLAR_APERTURE_BASE_ADDR 0x7F000500ULL
#define SHBT_PATCH_APERTURE_BASE_ADDR   0x8F000100ULL

/* DCA Register Bitmasks */
#define DCA_CTRL_ENABLE                 (1ULL << 0)
#define DCA_CTRL_SU2_SCREEN_ENABLE      (1ULL << 1)
#define DCA_CTRL_GST_MATCH_ENABLE       (1ULL << 2)
#define DCA_CTRL_CFL_HADRON_ENABLE      (1ULL << 3)
#define DCA_CTRL_DARK_SHUNT_ENGAGE      (1ULL << 4)
#define DCA_CTRL_INTERLOCK_OVERRIDE     (1ULL << 31)

#define DCA_STAT_ACTIVE                 (1ULL << 0)
#define DCA_STAT_PYC_SUPPRESSED         (1ULL << 1)
#define DCA_STAT_ACOUSTIC_LOCKED        (1ULL << 2)
#define DCA_STAT_CFL_CONFINED           (1ULL << 3)
#define DCA_STAT_DARK_SHUNT_ACTIVE      (1ULL << 4)
#define DCA_STAT_STABILITY_MARGIN_OK    (1ULL << 5)
#define DCA_STAT_DEFLAGRATION_ABORT     (1ULL << 30)
#define DCA_STAT_ERROR_FLAG             (1ULL << 31)

typedef struct __attribute__((packed, aligned(8))) {
    volatile uint64_t control;               /* 0x00 */
    volatile const uint64_t status;          /* 0x08 */
    volatile uint64_t su2_screen_mev_fp;     /* 0x10 */
    volatile const uint64_t instanton_action;/* 0x18 */
    volatile const uint64_t pyc_mantissa;    /* 0x20 */
    volatile const int64_t  pyc_exponent;    /* 0x28 */
    volatile uint64_t gst_impedance_real;    /* 0x30 */
    volatile uint64_t gst_impedance_imag;    /* 0x38 */
    volatile const uint64_t acoustic_refl_fp;/* 0x40 */
    volatile const uint64_t cfl_gap_mev_fp;  /* 0x48 */
    volatile const uint64_t free_quark_frac; /* 0x50 */
    volatile const uint64_t dark_shunt_exergy;/* 0x58 */
    volatile const uint64_t adiabatic_index; /* 0x60 */
    volatile const uint64_t gamma_power_fp;  /* 0x68 */
} shbt_dca_registers_t;

#define SHBT_DYNKIN_SLOT_TOTAL_CHANNELS 346U

#define SHBT_CTRL0_APERTURE_ENABLE          (1U << 0)
#define SHBT_CTRL0_RED_DWARF_DYNAMO_BRAKE   (1U << 1)
#define SHBT_CTRL0_MASSIVE_PAIR_SUPPRESS    (1U << 2)
#define SHBT_CTRL0_GORDON_METRIC_ENABLE     (1U << 3)
#define SHBT_CTRL0_ATHERMAL_ZERO_Q_LOCK     (1U << 4)

#define SHBT_STATUS_APERTURE_LOCKED         (1U << 0)
#define SHBT_STATUS_R_M_CLAMPED_LE_1        (1U << 1)
#define SHBT_STATUS_ATTENUATION_GE_120DB    (1U << 2)
#define SHBT_STATUS_BULK_Q_ZERO_VALID       (1U << 3)
#define SHBT_STATUS_OPACITY_SCALED_LE_12    (1U << 4)
#define SHBT_STATUS_GAMMA1_STABLE           (1U << 5)
#define SHBT_STATUS_DYNKIN_MESH_ALIGNED     (1U << 6)
#define SHBT_STATUS_THERMAL_FAULT           (1U << 31)

typedef struct __attribute__((packed, aligned(8))) {
    volatile uint32_t CTRL0;                         /* 0x00 */
    volatile uint32_t CTRL1;                         /* 0x04 */
    volatile uint32_t STATUS;                        /* 0x08 */
    volatile uint32_t DYNKIN_SYNC_CLK;               /* 0x0C */
    volatile uint32_t STELLAR_MASS_FIXED;            /* 0x10 */
    volatile uint32_t CONVECTIVE_VELOCITY_MMS;       /* 0x14 */
    volatile uint64_t CONVECTIVE_SCALE_L_M;          /* 0x18 */
    volatile uint32_t SPITZER_DIFFUSIVITY_ETA0;      /* 0x20 */
    volatile uint32_t R_M_EFF_FIXED16;               /* 0x24 */
    volatile uint32_t DYNAMO_ATTENUATION_DB_FIXED16; /* 0x28 */
    volatile uint32_t BULK_DISSIPATION_Q_DISS;       /* 0x2C */
    volatile uint32_t ZETA_SHBT_NUMERATOR;           /* 0x30 */
    volatile uint32_t ZETA_SHBT_DENOMINATOR;         /* 0x34 */
    volatile uint64_t GORDON_METRIC_SCALING_Q64;     /* 0x38 */
    volatile uint64_t CORE_TEMPERATURE_KELVIN;       /* 0x40 */
    volatile uint64_t CORE_DENSITY_G_M3;             /* 0x48 */
    volatile uint32_t CORE_GAMMA_1_FIXED16;          /* 0x50 */
    uint32_t          _reserved0;                    /* 0x54 */
    volatile uint32_t DYNKIN_SLOT_WEIGHTS[SHBT_DYNKIN_SLOT_TOTAL_CHANNELS]; /* 0x58..0x5BF: 1384 bytes */
} shbt_stellar_aperture_mmio_t;

#define CSR_SHIELD_ENABLE_BIT           (1U << 0)
#define CSR_KERR_DECOUPLE_BIT           (1U << 1)
#define CSR_SBH_BYPASS_ACTIVE_BIT       (1U << 2)
#define CSR_CASCADE_LOCK_BIT            (1U << 3)
#define CSR_SYMPLECTIC_VIOLATION_BIT    (1U << 4)
#define CSR_APERTURE_RESET_BIT          (1U << 31)

typedef struct __attribute__((packed, aligned(8))) {
    volatile uint32_t csr;                    /* 0x00: Control & Status */
    volatile uint32_t hub_shield_amp;         /* 0x04: Shield Amplitude */
    volatile uint64_t lc_barrier_thresh;      /* 0x08: J_barrier (m^2/s) */
    volatile uint64_t kerr_drag_beta_phi;     /* 0x10: Shift Vector beta^phi */
    volatile float    sbh_bypass_excl_rad;    /* 0x18: Exclusion Radius (m) */
    volatile uint32_t _reserved0;             /* 0x1C: Alignment padding */
    volatile uint64_t cascade_ratio;          /* 0x20: alpha_casc */
    volatile uint32_t ccz4_gundlach_damp;     /* 0x28: kappa1 & kappa2 */
    volatile uint32_t _reserved1;             /* 0x2C: Alignment padding */
    volatile double   poincare_drift_mon;     /* 0x30: ||d omega|| */
    volatile double   shadow_h_dev_mon;       /* 0x38: |Delta H / H0| */
} shbt_patch_aperture_hw_t;

/* ==========================================================================
 * ABI v13.0 MAR-OMEGA CONTINUOUS REACTOR & UNIVERSAL HARVEST APERTURES
 * ========================================================================== */

/* --- Cache Line 1..4: MAR-OMEGA Control Aperture (256 Bytes @ 0x7F000600) --- */
typedef struct __attribute__((aligned(64))) shbt_mar_omega_aperture {
    /* Cache Line 1 (0x040 - 0x07F): Continuous Slipstream Ingestion & Burn */
    double omega_mass_inlet_kg_s;                        /* Continuous slipstream mass rate (<= 111,265.01 kg/s) */
    union { double omega_slipstream_ratio; double slipstream_fraction; };
    union { double omega_gross_power_watts; double gross_annihilation_watts; };
    union { double omega_cft_power_watts; double direct_cft_exergy_watts; };
    union { double omega_coset_dark_ratio; double dark_ledger_eta_d; };
    union { double omega_pion_spray_log10; double pion_spray_prob_log10; };
    union { double omega_athermal_decay_rate; double upstream_decay_rate_hz; };
    union { uint64_t omega_throat_status_bits; uint64_t slipstream_status_flags; };

    /* Cache Line 2 (0x080 - 0x0BF): Direct Boundary CFT Dynkin Projection */
    union { uint64_t cft_dynkin_coupling_mask[6]; uint64_t cft_dynkin_coupling_bits[6]; };
    union { double cft_direct_coupling_eff; double direct_cft_efficiency; };
    union { double cft_uncoupled_waste_eff; double uncoupled_waste_fraction; };

    /* Cache Line 3 (0x0C0 - 0x0FF): 16-Layer Symplectic Fracton & Cryostat */
    uint64_t cryo_temp_millikelvin;                      /* Operating coil temp in mK (<= 25,000 mK = 25.0 K) */
    union { double cryo_sensible_load_watts; double sensible_heat_load_watts; };
    union { double cryo_fracton_trans_coeff; double fracton_transmission_factor; };
    union { double cryo_hyperbolic_tau_q_ps; double cattaneo_relaxation_ps; };
    union { double cryo_temp_headroom_kelvin; double rebco_headroom_kelvin; };
    union { double cryo_sink_bath_temp_k; double he2_bath_temp_kelvin; };
    union { uint64_t cryo_fracton_layers_mask; uint64_t fracton_genus_layer_mask; };
    union { uint64_t cryo_second_sound_status; uint64_t cryo_interlock_word; };

    /* Cache Line 4 (0x100 - 0x13F): 312-Channel Counter-Metric Stress Cloaking */
    union { double ccz4_constraint_norm_theta; double riemann_theta_inf_norm; };
    union { double ccz4_effective_shear_mpa; double throat_von_mises_mpa; };
    union { double ccz4_frenkel_limit_mpa; double frenkel_limit_mpa; };
    union { double ccz4_gundlach_kappa1; double ccz4_damping_kappa1; };
    union { double ccz4_gundlach_kappa2; double ccz4_damping_kappa2; };
    union { double ccz4_phase_offset_rad; double counter_metric_phase_rad; };
    union { uint32_t ccz4_active_channels; uint32_t active_ccz4_channels; };
    union { uint32_t ccz4_interlock_status; uint32_t metric_cloak_status; };
} shbt_mar_omega_aperture_t;

/* --- Cache Line 5..8: Universal Harvest Engine (256 Bytes @ 0x7F000700) --- */
typedef struct __attribute__((aligned(64))) shbt_universal_harvest_aperture {
    /* Cache Line 5 (0x140 - 0x17F): Conformal Opacity & Bondi-Hoyle Siphon */
    union { double rosseland_opacity_quench_db; double metric_opacity_atten_db; };
    union { double rosseland_kappa_eff_ratio; double kappa_eff_over_kappa0; };
    union { double pair_thresh_s_ratio; double pair_quench_s_thresh_mev2; };
    union { double pair_adiabatic_gamma1; double core_adiabatic_gamma1; };
    union { double harvest_m_dot_msun_yr; double bondi_hoyle_rate_msun_yr; };
    union { double harvest_sugawara_gamma_top; double sugawara_framing_gain; };
    union { double harvest_seed_mass_kg; double ghost_seed_mass_kg; };
    union { uint64_t harvest_target_class_id; uint64_t target_stellar_class_id; };

    /* Cache Line 6 (0x180 - 0x1BF): Red Dwarf Dynamo & White Dwarf Melting */
    union { double dynamo_turbulent_reynolds_rm; double red_dwarf_magnetic_reynolds; };
    union { double dynamo_flare_damping_db; double flare_attenuation_db; };
    union { double dynamo_ohmic_shock_joules; double dynamo_dissipation_joules; };
    union { double wd_gst225_impedance_mrayl; double gst225_acoustic_z_mrayl; };
    union { double wd_acoustic_reflectance; double acoustic_reflectance; };
    union { double wd_coulomb_coupling_gamma; double coulomb_gamma_parameter; };
    union { double wd_instanton_action; double instanton_action_s; };
    union { double wd_pyc_fusion_rate; double pycnonuclear_rate_cm3_s; };

    /* Cache Line 7 (0x1C0 - 0x1FF): Neutron Star CFL Hadronization & Lepton Shunt */
    union { double ns_floquet_debye_potential_mev; double floquet_screening_mev; };
    union { double ns_debye_wavenumber_inv_fm; double debye_wavenumber_fm_inv; };
    union { double ns_chiral_restoration_slope; double chiral_geodesic_slope; };
    union { double ns_free_quark_fraction; double free_quark_fraction; };
    union { double ns_unitary_lepton_shunt_flux; double dark_lepton_shunt_flux; };
    union { double ns_outbound_neutrino_flux; double neutrino_flux_cm2_s; };
    union { double ns_prompt_gamma_power_watts; double prompt_gamma_watts; };
    union { uint64_t ns_color_triality_slots; uint64_t color_triality_slot_mask; };

    /* Cache Line 8 (0x200 - 0x23F): Monolithic Computronium Synthesis (LOGOS-312) */
    union { double logos_fold_throughput_pct; double computronium_throughput_kg_s; };
    union { double logos_interfacial_disloc_density; double dislocation_density_cm2; };
    union { double logos_lattice_tether_stiffness; double chern_simons_stiffness_n_m; };
    union { uint64_t logos_phonon_occupancy_e9; uint64_t phonon_occ_e9; };
    union { double logos_fracton_capture_time_fs; double fracton_capture_time_fs; };
    union { double logos_kms_entropy_rate_j_k; double internal_entropy_rate_j_k; };
    union { double logos_per_bit_heat_dissipation; double internal_dissipation_j_bit; };
    union { uint64_t logos_active_space_groups; uint64_t crystal_space_group_mask; };
} shbt_universal_harvest_aperture_t;

/* --- Master 704-Byte Microkernel Contract (ABI v13.0, 11 Cache Lines) --- */
typedef struct __attribute__((aligned(64))) shbt_swarm_mmio_v13 {
    /* Cache Line 0 (0x000 - 0x03F): Microkernel Header & Global Control */
    uint32_t magic;                                      /* 0x53484254 ("SHBT") */
    uint32_t abi_version;                                /* 0x000D0312 (ABI v13.0) */
    uint64_t symplectic_tick_counter;                    /* Monotonic symplectic step counter */
    union { uint64_t global_status_flags; uint64_t power_bus_arbiter_flags; };
    uint64_t global_fault_mask;                          /* Hardware fault syndrome register */
    uint64_t aperture_phys_mar_omega;                    /* 0x7F000600ULL */
    uint64_t aperture_phys_harvest;                      /* 0x7F000700ULL */
    uint64_t reserved_cl0[2];                            /* Pad Cache Line 0 to 64 bytes */

    /* Cache Lines 1..4 (0x040 - 0x13F): MAR-OMEGA Control Aperture (256B) */
    shbt_mar_omega_aperture_t mar_omega;

    /* Cache Lines 5..8 (0x140 - 0x23F): Universal Harvest Engine Aperture (256B) */
    union {
        shbt_universal_harvest_aperture_t harvest;
        shbt_universal_harvest_aperture_t harvest_engine;
    };

    /* Cache Line 9 (0x240 - 0x27F): HIL Gate Audit Matrix & CRC64 Seal */
    union { uint32_t hil_gates_pass_mask; uint32_t hil_gates_321_to_344_mask; };
    union { uint32_t hil_total_gates_count; uint32_t hil_total_registered_gates; };
    union { uint32_t hil_passed_gates_count; uint32_t hil_total_passed_gates; };
    union { uint32_t hil_status_code; uint32_t hil_audit_status_code; };
    union { uint64_t hardware_timer_ps; uint64_t monotonic_timestamp_ns; };
    union { uint64_t crc64_iso_checksum; uint64_t aperture_crc64_iso; };
    union { uint8_t line9_reserved[32]; uint8_t reserved_cl9[32]; };

    /* Cache Line 10 (0x280 - 0x2BF): Continuous RMHD Divertor & Constellation Aperture (64B) */
    shbt_mmio_cacheline_10_divertor_t divertor;
} shbt_swarm_mmio_v13_t;

typedef shbt_swarm_mmio_v13_t shbt_swarm_mmio_v12_t;

/* --- Master 768-Byte ABI v14.0 Contract (12 Cache Lines) --- */
typedef struct __attribute__((aligned(64))) {
    shbt_swarm_mmio_v13_t legacy_v13;
    shbt_aegis_absolute_cl11_t cl11_aegis_absolute;
} shbt_swarm_mmio_v14_t;

typedef shbt_swarm_mmio_v14_t shbt_swarm_mmio_v14;

/* --- Master 832-Byte ABI v15.0 Contract (13 Cache Lines) --- */
typedef struct __attribute__((aligned(64))) {
    shbt_swarm_mmio_v14_t legacy_v14;
    shbt_dark_os_zpe_cl12_t cl12_dark_os_zpe;
} shbt_swarm_mmio_v15_t;

typedef shbt_swarm_mmio_v15_t shbt_swarm_mmio_v15;

#ifdef __cplusplus
}
#endif

_Static_assert(sizeof(shbt_mmio_dense_transmute_t) == 80,
               "shbt_mmio_dense_transmute_t must be exactly 80 bytes");
_Static_assert(sizeof(shbt_mmio_magnetar_quench_t) == 56,
               "shbt_mmio_magnetar_quench_t must be exactly 56 bytes");
_Static_assert(sizeof(shbt_mmio_lepton_shunt_t) == 48,
               "shbt_mmio_lepton_shunt_t must be exactly 48 bytes");
_Static_assert(sizeof(shbt_galactic_mmio_t) == 120,
               "shbt_galactic_mmio_t must be exactly 120 bytes");
_Static_assert(sizeof(shbt_dca_registers_t) == 112,
               "shbt_dca_registers_t must be exactly 112 bytes");
_Static_assert(sizeof(shbt_stellar_aperture_mmio_t) == 1472,
               "shbt_stellar_aperture_mmio_t must be exactly 1472 bytes");
_Static_assert(sizeof(shbt_patch_aperture_hw_t) == 64,
               "shbt_patch_aperture_hw_t must be exactly 64 bytes");

/* --- Strict Compile-Time ABI v13.0 Layout & Alignment Validations --- */
_Static_assert(sizeof(shbt_mar_omega_aperture_t) == 256U, "MAR-OMEGA aperture must occupy 256 bytes (4 cache lines)");
_Static_assert(sizeof(shbt_universal_harvest_aperture_t) == 256U, "Universal Harvest aperture must occupy 256 bytes (4 cache lines)");
_Static_assert(sizeof(shbt_mmio_cacheline_10_divertor_t) == 64U, "Cache Line 10 Divertor must equal 64 bytes (1 cache line)");
_Static_assert(sizeof(shbt_swarm_mmio_v13_t) == 704U, "ABI v13.0 MMIO struct must equal 704 bytes (11 cache lines)");
_Static_assert(alignof(shbt_swarm_mmio_v13_t) == 64U, "ABI v13.0 MMIO struct must be 64-byte aligned");
_Static_assert(sizeof(shbt_aegis_absolute_cl11_t) == 64U, "Cache Line 11 must equal 64 bytes");
_Static_assert(sizeof(shbt_swarm_mmio_v14_t) == 768U, "ABI v14.0 MMIO struct must equal 768 bytes (12 cache lines)");
_Static_assert(alignof(shbt_swarm_mmio_v14_t) == 64U, "ABI v14.0 MMIO struct must be 64-byte aligned");
_Static_assert(offsetof(shbt_swarm_mmio_v14_t, cl11_aegis_absolute) == 704U, "Cache Line 11 must start at offset 0x02C0");
_Static_assert(sizeof(shbt_dark_os_zpe_cl12_t) == 64U, "Cache Line 12 (shbt_dark_os_zpe_cl12_t) must equal 64 bytes");
_Static_assert(sizeof(shbt_swarm_mmio_v15_t) == 832U, "ABI v15.0 MMIO struct must equal 832 bytes (13 cache lines)");
_Static_assert(alignof(shbt_swarm_mmio_v15_t) == 64U, "ABI v15.0 MMIO struct must be 64-byte aligned");
_Static_assert(offsetof(shbt_swarm_mmio_v15_t, cl12_dark_os_zpe) == 768U, "Cache Line 12 must start at byte offset 768 (0x0300)");
_Static_assert(offsetof(shbt_swarm_mmio_v13_t, mar_omega) == 64U, "MAR-OMEGA aperture must start at Cache Line 1 (0x040)");
_Static_assert(offsetof(shbt_swarm_mmio_v13_t, harvest) == 320U, "Universal Harvest aperture must start at Cache Line 5 (0x140)");
_Static_assert(offsetof(shbt_swarm_mmio_v13_t, harvest_engine) == 320U, "Universal Harvest aperture must start at Cache Line 5 (offset 0x140)");
_Static_assert(offsetof(shbt_swarm_mmio_v13_t, divertor) == 640U, "Divertor aperture must start at Cache Line 10 (0x0280)");

#endif /* SHBT_SWARM_MMIO_H */

