//! `shbt-swarm-mmio`: Bare-Metal Microkernel ABI v10 (`0x000A0312`) Register Specification.
//! Zero-allocation `#![no_std]` FFI bindings matching `include/shbt_swarm_mmio.h`.

#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]
#![allow(dead_code, unused_imports)]

use core::mem::{align_of, offset_of, size_of};
use core::ptr::{read_volatile, write_volatile};
use shbt_swarm_core::{
    UnifiedStinespringHyperFrame, SHBT_SWARM_HYPERFRAME_BASE_ADDR,
    SHBT_SWARM_CMD_NOP, SHBT_SWARM_CMD_EXEC_CLONE_SHIP, SHBT_SWARM_CMD_WEYL_EXCITON_CROWBAR,
    SHBT_SWARM_FAULT_ADM_IMBALANCE, SHBT_SWARM_TRIAGE_X2_ASSIMILATE,
    SHBT_SWARM_TRIAGE_SHIFT, SHBT_SWARM_TRIAGE_MASK,
};

pub const SHBT_SWARM_MMIO_BASE_ADDR: u64 = 0x8000_0000;
pub const SHBT_SWARM_STINESPRING_BASE_ADDR: u64 = 0x7002_0000;
pub const SHBT_SWARM_STINESPRING_FRAME_BYTES: usize = 135_168;
pub const SHBT_SWARM_ABI_VERSION: u32 = 0x000A_0312;
pub const SHBT_SWARM_CHANNELS_TOTAL: u32 = 312;
pub const SHBT_SWARM_CFT_SLOTS_TOTAL: u32 = 346;
pub const SHBT_SWARM_FORMAL_PROOFS_COUNT: u32 = 88;
pub const SHBT_SWARM_HIL_GATES_COUNT: u32 = 304;
pub const SHBT_NESTED_SIM_MMIO_BASE: u64 = 0x7000_B000;
pub const SHBT_SWARM_HARVEST_APERTURE_BASE: u64 = 0x4000_E000;
pub const SHBT_SWARM_FOUNDRY_DENSE_BASE: u64 = 0x0000_7F80_0004_0000;
pub const SHBT_APOTH_EGRESS_SAFEWORD_IRQ: u32 = 0x8000_0000;
pub const SHBT_CANONICAL_KL: u32 = 26;
pub const SHBT_CANONICAL_KQ: u32 = 8;
pub const SHBT_CANONICAL_K_COS: u32 = 312;
pub const SHBT_DYNKIN_SLOTS_TOTAL: u32 = 346;

pub const SHBT_SWARM_CMD_CLAMP_GW_RECOIL: u32 = 0x0000_0100;
pub const SHBT_SWARM_CMD_ENGAGE_CONT_RIBBON: u32 = 0x0000_0200;
pub const SHBT_SWARM_CMD_ASSERT_QRF_COVARIANCE: u32 = 0x0000_0400;

pub const SHBT_CTRL_ENABLE_BIT: u64 = 1 << 0;
pub const SHBT_CTRL_LAZY_MERA_BIT: u64 = 1 << 1;
pub const SHBT_CTRL_MODE_SHIFT: u32 = 2;
pub const SHBT_CTRL_MODE_MASK: u64 = 0x3 << 2;

pub const SHBT_CBF_CLASS_X1_DETECTED_BIT: u64 = 1 << 0;
pub const SHBT_CBF_SANCTUARY_FORK_BIT: u64 = 1 << 1;
pub const SHBT_CBF_ZOMBIE_SWAP_BIT: u64 = 1 << 2;
pub const SHBT_CBF_QP_CLAMP_BIT: u64 = 1 << 3;
pub const SHBT_CBF_SIM_KILL_DISABLE_BIT: u64 = 1 << 32;

pub const MAGIC_SWRM: u32 = 0x5357_524D; // 'SWRM'
pub const ABI_VERSION_V4: u32 = 0x0004_0312;
pub const ABI_VERSION_V6: u32 = 0x0006_0312;
pub const ABI_VERSION_V7: u32 = 0x0007_0312;
pub const ABI_VERSION_V9: u32 = 0x0009_0312;
pub const ABI_VERSION_V10: u32 = 0x000A_0312;
pub const SHBT_APOTH_MMIO_BASE: usize = 0x4000_C000;

// Baseline Fault Bitmasks (Bits 0..31)
pub const SHBT_SWARM_FAULT_NONE: u64 = 0;
pub const SHBT_SWARM_FAULT_ABI_MISMATCH: u64 = 1 << 0;
pub const SHBT_SWARM_FAULT_LANDAUER_OVERFLOW: u64 = 1 << 1;
pub const SHBT_SWARM_FAULT_PT_GOI_NILPOTENCY: u64 = 1 << 2;
pub const SHBT_SWARM_FAULT_SHEAF_SPLIT_BRAIN: u64 = 1 << 3;
pub const SHBT_SWARM_FAULT_MAR312X_HOOP_STRESS: u64 = 1 << 4;
pub const SHBT_SWARM_FAULT_OMEGA_HORIZON: u64 = 1 << 5;
pub const SHBT_SWARM_FAULT_APOTH_QUENCH: u64 = 1 << 6;

// v7 First-Principles Closure Fault Bitmasks (Bits 32..39)
pub const SHBT_SWARM_FAULT_CHIRAL_RACEMIC: u64 = 1 << 32;
pub const SHBT_SWARM_FAULT_LLE_CHAOS: u64 = 1 << 33;
pub const SHBT_SWARM_FAULT_PACHNER_COCYCLE: u64 = 1 << 34;
pub const SHBT_SWARM_FAULT_ATMOS_SHOCK: u64 = 1 << 35;
pub const SHBT_SWARM_FAULT_FLUX_JUMP: u64 = 1 << 36;
pub const SHBT_SWARM_FAULT_POYNTING_JET: u64 = 1 << 37;
pub const SHBT_SWARM_FAULT_MOSSBAUER_DETUNE: u64 = 1 << 38;
pub const SHBT_SWARM_FAULT_BINDING_DECOHERENCE: u64 = 1 << 39;
pub const SHBT_SWARM_FAULT_V7_MASK: u64 = 0x0000_00FF_0000_0000;

// v9 Research Extensions Fault Bitmasks (Bits 47..53)
pub const SHBT_SWARM_FAULT_GW_MEMORY_STRAIN_TRIP: u64 = 1 << 47;
pub const SHBT_SWARM_FAULT_COM_HALO_DRIFT_TRIP: u64 = 1 << 48;
pub const SHBT_SWARM_FAULT_RIBBON_KZ_SINGULARITY: u64 = 1 << 49;
pub const SHBT_SWARM_FAULT_DISCRETIZATION_NONZERO: u64 = 1 << 50;
pub const SHBT_SWARM_FAULT_QRF_DECOHERENCE_TRIP: u64 = 1 << 51;
pub const SHBT_SWARM_FAULT_SHEAF_SPECTRUM_DIV: u64 = 1 << 52;
pub const SHBT_SWARM_FAULT_METRIC_SUP_CAUSALITY: u64 = 1 << 53;
pub const SHBT_SWARM_FAULT_V9_MASK: u64 = 0x003F_8000_0000_0000;

