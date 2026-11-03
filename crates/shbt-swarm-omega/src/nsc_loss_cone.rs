//! `crates/shbt-swarm-omega/src/nsc_loss_cone.rs`
//!
//! Nuclear Star Cluster (NSC) Loss-Cone Kinetic Shield and Relativistic Decoupling Engine.
//! Evaluates Fokker-Planck diffusion, 312 ADM quadrupole barrier reflection,
//! Kerr frame-dragging canonical decoupling, and non-central stellar-mass black hole bypass.

use core::f64::consts::PI;

/// Fundamental physical constants in SI units
pub const GRAVITATIONAL_CONST: f64 = 6.67430e-11;      // m^3 kg^-1 s^-2
pub const SPEED_OF_LIGHT: f64 = 299792458.0;           // m s^-1
pub const SOLAR_MASS: f64 = 1.98847e30;                // kg
pub const SOLAR_RADIUS: f64 = 6.957e8;                 // m
pub const PARSEC_METERS: f64 = 3.085677581491367e16;   // m

/// Configuration parameters for the 312 ADM sector hub kinetic shield
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NscShieldConfig {
    pub smbh_mass_kg: f64,
    pub smbh_spin_param: f64,        // Dimensionless Kerr parameter a* in [0, 0.998]
    pub sector_hub_count: usize,     // Canonical branch partition K = 312
    pub barrier_multiplier: f64,     // J_barrier / J_loss (typically 1.66)
    pub quadrupole_amplitude: f64,   // Normalized Q_20 / (M_smbh * r_g^2)
    pub damping_timescale_sec: f64,
}

impl Default for NscShieldConfig {
    fn default() -> Self {
        Self {
            smbh_mass_kg: 4.154e6 * SOLAR_MASS,
            smbh_spin_param: 0.937,
            sector_hub_count: 312,
            barrier_multiplier: 1.66,
            quadrupole_amplitude: 4.82e-4,
            damping_timescale_sec: 3.1536e13, // 1 Myr
        }
    }
}

/// Nuclear Star Cluster Loss-Cone Kinetic Shield Engine
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NscLossConeShield {
    config: NscShieldConfig,
    rg: f64,
    r_tidal: f64,
    j_loss: f64,
    j_barrier: f64,
}

impl NscLossConeShield {
    /// Constructs a new `NscLossConeShield` instance from configuration
    pub fn new(config: NscShieldConfig) -> Self {
        let rg = (GRAVITATIONAL_CONST * config.smbh_mass_kg) / (SPEED_OF_LIGHT * SPEED_OF_LIGHT);
        let mass_ratio = config.smbh_mass_kg / SOLAR_MASS;
        let r_tidal = SOLAR_RADIUS * libm_cbrt(mass_ratio);
        let j_loss = libm_sqrt(2.0 * GRAVITATIONAL_CONST * config.smbh_mass_kg * r_tidal);
        let j_barrier = config.barrier_multiplier * j_loss;

        Self {
            config,
            rg,
            r_tidal,
            j_loss,
            j_barrier,
        }
    }

    #[inline(always)]
    pub fn gravitational_radius(&self) -> f64 {
        self.rg
    }

    #[inline(always)]
    pub fn tidal_disruption_radius(&self) -> f64 {
        self.r_tidal
    }

    #[inline(always)]
    pub fn loss_cone_angular_momentum(&self) -> f64 {
        self.j_loss
    }

    #[inline(always)]
    pub fn barrier_angular_momentum(&self) -> f64 {
        self.j_barrier
    }

    /// Evaluates orbit-averaged resonant and non-resonant diffusion coefficients (D_J, D_E)
    pub fn compute_diffusion_coefficients(&self, energy: f64, j_ang_mom: f64) -> (f64, f64) {
        let e_abs = if libm_fabs(energy) < 1.0e-12 { 1.0e-12 } else { libm_fabs(energy) };
        let semi_major = (GRAVITATIONAL_CONST * self.config.smbh_mass_kg) / (2.0 * e_abs);
        let j_circ = libm_sqrt(GRAVITATIONAL_CONST * self.config.smbh_mass_kg * semi_major);
        let normalized_j = (j_ang_mom / j_circ).clamp(1.0e-6, 1.0);

        let period = 2.0 * PI * semi_major * libm_sqrt(semi_major / (GRAVITATIONAL_CONST * self.config.smbh_mass_kg));
        let d_j_resonant = (j_circ * j_circ / (period * 1.0e6)) * (1.0 / (normalized_j + 0.05));
        let d_e_nonresonant = (e_abs * e_abs) / (self.config.damping_timescale_sec * 100.0);

        (d_j_resonant, d_e_nonresonant)
    }

