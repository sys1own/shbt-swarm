#![no_std]
#![forbid(unsafe_code)]
#![allow(unused_parens)]
#![allow(clippy::manual_range_contains)]

//! Phase 5A `shbt-swarm-omega`: Kardashev Type III Galactic Synthesis & Megastructure Core
//! Strictly grounded in C_SHBT = Fib (x) C_GKO(26, 8, 312), eta_A = 10/33, eta_D = 23/33.

pub mod blandford_znajek_null;
pub use blandford_znajek_null::*;
pub mod stellar_deconstruct;
pub mod dynamo_brake;
pub mod smbh_siphon;
pub mod halo_clamp;
pub mod cross_lapse_bus;
pub mod virial_vlasov;
pub mod planetary_siphon;
pub mod elysium_metric;
pub mod gw_recoil_clamp;
pub use gw_recoil_clamp::{
    Ccz4GundlachConfig, GwRecoilClampEngine, SymTensor3, Vector3, CANONICAL_K_HUBS,
    DARK_LEDGER_ETA_D, G_NEWTON, MEMORY_NULL_TOLERANCE, QUADRANT_HUBS, SECULAR_COM_TOLERANCE,
    SOLAR_MASS,
};
pub mod magnetar_quench;
pub use magnetar_quench::*;
pub mod gw_recoil;
pub mod angular_momentum;
pub use angular_momentum::*;
pub mod red_dwarf_dynamo;
pub use red_dwarf_dynamo::*;
pub mod pair_instability;
pub use pair_instability::*;
pub mod nsc_loss_cone;
pub use nsc_loss_cone::{NscLossConeShield, NscShieldConfig};
pub mod universal_harvest;
pub use universal_harvest::*;
pub mod coulomb_melt;
pub use coulomb_melt::*;
pub mod pasta_decompress;
pub use pasta_decompress::*;
pub mod cross_lapse_airgap;
pub use cross_lapse_airgap::*;
pub mod dark_zpe_extractor;
pub use dark_zpe_extractor::*;



pub use stellar_deconstruct::{EddingtonBypassEngine, DeconstructionError};
pub use dynamo_brake::{DynkinBrakingRouter, DynamoError};
pub use smbh_siphon::{
    BoyerLindquistCoord, HexamericHarvesterNode, HexamericSiphonSystem, KerrBlackHole,
    SiphonError, SmbhSiphonController,
};
pub use halo_clamp::{HernquistDMHalo, GalacticVirialClampEngine, VirialClampError};
pub use cross_lapse_bus::*;
pub use virial_vlasov::*;
pub use planetary_siphon::*;
pub use elysium_metric::{
    compute_coriolis_acceleration, evaluate_boundary_junction_stress, evaluate_christoffel_symbols,
    evaluate_electric_weyl, evaluate_metric, evaluate_shift_curl, lapse_function, shift_vector,
    ElysiumPointMetric, G_TARGET, I_00_COUNTER_TERM,
};