// v10 Galactic Harvest & Dense Matter Invariant Fault Bitmasks (Bits 54..60)
pub const SHBT_SWARM_FAULT_EDDINGTON_OVERPRESSURE: u64 = 1 << 54;
pub const SHBT_SWARM_FAULT_CORE_ADIABAT_COLLAPSE: u64 = 1 << 55;
pub const SHBT_SWARM_FAULT_HAWKING_AREA_DECREASE: u64 = 1 << 56;
pub const SHBT_SWARM_FAULT_HALO_VIRIAL_JEANS_DRIFT: u64 = 1 << 57;
pub const SHBT_SWARM_FAULT_FLOQUET_COLOR_UNLOCKED: u64 = 1 << 58;
pub const SHBT_SWARM_FAULT_PLANETARY_ACOUSTIC_BREACH: u64 = 1 << 59;
pub const SHBT_SWARM_FAULT_BZ_JET_LEAKAGE: u64 = 1 << 60;
pub const SHBT_SWARM_FAULT_V10_MASK: u64 = 0x001F_C000_0000_0000;

// Nominal v7 Hardware Thresholds
pub const SHBT_SWARM_CHIRAL_EE_Q16_HOMOCHIRAL: u32 = 0x0001_0000;
pub const SHBT_SWARM_KERR_PHASE_MARGIN_MIN_RAD: f32 = 0.150_000;
pub const SHBT_SWARM_PACHNER_COCYCLE_CLEAN: u32 = 0x0000_0000;
pub const SHBT_SWARM_ATMOS_SHEAR_MAX_PA: f32 = 45.000_000;
pub const SHBT_SWARM_VORTEX_DRIFT_MAX_MPS: f64 = 1.0000e-15;
pub const SHBT_SWARM_POYNTING_LEAKAGE_MAX_W: f64 = 0.0;
pub const SHBT_SWARM_MOSSBAUER_FACTOR_MIN: f64 = 0.782_000;
pub const SHBT_SWARM_BORRMANN_TAU_MIN: f64 = 1_453.4;
pub const SHBT_SWARM_BINDING_PURITY_NOMINAL: f32 = 1.000_000;
pub const SHBT_SWARM_BRANCH_ENTROPY_DEBT_ZERO: f32 = 0.000_000;

#[repr(C, align(64))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShbtSwarmMmioRegs {
    // Cache Line 0 (0x0000..0x003F)
    pub stinespring_dma_phys_addr: u64,
    pub fault_status_mask: u64,
    pub active_ledger_bits: u64,
    pub dark_ledger_bits: u64,
    pub pt_goi_spectral_radius: f64,
    pub pt_goi_latency_ps: f64,
    pub sheaf_betti_h0: u32,
    pub sheaf_betti_h1: u32,
    pub bio_folding_window_ns: u32,
    pub cloner_mode_select: u32,

    // Cache Line 1 (0x0040..0x007F)
    pub mar312x_power_watts: f64,
    pub mar312x_magnetic_field_tesla: f64,
    pub mar312x_coil_temp_kelvin: f64,
    pub mar312x_hoop_stress_mpa: f64,
    pub foundry_mass_injection_kgps: f64,
    pub foundry_surface_temp_kelvin: f64,
    pub foundry_surface_pressure_bar: f64,
    pub architect_wyckoff_group_mask: u64,

    // Cache Line 2 (0x0080..0x00BF)
    pub omega_smbh_spin_param: f64,
    pub omega_horizon_area_deriv: f64,
    pub logos_mossbauer_factor_fm: f64,
    pub logos_borrmann_tau: f64,
    pub aegis_tmsv_squeezing_db: f64,
    pub apoth_ego_continuity_fidelity: f64,
    pub apoth_simulation_epoch_yr_log10: u64,
    pub pcss_crowbar_latency_ps: u64,

    // Cache Line 3 (0x00C0..0x00FF)
    pub abi_version: u32,
    pub formal_proofs_unsat_count: u32,
    pub hil_gates_pass_count: u32,
    pub active_phase_mask: u32,
    pub heartbeat_monotonic_ticks: u64,
    pub reserved_v6_pad: [u64; 5],

    // Cache Line 4 (0x0100..0x013F): ABI v7 Extension Bank
    pub chiral_ee_q16: u32,
    pub kerr_phase_margin_rad: f32,
    pub pachner_flip_count: u32,
    pub pachner_cocycle_status: u32,
    pub fluid_cft_reynolds_stress_max: f32,
    pub v7_align_guard: u32,
    pub clathrate_vortex_drift_velocity: f64,
    pub smbh_poynting_leakage_watts: f64,
    pub mossbauer_stark_detuning_hz: f64,
    pub phenomenal_binding_purity: f32,
    pub branch_prune_entropy_debt: f32,
    pub v7_reserved_tail: u64,

    // Cache Line 5 (0x0140..0x017F): ABI v8 Reserved Pad (64 bytes)
    pub reserved_v8_pad: [u64; 8],

    // Cache Line 6 (0x0180..0x01BF): ABI v9 Target Research Closures (64 bytes)
    pub gw_memory_residual_strain_norm: f64,
    pub secular_com_drift_meters: f64,
    pub continuous_ribbon_discretization_error: f64,
    pub ribbon_kz_connection_residual: f64,
    pub qrf_superposed_sheaf_spectral_radius: f32,
    pub qrf_microcausality_trace_fidelity: f32,
    pub v9_reserved_tail: [u64; 3],

    // Cache Line 7 (0x01C0..0x01FF): ABI v10 Research Closures (64 bytes)
    pub eddington_damping_factor: f32,
    pub core_adiabatic_index_gamma1: f32,
    pub hawking_area_delta_margin: f64,
    pub galactic_virial_drift_norm: f64,
    pub qgp_floquet_debye_mass_fm: f32,
    pub planetary_gst_reflection_coeff: f32,
    pub dark_ledger_anyon_braid_count: u64,
    pub bz_jet_attenuation_db: u32,
    pub v10_reserved_tail: [u32; 5],
}

const _: () = {
    assert!(size_of::<ShbtSwarmMmioRegs>() == 512);
    assert!(align_of::<ShbtSwarmMmioRegs>() == 64);
    assert!(offset_of!(ShbtSwarmMmioRegs, abi_version) == 0x00C0);
    assert!(offset_of!(ShbtSwarmMmioRegs, chiral_ee_q16) == 0x0100);
    assert!(offset_of!(ShbtSwarmMmioRegs, clathrate_vortex_drift_velocity) == 0x0118);
    assert!(offset_of!(ShbtSwarmMmioRegs, branch_prune_entropy_debt) == 0x0134);
    assert!(offset_of!(ShbtSwarmMmioRegs, gw_memory_residual_strain_norm) == 0x0180);
    assert!(offset_of!(ShbtSwarmMmioRegs, continuous_ribbon_discretization_error) == 0x0190);
    assert!(offset_of!(ShbtSwarmMmioRegs, qrf_superposed_sheaf_spectral_radius) == 0x01A0);
    assert!(offset_of!(ShbtSwarmMmioRegs, eddington_damping_factor) == 0x01C0);
    assert!(offset_of!(ShbtSwarmMmioRegs, dark_ledger_anyon_braid_count) == 0x01E0);
};

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShbtHarvesterNodeTelemetry {
    pub radial_distance_m: u32,
    pub polar_angle_mrad: i16,
    pub azimuth_angle_mrad: i16,
    pub synthetic_shift_mhz: u16,
}

const _: () = {
    assert!(size_of::<ShbtHarvesterNodeTelemetry>() == 10);
};

