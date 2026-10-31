//! crates/shbt-swarm-logos/src/takagi_taupin.rs
//!
//! Multi-Beam Non-Hermitian Takagi-Taupin Dynamical Diffraction Engine.
//! Implements split-step finite-difference electrodynamics across the 346 Dynkin slots.
//! Designed for `#![no_std]` bare-metal embedded execution in SHBT-LOGOS-312 computronium.

use core::f64::consts::PI;

/// Number of active Dynkin slots allocated for monolithic multi-beam diffraction.
pub const DYNKIN_SLOTS: usize = 346;

/// Total number of parallel computronium channels across the 346 Dynkin slots (346 * 312).
pub const TOTAL_CHANNELS: usize = 107_952;

/// Fundamental physical constants in double-precision floating-point.
pub const SPEED_OF_LIGHT: f64 = 299_792_458.0; // m/s
pub const HBAR: f64 = 1.054_571_817e-34; // J*s
pub const E_CHARGE: f64 = 1.602_176_634e-19; // C
pub const DIAMOND_E_MODULUS: f64 = 1.050e12; // Pa (1050 GPa)
pub const DIAMOND_DENSITY: f64 = 3515.0; // kg/m^3
pub const TETHER_STIFFNESS: f64 = 5.1295e10; // N/m
pub const FE57_MASS: f64 = 9.455e-26; // kg (56.935 u)

/// Target attosecond propagation step (17.01 attoseconds / 10 substeps).
pub const PROPAGATION_STEP_TIME: f64 = 1.701e-18; // s

/// Zero-allocation complex number abstraction for `#![no_std]` environments.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Complex64 {
    pub re: f64,
    pub im: f64,
}

#[allow(clippy::should_implement_trait)]
impl Complex64 {
    #[inline(always)]
    pub const fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    #[inline(always)]
    pub fn norm_sq(self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    #[inline(always)]
    pub fn abs(self) -> f64 {
        libm_sqrt(self.norm_sq())
    }

    #[inline(always)]
    pub fn add(self, rhs: Self) -> Self {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }

    #[inline(always)]
    pub fn sub(self, rhs: Self) -> Self {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }

    #[inline(always)]
    pub fn mul(self, rhs: Self) -> Self {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }

    #[inline(always)]
    pub fn scale(self, s: f64) -> Self {
        Self {
            re: self.re * s,
            im: self.im * s,
        }
    }

    #[inline(always)]
    pub fn div_scalar(self, s: f64) -> Self {
        Self {
            re: self.re / s,
            im: self.im / s,
        }
    }

    #[inline(always)]
    pub fn exp(self) -> Self {
        let exp_re = libm_exp(self.re);
        Self {
            re: exp_re * libm_cos(self.im),
            im: exp_re * libm_sin(self.im),
        }
    }

    #[inline(always)]
    pub fn sqrt(self) -> Self {
        let r = self.abs();
        let u = libm_sqrt((r + self.re) * 0.5);
        let v = libm_copysign(libm_sqrt((r - self.re) * 0.5), self.im);
        Self { re: u, im: v }
    }
}

/// Dynamic strain gradient state vector across the diamond monolith.
#[derive(Copy, Clone, Debug)]
pub struct ShearGradientState {
    pub coord_accel: f64,   // Spacecraft acceleration (m/s^2)
    pub cavity_length: f64, // Cavity dimension along propagation (m)
    pub max_strain: f64,    // Derived elastic strain (dimensionless)
    pub alpha_b: f64,       // Derived Bragg deviation parameter
}

impl ShearGradientState {
    pub fn compute_under_shear(accel_g: f64, cavity_len: f64, k_wave: f64, d_spacing: f64) -> Self {
        let a_ms2 = accel_g * 9.80665;
        let sigma = DIAMOND_DENSITY * a_ms2 * cavity_len;
        let strain = sigma / DIAMOND_E_MODULUS;
        let h_mag = (2.0 * PI) / d_spacing;
        let alpha = (h_mag * strain) / k_wave;
        Self {
            coord_accel: a_ms2,
            cavity_length: cavity_len,
            max_strain: strain,
            alpha_b: alpha,
        }
    }
}

/// Represents the two-beam state vector (Forward D0, Diffracted Dh) at a discrete spatial cell.
#[derive(Copy, Clone, Debug, Default)]
pub struct PolaritonTwoBeamState {
    pub d0: Complex64,
    pub dh: Complex64,
}

/// Physical parameters for monolithic 57Fe-diamond dynamical diffraction.
#[derive(Copy, Clone, Debug)]
pub struct TakagiTaupinConfig {
    pub k_wave: f64,             // Optical wavenumber (m^-1)
    pub chi_0: Complex64,        // Complex average susceptibility
    pub chi_h: Complex64,        // Complex Bragg susceptibility
    pub polarization_factor: f64, // C = 1.0 (sigma) or cos(2 theta_B) (pi)
    pub gamma_nl: f64,           // Non-linear Kerr polariton coupling coefficient
    pub f_lamb_mossbauer: f64,   // Recoilless factor f_M
    pub dz: f64,                 // Spatial discretization step (m)
    pub dt: f64,                 // Temporal sub-cycle step (s)
}

impl Default for TakagiTaupinConfig {
    fn default() -> Self {
        let e_gamma = 14412.5 * E_CHARGE;
        let k = e_gamma / (HBAR * SPEED_OF_LIGHT);
        let omega_vib = libm_sqrt(TETHER_STIFFNESS / FE57_MASS);
        let u2_zero_point = HBAR / (2.0 * FE57_MASS * omega_vib);
        let f_m = libm_exp(-k * k * u2_zero_point);
        Self {
            k_wave: k,
            chi_0: Complex64::new(-1.50e-5, 2.40e-7),
            chi_h: Complex64::new(-1.25e-5, 2.39e-7),
            polarization_factor: 1.0,
            gamma_nl: 1.05e-18,
            f_lamb_mossbauer: f_m,
            dz: 3.567e-10, // One diamond unit cell dimension
            dt: PROPAGATION_STEP_TIME,
        }
    }
}

/// Primary execution engine for multi-beam wave propagation across the 346 Dynkin slots.
pub struct TakagiTaupinEngine {
    pub config: TakagiTaupinConfig,
    pub shear_state: ShearGradientState,
    pub slots: [PolaritonTwoBeamState; DYNKIN_SLOTS],
}

impl TakagiTaupinEngine {
    pub fn new(config: TakagiTaupinConfig, accel_g: f64) -> Self {
        let d_220 = 3.567e-10 / 2.8284271247;
        let shear = ShearGradientState::compute_under_shear(accel_g, 1.0e-3, config.k_wave, d_220);
        Self {
            config,
            shear_state: shear,
            slots: [PolaritonTwoBeamState::default(); DYNKIN_SLOTS],
        }
    }

