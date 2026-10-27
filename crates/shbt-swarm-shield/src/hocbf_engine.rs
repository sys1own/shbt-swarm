//! crates/shbt-swarm-shield/src/hocbf_engine.rs
//! ============================================================================
//! STATIC HOLOGRAPHIC BOUNDARY THEORY (SHBT) - SAFETY CONTROL BARRIER ENGINE
//! Implements relative-degree-2 High-Order Control Barrier Functions (HOCBFs)
//! and real-time Quadratic Program (QP) active-set projection for physical constraints:
//!   1. Cryogenic Core Temperature (T <= T_max)
//!   2. Structural Shell Stress (sigma <= sigma_max)
//!   3. Reactor Magnetic Flux (|Phi| <= Phi_max)
//!   4. Swarm Population Density (rho <= rho_max)
//!
//! ============================================================================

#[inline(always)]
fn f64_abs(x: f64) -> f64 {
    if x < 0.0 {
        -x
    } else {
        x
    }
}

#[inline(always)]
fn f64_min(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}

#[inline(always)]
fn f64_max(a: f64, b: f64) -> f64 {
    if a > b {
        a
    } else {
        b
    }
}

#[inline(always)]
fn f64_clamp(val: f64, min: f64, max: f64) -> f64 {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhysicalState {
    pub temperature: f64,
    pub temperature_dot: f64,
    pub stress_mpa: f64,
    pub stress_rate: f64,
    pub magnetic_flux: f64,
    pub flux_rate: f64,
    pub population_density: f64,
    pub density_rate: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BarrierConfig {
    pub temp_max: f64,
    pub stress_max: f64,
    pub flux_max: f64,
    pub density_max: f64,
    pub kappa_1: f64,
    pub kappa_2: f64,
    pub slack_penalty: f64,
}

impl Default for BarrierConfig {
    fn default() -> Self {
        Self {
            temp_max: 380.0,      // Kelvin (cryogenic threshold)
            stress_max: 850.0,    // MPa (titanium-composite yield limit)
            flux_max: 45.0,       // Weber (tokamak core saturation limit)
            density_max: 128.0,   // Spacecraft per cubic kilometer
            kappa_1: 4.0,         // Class K parameter 1 (rad/s)
            kappa_2: 6.0,         // Class K parameter 2 (rad/s)
            slack_penalty: 1.0e6, // QP slack penalty
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LinearConstraint {
    pub a_cbf: f64, // A_cbf * u <= b_cbf
    pub b_cbf: f64,
    pub psi_0: f64,
    pub psi_1: f64,
}

impl LinearConstraint {
    /// Evaluates the Nagumo transversality condition at the safe-set boundary.
    /// Returns true if the boundary velocity is tangential or interior-directed.
    #[inline]
    pub fn satisfies_nagumo_transversality(&self, d_psi_0_dt: f64, d_psi_1_dt: f64) -> bool {
        let cond_psi_1 = if f64_abs(self.psi_1) <= 1.0e-5 {
            d_psi_1_dt >= -1.0e-6
        } else {
            true
        };
        let cond_psi_0 = if f64_abs(self.psi_0) <= 1.0e-5 && self.psi_1 >= 0.0 {
            d_psi_0_dt >= -1.0e-6
        } else {
            true
        };
        cond_psi_1 && cond_psi_0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QpFilterResult {
    pub u_filtered: f64,
    pub slack_applied: f64,
    pub is_safe: bool,
    pub active_constraint_index: Option<usize>,
}

pub struct HocbfEngine {
    config: BarrierConfig,
}

impl HocbfEngine {
    pub const fn new(config: BarrierConfig) -> Self {
        Self { config }
    }

    /// Computes the relative-degree-2 barrier components for cryogenic temperature.
    /// Constraint: h_T(x) = T_max - T >= 0
    /// Dynamics:   ddot{T} = Lf2_h + LgLf_h * u
    pub fn evaluate_temperature_barrier(&self, state: &PhysicalState) -> LinearConstraint {
        let psi_0 = self.config.temp_max - state.temperature;
        let lf_h = -state.temperature_dot;
        let psi_1 = lf_h + self.config.kappa_1 * psi_0;

        // Physical drift model: thermal dissipation and radiation
        let lf2_h = -(-0.05 * (state.temperature - 2.73) + 0.12 * state.temperature_dot);
        let lglf_h = -1.50; // Negative gain: positive control u (cooling power) decreases T

        let a_cbf = -lglf_h;
        let b_cbf = lf2_h
            + (self.config.kappa_1 + self.config.kappa_2) * lf_h
            + (self.config.kappa_1 * self.config.kappa_2) * psi_0;

        LinearConstraint { a_cbf, b_cbf, psi_0, psi_1 }
    }

    /// Computes the relative-degree-2 barrier components for structural shell mechanical stress.
    /// Constraint: h_sigma(x) = sigma_max - sigma >= 0
    pub fn evaluate_stress_barrier(&self, state: &PhysicalState) -> LinearConstraint {
        let psi_0 = self.config.stress_max - state.stress_mpa;
        let lf_h = -state.stress_rate;
        let psi_1 = lf_h + self.config.kappa_1 * psi_0;

        // Structural elasticity model: damped harmonic response
        let lf2_h = -(12.4 * state.stress_mpa + 0.85 * state.stress_rate);
        let lglf_h = -3.20; // Actuator strain-damping authority

        let a_cbf = -lglf_h;
        let b_cbf = lf2_h
            + (self.config.kappa_1 + self.config.kappa_2) * lf_h
            + (self.config.kappa_1 * self.config.kappa_2) * psi_0;

        LinearConstraint { a_cbf, b_cbf, psi_0, psi_1 }
    }

    /// Computes the relative-degree-2 barrier components for reactor core magnetic flux.
    /// Constraint: h_Phi(x) = Phi_max^2 - Phi^2 >= 0
    pub fn evaluate_flux_barrier(&self, state: &PhysicalState) -> LinearConstraint {
        let psi_0 = self.config.flux_max * self.config.flux_max - state.magnetic_flux * state.magnetic_flux;
        let lf_h = -2.0 * state.magnetic_flux * state.flux_rate;
        let psi_1 = lf_h + self.config.kappa_1 * psi_0;

        let lf2_h = -2.0 * state.flux_rate * state.flux_rate
            - 2.0 * state.magnetic_flux * (-0.45 * state.flux_rate);
        let lglf_h = -2.0 * state.magnetic_flux * (1.0 / 0.85); // Inductive actuator coupling

        let a_cbf = -lglf_h;
        let b_cbf = lf2_h
            + (self.config.kappa_1 + self.config.kappa_2) * lf_h
            + (self.config.kappa_1 * self.config.kappa_2) * psi_0;

        LinearConstraint { a_cbf, b_cbf, psi_0, psi_1 }
    }

    /// Computes the relative-degree-2 barrier components for swarm local population density.
    /// Constraint: h_rho(x) = rho_max - rho >= 0
    pub fn evaluate_population_barrier(&self, state: &PhysicalState) -> LinearConstraint {
        let psi_0 = self.config.density_max - state.population_density;
        let lf_h = -state.density_rate;
        let psi_1 = lf_h + self.config.kappa_1 * psi_0;

        let lf2_h = -(-0.35 * state.population_density - 0.10 * state.density_rate);
        let lglf_h = -0.80; // Repulsive dispersion control authority

        let a_cbf = -lglf_h;
        let b_cbf = lf2_h
            + (self.config.kappa_1 + self.config.kappa_2) * lf_h
            + (self.config.kappa_1 * self.config.kappa_2) * psi_0;

        LinearConstraint { a_cbf, b_cbf, psi_0, psi_1 }
    }

    /// Verifies Nagumo boundary transversality for positive invariance of composite safe set
    /// C* = { x | psi_0(x) >= 0 and psi_1(x) >= 0 }.
    pub fn check_nagumo_transversality(&self, state: &PhysicalState, u: f64) -> bool {
        let t_c = self.evaluate_temperature_barrier(state);
        let s_c = self.evaluate_stress_barrier(state);
        let f_c = self.evaluate_flux_barrier(state);
        let p_c = self.evaluate_population_barrier(state);

        let t_dpsi0 = -state.temperature_dot;
        let t_dpsi1 = -(-0.05 * (state.temperature - 2.73) + 0.12 * state.temperature_dot) - 1.50 * u + self.config.kappa_1 * t_dpsi0;

        let s_dpsi0 = -state.stress_rate;
        let s_dpsi1 = -(12.4 * state.stress_mpa + 0.85 * state.stress_rate) - 3.20 * u + self.config.kappa_1 * s_dpsi0;

        let f_dpsi0 = -2.0 * state.magnetic_flux * state.flux_rate;
        let f_dpsi1 = -2.0 * state.flux_rate * state.flux_rate - 2.0 * state.magnetic_flux * (-0.45 * state.flux_rate)
            - 2.0 * state.magnetic_flux * (1.0 / 0.85) * u + self.config.kappa_1 * f_dpsi0;

        let p_dpsi0 = -state.density_rate;
        let p_dpsi1 = -(-0.35 * state.population_density - 0.10 * state.density_rate) - 0.80 * u + self.config.kappa_1 * p_dpsi0;

        t_c.satisfies_nagumo_transversality(t_dpsi0, t_dpsi1)
            && s_c.satisfies_nagumo_transversality(s_dpsi0, s_dpsi1)
            && f_c.satisfies_nagumo_transversality(f_dpsi0, f_dpsi1)
            && p_c.satisfies_nagumo_transversality(p_dpsi0, p_dpsi1)
    }

    /// Projects a nominal scalar control input u_nom onto the safe half-space
    /// defined by the 4 concurrent relative-degree-2 physical barriers:
    ///   min 0.5 * (u - u_nom)^2 + 0.5 * p_slack * delta^2
    ///   s.t. A_cbf,i * u <= b_cbf,i + delta
    pub fn filter_control(
        &self,
        state: &PhysicalState,
        u_nom: f64,
        u_min: f64,
        u_max: f64,
    ) -> QpFilterResult {
        let constraints = [
            self.evaluate_temperature_barrier(state),
            self.evaluate_stress_barrier(state),
            self.evaluate_flux_barrier(state),
            self.evaluate_population_barrier(state),
        ];

        let mut u_candidate = f64_clamp(u_nom, u_min, u_max);
        let mut active_idx = None;
        let mut most_restrictive_bound = 1.0e30; // Positive infinity representation

        // Check each linear half-space: A_cbf * u <= b_cbf
        for (i, c) in constraints.iter().enumerate() {
            if f64_abs(c.a_cbf) > 1.0e-9 {
                let bound = c.b_cbf / c.a_cbf;
                if c.a_cbf > 0.0 {
                    // Upper bound: u <= b / a
                    if bound < most_restrictive_bound {
                        most_restrictive_bound = bound;
                        active_idx = Some(i);
                    }
                }
            }
        }

        let mut slack_applied = 0.0;
        if u_candidate > most_restrictive_bound {
            // Apply Euclidean active-set projection
            if most_restrictive_bound >= u_min {
                u_candidate = f64_min(most_restrictive_bound, u_max);
            } else {
                // If the required control exceeds physical actuator saturation u_min,
                // allocate minimal regularized slack to preserve microkernel survivability.
                u_candidate = u_min;
                slack_applied = f64_max(u_min - most_restrictive_bound, 0.0);
            }
        }

        let is_safe = constraints.iter().all(|c| c.psi_0 >= -1.0e-4 && c.psi_1 >= -1.0e-4);

        QpFilterResult {
            u_filtered: u_candidate,
            slack_applied,
            is_safe,
            active_constraint_index: active_idx,
        }
    }
}

impl Default for HocbfEngine {
    fn default() -> Self {
        Self::new(BarrierConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nominal_control_unaltered_when_safe() {
        let engine = HocbfEngine::new(BarrierConfig::default());
        let safe_state = PhysicalState {
            temperature: 300.0,
            temperature_dot: 0.1,
            stress_mpa: 400.0,
            stress_rate: 0.0,
            magnetic_flux: 20.0,
            flux_rate: 0.0,
            population_density: 50.0,
            density_rate: 0.0,
        };

        let u_nom = 5.0;
        let res = engine.filter_control(&safe_state, u_nom, -50.0, 50.0);
        assert!(res.is_safe);
        assert!((res.u_filtered - u_nom).abs() < 1.0e-6);
        assert_eq!(res.slack_applied, 0.0);
        assert!(engine.check_nagumo_transversality(&safe_state, res.u_filtered));
    }

    #[test]
    fn test_temperature_barrier_intervention() {
        let engine = HocbfEngine::new(BarrierConfig::default());
        let hot_state = PhysicalState {
            temperature: 379.5,   // 0.5 K below maximum threshold (380 K)
            temperature_dot: 15.0, // Rising rapidly toward safe set boundary
            stress_mpa: 100.0,
            stress_rate: 0.0,
            magnetic_flux: 5.0,
            flux_rate: 0.0,
            population_density: 10.0,
            density_rate: 0.0,
        };

        // Nominal control commands a heating action (u_nom = 20.0)
        let u_nom = 20.0;
        let res = engine.filter_control(&hot_state, u_nom, -100.0, 100.0);

        // Safety filter must intervene and command negative control (maximum cooling)
        assert!(res.u_filtered < u_nom);
        assert_eq!(res.active_constraint_index, Some(0)); // Temperature index
    }

    #[test]
    fn test_structural_stress_barrier_saturation() {
        let engine = HocbfEngine::new(BarrierConfig::default());
        let high_stress_state = PhysicalState {
            temperature: 200.0,
            temperature_dot: 0.0,
            stress_mpa: 848.0, // 2 MPa below maximum limit (850 MPa)
            stress_rate: 45.0, // Critical stress surge
            magnetic_flux: 10.0,
            flux_rate: 0.0,
            population_density: 20.0,
            density_rate: 0.0,
        };

        let u_nom = 30.0;
        let res = engine.filter_control(&high_stress_state, u_nom, -100.0, 100.0);
        assert!(res.u_filtered < 0.0);
        assert_eq!(res.active_constraint_index, Some(1)); // Stress index
    }
}
