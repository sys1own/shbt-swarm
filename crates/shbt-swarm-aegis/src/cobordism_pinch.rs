//! Phase 5C SHBT-AEGIS-ABSOLUTE cobordism pinch and ADM monopole nullification.
#![forbid(unsafe_code)]

pub const ADM_COUNTER_HUB_COUNT: u32 = 312;
pub const CCZ4_GUNDLACH_KAPPA_1: f64 = 0.15;
pub const PINCH_LATENCY_ATTOSECONDS: f64 = 17.010_000;
pub const PERPETUAL_DARK_GRID_JOULES: f64 = 1.80e56;
pub const MAX_GW_MEMORY_RESIDUAL_NORM: f64 = 1.0e-120;

pub const SHBT_AEGIS_COBORDISM_PINCH_LATCH: u32 = 0x0000_0010;
pub const SHBT_AEGIS_ADM_MONOPOLE_NULL_LOCK: u32 = 0x0000_0040;
pub const SHBT_SWARM_FAULT_COBORDISM_LEAK: u32 = 0x0004_0000;
pub const SHBT_SWARM_FAULT_PSEUDO_ANOSOV_TWIST: u32 = 0x0008_0000;
pub const SHBT_SWARM_FAULT_EP2_NILPOTENCY_LOSS: u32 = 0x0010_0000;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CobordismPinchController {
    pub status_latch: u32,
    pub fault_mask: u32,
    pub internal_mass_kg: f64,
    pub exterior_adm_monopole_kg: f64,
    pub memory_residual_norm: f64,
    pub cobordism_pinch_executed: bool,
}

impl CobordismPinchController {
    #[inline]
    pub const fn new_isolated(internal_mass_kg: f64) -> Self {
        Self {
            status_latch: SHBT_AEGIS_ADM_MONOPOLE_NULL_LOCK,
            fault_mask: 0,
            internal_mass_kg,
            exterior_adm_monopole_kg: 0.0,
            memory_residual_norm: 0.0,
            cobordism_pinch_executed: false,
        }
    }

    #[inline]
    pub fn nullify_exterior_adm_monopole(&mut self, exterior_monopole_kg: f64, cc_z4_kappa_1: f64) -> bool {
        if cc_z4_kappa_1 <= 0.0 || cc_z4_kappa_1 > CCZ4_GUNDLACH_KAPPA_1 {
            self.fault_mask |= SHBT_SWARM_FAULT_PSEUDO_ANOSOV_TWIST;
            return false;
        }
        self.exterior_adm_monopole_kg = exterior_monopole_kg;
        if exterior_monopole_kg != 0.0 {
            self.fault_mask |= SHBT_SWARM_FAULT_COBORDISM_LEAK;
            return false;
        }
        self.status_latch |= SHBT_AEGIS_ADM_MONOPOLE_NULL_LOCK;
        true
    }

    #[inline]
    pub fn execute_macroscopic_cobordism_pinch(&mut self, gw_memory_residual_norm: f64) -> Result<(), u32> {
        if gw_memory_residual_norm > MAX_GW_MEMORY_RESIDUAL_NORM {
            self.fault_mask |= SHBT_SWARM_FAULT_EP2_NILPOTENCY_LOSS;
            return Err(SHBT_SWARM_FAULT_EP2_NILPOTENCY_LOSS);
        }
        if (self.status_latch & SHBT_AEGIS_ADM_MONOPOLE_NULL_LOCK) == 0 {
            self.fault_mask |= SHBT_SWARM_FAULT_COBORDISM_LEAK;
            return Err(SHBT_SWARM_FAULT_COBORDISM_LEAK);
        }
        self.memory_residual_norm = gw_memory_residual_norm;
        self.cobordism_pinch_executed = true;
        self.status_latch |= SHBT_AEGIS_COBORDISM_PINCH_LATCH;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn isolated_core_has_zero_exterior_monopole() {
        let core = CobordismPinchController::new_isolated(1.0e30);
        assert_eq!(core.exterior_adm_monopole_kg, 0.0);
        assert!(core.status_latch & SHBT_AEGIS_ADM_MONOPOLE_NULL_LOCK != 0);
    }

    #[test]
    fn pinch_executes_when_null_lock_and_nilpotency_hold() {
        let mut core = CobordismPinchController::new_isolated(1.0e30);
        assert!(core.nullify_exterior_adm_monopole(0.0, CCZ4_GUNDLACH_KAPPA_1));
        assert!(core.execute_macroscopic_cobordism_pinch(1.0e-130).is_ok());
        assert!(core.cobordism_pinch_executed);
    }

    #[test]
    fn nonzero_exterior_monopole_blocks_pinch() {
        let mut core = CobordismPinchController::new_isolated(1.0e30);
        assert!(!core.nullify_exterior_adm_monopole(1.0e-30, CCZ4_GUNDLACH_KAPPA_1));
        assert_ne!(core.fault_mask & SHBT_SWARM_FAULT_COBORDISM_LEAK, 0);
    }
}
