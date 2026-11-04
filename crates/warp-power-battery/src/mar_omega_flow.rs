//! `mar_omega_flow`: SHBT-MAR-OMEGA 6th-Generation Continuous-Flow Boundary Annihilation Reactor.
//!
//! Operates on a continuous 0.01% to 0.1% baryonic slipstream (`f_slip`) siphoned directly from
//! the target star, with zero discrete fuel storage (`M_fuel(t) = 0`), 99.981146% direct boundary
//! CFT exergy coupling, 16-layer `Sp(16, Z)` fracton cryogenic shunting, and 312-channel CCZ4
//! metric stress cloaking.

/// Speed of light squared (`c^2` in m^2/s^2).
pub const C_LIGHT_SQ: f64 = 8.987_551_787_368_176e16;
/// Visible holographic partition ratio (`10/33`).
pub const ETA_A: f64 = 10.0 / 33.0;
/// Dark Stinespring ledger partition ratio (`23/33`).
pub const ETA_D: f64 = 23.0 / 33.0;
/// Second-order SHBT coupling parameter (`100/1089`).
pub const ZETA_SHBT: f64 = 100.0 / 1089.0;
/// 16-layer `Sp(16, Z)` fracton sensible heat attenuation factor (`2.44998856e-17`).
pub const F_SENSIBLE_OMEGA: f64 = 2.449_988_56e-17;
/// Critical superconducting temperature of `GdBa2Cu3O7-delta` throat coils (`92.0 K`).
pub const T_CRITICAL_REBCO_K: f64 = 92.0;
/// Maximum allowed operational temperature of the throat superconducting coils (`50.0 K`).
pub const T_OP_MAX_K: f64 = 50.0;
/// Diamond/graphene Frenkel theoretical shear strength limit (`130,000.0 MPa = 130.0 GPa`).
pub const FRENKEL_LIMIT_MPA: f64 = 130_000.0;

/// Telemetry snapshot for a single symplectic flow step of the SHBT-MAR-OMEGA reactor.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReactorTelemetry {
    pub m_dot_fuel_kg_s: f64,
    pub p_gross_watts: f64,
    pub p_cft_direct_watts: f64,
    pub p_dark_shunt_watts: f64,
    pub p_sensible_cryo_watts: f64,
    pub eta_direct_eff: f64,
    pub t_throat_kelvin: f64,
    pub residual_throat_shear_mpa: f64,
    pub stored_fuel_mass_kg: f64,
    pub safety_invariants_locked: bool,
}

/// 6th-Generation Continuous-Flow Boundary Annihilation Reactor (`SHBT-MAR-OMEGA`).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ContinuousFlowReactor {
    pub f_slip_ratio: f64,
    pub throat_radius_m: f64,
    pub ccz4_cloaking_gain: f64,
    pub base_cryo_temp_k: f64,
    pub cryo_thermal_conductance_w_k: f64,
}

impl ContinuousFlowReactor {
    /// Constructs a nominal 10.00 ZW `SHBT-MAR-OMEGA` continuous-flow boundary reactor.
    #[inline]
    pub const fn new_nominal() -> Self {
        Self {
            f_slip_ratio: 8.858_678_9e-4,
            throat_radius_m: 1.50,
            ccz4_cloaking_gain: 1.0 - 1.0e-18,
            base_cryo_temp_k: 4.20,
            cryo_thermal_conductance_w_k: 6_500.0,
        }
    }

    /// Evaluates a single continuous-flow boundary annihilation step for a given stellar siphon rate.
    #[inline]
    pub fn evaluate_flow_step(&self, m_dot_siphon_kg_s: f64) -> ReactorTelemetry {
        let m_dot_fuel = m_dot_siphon_kg_s * self.f_slip_ratio;
        let p_gross = m_dot_fuel * C_LIGHT_SQ;

        // 346-slot Dynkin CFT direct coupling efficiency: 1 - (100/1089) * (1/487.01)
        let eta_direct = 1.0 - (ZETA_SHBT / 487.010_346);
        let p_cft_direct = p_gross * eta_direct;
        let p_dark_shunt = p_gross * (1.0 - eta_direct) - (p_gross * F_SENSIBLE_OMEGA);
        let p_sensible = p_gross * F_SENSIBLE_OMEGA;

        let t_throat = self.base_cryo_temp_k + (p_sensible / self.cryo_thermal_conductance_w_k);

        // Uncloaked radiation pressure on throat: P_gross / (4 * pi * R^2 * c)
        let area = 4.0 * core::f64::consts::PI * self.throat_radius_m * self.throat_radius_m;
        let p_rad_uncloaked_pa = p_gross / (area * 299_792_458.0);
        let residual_pa = p_rad_uncloaked_pa * (1.0 - self.ccz4_cloaking_gain);
        let residual_mpa = residual_pa * 1.0e-6;

        let safety_ok = (t_throat < T_OP_MAX_K)
            && (residual_mpa < 1.0e-6)
            && (eta_direct >= 0.99980)
            && (p_sensible <= 500_000.0);

        ReactorTelemetry {
            m_dot_fuel_kg_s: m_dot_fuel,
            p_gross_watts: p_gross,
            p_cft_direct_watts: p_cft_direct,
            p_dark_shunt_watts: p_dark_shunt,
            p_sensible_cryo_watts: p_sensible,
            eta_direct_eff: eta_direct,
            t_throat_kelvin: t_throat,
            residual_throat_shear_mpa: residual_mpa,
            stored_fuel_mass_kg: 0.0,
            safety_invariants_locked: safety_ok,
        }
    }
}

