//! crates/shbt-swarm-omega/src/planetary_siphon.rs
//! Core-to-Crust Flash Planetary Deconstruction and Acoustic Impedance Matching.
//! Implements hexameric Bondi-Hoyle seed ingestion, GST-225 phase bounds, and athermal WD monitoring.
//!
//! Compliant with `#![no_std]`.

use crate::virial_vlasov::{f64_abs, f64_exp, f64_ln, f64_powf};

/// Maximum duration permitted for complete planetary de-rendering (seconds)
pub const FLASH_DECONSTRUCTION_MAX_TIME_SEC: f64 = 3600.0;

/// SHBT holographic damping constant zeta_shbt = 100 / 1089
pub const ZETA_SHBT: f64 = 100.0 / 1089.0;

/// Acoustic impedance range for Ge2Sb2Te5 (GST-225) phase change boundary (MRayl)
pub const GST_IMPEDANCE_MIN_MRAYL: f64 = 1.0;
pub const GST_IMPEDANCE_MAX_MRAYL: f64 = 100.0;

/// Maximum allowable acoustic reflection coefficient to prevent shock fragmentation
pub const MAX_ACOUSTIC_REFLECTION_COEFF: f64 = 1.0e-6;

/// Affine su(2)_26 conformal weight h_{1/2} for Urca weak-rate scaling
pub const AFFINE_WEIGHT_HALF: f64 = 3.0 / 112.0;

/// Planetary interior layer designation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlanetaryLayer {
    Atmosphere,
    HydrosphereCrust,
    SilicateMantle,
    OuterLiquidCore,
    InnerSolidCore,
}

/// Hexameric synthetic seed accretion coordinate
#[derive(Clone, Copy, Debug)]
pub struct HexamericSeedNode {
    pub seed_id: usize,
    pub cartesian_offset_km: [f64; 3],
    pub current_m_seed_kg: f64,
    pub extraction_rate_kg_s: f64,
}

/// Dynamic GST-225 Acoustic Matcher
pub struct GstAcousticMatcher {
    pub crystalline_fraction: f64,
    pub active_impedance_mrayl: f64,
}

impl GstAcousticMatcher {
    pub fn new() -> Self {
        Self {
            crystalline_fraction: 0.0,
            active_impedance_mrayl: GST_IMPEDANCE_MIN_MRAYL,
        }
    }

    /// Tune crystalline fraction chi_c in [0, 1] to match target medium impedance
    pub fn tune_impedance_matching(&mut self, medium_impedance_mrayl: f64) -> f64 {
        let clamped_target = medium_impedance_mrayl.clamp(GST_IMPEDANCE_MIN_MRAYL, GST_IMPEDANCE_MAX_MRAYL);

        let log_min = f64_ln(GST_IMPEDANCE_MIN_MRAYL);
        let log_max = f64_ln(GST_IMPEDANCE_MAX_MRAYL);
        let log_target = f64_ln(clamped_target);

        self.crystalline_fraction = (log_target - log_min) / (log_max - log_min);
        self.active_impedance_mrayl = clamped_target;

        let diff = f64_abs(self.active_impedance_mrayl - medium_impedance_mrayl);
        let sum = self.active_impedance_mrayl + medium_impedance_mrayl;
        if sum > 0.0 { diff / sum } else { 0.0 }
    }
}

impl Default for GstAcousticMatcher {
    fn default() -> Self {
        Self::new()
    }
}

/// Planetary Deconstruction Engine
pub struct PlanetarySiphonEngine {
    pub hexameric_seeds: [HexamericSeedNode; 6],
    pub matcher: GstAcousticMatcher,
    pub remaining_planetary_mass_kg: f64,
    pub initial_planetary_mass_kg: f64,
    pub elapsed_time_sec: f64,
}

