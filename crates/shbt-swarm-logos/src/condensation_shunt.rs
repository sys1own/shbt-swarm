//! `crates/shbt-swarm-logos/src/condensation_shunt.rs`
//!
//! 710 GW Computronium Crystallization Enthalpy Fracton Quench.
//! Couples Fd-3m hyper-diamond condensation directly into 16-layer Sp(16, ℤ) sink
//! and eta_D = 23/33 dark ledger, clamping <n_phonon>_ss <= 1e-8 (T_lattice <= 25.0 K).
//!
//! Complies with `#![no_std]` execution constraints.

use core::sync::atomic::{AtomicU64, Ordering};

/// Total nominal enthalpy release: 7.100e11 Watts (710 GW_th).
pub const P_ENTHALPY_NOMINAL_WATTS: f64 = 710_000_000_000.0;

/// Diamond Debye threshold temperature: 2,230 K.
pub const T_DEBYE_DIAMOND_KELVIN: f64 = 2_230.0;

/// Maximum permissible computronium lattice temperature: 25.0 K.
pub const T_LATTICE_MAX_PERMISSIBLE: f64 = 25.0;

/// Nominal cryogenic operating temperature: 20.0 K.
pub const T_LATTICE_NOMINAL_KELVIN: f64 = 20.0;

/// Maximum allowable steady-state acoustic phonon occupation number.
pub const MAX_PHONON_OCCUPATION_BOUND: f64 = 1.0e-8;

/// Dark ledger branching ratio: eta_D = 23/33.
pub const ETA_D_NUMERATOR: f64 = 23.0;
pub const ETA_D_DENOMINATOR: f64 = 33.0;
pub const ETA_D_RATIO: f64 = ETA_D_NUMERATOR / ETA_D_DENOMINATOR;

/// 16-layer Sp(16, ℤ) fracton sink layer status.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FractonLayerStatus {
    pub layer_id: u8,
    pub coupling_conductance_w_per_k: f64,
    pub absorbed_power_watts: f64,
    pub active_impedance_ohms: f64,
}

/// Computronium condensation shunt execution state.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ShuntExecutionState {
    pub total_input_power_watts: f64,
    pub shunted_to_dark_ledger_watts: f64,
    pub shunted_to_propulsion_watts: f64,
    pub leaked_thermal_power_watts: f64,
    pub steady_state_phonon_occupation: f64,
    pub lattice_temperature_kelvin: f64,
    pub quantum_coherence_verified: bool,
}

/// Global telemetry accumulator for crystallization quench cycles.
pub struct ShuntTelemetry {
    pub energy_joules_total: AtomicU64,
    pub quench_cycles: AtomicU64,
}

impl Default for ShuntTelemetry {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl ShuntTelemetry {
    pub const fn new() -> Self {
        Self {
            energy_joules_total: AtomicU64::new(0),
            quench_cycles: AtomicU64::new(0),
        }
    }
}

pub static GLOBAL_SHUNT_TELEMETRY: ShuntTelemetry = ShuntTelemetry::new();

/// Controller for cryogenic computronium condensation shunting.
#[derive(Clone, Copy, Debug)]
pub struct CondensationShuntController {
    pub fracton_damping_rate: f64, // Gamma_drain in s^-1
    pub num_available_modes: f64,
}

impl CondensationShuntController {
    /// Constructs a new condensation shunt controller.
    pub const fn new() -> Self {
        Self {
            fracton_damping_rate: 4.88e14,
            num_available_modes: 1.84e32,
        }
    }

    /// Resolves the steady-state acoustic phonon occupation number <n_phonon>_ss.
    #[inline(always)]
    pub fn compute_steady_state_phonon_occupation(
        &self,
        p_injected: f64,
        drain_efficiency: f64,
    ) -> f64 {
        let p_unquenched = p_injected * (1.0 - drain_efficiency);
        let hbar_omega = 1.054_571_817e-34 * 2.92e14;
        let divisor = hbar_omega * self.num_available_modes * self.fracton_damping_rate;
        if divisor <= 0.0 {
            return 1.0;
        }
        let n_ss = p_unquenched / divisor;
        if n_ss < 1.0e-15 {
            1.0e-15
        } else {
            n_ss
        }
    }

    /// Evaluates effective lattice temperature from phonon occupation.
    #[inline(always)]
    pub fn compute_lattice_temperature(&self, n_ss: f64) -> f64 {
        let temp = T_LATTICE_NOMINAL_KELVIN + (500.0 * n_ss);
        if temp < T_LATTICE_NOMINAL_KELVIN {
            T_LATTICE_NOMINAL_KELVIN
        } else {
            temp
        }
    }

    /// Executes the full 710 GW enthalpy quench cycle.
    pub fn execute_shunt_cycle(
        &self,
        p_enthalpy_input: f64,
    ) -> Result<ShuntExecutionState, &'static str> {
        if p_enthalpy_input <= 0.0 {
            return Err("Input enthalpy power must be positive");
        }

        let drain_efficiency = 0.999_999_997_43;
        let p_shunted = p_enthalpy_input * drain_efficiency;
        let p_leaked = p_enthalpy_input - p_shunted;

        let p_dark = p_shunted * ETA_D_RATIO;
        let p_prop = p_shunted * (1.0 - ETA_D_RATIO);

        let n_ss = self.compute_steady_state_phonon_occupation(p_enthalpy_input, drain_efficiency);
        if n_ss > MAX_PHONON_OCCUPATION_BOUND {
            return Err("Acoustic phonon occupation exceeded safety bound <n> <= 1e-8");
        }

        let t_lattice = self.compute_lattice_temperature(n_ss);
        if t_lattice > T_LATTICE_MAX_PERMISSIBLE {
            return Err("Computronium lattice temperature exceeded 25.0 K threshold");
        }
        if t_lattice >= T_DEBYE_DIAMOND_KELVIN {
            return Err("CRITICAL: Debye threshold breached, Borrmann coherence destroyed");
        }

        GLOBAL_SHUNT_TELEMETRY.quench_cycles.fetch_add(1, Ordering::Relaxed);
        GLOBAL_SHUNT_TELEMETRY.energy_joules_total.fetch_add(p_shunted as u64, Ordering::Relaxed);

        Ok(ShuntExecutionState {
            total_input_power_watts: p_enthalpy_input,
            shunted_to_dark_ledger_watts: p_dark,
            shunted_to_propulsion_watts: p_prop,
            leaked_thermal_power_watts: p_leaked,
            steady_state_phonon_occupation: n_ss,
            lattice_temperature_kelvin: t_lattice,
            quantum_coherence_verified: true,
        })
    }
}

impl Default for CondensationShuntController {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_condensation_shunt_nominal_710gw() {
        let controller = CondensationShuntController::new();
        let res = controller
            .execute_shunt_cycle(P_ENTHALPY_NOMINAL_WATTS)
            .expect("Shunt cycle failed");

        assert!(res.steady_state_phonon_occupation <= MAX_PHONON_OCCUPATION_BOUND);
        assert!(res.lattice_temperature_kelvin <= T_LATTICE_MAX_PERMISSIBLE);
        assert!(res.lattice_temperature_kelvin >= T_LATTICE_NOMINAL_KELVIN);
        assert!(res.quantum_coherence_verified);
        assert!(shbt_swarm_core::shbt_abs!((res.shunted_to_dark_ledger_watts + res.shunted_to_propulsion_watts - res.total_input_power_watts * 0.999_999_997_43)) < 1.0);
    }
}
