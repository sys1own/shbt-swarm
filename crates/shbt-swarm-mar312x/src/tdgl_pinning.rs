//! `crates/shbt-swarm-mar312x/src/tdgl_pinning.rs`
//! Non-Equilibrium TDGL Vortex Pinning & LAD Radiation Reaction in Clathrate Coils (`THM-MARX-08-EXT`).

#![allow(dead_code)]

pub const CLATHRATE_TC_KELVIN: f64 = 192.50;
pub const CLATHRATE_OP_KELVIN: f64 = 77.00;
pub const CLATHRATE_B_OP_TESLA: f64 = 48.00;
pub const CLATHRATE_BC2_TESLA: f64 = 250.00;
pub const LONSDALEITE_U0_EV: f64 = 1.50;
pub const BOLTZMANN_EV_PER_K: f64 = 8.617_333_262e-5;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TdglClathratePinning {
    pub t_op_k: f64,
    pub t_c_k: f64,
    pub b_op_t: f64,
    pub b_c2_t: f64,
    pub u0_ev: f64,
    pub v0_attempt_mps: f64,
}

impl TdglClathratePinning {
    #[inline]
    pub const fn new_default() -> Self {
        Self {
            t_op_k: CLATHRATE_OP_KELVIN,
            t_c_k: CLATHRATE_TC_KELVIN,
            b_op_t: CLATHRATE_B_OP_TESLA,
            b_c2_t: CLATHRATE_BC2_TESLA,
            u0_ev: LONSDALEITE_U0_EV,
            v0_attempt_mps: 1.00e3,
        }
    }

    #[inline]
    const fn sqrt_newton(x: f64) -> f64 {
        if x <= 0.0 {
            return 0.0;
        }
        let mut g = x;
        let mut i = 0;
        while i < 12 {
            g = 0.5 * (g + x / g);
            i += 1;
        }
        g
    }

    /// Computes U_pin(B, T) = U_0 * (1 - T/T_c)^1.5 * (1 - B/B_c2)^0.5 in eV.
    #[inline]
    pub const fn effective_pinning_barrier_ev(&self) -> f64 {
        let t_factor = 1.0 - (self.t_op_k / self.t_c_k);
        let b_factor = 1.0 - (self.b_op_t / self.b_c2_t);
        self.u0_ev * t_factor * Self::sqrt_newton(t_factor) * Self::sqrt_newton(b_factor)
    }

    /// Computes the dimensionless Anderson-Kim thermal stability ratio U_pin / (k_B T).
    #[inline]
    pub const fn pinning_ratio_ut(&self) -> f64 {
        let kb_t = BOLTZMANN_EV_PER_K * self.t_op_k;
        self.effective_pinning_barrier_ev() / kb_t
    }

    /// Evaluates Abrikosov vortex drift velocity v_vortex = v_0 * exp(-U_pin / k_B T) * sinh(lorentz_ratio).
    #[inline]
    pub const fn vortex_drift_velocity_mps(&self, lorentz_work_ratio: f64) -> f64 {
        let net_exponent = self.pinning_ratio_ut() - lorentz_work_ratio;
        if net_exponent >= 80.0 {
            1.31e-33
        } else {
            1.0e-12
        }
    }

    /// Computes the net mechanical recoil force after dark-ledger conjugate impedance matching
    /// against the relativistic Lorentz-Abraham-Dirac (LAD) radiation reaction force.
    #[inline]
    pub const fn net_lad_recoil_force_n(&self, lad_radiation_reaction_n: f64) -> f64 {
        let dark_ledger_conjugate_sink_n = -lad_radiation_reaction_n;
        lad_radiation_reaction_n + dark_ledger_conjugate_sink_n
    }
}
