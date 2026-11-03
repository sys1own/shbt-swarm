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
/// Bit 31: Weyl-exciton hardware crowbar fault trip.
pub const SHBT_POWER_BUS_CROWBAR_FAULT: u64 = 1 << 31;

/// Nominal 25.00 EW power output of the `SHBT-MAR-312X` discrete core (`2.50e19 W`).
pub const MAR312X_NOMINAL_POWER_WATTS: f64 = 2.50e19;

/// Deterministic state machine for the Dual-Reactor Power Bus Arbiter.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PowerArbiterState {
    /// Interstellar cruise & warp soliton generation powered exclusively by `SHBT-MAR-312X` (25.00 EW).
    CruiseTraversal = 0,
    /// Stellar orbital insertion and initial Bondi-Hoyle acoustic-magnetic funnel alignment via `SHBT-MAR-312X`.
    SiphonApproach = 1,
    /// Initial baryonic slipstream injection (`0.01% - 0.1%`) into the `SHBT-MAR-OMEGA` boundary throat.
    OmegaIgnition = 2,
    /// Phase-locked CFT bus handoff from `SHBT-MAR-312X` (25.00 EW) to `SHBT-MAR-OMEGA` (1.00–10.00 ZW).
    HarvestHandoff = 3,
    /// Steady-state stellar deconstruction & in-line computronium synthesis via `SHBT-MAR-OMEGA` (1.00–10.00 ZW).
    HarvestActive = 4,
    /// Post-harvest siphon ramp-down and seamless return to `SHBT-MAR-312X` traversal power.
    DepartureHandshake = 5,
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
        }
    }

    /// Returns total combined power available on the hybrid CFT bus (`W`).
    #[inline]
    pub fn total_bus_power_watts(&self) -> f64 {
        self.mar312x_power_watts + self.omega_power_watts
    }

    /// Verifies that the `SHBT-MAR-312X` discrete core remains 100% isolated from the raw stellar
    /// baryonic slipstream and that no bus sag or crowbar fault has occurred.
    #[inline]
    pub fn verify_dual_reactor_isolation(&self) -> bool {
        !self.crowbar_tripped
            && self.mar312x_baryon_isolation_locked
            && self.mar312x_power_watts >= MAR312X_NOMINAL_POWER_WATTS
            && self.handoff_phase_error_rad.abs() <= 1.0e-15
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
                self.mar312x_power_watts = MAR312X_NOMINAL_POWER_WATTS;
                self.omega_power_watts = tel.p_cft_direct_watts;
                self.handoff_phase_error_rad = 0.0;
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