/// Primary operational core integrating continuous flow kinetics and RMHD divertor solver.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MarOmegaReactorCore {
    pub flow_reactor: ContinuousFlowReactor,
    pub divertor: crate::divertor_rmhd::DivertorSolver,
}

impl MarOmegaReactorCore {
    /// Constructs a nominal `MarOmegaReactorCore`.
    #[inline]
    pub fn new() -> Self {
        Self {
            flow_reactor: ContinuousFlowReactor::new_nominal(),
            divertor: crate::divertor_rmhd::DivertorSolver::new(),
        }
    }

    /// Primary execution step for stellar deconstruction slipstream diversion and reactor flow.
    pub fn step_reactor_flow(
        &mut self,
        m_dot_siphon_kg_s: u64,
        velocity_beta_q16: i32,
        mmio_base: Option<*mut u8>,
    ) -> Result<ReactorTelemetry, &'static str> {
        let inflow_q32 = crate::divertor_rmhd::Q32::from_int(m_dot_siphon_kg_s as i64);
        let beta = crate::divertor_rmhd::Q16::from_raw(velocity_beta_q16);

        // 1. Solve RMHD equilibrium and update separatrix state
        let tel_div = self.divertor.step_equilibrium(inflow_q32, beta);

        // 2. Verify physical invariants before throat injection
        if (tel_div.status_bits & crate::divertor_rmhd::status_flags::FRENKEL_SAFE) == 0 {
            return Err("FATAL: Divertor effective shear stress violated Frenkel limit");
        }
        if (tel_div.status_bits & crate::divertor_rmhd::status_flags::LAMINAR) == 0 {
            return Err("FATAL: Boundary layer transitioned to turbulence");
        }

        // 3. Commit hardware telemetry to Cache Line 10 if MMIO mapped
        if let Some(base) = mmio_base {
            unsafe {
                self.divertor.write_to_mmio(base, &tel_div);
            }
        }

        // 4. Return continuous-flow reactor telemetry
        let tel_flow = self.flow_reactor.evaluate_flow_step(m_dot_siphon_kg_s as f64);
        Ok(tel_flow)
    }
}

impl Default for MarOmegaReactorCore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mar_omega_nominal_10zw_flow() {
        let reactor = ContinuousFlowReactor::new_nominal();
        let m_dot_siphon = 1.256e8; // 20 M_sun/yr
        let tel = reactor.evaluate_flow_step(m_dot_siphon);

        assert!(tel.p_gross_watts > 9.99e21 && tel.p_gross_watts <= 1.01e22);
        assert_eq!(tel.stored_fuel_mass_kg, 0.0);
        assert!(tel.eta_direct_eff >= 0.99980);
        assert!(tel.p_sensible_cryo_watts < 250_000.0);
        assert!(tel.t_throat_kelvin < T_OP_MAX_K);
        assert!(tel.residual_throat_shear_mpa < 1.0e-6);
        assert!(tel.safety_invariants_locked);
    }

    #[test]
    fn test_mar_omega_reactor_core_step() {
        let mut core = MarOmegaReactorCore::new();
        let mut mmio_buf = [0u8; 1024];
        let res = core.step_reactor_flow(125_600_000, 32768, Some(mmio_buf.as_mut_ptr()));
        assert!(res.is_ok());
        let tel = res.unwrap();
        assert!(tel.safety_invariants_locked);
        let psi_raw = u32::from_ne_bytes(mmio_buf[0x0280..0x0284].try_into().unwrap());
        assert_eq!(psi_raw, 0x000015C5);
    }
}