/// Architectural constants for Phase 5 SHBT-OMEGA-312
pub const MERA_BASE_LAYER: u8 = 94;
pub const MERA_GAL_MIN_LAYER: u8 = 102;
pub const MERA_GAL_MAX_LAYER: u8 = 234;
pub const STINESPRING_NUM_D: u64 = 23;
pub const STINESPRING_DENOM: u64 = 33;
pub const MAX_CORE_TEMP_MILLI_K: u32 = 28_400; // 28.400 K symplectic shunt ceiling
pub const STD_GRAVITY_UM_S2: u64 = 9_806_650;  // 9.80665 m/s^2 (1.000g)
pub const PT_GOI_SOLVE_FEMTO_S: u64 = 54_432;  // 54.432 ps (54,432 fs)
pub const FOUNDRY_GAIN_Q: u64 = 119_094;        // Mode 1c isobaric recycling gain Q_foundry
pub const CONTINUOUS_POWER_YW: f64 = 25.00;     // 25.00 YW continuous output
pub const DARK_RESERVE_JOULES: f64 = 1.8e56;    // 1.8 * 10^56 J from 1% galactic mass
pub const DARK_GRID_POWER_WATTS: f64 = 2.5e25;  // 25.00 YW
pub const OPEN_LOOP_DEPLETION_YEARS: f64 = 2.2817e23;
pub const POST_PINCH_CLOSED_LIFETIME_YEARS: f64 = 1.0e100;
pub const SYMPLECTIC_LANDAUER_QUOTA_BASE_J_PER_BIT: f64 = 4.019370e-23;
pub const SYMPLECTIC_LANDAUER_QUOTA_SUM_J_PER_BIT: f64 = 8.038740e-23;
pub const STARGATE_SPAN_LY: f64 = 105_700.0;    // Mode A SHBT-EW-312 diameter

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OmegaError {
    MeraLayerOutOfBounds,
    IsometricErrorExceeded,
    CoreTemperatureTooHigh,
    TidalShearNonZero,
    CbfSafetyViolation,
    PseudoAnosovEntropyNonZero,
    LapseOutOfSovereigntyBounds,
}

/// Represents a Galactic MERA-DAG Layer 102..234 hierarchical addressing node.
/// L(N) = 94 + ceil(log2(N / 10^28)) up to Layer 234 (N = 10^70 nucleons).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct GalacticMeraAddress {
    pub log2_nucleon_ratio_ceil: u8,
    pub layer_index: u8,
    pub sector_id: u16,
    pub isometric_error_scaled: u64, // Must be <= 1 (representing 10^-120 ceiling)
    pub steric_volume_m3: u64,       // Invariant: must equal 0 in dark ledger (eta_D = 23/33)
}

impl GalacticMeraAddress {
    /// Computes exact MERA layer L(N) = 94 + ceil(log2(N / 10^28)) up to Layer 234 (N = 10^70)
    pub const fn from_log2_ratio(
        log2_n_over_10e28_ceil: u8,
        sector_id: u16,
        isometric_error_scaled: u64,
    ) -> Result<Self, OmegaError> {
        let layer = MERA_BASE_LAYER.saturating_add(log2_n_over_10e28_ceil);
        if layer < MERA_GAL_MIN_LAYER || layer > MERA_GAL_MAX_LAYER || sector_id >= 312 {
            return Err(OmegaError::MeraLayerOutOfBounds);
        }
        if isometric_error_scaled > 1 {
            return Err(OmegaError::IsometricErrorExceeded);
        }
        Ok(Self {
            log2_nucleon_ratio_ceil: log2_n_over_10e28_ceil,
            layer_index: layer,
            sector_id,
            isometric_error_scaled,
            steric_volume_m3: 0,
        })
    }
}

/// Hexameric Bondi-Hoyle Stellar Deconstruction Scheduler.
/// Dismantles stars, gas giants, and stellar clusters into the SHBT-IBR-OMEGA Dark-Braid Mass-Energy Ledger.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct HexamericStellarDeconstructor {
    pub seed_masses_milli_solar: [u64; 6], // Each in [1 (10^-3 M_sun), 100_000 (10^2 M_sun)]
    pub su2_spin_triplet_locked: bool,     // Quenches p-p weak fusion cross-section
    pub stream_temp_milli_k: u32,          // Must be <= 28,400 mK (28.40 K)
    pub cross_section_gain_log10: u8,      // Must be >= 10 (sigma_grav / A_geom >= 10^10)
}

