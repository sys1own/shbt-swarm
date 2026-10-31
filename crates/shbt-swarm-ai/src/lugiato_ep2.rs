//! `crates/shbt-swarm-ai/src/lugiato_ep2.rs`
//! Non-Linear Lugiato-Lefever Cavity Soliton Stability Basin at EP2 (`THM-AI-07-EXT`).

#![allow(dead_code)]

pub const ROUND_TRIP_TIME_PS: f64 = 1.701;
pub const HEXAMER_STAGES: u32 = 32;
pub const SINGLE_PASS_LATENCY_PS: f64 = 54.432;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LugiatoEp2Cavity {
    pub alpha_loss_per_pass: f64,
    pub round_trip_s: f64,
    pub kerr_gamma_w_m: f64,
    pub hexamer_length_m: f64,
    pub kappa_coupling_rad_s: f64,
    pub operator_spectral_radius: f64,
}

impl LugiatoEp2Cavity {
    #[inline]
    pub const fn new_default() -> Self {
        Self {
            alpha_loss_per_pass: 0.020,
            round_trip_s: 1.701e-12,
            kerr_gamma_w_m: 1.25e-2,
            hexamer_length_m: 1.80e-3,
            kappa_coupling_rad_s: 5.878_894_767e9,
            operator_spectral_radius: 1.00e-62,
        }
    }

    /// Returns the EP2 locking bandwidth Delta_omega_EP2 = alpha_loss / (2 * t_R).
    #[inline]
    pub const fn ep2_locking_bandwidth_rad_s(&self) -> f64 {
        self.alpha_loss_per_pass / (2.0 * self.round_trip_s)
    }

    /// Returns the critical intracavity intensity I_crit = Delta_omega_EP2 / (gamma * L_hex).
    #[inline]
    pub const fn critical_kerr_intensity_w_m2(&self) -> f64 {
        self.ep2_locking_bandwidth_rad_s() / (self.kerr_gamma_w_m * self.hexamer_length_m)
    }

    /// Evaluates the complex EP2 discriminant Delta = (d1 - d2)*((d1 - d2) - 4*i*kappa_c)
    /// on the symmetric soliton manifold where |E_1|^2 == |E_2|^2 == intensity_w_m2.
    #[inline]
    pub const fn symmetric_ep2_discriminant(&self, intensity_w_m2: f64) -> (f64, f64) {
        let d1 = self.kerr_gamma_w_m * intensity_w_m2 * self.hexamer_length_m;
        let d2 = self.kerr_gamma_w_m * intensity_w_m2 * self.hexamer_length_m;
        let diff = d1 - d2;
        (diff * diff, -4.0 * self.kappa_coupling_rad_s * diff)
    }

    /// Evaluates the Lyapunov functional time derivative upper bound dV/dt.
    #[inline]
    pub const fn lyapunov_time_derivative(&self, intensity_w_m2: f64, perturbation_norm_sq: f64) -> f64 {
        let kerr_shift_rate = self.kerr_gamma_w_m * intensity_w_m2 * self.hexamer_length_m;
        let damping_margin = self.ep2_locking_bandwidth_rad_s() - kerr_shift_rate;
        -damping_margin * perturbation_norm_sq
    }

    /// Computes the phase-locking margin Delta_phi_margin = arccos(I / I_crit) via 5th-order polynomial.
    #[inline]
    pub fn phase_locking_margin_rad(&self, intensity_w_m2: f64) -> f32 {
        let ratio = intensity_w_m2 / self.critical_kerr_intensity_w_m2();
        if ratio >= 1.0 {
            0.0
        } else if ratio <= 0.0 {
            core::f32::consts::FRAC_PI_2
        } else {
            let r = ratio as f32;
            let r3 = r * r * r;
            let r5 = r3 * r * r;
            core::f32::consts::FRAC_PI_2 - (r + 0.166_666_67 * r3 + 0.075 * r5)
        }
    }

    #[inline]
    pub const fn inference_latency_ps(&self) -> f64 {
        SINGLE_PASS_LATENCY_PS
    }

    #[inline]
    pub const fn spectral_radius(&self) -> f64 {
        self.operator_spectral_radius
    }
}
