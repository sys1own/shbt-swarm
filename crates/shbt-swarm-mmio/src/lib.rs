//! `shbt-swarm-mmio`: Bare-Metal Microkernel ABI v7 (`0x00070312`) Register Specification.
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
pub const SHBT_SWARM_ABI_VERSION: u32 = 0x0007_0312;
pub const SHBT_SWARM_CHANNELS_TOTAL: u32 = 312;
pub const SHBT_SWARM_CFT_SLOTS_TOTAL: u32 = 346;
pub const SHBT_SWARM_FORMAL_PROOFS_COUNT: u32 = 65;
pub const SHBT_SWARM_HIL_GATES_COUNT: u32 = 232;

pub const MAGIC_SWRM: u32 = 0x5357_524D; // 'SWRM'
pub const ABI_VERSION_V4: u32 = 0x0004_0312;
pub const ABI_VERSION_V6: u32 = 0x0006_0312;
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
}

const _: () = {
    assert!(size_of::<ShbtSwarmMmioRegs>() == 320);
    assert!(align_of::<ShbtSwarmMmioRegs>() == 64);
    assert!(offset_of!(ShbtSwarmMmioRegs, abi_version) == 0x00C0);
    assert!(offset_of!(ShbtSwarmMmioRegs, chiral_ee_q16) == 0x0100);
    assert!(offset_of!(ShbtSwarmMmioRegs, clathrate_vortex_drift_velocity) == 0x0118);
    assert!(offset_of!(ShbtSwarmMmioRegs, branch_prune_entropy_debt) == 0x0134);
};

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
        }
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
}