#[repr(C, align(256))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShbtGalacticApertureMmioRegs {
    pub magic_id: u64,
    pub control_reg: u32,
    pub status_reg: u32,
    pub uptime_ticks: u64,
    pub error_code: u32,
    pub firmware_revision: u32,

    pub bh_mass_kg: f64,
    pub bh_spin_dimless: f64,
    pub horizon_radius_m: f64,
    pub ergosurface_radius_m: f64,

    pub nodes: [ShbtHarvesterNodeTelemetry; 6],
    pub constellation_sync_phase: u32,

    pub poloidal_current_amps: f64,
    pub quench_counter_pot_v: f64,
    pub synthetic_impedance: f64,
    pub jet_attenuation_db: u32,
    pub reserved_bz: u32,

    pub total_extracted_joules: f64,
    pub dark_ledger_joules: f64,
    pub grid_baseload_joules: f64,
    pub anyon_braid_count_lo: u64,
    pub anyon_braid_count_hi: u64,
    pub modular_flow_phase_rad: u32,
    pub dark_partition_ratio: u32,
    pub zero_volume_proof_hash: u64,
    pub reserved_dark: u64,

    pub hmac_signature: [u8; 16],
    pub ecc_checksum: u64,
    pub frame_canary: u64,
}

const _: () = {
    assert!(size_of::<ShbtGalacticApertureMmioRegs>() == 256);
    assert!(align_of::<ShbtGalacticApertureMmioRegs>() == 256);
};

#[repr(C, align(8))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShbtDenseMatterMmioRegs {
    pub engine_ctrl: u64,
    pub engine_status: u64,
    pub wzw_algebra_cfg: u64,
    pub floquet_freq_mev: u64,
    pub floquet_debye_mass: u64,
    pub gamma_power_uw: u64,
    pub central_density_kg_m3: u64,
    pub central_pressure_pa: u64,
    pub adiabatic_index_fixed: u64,
    pub mass_siphon_rate_kg_s: u64,
    pub magnetar_b_field_tesla: u64,
    pub dark_ledger_drain_ratio: u64,
    pub b_minus_l_accumulator: i64,
    pub visible_neutrino_flux: u64,
    pub activation_xsec_exponent: i64,
    pub crystal_space_group: u64,
    pub peak_crystal_temp_mk: u64,
    pub heegaard_floer_shock_factor: u64,
    pub borrmann_defect_density: u64,
    pub aperture_command_trigger: u64,
}

const _: () = {
    assert!(size_of::<ShbtDenseMatterMmioRegs>() == 160);
};

#[repr(C, align(64))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShbtNestedSimMmioRegs {
    pub root_ctrl: u64,
    pub depth_current: u64,
    pub merkle_seed: u64,
    pub _pad0: u64,
    pub entropy_budget: u64,
    pub _pad1: u64,
    pub safeword_latch: u64,
    pub _pad2: u64,
    pub cbf_trip_status: u64,
    pub _pad3: [u64; 7],
}

const _: () = {
    assert!(size_of::<ShbtNestedSimMmioRegs>() == 128);
    assert!(align_of::<ShbtNestedSimMmioRegs>() == 64);
    assert!(offset_of!(ShbtNestedSimMmioRegs, root_ctrl) == 0x00);
    assert!(offset_of!(ShbtNestedSimMmioRegs, depth_current) == 0x08);
    assert!(offset_of!(ShbtNestedSimMmioRegs, merkle_seed) == 0x10);
    assert!(offset_of!(ShbtNestedSimMmioRegs, entropy_budget) == 0x20);
    assert!(offset_of!(ShbtNestedSimMmioRegs, safeword_latch) == 0x30);
    assert!(offset_of!(ShbtNestedSimMmioRegs, cbf_trip_status) == 0x40);
};

macro_rules! const_assert {
    ($x:expr $(,)?) => {
        const _: () = assert!($x);
    };
}

pub const SHBT_MMIO_ABI_VERSION: u32 = 0x0008_0312;
pub const SHBT_MMIO_DENSE_BASE: u64 = 0x7F00_0000;
pub const SHBT_MMIO_QUENCH_BASE: u64 = 0x7F00_0100;
pub const SHBT_MMIO_SHUNT_BASE: u64 = 0x7F00_0200;
pub const SHBT_MMIO_HIL_DENSE_BASE: u64 = 0x7F00_0300;
pub const SHBT_MMIO_GALACTIC_BASE: u64 = 0x8F00_0000;

pub const MULTIPHYSICS_13_ENGINES_MASK: u32 = 0x0000_1FFF;
pub const HIL_GATES_233_241_MASK: u32 = 0x0000_01FF;
pub const HIL_GATES_242_250_MASK: u32 = 0x0000_01FF;

/* Dense Matter Siphon & Color Screening (0x7F000000) */
#[repr(C, align(8))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShbtMmioDenseTransmute {
    pub abi_version: u32,        /* 0x00: 0x00080312 */
    pub status_flags: u32,       /* 0x04: Bit 0=Enable, Bit 1=Trip */
    pub current_rho_b: f64,      /* 0x08: Central density (kg/m^3) */
    pub commanded_m_dot: f64,    /* 0x10: Commanded M_dot (kg/s) */
    pub rf_field_amplitude: f64, /* 0x18: E_RF (V/m) */
    pub drive_freq_omega: f64,   /* 0x20: Omega (rad/s) */
    pub eval_debye_mass: f64,    /* 0x28: mu_D (fm^-1) */
    pub screen_potential: f64,   /* 0x30: U_screen (MeV) */
    pub evaluated_gamma_1: f64,  /* 0x38: Relativistic Gamma_1 */
    pub fluid_drift_v_c: f64,    /* 0x40: v_fluid / c */
    pub pasta_phase: u32,        /* 0x48: 1=Gnocchi, 2=Spaghetti, 3=Lasagna, 4=Fluid */
    pub _pad_tail: u32,          /* 0x4C: 8-byte alignment pad */
}

/* Magnetar Helicity Quencher (0x7F000100) */
#[repr(C, align(8))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShbtMmioMagnetarQuench {
    pub control_bits: u32,       /* 0x00: Bit 0=Arm, Bit 1=Quench */
    pub status_enum: u32,        /* 0x04: QuenchStatus */
    pub b_init_tesla: f64,       /* 0x08: Initial B (T) */
    pub b_current_tesla: f64,    /* 0x10: Current B (T) */
    pub current_db_down: f64,    /* 0x18: Attenuation (dB) */
    pub dissipation_rate: f64,   /* 0x20: dU_B/dt (W) */
    pub topological_w: i64,      /* 0x28: pi_3(SU(2)) invariant */
    pub run_duration_s: f64,     /* 0x30: Elapsed time (s) */
}

/* Lepton Shunt & 25 EW CFT Bus (0x7F000200) */
#[repr(C, align(8))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShbtMmioLeptonShunt {
    pub coupling_mode: u32,      /* 0x00: ShuntCouplingMode */
    pub phase_locked: u32,       /* 0x04: 1=Locked */
    pub theta_k_rad: f64,        /* 0x08: Bogoliubov angle (rad) */
    pub phi_nu_observed: f64,    /* 0x10: Neutrino flux (cm^-2 s^-1) */
    pub cft_power_output_w: f64, /* 0x18: 25 EW Bus Power (W) */
    pub dark_debt_joules: f64,   /* 0x20: Cumulative dark exergy */
    pub disassembled_count: u64, /* 0x28: Baryons shunted */
}