impl HexamericStellarDeconstructor {
    pub const fn verify_laminar_harvest(&self) -> Result<u64, OmegaError> {
        if !self.su2_spin_triplet_locked || self.stream_temp_milli_k > MAX_CORE_TEMP_MILLI_K {
            return Err(OmegaError::CoreTemperatureTooHigh);
        }
        if self.cross_section_gain_log10 < 10 {
            return Err(OmegaError::CbfSafetyViolation);
        }
        let mut total_seed_milli_solar: u64 = 0;
        let mut i = 0;
        while i < 6 {
            let m = self.seed_masses_milli_solar[i];
            if m < 1 || m > 100_000 {
                return Err(OmegaError::CbfSafetyViolation);
            }
            total_seed_milli_solar = total_seed_milli_solar.saturating_add(m);
            i += 1;
        }
        Ok(total_seed_milli_solar)
    }
}

/// SHBT-ELYSIUM Pocket Manifold & Wish-to-Reality Pipeline Controller.
/// Mode A SHBT-EW-312 Stargate-linked pocket worlds across 105,700 ly at Delta tau == 0,
/// 1.000g Coriolis-free gravity, and zero-shear Eikonal framing (Delta_fr == 0 <=> E_uv == 0).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ElysiumPocketManifold {
    pub pocket_id: u64,
    pub gravity_um_s2: u64,
    pub lapse_milli: u64,          // [1 (alpha=10^-4 scaled) .. 100_000_000 (alpha=10^4 scaled)]
    pub weyl_shear_norm: u64,      // Invariant: E_uv == 0
    pub coriolis_norm: u64,        // Invariant: omega_Cor == 0
    pub stargate_ent_phi: u64,     // Invariant: Ent(phi) == 0
    pub bio_backup_fidelity_ok: bool,
}

impl ElysiumPocketManifold {
    pub const fn synthesize_wish_to_reality(
        &self,
        cbf_barrier_margin_positive: bool,
        post_sim_isomorphism_delta: u64,
    ) -> Result<u64, OmegaError> {
        if self.weyl_shear_norm != 0 || self.coriolis_norm != 0 {
            return Err(OmegaError::TidalShearNonZero);
        }
        if self.stargate_ent_phi != 0 {
            return Err(OmegaError::PseudoAnosovEntropyNonZero);
        }
        if self.lapse_milli < 1 || self.lapse_milli > 100_000_000 {
            return Err(OmegaError::LapseOutOfSovereigntyBounds);
        }
        if !cbf_barrier_margin_positive || post_sim_isomorphism_delta != 0 || !self.bio_backup_fidelity_ok {
            return Err(OmegaError::CbfSafetyViolation);
        }
        Ok(PT_GOI_SOLVE_FEMTO_S)
    }
}

