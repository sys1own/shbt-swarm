//! `crates/shbt-swarm-logos/src/acceleration_mossbauer.rs`
//! Acceleration Mossbauer Line Narrowing in 57Fe-Diamond Computronium (`THM-LOGOS-08-EXT`).

#![allow(dead_code)]

pub const FE57_WAVEVECTOR_M_INV: f64 = 7.304e10;
pub const FE57_LIFETIME_TAU_S: f64 = 1.411e-7;
pub const FE57_NATURAL_GAMMA_RAD_S: f64 = 7.087_172_2e6;
#[allow(clippy::approx_constant)]
pub const NOMINAL_LAMB_MOSSBAUER_FM: f64 = 0.7854;
pub const NOMINAL_BORRMANN_TAU: f64 = 1462.8;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MossbauerStarkCompensator {
    pub k_gamma_m_inv: f64,
    pub tau_nuc_s: f64,
    pub closed_loop_locked: bool,
}

impl MossbauerStarkCompensator {
    #[inline]
    pub const fn new_default() -> Self {
        Self {
            k_gamma_m_inv: FE57_WAVEVECTOR_M_INV,
            tau_nuc_s: FE57_LIFETIME_TAU_S,
            closed_loop_locked: true,
        }
    }

    /// Computes non-inertial acceleration Doppler shift Delta_omega_acc = k_gamma * a * tau_nuc (rad/s).
    #[inline]
    pub const fn acceleration_doppler_rad_per_s(&self, accel_mps2: f64) -> f64 {
        self.k_gamma_m_inv * accel_mps2 * self.tau_nuc_s
    }

    /// Returns the compensating quadratic electro-optic Stark/Zeeman shift in Hz.
    #[inline]
    pub const fn required_stark_detuning_hz(&self, accel_mps2: f64) -> f64 {
        if self.closed_loop_locked {
            -self.acceleration_doppler_rad_per_s(accel_mps2) / (2.0 * core::f64::consts::PI)
        } else {
            0.0
        }
    }

    /// Returns residual nuclear transition detuning (rad/s) after Stark/Zeeman compensation.
    #[inline]
    pub const fn residual_detuning_rad_per_s(&self, accel_mps2: f64) -> f64 {
        let doppler = self.acceleration_doppler_rad_per_s(accel_mps2);
        let stark = if self.closed_loop_locked { -doppler } else { 0.0 };
        doppler + stark
    }

    /// Returns (Lamb-Mossbauer factor f_M, Borrmann attenuation figure of merit tau_B).
    #[inline]
    pub const fn compensated_mossbauer_metrics(&self, accel_mps2: f64) -> (f64, f64) {
        let residual = self.residual_detuning_rad_per_s(accel_mps2);
        let half_gamma_sq = 0.25 * FE57_NATURAL_GAMMA_RAD_S * FE57_NATURAL_GAMMA_RAD_S;
        let lorentzian = half_gamma_sq / (residual * residual + half_gamma_sq);
        (NOMINAL_LAMB_MOSSBAUER_FM * lorentzian, NOMINAL_BORRMANN_TAU * lorentzian)
    }
}
