//! crates/shbt-swarm-mmio/src/lib.rs
//! #![no_std] Zero-cost volatile wrapper over SHBT-MMIO-SWARM v4 (0x70000000) and
//! the 64-Bank UnifiedStinespringHyperFrame (0x70020000) with host simulator backing.

#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]

use core::ptr::{read_volatile, write_volatile};
use shbt_swarm_core::*;

pub const MAGIC_SWRM: u32 = 0x5357_524D; // 'SWRM'
pub const ABI_VERSION_V4: u32 = 0x0004_0312;
pub const ABI_VERSION_V6: u32 = 0x0006_0312;
pub const SHBT_APOTH_MMIO_BASE: usize = 0x4000_C000;

/// Volatile MMIO controller supporting both bare-metal physical addresses (0x70000000)
/// and host-aligned simulation arenas.
pub struct SwarmMmioController {
    regs_ptr: *mut ShbtSwarmMmioRegs,
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
            regs_ptr: SHBT_SWARM_MMIO_BASE_ADDR as *mut ShbtSwarmMmioRegs,
            hyperframe_ptr: SHBT_SWARM_HYPERFRAME_BASE_ADDR as *mut UnifiedStinespringHyperFrame,
        }
    }

    /// Creates a controller backed by host-aligned memory buffers for deterministic verification.
    #[inline]
    pub fn from_backing(
        regs: &mut ShbtSwarmMmioRegs,
        hyperframe: &mut UnifiedStinespringHyperFrame,
    ) -> Self {
        Self {
            regs_ptr: regs as *mut ShbtSwarmMmioRegs,
            hyperframe_ptr: hyperframe as *mut UnifiedStinespringHyperFrame,
        }
    }

    #[inline]
    pub fn init_v4_defaults(&mut self) {
        unsafe {
            write_volatile(&mut (*self.regs_ptr).magic_id, MAGIC_SWRM);
            write_volatile(&mut (*self.regs_ptr).abi_version, ABI_VERSION_V4);
            write_volatile(&mut (*self.regs_ptr).cmd_reg, SHBT_SWARM_CMD_NOP);
            write_volatile(&mut (*self.regs_ptr).status_reg, SHBT_SWARM_TRIAGE_M0_REGOLITH);
            write_volatile(&mut (*self.regs_ptr).fault_mask_reg, SHBT_SWARM_FAULT_NONE);
            write_volatile(&mut (*self.regs_ptr).rom_bank_mask, 0xFFFF_FFFF);
            write_volatile(&mut (*self.regs_ptr).synth_bank_mask, 0xFFFF_FFFF);
            write_volatile(&mut (*self.regs_ptr).temp_qc_mk, 4200);
            write_volatile(&mut (*self.regs_ptr).temp_nacelle_mk, 21130);
            write_volatile(&mut (*self.regs_ptr).gst225_impedance_krayl, 1500);
            write_volatile(&mut (*self.regs_ptr).enclave_lapse_milli, 1000);
        }
    }

    #[inline]
    pub fn read_magic_and_abi(&self) -> (u32, u32) {
        unsafe {
            (
                read_volatile(&(*self.regs_ptr).magic_id),
                read_volatile(&(*self.regs_ptr).abi_version),
            )
        }
    }

    #[inline]
    pub fn read_fault_mask(&self) -> u32 {
        unsafe { read_volatile(&(*self.regs_ptr).fault_mask_reg) }
    }

    #[inline]
    pub fn write_fault_mask(&mut self, mask: u32) {
        unsafe {
            write_volatile(&mut (*self.regs_ptr).fault_mask_reg, mask);
            if mask != 0 {
                write_volatile(&mut (*self.regs_ptr).cmd_reg, SHBT_SWARM_CMD_WEYL_EXCITON_CROWBAR);
            }
        }
    }

    #[inline]
    pub fn read_cmd(&self) -> u32 {
        unsafe { read_volatile(&(*self.regs_ptr).cmd_reg) }
    }

    #[inline]
    pub fn issue_cmd(&mut self, cmd: u32) -> Result<(), u32> {
        let faults = self.read_fault_mask();
        if faults != 0 {
            unsafe {
                write_volatile(&mut (*self.regs_ptr).cmd_reg, SHBT_SWARM_CMD_WEYL_EXCITON_CROWBAR);
            }
            return Err(faults);
        }
        unsafe {
            write_volatile(&mut (*self.regs_ptr).cmd_reg, cmd);
        }
        Ok(())
    }

    #[inline]
    pub fn set_triage_class(&mut self, triage_bits: u32) {
        unsafe {
            let cur = read_volatile(&(*self.regs_ptr).status_reg) & !SHBT_SWARM_TRIAGE_MASK;
            write_volatile(
                &mut (*self.regs_ptr).status_reg,
                cur | (triage_bits & SHBT_SWARM_TRIAGE_MASK),
            );
        }
    }

    #[inline]
    pub fn read_triage_class(&self) -> u32 {
        unsafe { read_volatile(&(*self.regs_ptr).status_reg) & SHBT_SWARM_TRIAGE_MASK }
    }

    #[inline]
    pub fn set_mass_and_bl_ledger(&mut self, feedstock_ug: u64, payload_ug: u64, net_b_minus_l: i64) {
        unsafe {
            write_volatile(&mut (*self.regs_ptr).feedstock_mass_ug, feedstock_ug);
            write_volatile(&mut (*self.regs_ptr).target_payload_mass_ug, payload_ug);
            write_volatile(&mut (*self.regs_ptr).net_b_minus_l_balance, net_b_minus_l);
        }
    }

    #[inline]
    pub fn get_hyperframe_mut(&mut self) -> &mut UnifiedStinespringHyperFrame {
        unsafe { &mut *self.hyperframe_ptr }
    }
}

/// Pillar IV Causal Point Observer Frame (128 bytes, aligned 64).
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
pub struct ShbtCausalObserverFrame {
    pub observer_uuid_lo: u64,
    pub observer_uuid_hi: u64,
    pub ramp_param_s: f64,
    pub lapse_alpha_obs: f64,
    pub c_local_budget_log10: f64,
    pub c_op_active_log10: f64,
    pub r_entropy_margin: f64,
    pub h_syn_cbf_margin: f64,
    pub h_cog_homeostasis_margin: f64,
    pub verlinde_avatar_count_k: u32,
    pub nested_recursion_depth_d: u32,
    pub burau_cayley_memory_hash: [u64; 4],
    pub stinespring_vault_ptr: u64,
    pub reserved_pad: u64,
}

impl Default for ShbtCausalObserverFrame {
    fn default() -> Self {
        Self {
            observer_uuid_lo: 0,
            observer_uuid_hi: 0,
            ramp_param_s: 0.0,
            lapse_alpha_obs: 1.0,
            c_local_budget_log10: 122.5,
            c_op_active_log10: 42.0,
            r_entropy_margin: 80.5,
            h_syn_cbf_margin: 1.0,
            h_cog_homeostasis_margin: 1.0,
            verlinde_avatar_count_k: 1,
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
        let mut regs = ShbtSwarmMmioRegs::default();
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
}

