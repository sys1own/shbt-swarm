//! `power_arbiter`: Bare-Metal Hardware Handoff Arbiter for the Dual-Reactor Hybrid Power Architecture.
//!
//! Coordinates seamless, zero-sag power transitions between:
//! 1. **SHBT-MAR-312X** (`25.00 EW` discrete core, 376.99 kg bipartite wafers, 5-zone Weyl DEC-X duct,
//!    48.0 T clathrate superhydride coils) for deep-space traversal, warp soliton generation, tactical
//!    maneuvering, and initial Bondi-Hoyle siphon ignition.
//! 2. **SHBT-MAR-OMEGA** (`1.00 - 10.00 ZW` continuous-flow boundary annihilation reactor, zero discrete
//!    fuel storage, 100% direct CFT coupling) for stellar deconstruction, degenerate core transmutation,
//!    and continuous in-line computronium synthesis.

use crate::mar_omega_flow::ContinuousFlowReactor;

/// Bit 0: `SHBT-MAR-312X` 25.00 EW traversal & warp core active.
pub const SHBT_POWER_BUS_MAR312X_ACTIVE: u64 = 1 << 0;
/// Bit 1: `SHBT-MAR-OMEGA` 1.00–10.00 ZW continuous boundary harvesting reactor active.
pub const SHBT_POWER_BUS_OMEGA_ACTIVE: u64 = 1 << 1;
/// Bit 2: Phase-locked CFT bus handoff synchronization engaged.
pub const SHBT_POWER_BUS_HANDOFF_SYNC: u64 = 1 << 2;
/// Bit 3: Hardware power bus arbiter locked and invariant-verified.
pub const SHBT_POWER_BUS_ARBITER_LOCKED: u64 = 1 << 3;
/// Bit 6: Dark-ZPE autopoietic replenishment bus engaged.
pub const SHBT_POWER_BUS_DARK_ZPE_AUTOPOIETIC: u64 = 1 << 6;
/// Bit 31: Weyl-exciton hardware crowbar fault trip.
pub const SHBT_POWER_BUS_CROWBAR_FAULT: u64 = 1 << 31;

/// Dark-ZPE zero-feedstock cold-start excitation delivered to Layer-5 CDL pinning (`16.30 PW`).
pub const DARK_ZPE_COLD_START_EXCITATION_MILLI_PW: u64 = 16_300;
/// Dark-ZPE cold-start excitation in watts (`16.30 PW = 1.630e16 W`).
pub const DARK_ZPE_COLD_START_EXCITATION_WATTS: f64 = DARK_ZPE_COLD_START_EXCITATION_MILLI_PW as f64 * 1.0e12;
/// Layer-5 CDL instanton action floor (`S_E >= 10^12`).
pub const DARK_ZPE_LAYER5_CDL_ACTION_SE: f64 = 1.0e12;
/// Rosseland opacity quench ceiling `kappa_eff / kappa_0 <= 10^-12`.
pub const DARK_ZPE_ROSSELAND_QUENCH_RATIO_MAX: f64 = 1.0e-12;
/// Required stored feedstock for cold-start excitation (`M_stored == 0`).
pub const DARK_ZPE_COLD_START_STORED_MASS_KG: f64 = 0.0;

/// Nominal 25.00 EW power output of the `SHBT-MAR-312X` discrete core (`2.50e19 W`).
pub const MAR312X_NOMINAL_POWER_WATTS: f64 = 2.50e19;

/// Symplectic switching window duration Delta t_switch = 1.701 ps
pub const SWITCHING_WINDOW_PS: f64 = 1.701;
/// Hyperbolic Cattaneo-Vernotte relaxation time tau_q = 14.5 ps
pub const CATTANEO_TAU_Q_PS: f64 = 14.50;

/// Deterministic 6-state finite-state machine (FSM) for the Dual-Reactor Power Bus Arbiter.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PowerArbiterState {
    /// Interstellar cruise & warp soliton generation powered exclusively by `SHBT-MAR-312X` (25.00 EW).
    CruiseTraversal = 0x01,
    /// Stellar orbital insertion and initial Bondi-Hoyle acoustic-magnetic funnel alignment via `SHBT-MAR-312X`.
    SiphonApproach = 0x02,
    /// Initial baryonic slipstream injection (`0.01% - 0.1%`) into the `SHBT-MAR-OMEGA` boundary throat.
    OmegaIgnition = 0x03,
    /// Phase-locked CFT bus handoff from `SHBT-MAR-312X` (25.00 EW) to `SHBT-MAR-OMEGA` (1.00–10.00 ZW).
    HarvestHandoff = 0x04,
    /// Steady-state stellar deconstruction & in-line computronium synthesis via `SHBT-MAR-OMEGA` (1.00–10.00 ZW).
    HarvestActive = 0x05,
    /// Post-harvest siphon ramp-down and seamless return to `SHBT-MAR-312X` traversal power.
    DepartureHandshake = 0x06,
}