/* Galactic Astrodynamics Engine (0x8F000000) */
#[repr(C, align(8))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShbtGalacticMmio {
    pub symplectic_ctrl: u32,    /* 0x00: Bit 0=Engage, Bit 1=Reset */
    pub symplectic_status: u32,  /* 0x04: Bit 0=Preserved (d_omega == 0) */
    pub cascade_ratio: u64,      /* 0x08: alpha_casc (default 1e12) */
    pub energy_drift_rel: f64,   /* 0x10: Delta H / H(0) */
    pub ccz4_kappa1: f64,        /* 0x18: Gundlach damping parameter */
    pub ccz4_z_norm: f64,        /* 0x20: ||Z_i|| constraint norm */
    pub gw_mem_residual: f64,    /* 0x28: ||h_ij^mem + h_ij^counter|| */
    pub com_drift_x: f64,        /* 0x30: CoM drift X (m) */
    pub com_drift_y: f64,        /* 0x38: CoM drift Y (m) */
    pub com_drift_z: f64,        /* 0x40: CoM drift Z (m) */
    pub cs_level_k: u64,         /* 0x48: Level K (312) */
    pub cs_winding_x: i64,       /* 0x50: Sp(2g, Z) Winding X */
    pub cs_winding_y: i64,       /* 0x58: Sp(2g, Z) Winding Y */
    pub cs_winding_z: i64,       /* 0x60: Sp(2g, Z) Winding Z */
    pub shift_anomaly_norm: f64, /* 0x68: ||beta^i||_inf */
    pub multiphysics_bits: u32,  /* 0x70: Bits 0..12 engine status */
    pub hil_gate_status: u32,    /* 0x74: Gates 242-250 pass mask */
}

const_assert!(size_of::<ShbtMmioDenseTransmute>() == 80);
const_assert!(align_of::<ShbtMmioDenseTransmute>() == 8);
const_assert!(offset_of!(ShbtMmioDenseTransmute, abi_version) == 0x00);
const_assert!(offset_of!(ShbtMmioDenseTransmute, status_flags) == 0x04);
const_assert!(offset_of!(ShbtMmioDenseTransmute, current_rho_b) == 0x08);
const_assert!(offset_of!(ShbtMmioDenseTransmute, commanded_m_dot) == 0x10);
const_assert!(offset_of!(ShbtMmioDenseTransmute, rf_field_amplitude) == 0x18);
const_assert!(offset_of!(ShbtMmioDenseTransmute, drive_freq_omega) == 0x20);
const_assert!(offset_of!(ShbtMmioDenseTransmute, eval_debye_mass) == 0x28);
const_assert!(offset_of!(ShbtMmioDenseTransmute, screen_potential) == 0x30);
const_assert!(offset_of!(ShbtMmioDenseTransmute, evaluated_gamma_1) == 0x38);
const_assert!(offset_of!(ShbtMmioDenseTransmute, fluid_drift_v_c) == 0x40);
const_assert!(offset_of!(ShbtMmioDenseTransmute, pasta_phase) == 0x48);

const_assert!(size_of::<ShbtMmioMagnetarQuench>() == 56);
const_assert!(align_of::<ShbtMmioMagnetarQuench>() == 8);
const_assert!(offset_of!(ShbtMmioMagnetarQuench, control_bits) == 0x00);
const_assert!(offset_of!(ShbtMmioMagnetarQuench, status_enum) == 0x04);
const_assert!(offset_of!(ShbtMmioMagnetarQuench, b_init_tesla) == 0x08);
const_assert!(offset_of!(ShbtMmioMagnetarQuench, b_current_tesla) == 0x10);
const_assert!(offset_of!(ShbtMmioMagnetarQuench, current_db_down) == 0x18);
const_assert!(offset_of!(ShbtMmioMagnetarQuench, dissipation_rate) == 0x20);
const_assert!(offset_of!(ShbtMmioMagnetarQuench, topological_w) == 0x28);
const_assert!(offset_of!(ShbtMmioMagnetarQuench, run_duration_s) == 0x30);

const_assert!(size_of::<ShbtMmioLeptonShunt>() == 48);
const_assert!(align_of::<ShbtMmioLeptonShunt>() == 8);
const_assert!(offset_of!(ShbtMmioLeptonShunt, coupling_mode) == 0x00);
const_assert!(offset_of!(ShbtMmioLeptonShunt, phase_locked) == 0x04);
const_assert!(offset_of!(ShbtMmioLeptonShunt, theta_k_rad) == 0x08);
const_assert!(offset_of!(ShbtMmioLeptonShunt, phi_nu_observed) == 0x10);
const_assert!(offset_of!(ShbtMmioLeptonShunt, cft_power_output_w) == 0x18);
const_assert!(offset_of!(ShbtMmioLeptonShunt, dark_debt_joules) == 0x20);
const_assert!(offset_of!(ShbtMmioLeptonShunt, disassembled_count) == 0x28);

const_assert!(size_of::<ShbtGalacticMmio>() == 120);
const_assert!(align_of::<ShbtGalacticMmio>() == 8);
const_assert!(offset_of!(ShbtGalacticMmio, symplectic_ctrl) == 0x00);
const_assert!(offset_of!(ShbtGalacticMmio, symplectic_status) == 0x04);
const_assert!(offset_of!(ShbtGalacticMmio, cascade_ratio) == 0x08);
const_assert!(offset_of!(ShbtGalacticMmio, energy_drift_rel) == 0x10);
const_assert!(offset_of!(ShbtGalacticMmio, ccz4_kappa1) == 0x18);
const_assert!(offset_of!(ShbtGalacticMmio, ccz4_z_norm) == 0x20);
const_assert!(offset_of!(ShbtGalacticMmio, gw_mem_residual) == 0x28);
const_assert!(offset_of!(ShbtGalacticMmio, com_drift_x) == 0x30);
const_assert!(offset_of!(ShbtGalacticMmio, com_drift_y) == 0x38);
const_assert!(offset_of!(ShbtGalacticMmio, com_drift_z) == 0x40);
const_assert!(offset_of!(ShbtGalacticMmio, cs_level_k) == 0x48);
const_assert!(offset_of!(ShbtGalacticMmio, cs_winding_x) == 0x50);
const_assert!(offset_of!(ShbtGalacticMmio, cs_winding_y) == 0x58);
const_assert!(offset_of!(ShbtGalacticMmio, cs_winding_z) == 0x60);
const_assert!(offset_of!(ShbtGalacticMmio, shift_anomaly_norm) == 0x68);
const_assert!(offset_of!(ShbtGalacticMmio, multiphysics_bits) == 0x70);
const_assert!(offset_of!(ShbtGalacticMmio, hil_gate_status) == 0x74);


