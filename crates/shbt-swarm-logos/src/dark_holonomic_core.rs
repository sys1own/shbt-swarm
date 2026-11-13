//! Phase 5C pure dark holonomic computronium core.
#![forbid(unsafe_code)]

pub const LOGOS_SATURATED_BIT_OPS_PER_SEC: f64 = 1.080e92;
pub const DARK_WEIL_PRIMARY_MODULES: u64 = 2_901_360;
pub const C_DARK_COMP_NUM: u64 = 1_197_103;
pub const C_DARK_COMP_DEN: u64 = 362_670;
pub const SYMPLECTIC_SHUNT_NUM: u64 = 100;
pub const SYMPLECTIC_SHUNT_DEN: u64 = 1_089;

pub const SHBT_LOGOS_PURE_DARK_MODE: u32 = 0x0000_0008;
pub const SHBT_SWARM_FAULT_BLUESHEET_SURGE: u32 = 0x0020_0000;
pub const SHBT_SWARM_FAULT_WEYL_CROWBAR_TRIP: u32 = 0x8000_0000;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DarkHolonomicCore {
    pub mode_flags: u32,
    pub fault_mask: u32,
    pub logical_bit_ops_per_sec: f64,
    pub dark_weil_modules: u64,
    pub unipotent_shear_upper_left: f64,
    pub unipotent_shear_lower_right: f64,
    pub symplectic_form_residual: f64,
}

impl DarkHolonomicCore {
    #[inline]
    pub const fn new_pure_dark() -> Self {
        Self {
            mode_flags: SHBT_LOGOS_PURE_DARK_MODE,
            fault_mask: 0,
            logical_bit_ops_per_sec: 0.0,
            dark_weil_modules: DARK_WEIL_PRIMARY_MODULES,
            unipotent_shear_upper_left: 1.0,
            unipotent_shear_lower_right: 1.0,
            symplectic_form_residual: 0.0,
        }
    }

    #[inline]
    pub fn verify_unipotent_symplectic_shear(
        &mut self,
        shear_a: f64,
        shear_b: f64,
        proposed_ops_per_sec: f64,
    ) -> Result<(), u32> {
        if proposed_ops_per_sec > LOGOS_SATURATED_BIT_OPS_PER_SEC {
            self.fault_mask |= SHBT_SWARM_FAULT_BLUESHEET_SURGE;
            return Err(SHBT_SWARM_FAULT_BLUESHEET_SURGE);
        }
        self.unipotent_shear_upper_left = shear_a;
        self.unipotent_shear_lower_right = shear_b;
        let det = shear_a * shear_b - 1.0;
        self.symplectic_form_residual = if det < 0.0 { -det } else { det };
        if self.symplectic_form_residual > 1.0e-12 {
            self.fault_mask |= SHBT_SWARM_FAULT_WEYL_CROWBAR_TRIP;
            return Err(SHBT_SWARM_FAULT_WEYL_CROWBAR_TRIP);
        }
        self.logical_bit_ops_per_sec = proposed_ops_per_sec;
        self.mode_flags |= SHBT_LOGOS_PURE_DARK_MODE;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pure_dark_core_boots_in_dark_mode() {
        let core = DarkHolonomicCore::new_pure_dark();
        assert_ne!(core.mode_flags & SHBT_LOGOS_PURE_DARK_MODE, 0);
        assert_eq!(core.dark_weil_modules, DARK_WEIL_PRIMARY_MODULES);
    }

    #[test]
    fn unipotent_shear_accepts_symplectic_pair() {
        let mut core = DarkHolonomicCore::new_pure_dark();
        assert!(core
            .verify_unipotent_symplectic_shear(1.0, 1.0, 1.0e90)
            .is_ok());
        assert_eq!(core.symplectic_form_residual, 0.0);
    }

    #[test]
    fn bluesheet_surge_fault_latches() {
        let mut core = DarkHolonomicCore::new_pure_dark();
        assert_eq!(
            core.verify_unipotent_symplectic_shear(1.0, 1.0, 2.0e92),
            Err(SHBT_SWARM_FAULT_BLUESHEET_SURGE)
        );
    }
}