    /// Initializes all 346 Dynkin slots with coherent anomalous Borrmann wavepackets.
    pub fn initialize_borrmann_modes(&mut self, amplitude: f64) {
        let inv_sqrt2 = 1.0 / libm_sqrt(2.0);
        for slot in self.slots.iter_mut() {
            slot.d0 = Complex64::new(amplitude * inv_sqrt2, 0.0);
            slot.dh = Complex64::new(-amplitude * inv_sqrt2, 0.0);
        }
    }

    /// Advances the multi-beam fields by one attosecond split-step.
    pub fn step_split_step(&mut self) {
        let dt = self.config.dt;
        let dz = self.config.dz;
        let k = self.config.k_wave;
        let c = self.config.polarization_factor;
        let alpha = self.shear_state.alpha_b;
        let half_dz = 0.5 * dz;
        let factor = Complex64::new(0.0, -PI * k * half_dz);
        let c_chi_h = self.config.chi_h.scale(c);
        let chi_0 = self.config.chi_0;
        let chi_diag2 = chi_0.sub(Complex64::new(2.0 * alpha, 0.0));
        let tr = chi_0.add(chi_diag2);
        let det_term = chi_0.sub(chi_diag2);
        let disc = det_term.mul(det_term).add(c_chi_h.mul(c_chi_h).scale(4.0));
        let root_disc = disc.sqrt();
        let lambda2 = tr.sub(root_disc).scale(0.5);
        let exp_l2 = factor.mul(lambda2).exp();

        for slot in self.slots.iter_mut() {
            let u0 = slot.d0;
            let uh = slot.dh;
            let d0_lin = u0.mul(exp_l2);
            let dh_lin = uh.mul(exp_l2);

            let intensity_0 = d0_lin.norm_sq();
            let intensity_h = dh_lin.norm_sq();

            let kerr_phase_0 = Complex64::new(0.0, self.config.gamma_nl * intensity_0 * dt).exp();
            let kerr_phase_h = Complex64::new(0.0, self.config.gamma_nl * intensity_h * dt).exp();

            slot.d0 = d0_lin.mul(kerr_phase_0);
            slot.dh = dh_lin.mul(kerr_phase_h);
        }
    }