impl ShbtSwarmMmioRegs {
    #[inline]
    #[allow(clippy::approx_constant)]
    pub const fn new_nominal_v7() -> Self {
        Self {
            stinespring_dma_phys_addr: SHBT_SWARM_STINESPRING_BASE_ADDR,
            fault_status_mask: SHBT_SWARM_FAULT_NONE,
            active_ledger_bits: 10_000_000_000,
            dark_ledger_bits: 23_000_000_000,
            pt_goi_spectral_radius: 1.0e-62,
            pt_goi_latency_ps: 54.432,
            sheaf_betti_h0: 1,
            sheaf_betti_h1: 0,
            bio_folding_window_ns: 11_400,
            cloner_mode_select: 2,
            mar312x_power_watts: 2.50e19,
            mar312x_magnetic_field_tesla: 48.0,
            mar312x_coil_temp_kelvin: 77.0,
            mar312x_hoop_stress_mpa: 312.5,
            foundry_mass_injection_kgps: 1.0e15,
            foundry_surface_temp_kelvin: 298.15,
            foundry_surface_pressure_bar: 1.01325,
            architect_wyckoff_group_mask: u64::MAX,
            omega_smbh_spin_param: 0.998,
            omega_horizon_area_deriv: 1.25e-4,
            logos_mossbauer_factor_fm: 0.7854,
            logos_borrmann_tau: 1462.8,
            aegis_tmsv_squeezing_db: 21.715,
            apoth_ego_continuity_fidelity: 1.0,
            apoth_simulation_epoch_yr_log10: 105,
            pcss_crowbar_latency_ps: 1850,
            abi_version: SHBT_SWARM_ABI_VERSION,
            formal_proofs_unsat_count: SHBT_SWARM_FORMAL_PROOFS_COUNT,
            hil_gates_pass_count: SHBT_SWARM_HIL_GATES_COUNT,
            active_phase_mask: 0x0000_007F,
            heartbeat_monotonic_ticks: 1,
            reserved_v6_pad: [0; 5],
            chiral_ee_q16: SHBT_SWARM_CHIRAL_EE_Q16_HOMOCHIRAL,
            kerr_phase_margin_rad: 0.785_398_2,
            pachner_flip_count: 0,
            pachner_cocycle_status: SHBT_SWARM_PACHNER_COCYCLE_CLEAN,
            fluid_cft_reynolds_stress_max: 4.6818,
            v7_align_guard: 0,
            clathrate_vortex_drift_velocity: 1.31e-33,
            smbh_poynting_leakage_watts: 0.0,
            mossbauer_stark_detuning_hz: -1.6085e8,
            phenomenal_binding_purity: SHBT_SWARM_BINDING_PURITY_NOMINAL,
            branch_prune_entropy_debt: SHBT_SWARM_BRANCH_ENTROPY_DEBT_ZERO,
            v7_reserved_tail: 0,
            reserved_v8_pad: [0; 8],
            gw_memory_residual_strain_norm: 0.0,
            secular_com_drift_meters: 0.0,
            continuous_ribbon_discretization_error: 0.0,
            ribbon_kz_connection_residual: 0.0,
            qrf_superposed_sheaf_spectral_radius: 1.0,
            qrf_microcausality_trace_fidelity: 1.0,
            v9_reserved_tail: [0; 3],
            eddington_damping_factor: 1.0,
            core_adiabatic_index_gamma1: 4.0 / 3.0,
            hawking_area_delta_margin: 0.0,
            galactic_virial_drift_norm: 0.0,
            qgp_floquet_debye_mass_fm: 0.25,
            planetary_gst_reflection_coeff: 0.0,
            dark_ledger_anyon_braid_count: 0,
            bz_jet_attenuation_db: 120,
            v10_reserved_tail: [0; 5],
        }
    }

    #[inline]
    #[allow(clippy::approx_constant)]
    pub const fn new_nominal_v9() -> Self {
        Self::new_nominal_v7()
    }

    #[inline]
    #[allow(clippy::approx_constant)]
    pub const fn new_nominal_v10() -> Self {
        Self::new_nominal_v7()
    }

    #[inline]
    pub fn audit_v7_invariants(&mut self) -> u64 {
        let mut faults = SHBT_SWARM_FAULT_NONE;
        if self.abi_version != SHBT_SWARM_ABI_VERSION {
            faults |= SHBT_SWARM_FAULT_ABI_MISMATCH;
        }
        if self.chiral_ee_q16 < SHBT_SWARM_CHIRAL_EE_Q16_HOMOCHIRAL {
            faults |= SHBT_SWARM_FAULT_CHIRAL_RACEMIC;
        }
        if self.kerr_phase_margin_rad < SHBT_SWARM_KERR_PHASE_MARGIN_MIN_RAD
            || self.kerr_phase_margin_rad.is_nan()
        {
            faults |= SHBT_SWARM_FAULT_LLE_CHAOS;
        }
        if self.pachner_cocycle_status != SHBT_SWARM_PACHNER_COCYCLE_CLEAN || self.sheaf_betti_h1 != 0 {
            faults |= SHBT_SWARM_FAULT_PACHNER_COCYCLE;
        }
        if self.fluid_cft_reynolds_stress_max > SHBT_SWARM_ATMOS_SHEAR_MAX_PA
            || self.fluid_cft_reynolds_stress_max.is_nan()
        {
            faults |= SHBT_SWARM_FAULT_ATMOS_SHOCK;
        }
        if self.clathrate_vortex_drift_velocity < 0.0
            || self.clathrate_vortex_drift_velocity > SHBT_SWARM_VORTEX_DRIFT_MAX_MPS
            || self.clathrate_vortex_drift_velocity.is_nan()
        {
            faults |= SHBT_SWARM_FAULT_FLUX_JUMP;
        }
        if self.smbh_poynting_leakage_watts.abs() > 1.0e-12 {
            faults |= SHBT_SWARM_FAULT_POYNTING_JET;
        }
        if self.logos_mossbauer_factor_fm < SHBT_SWARM_MOSSBAUER_FACTOR_MIN
            || self.logos_borrmann_tau < SHBT_SWARM_BORRMANN_TAU_MIN
            || self.logos_mossbauer_factor_fm.is_nan()
            || self.logos_borrmann_tau.is_nan()
        {
            faults |= SHBT_SWARM_FAULT_MOSSBAUER_DETUNE;
        }
        if (self.phenomenal_binding_purity - 1.0).abs() > 1.0e-6
            || self.branch_prune_entropy_debt.abs() > 0.0
        {
            faults |= SHBT_SWARM_FAULT_BINDING_DECOHERENCE;
        }
        self.fault_status_mask |= faults;
        faults
    }

    #[inline]
    pub fn audit_v9_invariants(&mut self) -> u64 {
        let mut faults = self.audit_v7_invariants();
        if self.gw_memory_residual_strain_norm > 1.0e-120 || self.gw_memory_residual_strain_norm.is_nan() {
            faults |= SHBT_SWARM_FAULT_GW_MEMORY_STRAIN_TRIP;
        }
        if self.secular_com_drift_meters > 1.0e-15 || self.secular_com_drift_meters.is_nan() {
            faults |= SHBT_SWARM_FAULT_COM_HALO_DRIFT_TRIP;
        }
        if self.ribbon_kz_connection_residual > 1.0e-120 || self.ribbon_kz_connection_residual.is_nan() {
            faults |= SHBT_SWARM_FAULT_RIBBON_KZ_SINGULARITY;
        }
        if self.continuous_ribbon_discretization_error != 0.0 || self.continuous_ribbon_discretization_error.is_nan() {
            faults |= SHBT_SWARM_FAULT_DISCRETIZATION_NONZERO;
        }
        if (self.qrf_superposed_sheaf_spectral_radius - 1.0).abs() > 1.0e-6 || self.qrf_superposed_sheaf_spectral_radius.is_nan() {
            faults |= SHBT_SWARM_FAULT_SHEAF_SPECTRUM_DIV;
        }
        if self.qrf_microcausality_trace_fidelity < 0.999999 || self.qrf_microcausality_trace_fidelity.is_nan() {
            faults |= SHBT_SWARM_FAULT_METRIC_SUP_CAUSALITY;
        }
        self.fault_status_mask |= faults;
        faults
    }

