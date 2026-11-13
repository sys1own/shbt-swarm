//! Phase 5C SHBT-AEGIS-ABSOLUTE vacuum pinning and CDL instanton suppression.
#![forbid(unsafe_code)]

pub const CASIMIR_DENOM_SU2_26: f64 = 28.0;
pub const CASIMIR_DENOM_SU3_8: f64 = 22.0;
pub const CASIMIR_DENOM_SO10_312: f64 = 640.0;
pub const FIBONACCI_BRAID_CHANNELS: u32 = 124;
pub const DYNKIN_SLOTS_TOTAL: u32 = 346;
pub const MIN_CDL_INSTANTON_ACTION_SE: f64 = 1.0e12;
pub const MAX_FRAMING_DEFECT_TOL: f64 = 1.0e-14;
pub const MAX_HARVEST_EXPOSURE_SEC_PER_MSUN: f64 = 18.26 * 86_400.0;

pub const SHBT_LOGOS_SIM_X1_PROTECT_LATCH: u32 = 0x0000_0002;
pub const SHBT_AEGIS_VACUUM_DECAY_LOCK: u32 = 0x0000_0020;
pub const SHBT_SWARM_FAULT_CBF_VIOLATION: u32 = 0x0000_0004;
pub const SHBT_SWARM_FAULT_CBF_BREACH: u32 = 0x0000_0100;
pub const SHBT_SWARM_FAULT_FRAMING_BREACH: u32 = 0x0001_0000;
pub const SHBT_SWARM_FAULT_INSTANTON_DROP: u32 = 0x0002_0000;
pub const SHBT_SWARM_FAULT_COBORDISM_LEAK: u32 = 0x0004_0000;
pub const SHBT_SWARM_FAULT_PSEUDO_ANOSOV_TWIST: u32 = 0x0008_0000;
pub const SHBT_SWARM_FAULT_EP2_NILPOTENCY_LOSS: u32 = 0x0010_0000;
pub const SHBT_SWARM_FAULT_BLUESHEET_SURGE: u32 = 0x0020_0000;
pub const SHBT_SWARM_FAULT_WEYL_CROWBAR_TRIP: u32 = 0x8000_0000;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VacuumPinningState {
    pub status_latch: u32,
    pub fault_mask: u32,
    pub cd_l_instanton_action_se: f64,
    pub framing_defect_norm: f64,
    pub vacuum_decay_pinned: bool,
    pub harvest_exposure_sec: f64,
    pub harvest_mass_msun: f64,
}

impl VacuumPinningState {
    #[inline]
    pub const fn new_pinned() -> Self {
        Self {
            status_latch: SHBT_AEGIS_VACUUM_DECAY_LOCK | SHBT_LOGOS_SIM_X1_PROTECT_LATCH,
            fault_mask: 0,
            cd_l_instanton_action_se: MIN_CDL_INSTANTON_ACTION_SE,
            framing_defect_norm: 0.0,
            vacuum_decay_pinned: true,
            harvest_exposure_sec: 0.0,
            harvest_mass_msun: 0.0,
        }
    }

    #[inline]
    pub fn enforce_sugawara_framing_closure(
        &mut self,
        framing_defect: f64,
        cd_l_instanton_action: f64,
        harvest_mass_kg: f64,
        harvest_window_sec: f64,
    ) -> Result<(), u32> {
        self.framing_defect_norm = if framing_defect < 0.0 { -framing_defect } else { framing_defect };
        self.cd_l_instanton_action_se = cd_l_instanton_action;
        self.harvest_mass_msun = harvest_mass_kg / 1.988_409_870_698_051e30;
        self.harvest_exposure_sec = harvest_window_sec;

        if self.framing_defect_norm > MAX_FRAMING_DEFECT_TOL {
            self.fault_mask |= SHBT_SWARM_FAULT_FRAMING_BREACH;
            return Err(SHBT_SWARM_FAULT_FRAMING_BREACH);
        }
        if cd_l_instanton_action < MIN_CDL_INSTANTON_ACTION_SE {
            self.fault_mask |= SHBT_SWARM_FAULT_INSTANTON_DROP;
            return Err(SHBT_SWARM_FAULT_INSTANTON_DROP);
        }
        let exposure_limit = self.harvest_mass_msun * MAX_HARVEST_EXPOSURE_SEC_PER_MSUN;
        if harvest_window_sec > exposure_limit {
            self.fault_mask |= SHBT_SWARM_FAULT_CBF_BREACH;
            return Err(SHBT_SWARM_FAULT_CBF_BREACH);
        }
        self.status_latch |= SHBT_AEGIS_VACUUM_DECAY_LOCK;
        Ok(())
    }

    #[inline]
    pub fn pin_coleman_de_luccia_action(&mut self, bounce_action_se: f64, requested_latches: u32) -> bool {
        if bounce_action_se >= MIN_CDL_INSTANTON_ACTION_SE {
            self.cd_l_instanton_action_se = bounce_action_se;
            self.vacuum_decay_pinned = true;
            self.status_latch |= SHBT_AEGIS_VACUUM_DECAY_LOCK | requested_latches;
            true
        } else {
            self.fault_mask |= SHBT_SWARM_FAULT_INSTANTON_DROP;
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vacuum_pinning_baseline_is_pinned() {
        let state = VacuumPinningState::new_pinned();
        assert!(state.vacuum_decay_pinned);
        assert_eq!(state.cd_l_instanton_action_se, MIN_CDL_INSTANTON_ACTION_SE);
        assert_eq!(state.fault_mask, 0);
    }

    #[test]
    fn framing_closure_accepts_nominal_harvest() {
        let mut state = VacuumPinningState::new_pinned();
        assert!(state
            .enforce_sugawara_framing_closure(1.0e-15, 1.2e12, 1.988_409_870_698_051e30, 86_400.0)
            .is_ok());
        assert_eq!(state.fault_mask, 0);
    }

    #[test]
    fn instanton_drop_fault_latches() {
        let mut state = VacuumPinningState::new_pinned();
        assert_eq!(
            state.enforce_sugawara_framing_closure(0.0, 9.9e11, 0.0, 0.0),
            Err(SHBT_SWARM_FAULT_INSTANTON_DROP)
        );
        assert_ne!(state.fault_mask & SHBT_SWARM_FAULT_INSTANTON_DROP, 0);
    }

    #[test]
    fn pin_action_rejects_subthreshold_bounce() {
        let mut state = VacuumPinningState::new_pinned();
        assert!(!state.pin_coleman_de_luccia_action(1.0e11, SHBT_AEGIS_VACUUM_DECAY_LOCK));
        assert_ne!(state.fault_mask & SHBT_SWARM_FAULT_INSTANTON_DROP, 0);
    }
}
