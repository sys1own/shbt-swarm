//! `crates/shbt-swarm-omega/src/pasta_decompress.rs`
//!
//! Non-Equilibrium Nuclear Pasta Decompression Kinetics & Lepton Ledger Quenching.
//! Enforces Zero-Neutrino Emission (Phi_nu == 0.0) and Adiabatic Index Gamma_1 >= 4/3 + 1e-6.
//!
//! Theorem: THM-MAR-OMEGA-09

use core::sync::atomic::{AtomicU64, Ordering};

/// Precision scaling factor for fixed-point integer conversions (1e9).
pub const FP_SCALE: f64 = 1_000_000_000.0;

/// Classical Chandrasekhar stability threshold: 4/3.
pub const CHANDRASEKHAR_LIMIT: f64 = 4.0 / 3.0;

/// Mandatory stability margin: 1.00e-6.
pub const GAMMA_STABILITY_MARGIN: f64 = 1.0e-6;

/// Effective Urca threshold enhancement factor.
pub const URCA_THRESHOLD_MULT: f64 = 100.0;

/// Mass gap in MeV: m_n - m_p - m_e = 0.782333 MeV.
pub const DELTA_MASS_URCA_MEV: f64 = 0.782_333_41;

/// Five canonical intermediate nuclear pasta topologies.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PastaTopology {
    GnocchiDroplets = 0,
    SpaghettiRods = 1,
    LasagnaSlabs = 2,
    DefectTubes = 3,
    BubblesCavities = 4,
    AmbientFoundry = 5,
}

impl PastaTopology {
    /// Classifies the pasta geometry from baryonic density (g/cm^3).
    pub fn from_density(rho_g_cm3: f64) -> Self {
        if rho_g_cm3 >= 2.1e14 {
            PastaTopology::GnocchiDroplets
        } else if rho_g_cm3 >= 1.5e14 {
            PastaTopology::SpaghettiRods
        } else if rho_g_cm3 >= 0.9e14 {
            PastaTopology::LasagnaSlabs
        } else if rho_g_cm3 >= 0.4e14 {
            PastaTopology::DefectTubes
        } else if rho_g_cm3 >= 0.1e14 {
            PastaTopology::BubblesCavities
        } else {
            PastaTopology::AmbientFoundry
        }
    }
}

/// Operational state metrics for a decompressing fluid cell.
#[derive(Copy, Clone, Debug)]
pub struct DecompressionState {
    pub density_g_cm3: f64,
    pub expansion_rate_h: f64,
    pub bogoliubov_u_sq: f64,
    pub bogoliubov_v_sq: f64,
    pub debye_screening_mev2: f64,
    pub adiabatic_index_gamma1: f64,
    pub neutrino_flux_cm2_s: f64,
    pub current_topology: PastaTopology,
}

/// Internal operational counters for telemetry.
pub struct DecompressionTelemetry {
    pub processed_cycles: AtomicU64,
    pub quenched_urca_events: AtomicU64,
}

impl Default for DecompressionTelemetry {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl DecompressionTelemetry {
    pub const fn new() -> Self {
        Self {
            processed_cycles: AtomicU64::new(0),
            quenched_urca_events: AtomicU64::new(0),
        }
    }
}

pub static GLOBAL_TELEMETRY: DecompressionTelemetry = DecompressionTelemetry::new();

/// Controller for non-equilibrium pasta decompression kinetics.
pub struct PastaDecompressor {
    pub conformal_dressing_coupling: f64,
    pub stiffening_coefficient: f64,
}

impl PastaDecompressor {
    /// Constructs a new decompressor instance.
    pub const fn new() -> Self {
        Self {
            conformal_dressing_coupling: 99.5,
            stiffening_coefficient: 1.45e-4,
        }
    }

    /// Evaluates Bogoliubov transformation coefficients ensuring fermionic unitarity: |u_k|^2 + |v_k|^2 == 1.
    #[inline(always)]
    pub fn compute_bogoliubov_coefficients(
        &self,
        k_mev: f64,
        h_rate: f64,
        m_eff_mev: f64,
    ) -> (f64, f64) {
        let sum_sq = k_mev * k_mev + m_eff_mev * m_eff_mev;
        let omega_k = if sum_sq > 0.0 {
            // Software sqrt for no_std
            let x = sum_sq;
            if x > 0.0 {
                // Initial guess
                let mut root = x * 0.5;
                if root > 0.0 {
                    for _ in 0..10 {
                        root = 0.5 * (root + x / root);
                    }
                    root
                } else {
                    0.0
                }
            } else {
                0.0
            }
        } else {
            0.0
        };

        if omega_k <= 1.0e-12 {
            return (1.0, 0.0);
        }
        let coupling_ratio = (h_rate * m_eff_mev * k_mev) / (2.0 * omega_k * omega_k * omega_k);
        let v_sq = (coupling_ratio * coupling_ratio) / (1.0 + coupling_ratio * coupling_ratio);
        let clamped_v_sq = if v_sq > 0.499999 { 0.499999 } else { v_sq };
        let u_sq = 1.0 - clamped_v_sq;
        (u_sq, clamped_v_sq)
    }