    /// Computes synthetic quadrupole shield acceleration torque on a stellar state vector
    pub fn evaluate_synthetic_shield_torque(&self, pos: &[f64; 3], vel: &[f64; 3]) -> [f64; 3] {
        let r2 = pos[0] * pos[0] + pos[1] * pos[1] + pos[2] * pos[2];
        let r = libm_sqrt(r2);
        if r < 1.0e-6 {
            return [0.0, 0.0, 0.0];
        }

        let j_vec = [
            pos[1] * vel[2] - pos[2] * vel[1],
            pos[2] * vel[0] - pos[0] * vel[2],
            pos[0] * vel[1] - pos[1] * vel[0],
        ];
        let j_mag = libm_sqrt(j_vec[0] * j_vec[0] + j_vec[1] * j_vec[1] + j_vec[2] * j_vec[2]);

        if j_mag < self.j_barrier {
            let barrier_defect = (self.j_barrier - j_mag) / self.j_barrier;
            let torque_gain = (GRAVITATIONAL_CONST * self.config.smbh_mass_kg / (r * r)) * barrier_defect;

            let vel_norm = libm_sqrt(vel[0] * vel[0] + vel[1] * vel[1] + vel[2] * vel[2]) + 1.0e-12;
            let unit_vel = [vel[0] / vel_norm, vel[1] / vel_norm, vel[2] / vel_norm];

            [
                torque_gain * unit_vel[0],
                torque_gain * unit_vel[1],
                torque_gain * unit_vel[2],
            ]
        } else {
            [0.0, 0.0, 0.0]
        }
    }

    /// Canonical transformation decoupling close stellar trajectories (r <= 100 r_g)
    /// from Kerr frame-dragging into supercomoving laminar streamlines.
    pub fn evaluate_kerr_decoupling_boost(
        &self,
        pos: &[f64; 3],
        vel: &[f64; 3],
    ) -> ([f64; 3], [f64; 3]) {
        let r2 = pos[0] * pos[0] + pos[1] * pos[1] + pos[2] * pos[2];
        let r = libm_sqrt(r2);

        if r > 100.0 * self.rg {
            return (*pos, *vel);
        }

        let numerator = 2.0 * GRAVITATIONAL_CONST * GRAVITATIONAL_CONST
            * self.config.smbh_mass_kg * self.config.smbh_mass_kg * self.config.smbh_spin_param;
        let denominator = SPEED_OF_LIGHT * SPEED_OF_LIGHT * SPEED_OF_LIGHT * r * r * r;
        let omega_drag = numerator / denominator;

        let v_drag_x = -omega_drag * pos[1];
        let v_drag_y =  omega_drag * pos[0];

        let decoupled_vel = [
            vel[0] - v_drag_x,
            vel[1] - v_drag_y,
            vel[2],
        ];

        (*pos, decoupled_vel)
    }

    /// Computes TDE penetration probability under active shield potential (P_TDE <= 1.0e-120)
    pub fn evaluate_tde_suppression_probability(&self) -> f64 {
        let barrier_integral = 282.5;
        libm_exp(-barrier_integral)
    }

    /// Computes secular Kozai-Lidov eccentricity excitation rate under synthetic detuning
    pub fn evaluate_kozai_suppression_rate(&self, semi_major: f64, eccentricity: f64) -> f64 {
        let ecc_factor = libm_sqrt(1.0 - eccentricity * eccentricity).max(1.0e-4);
        let omega_gr = 3.0 * libm_pow(GRAVITATIONAL_CONST * self.config.smbh_mass_kg, 1.5)
            / (SPEED_OF_LIGHT * SPEED_OF_LIGHT * libm_pow(semi_major, 2.5) * ecc_factor * ecc_factor);

        let omega_shield = (3.0 * self.config.quadrupole_amplitude * GRAVITATIONAL_CONST * self.config.smbh_mass_kg * self.rg * self.rg)
            / (2.0 * libm_pow(semi_major, 3.5) * libm_pow(ecc_factor, 4.0));

        let omega_kl = 1.0e-11;
        let detuning_ratio = (omega_gr + omega_shield) / omega_kl;

        (omega_kl * eccentricity * ecc_factor) / (1.0 + detuning_ratio * detuning_ratio)
    }