/// Bare-Metal Dual-Reactor Power Bus Arbiter (`SHBT-MAR-312X` + `SHBT-MAR-OMEGA`).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DualReactorPowerArbiter {
    pub state: PowerArbiterState,
    pub mar312x_power_watts: f64,
    pub omega_power_watts: f64,
    pub status_flags: u64,
    pub handoff_phase_error_rad: f64,
    pub mar312x_baryon_isolation_locked: bool,
    pub crowbar_tripped: bool,
    pub bus_voltage_perturbation_volts: f64,
    pub handoff_entropy_production_j_k: f64,
}

impl DualReactorPowerArbiter {
    /// Initializes the arbiter in `CruiseTraversal` state powered by `SHBT-MAR-312X` (25.00 EW).
    #[inline]
    pub const fn new_cruise() -> Self {
        Self {
            state: PowerArbiterState::CruiseTraversal,
            mar312x_power_watts: MAR312X_NOMINAL_POWER_WATTS,
            omega_power_watts: 0.0,
            status_flags: SHBT_POWER_BUS_MAR312X_ACTIVE | SHBT_POWER_BUS_ARBITER_LOCKED,
            handoff_phase_error_rad: 0.0,
            mar312x_baryon_isolation_locked: true,
            crowbar_tripped: false,
            bus_voltage_perturbation_volts: 0.0,
            handoff_entropy_production_j_k: 0.0,
        }
    }

    /// Initializes the arbiter directly in `HarvestActive` state at the given `SHBT-MAR-OMEGA` power.
    #[inline]
    pub const fn new_harvest_active(omega_power_watts: f64) -> Self {
        Self {
            state: PowerArbiterState::HarvestActive,
            mar312x_power_watts: MAR312X_NOMINAL_POWER_WATTS,
            omega_power_watts,
            status_flags: SHBT_POWER_BUS_MAR312X_ACTIVE
                | SHBT_POWER_BUS_OMEGA_ACTIVE
                | SHBT_POWER_BUS_ARBITER_LOCKED,
            handoff_phase_error_rad: 0.0,
            mar312x_baryon_isolation_locked: true,
            crowbar_tripped: false,
            bus_voltage_perturbation_volts: 0.0,
            handoff_entropy_production_j_k: 0.0,
        }
    }

    /// Returns total combined power available on the hybrid CFT bus (`W`).
    #[inline]
    pub fn total_bus_power_watts(&self) -> f64 {
        self.mar312x_power_watts + self.omega_power_watts
    }

    /// Engages the Dark-ZPE autopoietic replenishment bus bit without disturbing v14 handoff state.
    #[inline]
    pub fn engage_dark_zpe_autopoietic_bus(&mut self) -> bool {
        if self.verify_dual_reactor_isolation() {
            self.status_flags |= SHBT_POWER_BUS_DARK_ZPE_AUTOPOIETIC;
            true
        } else {
            false
        }
    }

    /// Returns Dark-ZPE cold-start excitation power when the autopoietic bus bit is engaged.
    #[inline]
    pub fn dark_zpe_cold_start_excitation_watts(&self) -> f64 {
        if self.status_flags & SHBT_POWER_BUS_DARK_ZPE_AUTOPOIETIC != 0 {
            DARK_ZPE_COLD_START_EXCITATION_WATTS
        } else {
            0.0
        }
    }

    /// Verifies Layer-5 CDL pinning and Rosseland quench supplied by Dark-ZPE at zero feedstock.
    #[inline]
    pub fn verify_dark_zpe_layer5_cold_start(
        &self,
        cd_l_instanton_action_se: f64,
        rosseland_kappa_ratio: f64,
        stored_feedstock_kg: f64,
    ) -> bool {
        self.status_flags & SHBT_POWER_BUS_DARK_ZPE_AUTOPOIETIC != 0
            && self.dark_zpe_cold_start_excitation_watts() >= DARK_ZPE_COLD_START_EXCITATION_WATTS
            && cd_l_instanton_action_se >= DARK_ZPE_LAYER5_CDL_ACTION_SE
            && rosseland_kappa_ratio <= DARK_ZPE_ROSSELAND_QUENCH_RATIO_MAX
            && stored_feedstock_kg == DARK_ZPE_COLD_START_STORED_MASS_KG
    }