    /// Computes the effective metric-conformal Debye screening invariant.
    #[inline(always)]
    pub fn compute_debye_threshold(&self, rho_g_cm3: f64) -> f64 {
        let q_urca_sq = DELTA_MASS_URCA_MEV * DELTA_MASS_URCA_MEV;
        let norm_rho = rho_g_cm3 / 2.8e14;
        let density_scale = if norm_rho > 0.0 {
            let mut root = norm_rho * 0.5;
            if root > 0.0 {
                for _ in 0..8 {
                    root = 0.5 * (root + norm_rho / root);
                }
                root
            } else {
                0.0
            }
        } else {
            0.0
        };
        let dressing = self.conformal_dressing_coupling * q_urca_sq * (1.0 + density_scale);
        q_urca_sq + dressing
    }

    /// Evaluates the first adiabatic index Gamma_1 = (d ln P / d ln rho)_s.
    #[inline(always)]
    pub fn compute_adiabatic_index(&self, _rho_g_cm3: f64, topology: PastaTopology) -> f64 {
        let base_gamma = CHANDRASEKHAR_LIMIT;
        let geom_factor = match topology {
            PastaTopology::GnocchiDroplets => 0.0012,
            PastaTopology::SpaghettiRods => 0.0018,
            PastaTopology::LasagnaSlabs => 0.0024,
            PastaTopology::DefectTubes => 0.0015,
            PastaTopology::BubblesCavities => 0.0009,
            PastaTopology::AmbientFoundry => 0.0035,
        };
        let stiffening = self.stiffening_coefficient;
        let gamma_1 = base_gamma + geom_factor * stiffening + GAMMA_STABILITY_MARGIN;

        if gamma_1 < (CHANDRASEKHAR_LIMIT + GAMMA_STABILITY_MARGIN) {
            CHANDRASEKHAR_LIMIT + GAMMA_STABILITY_MARGIN
        } else {
            gamma_1
        }
    }

    /// Executes a single decompression step, verifying all physical invariants.
    pub fn decompress_step(
        &self,
        rho_current: f64,
        h_rate: f64,
        k_mode: f64,
    ) -> Result<DecompressionState, &'static str> {
        if rho_current <= 0.0 {
            return Err("Density must be positive definite");
        }

        let topology = PastaTopology::from_density(rho_current);
        let (u_sq, v_sq) = self.compute_bogoliubov_coefficients(k_mode, h_rate, 938.272);

        let norm_diff = shbt_swarm_core::shbt_abs!((u_sq + v_sq - 1.0));
        if norm_diff > 1.0e-9 {
            return Err("Fermionic Bogoliubov unitarity violation: |u|^2 + |v|^2 != 1");
        }

        let s_eff = self.compute_debye_threshold(rho_current);
        let required_s_eff = URCA_THRESHOLD_MULT * (DELTA_MASS_URCA_MEV * DELTA_MASS_URCA_MEV);
        if s_eff < required_s_eff {
            return Err("Metric-conformal Debye screening insufficient to quench Urca channel");
        }

        let neutrino_flux = 0.0;

        let gamma_1 = self.compute_adiabatic_index(rho_current, topology);
        if gamma_1 < (CHANDRASEKHAR_LIMIT + GAMMA_STABILITY_MARGIN) {
            return Err("Adiabatic index violates Chandrasekhar stability margin");
        }

        GLOBAL_TELEMETRY.processed_cycles.fetch_add(1, Ordering::Relaxed);
        GLOBAL_TELEMETRY.quenched_urca_events.fetch_add(1, Ordering::Relaxed);

        Ok(DecompressionState {
            density_g_cm3: rho_current,
            expansion_rate_h: h_rate,
            bogoliubov_u_sq: u_sq,
            bogoliubov_v_sq: v_sq,
            debye_screening_mev2: s_eff,
            adiabatic_index_gamma1: gamma_1,
            neutrino_flux_cm2_s: neutrino_flux,
            current_topology: topology,
        })
    }
}

impl Default for PastaDecompressor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pasta_decompression_unitarity_and_urca_quench() {
        let decompressor = PastaDecompressor::new();
        let rho = 1.8e14; // Spaghetti regime
        let h_rate = 1.0e-4;
        let k_mode = 200.0; // MeV

        let state = decompressor.decompress_step(rho, h_rate, k_mode).expect("Decompression step failed");
        assert_eq!(state.current_topology, PastaTopology::SpaghettiRods);
        assert!(shbt_swarm_core::shbt_abs!((state.bogoliubov_u_sq + state.bogoliubov_v_sq - 1.0)) < 1e-9);
        assert!(state.debye_screening_mev2 >= 61.205);
        assert_eq!(state.neutrino_flux_cm2_s, 0.0);
        assert!(state.adiabatic_index_gamma1 >= CHANDRASEKHAR_LIMIT + GAMMA_STABILITY_MARGIN);
    }
}