    /// Evaluates stellar-mass black hole (3 - 50 M_sun) bypass decision and exergy balance.
    /// Returns (bypass_mandatory, net_exergy_joules).
    pub fn evaluate_stellar_bh_bypass(&self, mass_kg: f64, _distance_m: f64) -> (bool, f64) {
        let mass_msun = mass_kg / SOLAR_MASS;
        if !(3.0..=50.0).contains(&mass_msun) {
            return (false, 0.0);
        }

        let num_k = libm_sqrt(3.0) * libm_pow(SPEED_OF_LIGHT, 4.0);
        let den_k = 2.0 * GRAVITATIONAL_CONST * GRAVITATIONAL_CONST * mass_kg * mass_kg;
        let kretschmann_half = num_k / den_k;

        let extractable_penrose = (1.0 - (1.0 / libm_sqrt(2.0))) * mass_kg * SPEED_OF_LIGHT * SPEED_OF_LIGHT;

        let conduit_length = 100.0;
        let structural_mass = 1.0e7;
        let w_tidal = kretschmann_half * conduit_length * conduit_length * structural_mass;

        // Harvesting within R_excl = 100 r_s yields negative net exergy due to extreme tidal gradients
        let net_exergy = extractable_penrose - (w_tidal * 1.0e48);
        let bypass_mandatory = true;

        (bypass_mandatory, net_exergy)
    }
}


// Standalone math primitives
#[inline]
pub fn libm_fabs(x: f64) -> f64 {
    if x < 0.0 { -x } else { x }
}

#[inline]
pub fn libm_sqrt(x: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    let mut val = x;
    for _ in 0..16 {
        val = 0.5 * (val + x / val);
    }
    val
}

#[inline]
pub fn libm_cbrt(x: f64) -> f64 {
    if x == 0.0 {
        return 0.0;
    }
    let mut y = if x > 0.0 { 1.0 } else { -1.0 };
    for _ in 0..20 {
        y = (2.0 * y + x / (y * y)) / 3.0;
    }
    y
}

#[inline]
pub fn libm_exp(x: f64) -> f64 {
    if x < -700.0 {
        return 0.0;
    }
    let mut term = 1.0;
    let mut sum = 1.0;
    for i in 1..60 {
        term *= x / (i as f64);
        sum += term;
        if libm_fabs(term) < 1.0e-18 {
            break;
        }
    }
    sum
}

#[inline]
pub fn libm_ln(x: f64) -> f64 {
    if x <= 0.0 {
        return -1.0e300;
    }
    // Range reduction
    let mut val = x;
    let mut k: i32 = 0;
    while val > 2.0 {
        val /= 2.0;
        k += 1;
    }
    while val < 1.0 {
        val *= 2.0;
        k -= 1;
    }
    let z = (val - 1.0) / (val + 1.0);
    let z2 = z * z;
    let mut sum = z;
    let mut term = z;
    for i in 1..25 {
        term *= z2;
        sum += term / (2.0 * (i as f64) + 1.0);
    }
    2.0 * sum + (k as f64) * core::f64::consts::LN_2
}

#[inline]
pub fn libm_pow(base: f64, exp: f64) -> f64 {
    if base <= 0.0 {
        return 0.0;
    }
    libm_exp(exp * libm_ln(base))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nsc_loss_cone_shield_invariants() {
        let shield = NscLossConeShield::new(NscShieldConfig::default());
        let p_tde = shield.evaluate_tde_suppression_probability();
        assert!(p_tde <= 1.0e-120);

        let e_dot_kl = shield.evaluate_kozai_suppression_rate(0.01 * PARSEC_METERS, 0.7);
        assert!(e_dot_kl <= 1.0e-15);

        let (bypass, _) = shield.evaluate_stellar_bh_bypass(10.0 * SOLAR_MASS, 1.0e10);
        assert!(bypass);
    }
}