    #[inline]
    pub fn audit_v10_invariants(&mut self) -> u64 {
        let mut faults = self.audit_v9_invariants();
        if self.eddington_damping_factor < 0.0 || self.eddington_damping_factor > 1.0 || self.eddington_damping_factor.is_nan() {
            faults |= SHBT_SWARM_FAULT_EDDINGTON_OVERPRESSURE;
        }
        if self.core_adiabatic_index_gamma1 < (4.0 / 3.0) || self.core_adiabatic_index_gamma1.is_nan() {
            faults |= SHBT_SWARM_FAULT_CORE_ADIABAT_COLLAPSE;
        }
        if self.hawking_area_delta_margin < 0.0 || self.hawking_area_delta_margin.is_nan() {
            faults |= SHBT_SWARM_FAULT_HAWKING_AREA_DECREASE;
        }
        if self.galactic_virial_drift_norm > 1.0e-12 || self.galactic_virial_drift_norm.is_nan() {
            faults |= SHBT_SWARM_FAULT_HALO_VIRIAL_JEANS_DRIFT;
        }
        if self.qgp_floquet_debye_mass_fm < 0.15 || self.qgp_floquet_debye_mass_fm.is_nan() {
            faults |= SHBT_SWARM_FAULT_FLOQUET_COLOR_UNLOCKED;
        }
        if self.planetary_gst_reflection_coeff > 0.05 || self.planetary_gst_reflection_coeff.is_nan() {
            faults |= SHBT_SWARM_FAULT_PLANETARY_ACOUSTIC_BREACH;
        }
        if self.bz_jet_attenuation_db < 60 {
            faults |= SHBT_SWARM_FAULT_BZ_JET_LEAKAGE;
        }
        self.fault_status_mask |= faults;
        faults
    }
}

/// Volatile MMIO controller supporting both bare-metal physical addresses (0x70000000)
/// and host-aligned simulation arenas.
pub struct SwarmMmioController {
    regs_ptr: *mut shbt_swarm_core::ShbtSwarmMmioRegs,
    hyperframe_ptr: *mut UnifiedStinespringHyperFrame,
}

impl SwarmMmioController {
    /// Creates a bare-metal controller pointing directly to 0x70000000 and 0x70020000.
    ///
    /// # Safety
    /// Caller must ensure execution is on `shbt-os` bare-metal hardware with MMIO mapped.
    #[inline]
    pub const unsafe fn from_hardware_base() -> Self {
        Self {
            regs_ptr: 0x7000_0000 as *mut shbt_swarm_core::ShbtSwarmMmioRegs,
            hyperframe_ptr: SHBT_SWARM_HYPERFRAME_BASE_ADDR as *mut UnifiedStinespringHyperFrame,
        }
    }

    /// Creates a controller backed by host-aligned memory buffers for deterministic verification.
    #[inline]
    pub fn from_backing(
        regs: &mut shbt_swarm_core::ShbtSwarmMmioRegs,
        hyperframe: &mut UnifiedStinespringHyperFrame,
    ) -> Self {
        Self {
            regs_ptr: regs as *mut shbt_swarm_core::ShbtSwarmMmioRegs,
            hyperframe_ptr: hyperframe as *mut UnifiedStinespringHyperFrame,
        }
    }

    #[inline]
    pub fn init_v4_defaults(&mut self) {
        unsafe {
            write_volatile(&mut (*self.regs_ptr).magic_id, MAGIC_SWRM);
            write_volatile(&mut (*self.regs_ptr).abi_version, ABI_VERSION_V4);
            write_volatile(&mut (*self.regs_ptr).cmd_reg, SHBT_SWARM_CMD_NOP);
            write_volatile(&mut (*self.regs_ptr).status_reg, 0);
            write_volatile(&mut (*self.regs_ptr).fault_mask_reg, 0);
            write_volatile(&mut (*self.regs_ptr).rom_bank_mask, 0x0000_0001);
            write_volatile(&mut (*self.regs_ptr).synth_bank_mask, 0x0000_0001);
            write_volatile(&mut (*self.regs_ptr).power_tier_sel, 1);
            write_volatile(&mut (*self.regs_ptr).generation_counter, 0);
            write_volatile(&mut (*self.regs_ptr).temp_qc_mk, 4200);
            write_volatile(&mut (*self.regs_ptr).temp_nacelle_mk, 21130);
        }
    }

    #[inline]
    pub fn read_magic_and_abi(&self) -> (u32, u32) {
        unsafe {
            let magic = read_volatile(&(*self.regs_ptr).magic_id);
            let abi = read_volatile(&(*self.regs_ptr).abi_version);
            (magic, abi)
        }
    }

    #[inline]
    pub fn read_cmd(&self) -> u32 {
        unsafe { read_volatile(&(*self.regs_ptr).cmd_reg) }
    }

    #[inline]
    pub fn issue_cmd(&mut self, cmd: u32) -> Result<(), u32> {
        unsafe {
            let faults = read_volatile(&(*self.regs_ptr).fault_mask_reg);
            if faults != 0 {
                return Err(faults);
            }
            write_volatile(&mut (*self.regs_ptr).cmd_reg, cmd);
            Ok(())
        }
    }

    #[inline]
    pub fn read_status(&self) -> u32 {
        unsafe { read_volatile(&(*self.regs_ptr).status_reg) }
    }

    #[inline]
    pub fn read_fault_mask(&self) -> u32 {
        unsafe { read_volatile(&(*self.regs_ptr).fault_mask_reg) }
    }

    #[inline]
    pub fn write_fault_mask(&mut self, fault: u32) {
        unsafe {
            write_volatile(&mut (*self.regs_ptr).fault_mask_reg, fault);
            if fault != 0 {
                // Instantly trip hardware crowbar interlock
                write_volatile(&mut (*self.regs_ptr).cmd_reg, SHBT_SWARM_CMD_WEYL_EXCITON_CROWBAR);
            }
        }
    }

    #[inline]
    pub fn set_triage_class(&mut self, triage: u32) {
        unsafe {
            let cur = read_volatile(&(*self.regs_ptr).status_reg);
            let updated = (cur & !SHBT_SWARM_TRIAGE_MASK) | (triage & SHBT_SWARM_TRIAGE_MASK);
            write_volatile(&mut (*self.regs_ptr).status_reg, updated);
        }
    }

    #[inline]
    pub fn read_triage_class(&self) -> u32 {
        unsafe {
            let status = read_volatile(&(*self.regs_ptr).status_reg);
            status & SHBT_SWARM_TRIAGE_MASK
        }
    }
}

/// Continuous Causal Awareness State for Phase 6 Observer Ascension (128 bytes, aligned 64).
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
pub struct ShbtCausalObserverFrame {
    pub observer_uuid: u64,
    pub s_adiabatic_ramp: f64,
    pub alpha_hyper_clock: f64,
    pub c_local_capacity_log10: f64,
    pub h_syn_cbf_margin: f64,
    pub f_ego_continuity: f64,
    pub active_avatar_count: u32,
    pub somatic_stasis_engaged: u32,
    pub nested_recursion_depth_d: u64,
    pub burau_cayley_memory_hash: [u64; 4],
    pub stinespring_vault_ptr: u64,
    pub reserved_pad: u64,
}

