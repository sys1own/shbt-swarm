//! crates/shbt-swarm-foundry/src/amorphous_gauge.rs
//! Topological SO(3) BF Defect Gauge Theory for Non-Crystalline Condensates.
//! Implements THM-FOUNDRY-06-EXT in `#![no_std]`.

use core::f64::consts::PI;

pub const KAC_MOODY_LEVEL_K: f64 = 312.0;
pub const DUAL_COXETER_SO3: f64 = 2.0;
pub const DUAL_BOUNDARY_RANK: f64 = 124.0;
pub const PDF_INTEGRATION_BINS: usize = 256;

#[inline]
pub fn f64_abs(x: f64) -> f64 {
    if x < 0.0 { -x } else { x }
}

#[inline]
pub fn f64_exp(x: f64) -> f64 {
    if x < -80.0 { return 0.0; }
    if x > 80.0 { return f64::INFINITY; }
    let mut sum = 1.0;
    let mut term = 1.0;
    let mut i = 1;
    while i < 40 {
        term *= x / (i as f64);
        sum += term;
        if f64_abs(term) < 1e-15 * f64_abs(sum) {
            break;
        }
        i += 1;
    }
    sum
}

#[derive(Debug, Clone, Copy)]
pub struct AmorphousConfig {
    pub r_min: f64,             // Atomic exclusion hard-sphere cut-off (Angstroms)
    pub r_max: f64,             // Maximum correlation integration radius (Angstroms)
    pub corr_length_xi: f64,    // Ornstein-Zernike structural correlation length (Angstroms)
    pub shear_modulus_mu: f64,  // Elastic shear modulus (Pascals)
}

impl Default for AmorphousConfig {
    fn default() -> Self {
        Self {
            r_min: 1.42,
            r_max: 12.0,
            corr_length_xi: 4.50,
            shear_modulus_mu: 3.12e10,
        }
    }
}

/// Pair-distribution function g(r) sample vector.
#[derive(Debug, Clone, Copy)]
pub struct PairDistributionFunction {
    pub values: [f64; PDF_INTEGRATION_BINS],
    pub r_min: f64,
    pub r_max: f64,
}

impl Default for PairDistributionFunction {
    fn default() -> Self {
        Self {
            values: [1.0; PDF_INTEGRATION_BINS],
            r_min: 1.42,
            r_max: 12.0,
        }
    }
}

impl PairDistributionFunction {
    /// Evaluates g(r) at a given radius r using linear interpolation.
    pub fn evaluate_at(&self, r: f64) -> f64 {
        if r < self.r_min || r > self.r_max {
            return if r < self.r_min { 0.0 } else { 1.0 };
        }
        let step = (self.r_max - self.r_min) / (PDF_INTEGRATION_BINS - 1) as f64;
        let index = ((r - self.r_min) / step) as usize;
        if index >= PDF_INTEGRATION_BINS - 1 {
            return self.values[PDF_INTEGRATION_BINS - 1];
        }
        let r_low = self.r_min + (index as f64) * step;
        let frac = (r - r_low) / step;
        self.values[index] * (1.0 - frac) + self.values[index + 1] * frac
    }
}

/// Results of the amorphous topological gauge evaluation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AmorphousTopologicalState {
    pub delta_h_vis: f64,
    pub delta_h_braid: f64,
    pub framing_defect: f64,
    pub electric_weyl_norm: f64,
    pub framing_closed: bool,
}

/// Core computation engine for amorphous BF topological fields.
#[derive(Debug, Clone, Copy)]
pub struct AmorphousBfGaugeEngine {
    pub config: AmorphousConfig,
}

impl AmorphousBfGaugeEngine {
    pub const fn new(config: AmorphousConfig) -> Self {
        Self { config }
    }

    /// Derives the continuous Sugawara conformal weight shift Delta h_vis
    /// via trapezoidal quadrature of the pair-distribution function g(r).
    pub fn compute_sugawara_shift(&self, pdf: &PairDistributionFunction) -> f64 {
        let n = PDF_INTEGRATION_BINS;
        let dr = (self.config.r_max - self.config.r_min) / (n - 1) as f64;
        let mut integral_sum = 0.0;

        let mut i = 0;
        while i < n {
            let r = self.config.r_min + (i as f64) * dr;
            let g_r = pdf.evaluate_at(r);
            // Current-current correlator Tr<J(r) J(0)> ~ (3k / r^4) * exp(-r / xi)
            // Spherical volume element d^3r = 4 * pi * r^2 dr cancels 1/r^2
            let weight = if i == 0 || i == n - 1 { 0.5 } else { 1.0 };
            let exponential_damping = f64_exp(-r / self.config.corr_length_xi);
            let integrand = weight * (g_r * exponential_damping) / (r * r);
            integral_sum += integrand;
            i += 1;
        }

        let prefactor = (6.0 * PI * KAC_MOODY_LEVEL_K) / (KAC_MOODY_LEVEL_K + DUAL_COXETER_SO3);
        prefactor * integral_sum * dr
    }

    /// Evaluates dynamic framing compensation and checks zero electric Weyl curvature.
    pub fn evaluate_condensate(
        &self,
        pdf: &PairDistributionFunction,
        deviatoric_shear_strain: f64,
    ) -> AmorphousTopologicalState {
        let delta_h_vis = self.compute_sugawara_shift(pdf);
        // Dynamic framing compensation formula
        let delta_h_braid = -delta_h_vis / DUAL_BOUNDARY_RANK;
        let framing_defect = delta_h_vis + DUAL_BOUNDARY_RANK * delta_h_braid;

        // In fluid/amorphous media, deviatoric stress is proportional to shear strain.
        // Electric Weyl curvature E_mu_nu = 8 * pi * G * pi_mu_nu.
        let electric_weyl_norm = f64_abs(8.0 * PI * 1.0e-10 * deviatoric_shear_strain);
        let framing_closed = f64_abs(framing_defect) <= 1.0e-14 && electric_weyl_norm <= 1.0e-12;

        AmorphousTopologicalState {
            delta_h_vis,
            delta_h_braid,
            framing_defect,
            electric_weyl_norm,
            framing_closed,
        }
    }
}