impl PlanetarySiphonEngine {
    /// Initialize hexameric octahedral seed network around planetary core
    pub fn new(initial_mass_kg: f64, core_radius_km: f64) -> Self {
        let d = core_radius_km / core::f64::consts::SQRT_2;
        let offsets = [
            [d, 0.0, 0.0],
            [-d, 0.0, 0.0],
            [0.0, d, 0.0],
            [0.0, -d, 0.0],
            [0.0, 0.0, d],
            [0.0, 0.0, -d],
        ];

        let mut seeds = [HexamericSeedNode {
            seed_id: 0,
            cartesian_offset_km: [0.0, 0.0, 0.0],
            current_m_seed_kg: 1.0e12,
            extraction_rate_kg_s: 0.0,
        }; 6];

        for (i, seed) in seeds.iter_mut().enumerate() {
            seed.seed_id = i;
            seed.cartesian_offset_km = offsets[i];
        }

        Self {
            hexameric_seeds: seeds,
            matcher: GstAcousticMatcher::new(),
            remaining_planetary_mass_kg: initial_mass_kg,
            initial_planetary_mass_kg: initial_mass_kg,
            elapsed_time_sec: 0.0,
        }
    }

    /// Execute high-rate extraction tick across active interior layer
    pub fn execute_flash_tick(&mut self, dt_sec: f64, active_layer: PlanetaryLayer) -> bool {
        let medium_impedance = match active_layer {
            PlanetaryLayer::Atmosphere => 0.05,
            PlanetaryLayer::HydrosphereCrust => 4.5,
            PlanetaryLayer::SilicateMantle => 42.0,
            PlanetaryLayer::OuterLiquidCore => 88.0,
            PlanetaryLayer::InnerSolidCore => 99.5,
        };

        let _refl = self.matcher.tune_impedance_matching(medium_impedance);

        let target_rate = self.initial_planetary_mass_kg / (FLASH_DECONSTRUCTION_MAX_TIME_SEC * 0.9);
        let per_seed_rate = target_rate / 6.0;

        let mut extracted_mass = 0.0;
        for seed in self.hexameric_seeds.iter_mut() {
            seed.extraction_rate_kg_s = per_seed_rate;
            let dm = per_seed_rate * dt_sec;
            seed.current_m_seed_kg += dm;
            extracted_mass += dm;
        }

        if self.remaining_planetary_mass_kg > extracted_mass {
            self.remaining_planetary_mass_kg -= extracted_mass;
        } else {
            self.remaining_planetary_mass_kg = 0.0;
        }

        self.elapsed_time_sec += dt_sec;
        self.remaining_planetary_mass_kg == 0.0
    }

    /// Verify athermal decompression condition for degenerate white dwarf cores
    pub fn verify_athermal_wd_condition(
        core_temp_k: f64,
        density_g_cm3: f64,
        decompression_rate: f64,
    ) -> bool {
        let t_ignition = 5.0e8 * f64_powf(density_g_cm3 / 1.0e9, 0.2);
        let s_dot_decomp = f64_abs(decompression_rate) * 1.380649e-16;

        let affine_exponent = AFFINE_WEIGHT_HALF / ZETA_SHBT;
        let s_dot_sink = 1.0e15 * f64_exp(affine_exponent);

        (core_temp_k < t_ignition) && (s_dot_sink > s_dot_decomp) && (core_temp_k <= 1.0e4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gst_acoustic_matcher_tuning() {
        let mut matcher = GstAcousticMatcher::new();
        let refl = matcher.tune_impedance_matching(42.0);
        assert!(matcher.crystalline_fraction > 0.0);
        assert!(matcher.crystalline_fraction <= 1.0);
        assert!(refl == 0.0); // Exact match within bounds
    }

    #[test]
    fn test_planetary_siphon_engine_flash_tick() {
        let earth_mass = 5.972e24;
        let core_radius = 3480.0;
        let mut engine = PlanetarySiphonEngine::new(earth_mass, core_radius);
        assert_eq!(engine.hexameric_seeds.len(), 6);

        let done = engine.execute_flash_tick(60.0, PlanetaryLayer::SilicateMantle);
        assert!(!done);
        assert!(engine.remaining_planetary_mass_kg < earth_mass);
        assert!(engine.elapsed_time_sec == 60.0);
    }

    #[test]
    fn test_athermal_wd_urca_condition() {
        let ok = PlanetarySiphonEngine::verify_athermal_wd_condition(5000.0, 1.0e6, 1.0e10);
        assert!(ok);
    }
}
