//! crates/shbt-swarm-mmio/src/lib.rs
//! #![no_std] Zero-cost volatile wrapper over SHBT-MMIO-SWARM v4 (0x70000000) and
//! the 64-Bank UnifiedStinespringHyperFrame (0x70020000) with host simulator backing.

#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]

use core::ptr::{read_volatile, write_volatile};
use shbt_swarm_core::*;

pub const MAGIC_SWRM: u32 = 0x5357_524D; // 'SWRM'
pub const ABI_VERSION_V4: u32 = 0x0004_0312;

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
}
