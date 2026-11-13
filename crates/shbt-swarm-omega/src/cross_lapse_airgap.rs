//! Phase 5C cross-lapse symplectic airgap fuse for causal horizon isolation.
#![forbid(unsafe_code)]

pub const ENCLAVE_LAPSE_ALPHA_1: f64 = 1.0e12;
pub const EXTERIOR_LAPSE_ALPHA_2: f64 = 1.0;
pub const SYMPLECTIC_DILATION_LAMBDA: f64 = 1.0e6;
pub const MAX_BOGOLIUBOV_BETA_SQ_INTEGRAL: f64 = 1.0e-120;
pub const MAX_GHOST_POTENTIAL_SPOOF_FRAC: f64 = 1.0e-15;
pub const FUSE_TRIP_LATENCY_AS: f64 = 17.010_000;

pub const SHBT_OMEGA_CROSS_LAPSE_AIRGAP_LOCK: u32 = 0x0000_0080;
pub const SHBT_SWARM_FAULT_CBF_VIOLATION: u32 = 0x0000_0004;
pub const SHBT_SWARM_FAULT_CBF_BREACH: u32 = 0x0000_0100;
pub const SHBT_SWARM_FAULT_FRAMING_BREACH: u32 = 0x0001_0000;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CrossLapseAirgapFuse {
    pub status_latch: u32,
    pub fault_mask: u32,
    pub enclave_lapse_alpha: f64,
    pub exterior_lapse_alpha: f64,
    pub bogoliubov_beta_sq_integral: f64,
    pub ghost_potential_spoof_frac: f64,
    pub fuse_armed: bool,
    pub exterior_h1_cycles_fused: bool,
    pub interior_h1_bus_cycles_locked: bool,
}

impl CrossLapseAirgapFuse {
    #[inline]
    pub const fn new_armed() -> Self {
        Self {
            status_latch: SHBT_OMEGA_CROSS_LAPSE_AIRGAP_LOCK,
            fault_mask: 0,
            enclave_lapse_alpha: ENCLAVE_LAPSE_ALPHA_1,
            exterior_lapse_alpha: EXTERIOR_LAPSE_ALPHA_2,
            bogoliubov_beta_sq_integral: 0.0,
            ghost_potential_spoof_frac: 0.0,
            fuse_armed: true,
            exterior_h1_cycles_fused: false,
            interior_h1_bus_cycles_locked: false,
        }
    }

    #[inline]
    pub fn audit_and_clamp_junction(
        &mut self,
        bogoliubov_beta_sq_integral: f64,
        ghost_potential_spoof_frac: f64,
        framing_defect: f64,
    ) -> Result<(), u32> {
        self.bogoliubov_beta_sq_integral = bogoliubov_beta_sq_integral;
        self.ghost_potential_spoof_frac = ghost_potential_spoof_frac;
        if bogoliubov_beta_sq_integral > MAX_BOGOLIUBOV_BETA_SQ_INTEGRAL {
            self.fault_mask |= SHBT_SWARM_FAULT_CBF_VIOLATION;
            self.fuse_armed = false;
            return Err(SHBT_SWARM_FAULT_CBF_VIOLATION);
        }
        if ghost_potential_spoof_frac > MAX_GHOST_POTENTIAL_SPOOF_FRAC {
            self.fault_mask |= SHBT_SWARM_FAULT_CBF_BREACH;
            self.fuse_armed = false;
            return Err(SHBT_SWARM_FAULT_CBF_BREACH);
        }
        if framing_defect > 1.0e-14 {
            self.fault_mask |= SHBT_SWARM_FAULT_FRAMING_BREACH;
            self.fuse_armed = false;
            return Err(SHBT_SWARM_FAULT_FRAMING_BREACH);
        }
        self.status_latch |= SHBT_OMEGA_CROSS_LAPSE_AIRGAP_LOCK;
        Ok(())
    }

    #[inline]
    pub fn sever_exterior_cycles_and_lock_interior_bus(&mut self) -> Result<(), u32> {
        if !self.fuse_armed {
            self.fault_mask |= SHBT_SWARM_FAULT_CBF_BREACH;
            return Err(SHBT_SWARM_FAULT_CBF_BREACH);
        }
        self.exterior_h1_cycles_fused = true;
        self.interior_h1_bus_cycles_locked = true;
        self.status_latch |= SHBT_OMEGA_CROSS_LAPSE_AIRGAP_LOCK;
        Ok(())
    }

    #[inline]
    pub fn verify_cycle_partition(&self) -> bool {
        self.exterior_h1_cycles_fused && self.interior_h1_bus_cycles_locked && self.fuse_armed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn armed_fuse_accepts_nominal_junction() {
        let mut fuse = CrossLapseAirgapFuse::new_armed();
        assert!(fuse.fuse_armed);
        assert!(fuse
            .audit_and_clamp_junction(1.0e-130, 1.0e-16, 1.0e-15)
            .is_ok());
        assert_eq!(fuse.fault_mask, 0);
    }

    #[test]
    fn bogoliubov_leak_disarms_fuse() {
        let mut fuse = CrossLapseAirgapFuse::new_armed();
        assert_eq!(
            fuse.audit_and_clamp_junction(1.0e-110, 0.0, 0.0),
            Err(SHBT_SWARM_FAULT_CBF_VIOLATION)
        );
        assert!(!fuse.fuse_armed);
    }

    #[test]
    fn ghost_spoof_trips_cbf_breach() {
        let mut fuse = CrossLapseAirgapFuse::new_armed();
        assert_eq!(
            fuse.audit_and_clamp_junction(0.0, 1.0e-14, 0.0),
            Err(SHBT_SWARM_FAULT_CBF_BREACH)
        );
    }
}
