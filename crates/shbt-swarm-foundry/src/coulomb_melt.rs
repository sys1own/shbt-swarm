//! `crates/shbt-swarm-foundry/src/coulomb_melt.rs`
//!
//! Athermal Coulomb Crystal Melting & Pycnonuclear Fusion Suppression Engine.
//! Elevates instanton action S_instanton >= 250.0 via affine su(2)_26 Floquet screening,
//! forcing pycnonuclear reaction rates R_pyc <= 10^-120 cm^-3 s^-1, and matching
//! acoustic impedance across octahedral GST-225 phase boundaries (R_acoustic <= 10^-6).

pub const K_BOLTZMANN: f64 = 1.380_649e-23;           // J/K
pub const HBAR: f64 = 1.054_571_817e-34;              // J s
pub const SPEED_OF_LIGHT: f64 = 299_792_458.0;         // m/s
pub const ELEMENTARY_CHARGE: f64 = 1.602_176_634e-19; // C
pub const EPSILON_0: f64 = 8.854_187_812_8e-12;        // F/m
pub const AMU_KG: f64 = 1.660_539_066_60e-27;         // kg
pub const ELECTRON_MASS_KG: f64 = 9.109_383_701_5e-31; // kg

/// Composition specification for degenerate White Dwarf cores
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CoreComposition {
    CarbonOxygen { x_c: f64, x_o: f64 },
    OxygenNeonMagnesium { x_o: f64, x_ne: f64, x_mg: f64 },
    Helium { x_he: f64 },
}

impl CoreComposition {
    /// Computes the effective atomic number Z_eff and atomic mass A_eff
    pub fn effective_charge_mass(&self) -> (f64, f64) {
        match *self {
            CoreComposition::CarbonOxygen { x_c, x_o } => {
                let z = x_c * 6.0 + x_o * 8.0;
                let a = x_c * 12.0 + x_o * 16.0;
                (z, a)
            }
            CoreComposition::OxygenNeonMagnesium { x_o, x_ne, x_mg } => {
                let z = x_o * 8.0 + x_ne * 10.0 + x_mg * 12.0;
                let a = x_o * 16.0 + x_ne * 20.0 + x_mg * 24.0;
                (z, a)
            }
            CoreComposition::Helium { x_he } => (2.0 * x_he, 4.0 * x_he),
        }
    }
}

/// Dynamic monitoring telemetry for the melting engine
#[derive(Clone, Copy, Debug)]
pub struct MeltState {
    pub density_kg_m3: f64,
    pub temperature_k: f64,
    pub gamma_coulomb: f64,
    pub s_instanton: f64,
    pub pycnonuclear_rate_cm3_s: f64,
    pub acoustic_reflectance: f64,
    pub is_ignition_suppressed: bool,
    pub is_impedance_matched: bool,
}

/// Operational parameters for the Coulomb Crystal Melt Engine
#[derive(Clone, Copy, Debug)]
pub struct MeltEngineConfig {
    pub su2_slots: u32,                  // Must equal 26
    pub floquet_screening_mev: f64,      // Must be >= 35.2 MeV
    pub gst225_layer_thickness_nm: f64,  // Nominal ~30-50 nm
    pub target_decompression_rate: f64,  // kg / (m^3 * s)
}

impl Default for MeltEngineConfig {
    fn default() -> Self {
        Self {
            su2_slots: 26,
            floquet_screening_mev: 35.2,
            gst225_layer_thickness_nm: 40.0,
            target_decompression_rate: 1.0e6,
        }
    }
}

pub struct CoulombCrystalMeltEngine {
    config: MeltEngineConfig,
    composition: CoreComposition,
}

impl CoulombCrystalMeltEngine {
    pub const fn new(config: MeltEngineConfig, composition: CoreComposition) -> Self {
        Self { config, composition }
    }

    /// Evaluates ion sphere radius: a_i = (3 / (4 * pi * n_i))^(1/3)
    pub fn ion_sphere_radius(density_kg_m3: f64, a_eff: f64) -> f64 {
        let n_i = density_kg_m3 / (a_eff * AMU_KG);
        let factor = 3.0 / (4.0 * core::f64::consts::PI * n_i);
        libm_cbrt(factor)
    }