/// Perpetual Galactic Power Grid Model.
/// 1% galactic mass (1.989 * 10^39 kg) in Purcell-inhibited SHBT-MAR-312X cores (1.8 * 10^56 J)
/// with Mode 1c isobaric dark-braid recycling (Q_foundry = 119,094), yielding 25.00 YW continuous
/// output over > 10^100 years.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PerpetualDarkPowerGrid {
    pub fraction_galactic_mass: f64,
    pub stored_energy_joules: f64,
    pub foundry_recycling_q: u64,
    pub continuous_output_yw: f64,
    pub purcell_inhibition_active: bool,
    pub zpe_extractor: DarkZpeExtractor,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DarkPowerGridRegime {
    RegimeIOpenLoopPrePinch,
    RegimeIIPostPinchKmsClosed,
    RegimeIIIAutopoieticReplenishment,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DarkPowerGridRegimeMetrics {
    pub regime: DarkPowerGridRegime,
    pub p_grid_watts: f64,
    pub d_energy_dt_watts: f64,
    pub foundry_gain_q: f64,
    pub depletion_lifetime_years: f64,
    pub dissipation_j_per_bit: f64,
}

impl PerpetualDarkPowerGrid {
    pub const fn canonical() -> Self {
        Self {
            fraction_galactic_mass: 0.01,
            stored_energy_joules: DARK_RESERVE_JOULES,
            foundry_recycling_q: FOUNDRY_GAIN_Q,
            continuous_output_yw: CONTINUOUS_POWER_YW,
            purcell_inhibition_active: true,
            zpe_extractor: DarkZpeExtractor::new(),
        }
    }

    pub fn compute_burn_lifetime_years(&self) -> f64 {
        // Regime I open-loop: finite reserve / 25 YW with Q_foundry recycling.
        // Regime II post-pinch: KMS reactive circulation has dE/dt == 0 and Q_foundry -> infinity.
        self.open_loop_regime_metrics().depletion_lifetime_years.max(POST_PINCH_CLOSED_LIFETIME_YEARS)
    }

    #[inline]
    pub fn open_loop_regime_metrics(&self) -> DarkPowerGridRegimeMetrics {
        DarkPowerGridRegimeMetrics {
            regime: DarkPowerGridRegime::RegimeIOpenLoopPrePinch,
            p_grid_watts: self.continuous_output_yw * 1.0e24,
            d_energy_dt_watts: -self.continuous_output_yw * 1.0e24,
            foundry_gain_q: self.foundry_recycling_q as f64,
            depletion_lifetime_years: OPEN_LOOP_DEPLETION_YEARS,
            dissipation_j_per_bit: SYMPLECTIC_LANDAUER_QUOTA_SUM_J_PER_BIT,
        }
    }

    #[inline]
    pub fn post_pinch_kms_metrics(&self) -> DarkPowerGridRegimeMetrics {
        DarkPowerGridRegimeMetrics {
            regime: DarkPowerGridRegime::RegimeIIPostPinchKmsClosed,
            p_grid_watts: 0.0,
            d_energy_dt_watts: 0.0,
            foundry_gain_q: f64::INFINITY,
            depletion_lifetime_years: POST_PINCH_CLOSED_LIFETIME_YEARS,
            dissipation_j_per_bit: 0.0,
        }
    }

    /// Regime III autopoietic replenishment metrics from `Dark-ZPE` Channels 1-3.
    #[inline]
    pub fn regime_three_autopoietic_metrics(&self) -> DarkPowerGridRegimeMetrics {
        DarkPowerGridRegimeMetrics {
            regime: DarkPowerGridRegime::RegimeIIIAutopoieticReplenishment,
            p_grid_watts: self.zpe_extractor.net_power_w(),
            d_energy_dt_watts: self.zpe_extractor.net_power_w(),
            foundry_gain_q: f64::INFINITY,
            depletion_lifetime_years: POST_PINCH_CLOSED_LIFETIME_YEARS,
            dissipation_j_per_bit: 0.0,
        }
    }

    /// Runs one Dark-ZPE symplectic exergy governor cycle and returns Regime III lock.
    #[inline]
    pub fn step_dark_zpe_governor_cycle(&mut self) -> bool {
        self.zpe_extractor.step_governor_cycle()
    }
}

#[inline]
pub fn symplectic_landauer_quota_j_per_bit(k: u32) -> f64 {
    if k >= 1024 {
        return 0.0;
    }
    let mut value = SYMPLECTIC_LANDAUER_QUOTA_BASE_J_PER_BIT;
    for _ in 0..k {
        value *= 0.5;
    }
    value
}

#[inline]
pub fn actual_post_pinch_dark_dissipation_j_per_bit(_k: u32) -> f64 {
    0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mera_dag_scaling() {
        // Layer 102 (N = 10^30)
        let addr102 = GalacticMeraAddress::from_log2_ratio(8, 0, 1).unwrap();
        assert_eq!(addr102.layer_index, 102);
        assert_eq!(addr102.steric_volume_m3, 0);

        // Layer 234 (N = 10^70, log2_ratio = 140)
        let addr234 = GalacticMeraAddress::from_log2_ratio(140, 311, 1).unwrap();
        assert_eq!(addr234.layer_index, 234);

        // Out of bounds
        assert_eq!(
            GalacticMeraAddress::from_log2_ratio(7, 0, 1),
            Err(OmegaError::MeraLayerOutOfBounds)
        );
        assert_eq!(
            GalacticMeraAddress::from_log2_ratio(141, 0, 1),
            Err(OmegaError::MeraLayerOutOfBounds)
        );
        assert_eq!(
            GalacticMeraAddress::from_log2_ratio(10, 312, 1),
            Err(OmegaError::MeraLayerOutOfBounds)
        );
        assert_eq!(
            GalacticMeraAddress::from_log2_ratio(10, 0, 2),
            Err(OmegaError::IsometricErrorExceeded)
        );
    }

    #[test]
    fn test_hexameric_stellar_deconstructor() {
        let dec = HexamericStellarDeconstructor {
            seed_masses_milli_solar: [1000, 1000, 1000, 1000, 1000, 1000],
            su2_spin_triplet_locked: true,
            stream_temp_milli_k: 24_850,
            cross_section_gain_log10: 10,
        };
        let tot = dec.verify_laminar_harvest().unwrap();
        assert_eq!(tot, 6000);
    }

    #[test]
    fn test_elysium_pocket_manifold() {
        let hab = ElysiumPocketManifold {
            pocket_id: 1,
            gravity_um_s2: STD_GRAVITY_UM_S2,
            lapse_milli: 1000,
            weyl_shear_norm: 0,
            coriolis_norm: 0,
            stargate_ent_phi: 0,
            bio_backup_fidelity_ok: true,
        };
        let solve = hab.synthesize_wish_to_reality(true, 0).unwrap();
        assert_eq!(solve, PT_GOI_SOLVE_FEMTO_S);
    }

    #[test]
    fn test_perpetual_dark_power_grid() {
        let grid = PerpetualDarkPowerGrid::canonical();
        assert_eq!(grid.fraction_galactic_mass, 0.01);
        assert!(grid.compute_burn_lifetime_years() >= 1.0e100);
    }

    #[test]
    fn test_smbh_siphon_engine_and_area_non_decrease() {
        let mass = 4.154e6 * 1.98847e30; // Sgr A* mass in kg (~4.154e6 M_sun)
        let spin = 0.90;
        let mut siphon = HexamericSiphonSystem::new(mass, spin).unwrap();

        let a_init = siphon.black_hole.horizon_area();
        let m_irr_init = siphon.black_hole.irreducible_mass();
        let max_extract = siphon.black_hole.max_extractable_rotational_energy();
        assert!(max_extract > 0.0);
        assert!(max_extract <= mass * (1.0 - 1.0 / core::f64::consts::SQRT_2) * 299_792_458.0 * 299_792_458.0);

        // Infinitesimal infalling negative energy and retrograde angular momentum
        let dm = -100.0; // kg
        let omega_h = siphon.black_hole.horizon_angular_velocity();
        let dj = (dm * 299_792_458.0 * 299_792_458.0 / omega_h) * 1.05; // effective_flux > 0

        let harvested = siphon.execute_siphon_step(dm, dj).unwrap();
        assert!(harvested > 0.0);
        assert!(siphon.black_hole.horizon_area() + 1e-12 * a_init >= a_init);
        assert!(siphon.black_hole.irreducible_mass() + 1e-12 * m_irr_init >= m_irr_init);
    }

    #[test]
    fn test_galactic_virial_clamp_engine() {
        let halo_mass = 1.0e12 * 1.98847e30; // 1e12 M_sun in kg
        let halo_scale = 15.0 * 3.085677581e19; // 15 kpc
        let baryon_mass = 1.0e9 * 1.98847e30; // 1e9 M_sun in kg
        let baryon_scale = 3.0 * 3.085677581e19; // 3 kpc

        let mut clamp_engine = GalacticVirialClampEngine::new(halo_mass, halo_scale, baryon_mass, baryon_scale).unwrap();

        // Harvest 10% of baryonic mass into dark ledger
        let harvest_fraction = clamp_engine.harvest_and_clamp(0.10 * baryon_mass).unwrap();
        assert!(shbt_swarm_core::shbt_abs!((harvest_fraction - 0.10)) < 1e-9);
    }
}