impl Default for ShbtCausalObserverFrame {
    fn default() -> Self {
        Self {
            observer_uuid: 0xDEAD_BEEF_CAFE_0001,
            s_adiabatic_ramp: 0.0,
            alpha_hyper_clock: 1.0,
            c_local_capacity_log10: 122.5,
            h_syn_cbf_margin: 1.0,
            f_ego_continuity: 1.0,
            active_avatar_count: 1,
            somatic_stasis_engaged: 0,
            nested_recursion_depth_d: 0,
            burau_cayley_memory_hash: [0; 4],
            stinespring_vault_ptr: 0x7002_0000,
            reserved_pad: 0,
        }
    }
}

/// Theorem THM-APOTH-05 Vessel Neural-Substrate Admissibility Certificate (64 bytes, aligned 64).
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
pub struct ShbtVesselAdmissibilityCert {
    pub vessel_blueprint_hash_lo: u64,
    pub vessel_blueprint_hash_hi: u64,
    pub phi_substrate_max_log10: f64,
    pub phi_observer_req_log10: f64,
    pub c_local_vessel_log10: f64,
    pub bisimulation_error_log10: f64,
    pub thermal_cbf_margin: f64,
    pub space_group_id: u8,
    pub standalone_admissible_flag: u8,
    pub omnibus_tether_engaged: u8,
    pub non_harm_cbf_verified: u8,
    pub cert_crc32c: u32,
}

impl Default for ShbtVesselAdmissibilityCert {
    fn default() -> Self {
        Self {
            vessel_blueprint_hash_lo: 0,
            vessel_blueprint_hash_hi: 0,
            phi_substrate_max_log10: 16.0,
            phi_observer_req_log10: 12.0,
            c_local_vessel_log10: 60.0,
            bisimulation_error_log10: -125.0,
            thermal_cbf_margin: 1.0,
            space_group_id: 227,
            standalone_admissible_flag: 1,
            omnibus_tether_engaged: 0,
            non_harm_cbf_verified: 1,
            cert_crc32c: 0,
        }
    }
}

/// Top-Level Apotheosis Control Aperture at 0x4000C000 (256 bytes, aligned 64).
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy, Default)]
pub struct ShbtApotheosisCtrl {
    pub ctrl_cmd: u32,
    pub sandbox_status_latch: u32,
    pub fault_code: u32,
    pub lagrangian_submanifold: u32,
    pub phi_mera_max_detected: f64,
    pub x1_cbf_min_margin: f64,
    pub x1_sanctuary_fork_cnt: u64,
    pub goi_pass_counter: u64,
    pub reserved_hdr: [u64; 2],
    pub observer: ShbtCausalObserverFrame,
    pub vessel: ShbtVesselAdmissibilityCert,
}

/// Extended MMIO registers and structure for Phase 5A (0x40008060, 32 bytes).
#[repr(C, align(8))]
#[derive(Debug, Clone, Copy, Default)]
pub struct ShbtOmegaExtCtrl {
    pub smbh_horizon_area_fp: u64,
    pub halo_virial_dispersion_fp: u64,
    pub smbh_ergosphere_spin_reg: u32,
    pub smbh_siphon_status_latch: u32,
    pub reserved_omega_ext: u64,
}

/// Extended MMIO registers and structure for Phase 5B (0x4000A060, 32 bytes).
#[repr(C, align(8))]
#[derive(Debug, Clone, Copy, Default)]
pub struct ShbtLogosExtCtrl {
    pub dce_vacuum_photon_flux_fp: u64,
    pub dce_nodal_cancellation_reg: u32,
    pub dce_kagan_afanasev_latch: u32,
    pub reserved_logos_ext: [u64; 2],
}

/// Extended MMIO registers and structure for Phase 6 (0x4000C060, 32 bytes).
#[repr(C, align(8))]
#[derive(Debug, Clone, Copy, Default)]
pub struct ShbtApothExtCtrl {
    pub qualia_betti_signature: u64,
    pub poset_sheaf_holonomy_latch: u32,
    pub cobordism_op_reg: u32,
    pub qualia_entropy_margin_fp: u64,
    pub reserved_apoth_ext: u64,
}

pub const SHBT_OMEGA_REG_SMBH_HORIZON_AREA: u32 = 0x0000_8060;
pub const SHBT_OMEGA_REG_HALO_VIRIAL_DISP: u32 = 0x0000_8068;
pub const SHBT_OMEGA_REG_SMBH_ERGOSPHERE_SPIN: u32 = 0x0000_8070;
pub const SHBT_OMEGA_REG_SMBH_SIPHON_STATUS: u32 = 0x0000_8074;
pub const SHBT_OMEGA_REG_RESERVED_EXT: u32 = 0x0000_8078;

pub const SHBT_LOGOS_REG_DCE_VACUUM_PHOTON_FLUX: u32 = 0x0000_A060;
pub const SHBT_LOGOS_REG_DCE_NODAL_CANCELLATION: u32 = 0x0000_A068;
pub const SHBT_LOGOS_REG_DCE_KAGAN_AFANASEV: u32 = 0x0000_A06C;
pub const SHBT_LOGOS_REG_RESERVED_EXT: u32 = 0x0000_A070;

pub const SHBT_APOTH_REG_QUALIA_BETTI_SIGNATURE: u32 = 0x0000_C060;
pub const SHBT_APOTH_REG_POSET_SHEAF_HOLONOMY: u32 = 0x0000_C068;
pub const SHBT_APOTH_REG_COBORDISM_OP: u32 = 0x0000_C06C;
pub const SHBT_APOTH_REG_QUALIA_ENTROPY_MARGIN: u32 = 0x0000_C070;
pub const SHBT_APOTH_REG_RESERVED_EXT: u32 = 0x0000_C078;

pub const SHBT_OMEGA_CMD_SIPHON_SMBH_START: u32 = 0x0000_0040;
pub const SHBT_OMEGA_CMD_HALO_VIRIAL_CLAMP: u32 = 0x0000_0080;
pub const SHBT_FOUNDRY_CMD_AMORPHOUS_BF_ENGAGE: u32 = 0x0000_0100;
pub const SHBT_LOGOS_CMD_DCE_SUPPRESSION_LOCK: u32 = 0x0000_0200;
pub const SHBT_SHIELD_CMD_POSET_SHEAF_SYNC: u32 = 0x0000_0400;
pub const SHBT_APOTH_CMD_EXEC_COBORDISM_FISS: u32 = 0x0000_0800;
pub const SHBT_APOTH_CMD_EXEC_COBORDISM_FUSE: u32 = 0x0000_1000;

pub const SHBT_SWARM_FAULT_SMBH_AREA_DECREASE: u32 = 0x0002_0000;
pub const SHBT_SWARM_FAULT_HALO_VIRIAL_UNBIND: u32 = 0x0004_0000;
pub const SHBT_SWARM_FAULT_AMORPHOUS_WEYL_SPIKE: u32 = 0x0008_0000;
pub const SHBT_SWARM_FAULT_DCE_PARAMETRIC_TRIP: u32 = 0x0010_0000;
pub const SHBT_SWARM_FAULT_POSET_COCYCLE_OBSTR: u32 = 0x0020_0000;
pub const SHBT_SWARM_FAULT_QUALIA_DISTORTION: u32 = 0x0040_0000;