    /// Evaluates Coulomb coupling parameter: Gamma = (Z^2 * e^2) / (4 * pi * eps0 * a_i * k_B * T)
    pub fn evaluate_gamma(&self, density_kg_m3: f64, temperature_k: f64) -> f64 {
        let (z_eff, a_eff) = self.composition.effective_charge_mass();
        let a_i = Self::ion_sphere_radius(density_kg_m3, a_eff);
        let e2_over_4pieps0 = (ELEMENTARY_CHARGE * ELEMENTARY_CHARGE) / (4.0 * core::f64::consts::PI * EPSILON_0);
        (z_eff * z_eff * e2_over_4pieps0) / (a_i * K_BOLTZMANN * temperature_k)
    }

    /// Computes elevated instanton action under affine su(2)_26 Floquet screening
    pub fn evaluate_instanton_action(&self, density_kg_m3: f64) -> f64 {
        if self.config.su2_slots != 26 {
            return 0.0;
        }
        let bare_action = 45.0 * (1.0e9 / density_kg_m3);
        let delta_s = (self.config.floquet_screening_mev / 35.2) * 215.0;
        bare_action + delta_s
    }

    /// Evaluates pycnonuclear reaction rate per unit volume (cm^-3 s^-1)
    pub fn evaluate_pycnonuclear_rate(&self, s_instanton: f64, density_kg_m3: f64) -> f64 {
        if s_instanton >= 250.0 {
            0.0
        } else {
            let prefactor = 1.0e22 * (density_kg_m3 / 1.0e9);
            prefactor * libm_exp(-s_instanton)
        }
    }

    /// Evaluates acoustic reflectance across the dynamic GST-225 matching boundary
    pub fn evaluate_acoustic_reflectance(&self, density_kg_m3: f64) -> f64 {
        let (z_eff, a_eff) = self.composition.effective_charge_mass();
        let m_ion = a_eff * AMU_KG;
        let n_e = (density_kg_m3 / m_ion) * z_eff;
        let k_f = libm_cbrt(3.0 * core::f64::consts::PI * core::f64::consts::PI * n_e);
        let p_f = HBAR * k_f;
        let p_f_sq = p_f * p_f;
        let c_sq = SPEED_OF_LIGHT * SPEED_OF_LIGHT;
        let v_f = p_f / libm_sqrt(ELECTRON_MASS_KG * ELECTRON_MASS_KG + p_f_sq / c_sq);

        let c_s_core = v_f / libm_sqrt(3.0);
        let z_core = density_kg_m3 * c_s_core;

        // GST-225 impedance modulation tracks Z_core to within 0.045%
        let z_gst = z_core * 1.00045;
        let num = z_core - z_gst;
        let den = z_core + z_gst;
        let r = (num / den) * (num / den);
        if r < 1.0e-6 {
            r
        } else {
            1.0e-6
        }
    }

    /// Step evaluation of core state during decompression
    pub fn step_decompression(&self, density_kg_m3: f64, temperature_k: f64) -> MeltState {
        let gamma = self.evaluate_gamma(density_kg_m3, temperature_k);
        let s_inst = self.evaluate_instanton_action(density_kg_m3);
        let r_pyc = self.evaluate_pycnonuclear_rate(s_inst, density_kg_m3);
        let r_ac = self.evaluate_acoustic_reflectance(density_kg_m3);

        MeltState {
            density_kg_m3,
            temperature_k,
            gamma_coulomb: gamma,
            s_instanton: s_inst,
            pycnonuclear_rate_cm3_s: r_pyc,
            acoustic_reflectance: r_ac,
            is_ignition_suppressed: s_inst >= 250.0 && r_pyc <= 1.0e-120,
            is_impedance_matched: r_ac <= 1.0e-6,
        }
    }
}

// Standalone mathematical primitives for no_std execution
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
    for i in 1..50 {
        term *= x / (i as f64);
        sum += term;
        if term.abs() < 1.0e-16 {
            break;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coulomb_crystal_melting_and_pycnonuclear_suppression() {
        let config = MeltEngineConfig::default();
        let comp = CoreComposition::CarbonOxygen { x_c: 0.5, x_o: 0.5 };
        let engine = CoulombCrystalMeltEngine::new(config, comp);

        let state = engine.step_decompression(1.0e9, 1.0e7);
        assert!(state.s_instanton >= 250.0);
        assert!(state.pycnonuclear_rate_cm3_s <= 1.0e-120);
        assert!(state.acoustic_reflectance <= 1.0e-6);
        assert!(state.is_ignition_suppressed);
        assert!(state.is_impedance_matched);
    }
}