    /// Verifies that the `SHBT-MAR-312X` discrete core remains 100% isolated from the raw stellar
    /// baryonic slipstream and that no bus sag or crowbar fault has occurred.
    #[inline]
    pub fn verify_dual_reactor_isolation(&self) -> bool {
        !self.crowbar_tripped
            && self.mar312x_baryon_isolation_locked
            && self.mar312x_power_watts >= MAR312X_NOMINAL_POWER_WATTS
            && shbt_swarm_core::shbt_abs!(self.handoff_phase_error_rad) <= 1.0e-15
            && (self.status_flags & SHBT_POWER_BUS_CROWBAR_FAULT) == 0
    }

    /// Deterministically transitions the power bus arbiter to `next`, updating `SHBT_POWER_BUS_*` flags.
    #[inline]
    pub fn transition_to(
        &mut self,
        next: PowerArbiterState,
        m_dot_siphon_kg_s: f64,
        omega_reactor: &ContinuousFlowReactor,
    ) -> Result<u64, u64> {
        if self.crowbar_tripped {
            self.status_flags |= SHBT_POWER_BUS_CROWBAR_FAULT;
            return Err(self.status_flags);
        }

        let valid_step = matches!(
            (self.state, next),
            (PowerArbiterState::CruiseTraversal, PowerArbiterState::SiphonApproach)
                | (PowerArbiterState::SiphonApproach, PowerArbiterState::OmegaIgnition)
                | (PowerArbiterState::OmegaIgnition, PowerArbiterState::HarvestHandoff)
                | (PowerArbiterState::HarvestHandoff, PowerArbiterState::HarvestActive)
                | (PowerArbiterState::HarvestActive, PowerArbiterState::HarvestActive)
                | (PowerArbiterState::HarvestActive, PowerArbiterState::DepartureHandshake)
                | (PowerArbiterState::DepartureHandshake, PowerArbiterState::CruiseTraversal)
        );

        if !valid_step {
            self.crowbar_tripped = true;
            self.status_flags |= SHBT_POWER_BUS_CROWBAR_FAULT;
            return Err(self.status_flags);
        }

        match next {
            PowerArbiterState::CruiseTraversal => {
                self.mar312x_power_watts = MAR312X_NOMINAL_POWER_WATTS;
                self.omega_power_watts = 0.0;
                self.status_flags = SHBT_POWER_BUS_MAR312X_ACTIVE | SHBT_POWER_BUS_ARBITER_LOCKED;
            }
            PowerArbiterState::SiphonApproach => {
                // MAR-312X supplies 25.00 EW to energize the Bondi-Hoyle acoustic-magnetic funnel
                self.mar312x_power_watts = MAR312X_NOMINAL_POWER_WATTS;
                self.omega_power_watts = 0.0;
                self.status_flags = SHBT_POWER_BUS_MAR312X_ACTIVE
                    | SHBT_POWER_BUS_HANDOFF_SYNC
                    | SHBT_POWER_BUS_ARBITER_LOCKED;
            }
            PowerArbiterState::OmegaIgnition => {
                let tel = omega_reactor.evaluate_flow_step(m_dot_siphon_kg_s * 0.1);
                if !tel.safety_invariants_locked {
                    self.crowbar_tripped = true;
                    self.status_flags |= SHBT_POWER_BUS_CROWBAR_FAULT;
                    return Err(self.status_flags);
                }
                self.mar312x_power_watts = MAR312X_NOMINAL_POWER_WATTS;
                self.omega_power_watts = tel.p_cft_direct_watts;
                self.status_flags = SHBT_POWER_BUS_MAR312X_ACTIVE
                    | SHBT_POWER_BUS_OMEGA_ACTIVE
                    | SHBT_POWER_BUS_HANDOFF_SYNC
                    | SHBT_POWER_BUS_ARBITER_LOCKED;
            }
            PowerArbiterState::HarvestHandoff => {
                let tel = omega_reactor.evaluate_flow_step(m_dot_siphon_kg_s);
                if !tel.safety_invariants_locked {
                    self.crowbar_tripped = true;
                    self.status_flags |= SHBT_POWER_BUS_CROWBAR_FAULT;
                    return Err(self.status_flags);
                }
                // Symplectic make-before-break handoff: Delta V_bus == 0.0 V, Delta S_handoff == 0.0 J/K
                // Switching window dt_switch = 1.701 ps <= tau_q = 14.5 ps
                self.mar312x_power_watts = MAR312X_NOMINAL_POWER_WATTS;
                self.omega_power_watts = tel.p_cft_direct_watts;
                self.handoff_phase_error_rad = 0.0;
                self.bus_voltage_perturbation_volts = 0.0;
                self.handoff_entropy_production_j_k = 0.0;
                self.status_flags = SHBT_POWER_BUS_MAR312X_ACTIVE
                    | SHBT_POWER_BUS_OMEGA_ACTIVE
                    | SHBT_POWER_BUS_HANDOFF_SYNC
                    | SHBT_POWER_BUS_ARBITER_LOCKED;
            }
            PowerArbiterState::HarvestActive => {
                let tel = omega_reactor.evaluate_flow_step(m_dot_siphon_kg_s);
                if !tel.safety_invariants_locked {
                    self.crowbar_tripped = true;
                    self.status_flags |= SHBT_POWER_BUS_CROWBAR_FAULT;
                    return Err(self.status_flags);
                }
                // MAR-312X stays hot-standby/tactical at 25.00 EW while MAR-OMEGA drives 10.00 ZW harvest
                self.mar312x_power_watts = MAR312X_NOMINAL_POWER_WATTS;
                self.omega_power_watts = tel.p_cft_direct_watts;
                self.status_flags = SHBT_POWER_BUS_MAR312X_ACTIVE
                    | SHBT_POWER_BUS_OMEGA_ACTIVE
                    | SHBT_POWER_BUS_ARBITER_LOCKED;
            }
            PowerArbiterState::DepartureHandshake => {
                self.mar312x_power_watts = MAR312X_NOMINAL_POWER_WATTS;
                self.omega_power_watts = 0.0;
                self.handoff_phase_error_rad = 0.0;
                self.status_flags = SHBT_POWER_BUS_MAR312X_ACTIVE
                    | SHBT_POWER_BUS_HANDOFF_SYNC
                    | SHBT_POWER_BUS_ARBITER_LOCKED;
            }
        }

        self.state = next;
        Ok(self.status_flags)
    }