/// Volatile MMIO controller for the Phase 6 Apotheosis aperture at 0x4000C000.
pub struct ApotheosisMmioController {
    ctrl_ptr: *mut ShbtApotheosisCtrl,
}

impl ApotheosisMmioController {
    /// Creates a bare-metal controller pointing directly to 0x4000C000.
    ///
    /// # Safety
    /// Caller must ensure execution is on bare-metal hardware with MMIO aperture mapped.
    #[inline]
    pub const unsafe fn from_hardware_base() -> Self {
        Self {
            ctrl_ptr: SHBT_APOTH_MMIO_BASE as *mut ShbtApotheosisCtrl,
        }
    }

    /// Creates a controller backed by host memory.
    #[inline]
    pub fn from_backing(ctrl: &mut ShbtApotheosisCtrl) -> Self {
        Self {
            ctrl_ptr: ctrl as *mut ShbtApotheosisCtrl,
        }
    }

    #[inline]
    pub fn read_cmd(&self) -> u32 {
        unsafe { read_volatile(&(*self.ctrl_ptr).ctrl_cmd) }
    }

    #[inline]
    pub fn write_cmd(&mut self, cmd: u32) {
        unsafe { write_volatile(&mut (*self.ctrl_ptr).ctrl_cmd, cmd) }
    }

    #[inline]
    pub fn read_status_latch(&self) -> u32 {
        unsafe { read_volatile(&(*self.ctrl_ptr).sandbox_status_latch) }
    }

    #[inline]
    pub fn write_status_latch(&mut self, latch: u32) {
        unsafe { write_volatile(&mut (*self.ctrl_ptr).sandbox_status_latch, latch) }
    }

    #[inline]
    pub fn read_fault_code(&self) -> u32 {
        unsafe { read_volatile(&(*self.ctrl_ptr).fault_code) }
    }

    #[inline]
    pub fn write_fault_code(&mut self, fault: u32) {
        unsafe { write_volatile(&mut (*self.ctrl_ptr).fault_code, fault) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mmio_volatile_simulator_and_crowbar() {
        let mut regs = shbt_swarm_core::ShbtSwarmMmioRegs::default();
        let mut hyper = UnifiedStinespringHyperFrame::default();
        let mut ctrl = SwarmMmioController::from_backing(&mut regs, &mut hyper);
        ctrl.init_v4_defaults();

        assert_eq!(ctrl.read_magic_and_abi(), (MAGIC_SWRM, ABI_VERSION_V4));
        ctrl.set_triage_class(SHBT_SWARM_TRIAGE_X2_ASSIMILATE);
        assert_eq!(ctrl.read_triage_class(), SHBT_SWARM_TRIAGE_X2_ASSIMILATE);

        assert!(ctrl.issue_cmd(SHBT_SWARM_CMD_EXEC_CLONE_SHIP).is_ok());
        assert_eq!(ctrl.read_cmd(), SHBT_SWARM_CMD_EXEC_CLONE_SHIP);

        ctrl.write_fault_mask(SHBT_SWARM_FAULT_ADM_IMBALANCE);
        assert_eq!(ctrl.read_cmd(), SHBT_SWARM_CMD_WEYL_EXCITON_CROWBAR);
    }

    #[test]
    fn test_apotheosis_mmio_struct_sizes_and_controller() {
        assert_eq!(core::mem::size_of::<ShbtCausalObserverFrame>(), 128);
        assert_eq!(core::mem::size_of::<ShbtVesselAdmissibilityCert>(), 64);
        assert_eq!(core::mem::size_of::<ShbtApotheosisCtrl>(), 256);

        let mut ctrl_backing = ShbtApotheosisCtrl::default();
        let mut apoth_ctrl = ApotheosisMmioController::from_backing(&mut ctrl_backing);

        assert_eq!(apoth_ctrl.read_cmd(), 0);
        apoth_ctrl.write_cmd(0x0000_0001); // SHBT_APOTH_CMD_ASCEND
        assert_eq!(apoth_ctrl.read_cmd(), 0x0000_0001);

        apoth_ctrl.write_status_latch(0x0000_0003); // AIRGAP_LOCK | X1_PROTECT_LATCH
        assert_eq!(apoth_ctrl.read_status_latch(), 0x0000_0003);
    }

    #[test]
    fn test_v7_nominal_and_fault_invariants() {
        let mut regs = ShbtSwarmMmioRegs::new_nominal_v7();
        assert_eq!(regs.abi_version, SHBT_SWARM_ABI_VERSION);
        assert_eq!(regs.audit_v7_invariants(), SHBT_SWARM_FAULT_NONE);

        let mut fault_regs = regs;
        fault_regs.chiral_ee_q16 = 0;
        assert_ne!(fault_regs.audit_v7_invariants() & SHBT_SWARM_FAULT_CHIRAL_RACEMIC, 0);
    }

    #[test]
    fn test_v9_nominal_and_fault_invariants() {
        let mut regs = ShbtSwarmMmioRegs::new_nominal_v9();
        assert_eq!(regs.abi_version, SHBT_SWARM_ABI_VERSION);
        assert_eq!(regs.audit_v9_invariants(), SHBT_SWARM_FAULT_NONE);

        let mut fault_regs = regs;
        fault_regs.gw_memory_residual_strain_norm = 1.0e-50;
        assert_ne!(fault_regs.audit_v9_invariants() & SHBT_SWARM_FAULT_GW_MEMORY_STRAIN_TRIP, 0);

        let mut fault_regs2 = regs;
        fault_regs2.secular_com_drift_meters = 1.0e-12;
        assert_ne!(fault_regs2.audit_v9_invariants() & SHBT_SWARM_FAULT_COM_HALO_DRIFT_TRIP, 0);
    }

    #[test]
    fn test_v10_nominal_and_fault_invariants() {
        let mut regs = ShbtSwarmMmioRegs::new_nominal_v10();
        assert_eq!(regs.abi_version, SHBT_SWARM_ABI_VERSION);
        assert_eq!(regs.audit_v10_invariants(), SHBT_SWARM_FAULT_NONE);

        let mut fault_regs = regs;
        fault_regs.eddington_damping_factor = 1.5;
        assert_ne!(fault_regs.audit_v10_invariants() & SHBT_SWARM_FAULT_EDDINGTON_OVERPRESSURE, 0);

        let mut fault_regs2 = regs;
        fault_regs2.core_adiabatic_index_gamma1 = 1.2;
        assert_ne!(fault_regs2.audit_v10_invariants() & SHBT_SWARM_FAULT_CORE_ADIABAT_COLLAPSE, 0);

        let mut fault_regs3 = regs;
        fault_regs3.hawking_area_delta_margin = -0.01;
        assert_ne!(fault_regs3.audit_v10_invariants() & SHBT_SWARM_FAULT_HAWKING_AREA_DECREASE, 0);

        let mut fault_regs4 = regs;
        fault_regs4.galactic_virial_drift_norm = 1.0e-10;
        assert_ne!(fault_regs4.audit_v10_invariants() & SHBT_SWARM_FAULT_HALO_VIRIAL_JEANS_DRIFT, 0);

        let mut fault_regs5 = regs;
        fault_regs5.bz_jet_attenuation_db = 30;
        assert_ne!(fault_regs5.audit_v10_invariants() & SHBT_SWARM_FAULT_BZ_JET_LEAKAGE, 0);
    }
}