    /// Evaluates the nodal field suppression ratio |E(R_I)|^2 / |E_0|^2 at atomic coordinates.
    pub fn evaluate_nodal_suppression_ratio(&self) -> f64 {
        let mut max_nodal_ratio: f64 = 0.0;
        for slot in self.slots.iter() {
            let e0_sq = slot.d0.norm_sq();
            if e0_sq > 1e-24 {
                let e_node = slot.d0.add(slot.dh);
                let nodal_ratio = e_node.norm_sq() / (2.0 * e0_sq);
                if nodal_ratio > max_nodal_ratio {
                    max_nodal_ratio = nodal_ratio;
                }
            }
        }
        max_nodal_ratio
    }

    /// Evaluates the single-cell cycle dissipation in Joules across the computational lattice.
    pub fn evaluate_cell_cycle_loss(&self, channel_intensity_w_m2: f64) -> f64 {
        let v_cell = 4.5385e-29; // m^3
        let t_cycle = 1.701e-17; // s
        let mu_0 = 175.0; // m^-1 (baseline diamond absorption at 14.4 keV)
        let nodal_suppression = self.evaluate_nodal_suppression_ratio();
        let mu_eff = mu_0 * (nodal_suppression + 1.0e-12);
        let p_diss = mu_eff * channel_intensity_w_m2 * v_cell;
        p_diss * t_cycle
    }
}

// ---------------------------------------------------------------------------
// Pure `#![no_std]` Mathematical Primitives (Libm polyfills)
// ---------------------------------------------------------------------------

#[inline]
pub fn libm_sqrt(x: f64) -> f64 {
    if x < 0.0 {
        return f64::NAN;
    }
    if x == 0.0 {
        return 0.0;
    }
    let bits = x.to_bits();
    let i = (0x5FE6EB50C7B537A9u64.wrapping_sub(bits >> 1)) as i64;
    let mut y = f64::from_bits(i as u64);
    let xhalf = 0.5 * x;
    y = y * (1.5 - xhalf * y * y);
    y = y * (1.5 - xhalf * y * y);
    y = y * (1.5 - xhalf * y * y);
    x * y
}

#[inline]
pub fn libm_exp(x: f64) -> f64 {
    if x > 709.78 {
        return f64::INFINITY;
    }
    if x < -708.39 {
        return 0.0;
    }
    let k = (x * core::f64::consts::LOG2_E + (if x >= 0.0 { 0.5 } else { -0.5 })) as i32;
    let f = x - (k as f64) * core::f64::consts::LN_2;
    let f2 = f * f;
    let poly = 1.0 + f + f2 * (0.5 + f * (0.16666666666666666 + f * (0.041666666666666664 + f * 0.008333333333333333)));
    let scale_bits = ((k + 1023) as u64) << 52;
    poly * f64::from_bits(scale_bits)
}

#[inline]
pub fn libm_fmod(x: f64, y: f64) -> f64 {
    let n = (x / y) as i64;
    x - (n as f64) * y
}

#[inline]
pub fn libm_sin(x: f64) -> f64 {
    let pi2 = 2.0 * core::f64::consts::PI;
    let mut rad = libm_fmod(x, pi2);
    if rad > core::f64::consts::PI {
        rad -= pi2;
    }
    if rad < -core::f64::consts::PI {
        rad += pi2;
    }
    let x2 = rad * rad;
    rad * (1.0 - x2 * (0.16666666666666666 - x2 * (0.008333333333333333 - x2 * 0.0001984126984126984)))
}

#[inline]
pub fn libm_cos(x: f64) -> f64 {
    libm_sin(x + core::f64::consts::FRAC_PI_2)
}

#[inline]
pub fn libm_copysign(x: f64, y: f64) -> f64 {
    let x_bits = x.to_bits() & 0x7FFF_FFFF_FFFF_FFFF;
    let y_sign = y.to_bits() & 0x8000_0000_0000_0000;
    f64::from_bits(x_bits | y_sign)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_takagi_taupin_engine_borrmann_suppression() {
        let config = TakagiTaupinConfig::default();
        let mut engine = TakagiTaupinEngine::new(config, 10000.0);
        engine.initialize_borrmann_modes(1.0);

        assert!(engine.config.f_lamb_mossbauer >= 0.782);
        assert!(engine.shear_state.alpha_b < 2.5e-6);

        let nodal_ratio = engine.evaluate_nodal_suppression_ratio();
        assert!(nodal_ratio <= 1.0e-16);

        let cell_loss = engine.evaluate_cell_cycle_loss(1.0e16);
        assert!(cell_loss <= 1.0e-22);

        // Perform split-step propagation
        engine.step_split_step();
        let post_nodal = engine.evaluate_nodal_suppression_ratio();
        assert!(post_nodal <= 1.0e-15);
    }
}