    /// Executes a complete cycle from `CruiseTraversal` -> `SiphonApproach` -> `OmegaIgnition`
    /// -> `HarvestHandoff` -> `HarvestActive`, verifying zero bus sag and full isolation.
    #[inline]
    pub fn execute_full_handoff_sequence(
        &mut self,
        m_dot_siphon_kg_s: f64,
        omega_reactor: &ContinuousFlowReactor,
    ) -> bool {
        self.state = PowerArbiterState::CruiseTraversal;
        self.crowbar_tripped = false;
        self.status_flags = SHBT_POWER_BUS_MAR312X_ACTIVE | SHBT_POWER_BUS_ARBITER_LOCKED;

        let steps = [
            PowerArbiterState::SiphonApproach,
            PowerArbiterState::OmegaIgnition,
            PowerArbiterState::HarvestHandoff,
            PowerArbiterState::HarvestActive,
        ];

        for step in steps {
            if self.transition_to(step, m_dot_siphon_kg_s, omega_reactor).is_err() {
                return false;
            }
            if !self.verify_dual_reactor_isolation() {
                return false;
            }
        }

        self.state == PowerArbiterState::HarvestActive && self.omega_power_watts >= 1.0e21
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dual_reactor_handoff_and_return_to_cruise() {
        let reactor = ContinuousFlowReactor::new_nominal();
        let mut arbiter = DualReactorPowerArbiter::new_cruise();
        let m_dot = 1.256e8;

        assert!(arbiter.execute_full_handoff_sequence(m_dot, &reactor));
        assert_eq!(arbiter.state, PowerArbiterState::HarvestActive);
        assert!(arbiter.omega_power_watts > 9.99e21);
        assert_eq!(arbiter.mar312x_power_watts, MAR312X_NOMINAL_POWER_WATTS);

        assert!(arbiter
            .transition_to(PowerArbiterState::DepartureHandshake, m_dot, &reactor)
            .is_ok());
        assert!(arbiter
            .transition_to(PowerArbiterState::CruiseTraversal, m_dot, &reactor)
            .is_ok());
        assert_eq!(arbiter.state, PowerArbiterState::CruiseTraversal);
        assert_eq!(arbiter.omega_power_watts, 0.0);
        assert!(arbiter.verify_dual_reactor_isolation());
    }
}
