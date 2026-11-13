//! `crates/shbt-swarm-sim/src/galactic_twin.rs`
//!
//! Phase 5 Kardashev Type III Galactic Synthesis Digital Twin (`SHBT-OMEGA-312` & `SHBT-LOGOS-312`).
//! Establishes Galactic Synthesis as the primary default operational mode of the swarm platform,
//! integrating Bondi-Hoyle stellar deconstruction, Layer-234 MERA-DAG addressing, Elysium stargates,
//! and the 25 YW perpetual power grid.

use shbt_swarm_omega::*;
use shbt_swarm_aegis::*;
use shbt_swarm_logos::*;

/// Operational mode of the SHBT Swarm platform.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SwarmOperationalMode {
    /// Legacy local planetary terraforming mode (Phase 3/4).
    LegacyLocalTerraforming,
    /// Primary default operational mode: Kardashev Type III Galactic Synthesis & Hyper-Compute (Phase 5).
    GalacticSynthesis,
}

impl Default for SwarmOperationalMode {
    #[inline]
    fn default() -> Self {
        SwarmOperationalMode::GalacticSynthesis
    }
}

/// Galactic synthesis simulation state and telemetry.
#[derive(Clone, Debug, PartialEq)]
pub struct GalacticSynthesisSimulation {
    pub mode: SwarmOperationalMode,
    pub power_grid: PerpetualDarkPowerGrid,
    pub deconstructor: HexamericStellarDeconstructor,
    pub elysium: ElysiumPocketManifold,
    pub aegis: AegisOmegaController,
    pub logos_core: LogosCoreController,
    pub omnibus: OmnibusFabricController,
    pub vacuum_pinning_nodes_transient: u32,
    pub causal_horizon_isolated: bool,
}

impl Default for GalacticSynthesisSimulation {
    fn default() -> Self {
        Self::canonical()
    }
}

impl GalacticSynthesisSimulation {
    /// Creates the canonical galactic synthesis digital twin.
    pub fn canonical() -> Self {
        Self {
            mode: SwarmOperationalMode::GalacticSynthesis,
            power_grid: PerpetualDarkPowerGrid::canonical(),
            deconstructor: HexamericStellarDeconstructor {
                seed_masses_milli_solar: [1000, 1000, 1000, 1000, 1000, 1000],
                su2_spin_triplet_locked: true,
                stream_temp_milli_k: 24_850,
                cross_section_gain_log10: 10,
            },
            elysium: ElysiumPocketManifold {
                pocket_id: 1,
                gravity_um_s2: STD_GRAVITY_UM_S2,
                lapse_milli: 1000,
                weyl_shear_norm: 0,
                coriolis_norm: 0,
                stargate_ent_phi: 0,
                bio_backup_fidelity_ok: true,
            },
            aegis: AegisOmegaController::canonical(),
            logos_core: LogosCoreController::canonical(),
            omnibus: OmnibusFabricController::canonical(),
            vacuum_pinning_nodes_transient: 312,
            causal_horizon_isolated: false,
        }
    }

    /// Verifies all galactic synthesis operational invariants.
    pub fn verify_galactic_invariants(&self) -> Result<(), &'static str> {
        if self.mode != SwarmOperationalMode::GalacticSynthesis {
            return Err("Swarm not operating in primary Galactic Synthesis mode");
        }
        if self.deconstructor.verify_laminar_harvest().is_err() {
            return Err("Hexameric Bondi-Hoyle deconstruction violated laminar harvest bounds");
        }
        if self.elysium.synthesize_wish_to_reality(true, 0).is_err() {
            return Err("Elysium pocket manifold failed wish-to-reality synthesis verification");
        }
        if self.logos_core.verify_monolithic_core().is_err() {
            return Err("Logos computronium core failed monolithic verification");
        }
        if self.power_grid.compute_burn_lifetime_years() < 1.0e100 {
            return Err("Power grid burn lifetime below 10^100 years perpetual bound");
        }
        if self.causal_horizon_isolated {
            if !self.aegis.layer6_cobordism_pinch.cobordism_pinch_executed
                || self.logos_core.dark_holonomic.logical_bit_ops_per_sec
                    != shbt_swarm_logos::OPS_MARGOLUS_LEVITIN
                || self.vacuum_pinning_nodes_transient != 0
            {
                return Err("Phase 5C causal horizon isolation invariants violated");
            }
        }
        Ok(())
    }

    /// Executes the Phase 5A->5C transition: transient vacuum pinning nodes fold into
    /// pure-dark holonomic computronium and the exterior horizon is pinched off.
    pub fn execute_phase5c_causal_horizon_isolation(
        &mut self,
        harvest_mass_kg: f64,
        harvest_window_sec: f64,
    ) -> Result<(), &'static str> {
        self.aegis
            .execute_harvest_to_isolation_transition(harvest_mass_kg, harvest_window_sec)
            .map_err(|_| "AEGIS-ABSOLUTE harvest-to-isolation transition failed")?;
        self.logos_core
            .rotate_to_pure_dark_holonomic_execution()
            .map_err(|_| "Pure dark holonomic execution rotation failed")?;
        let mut fuse = shbt_swarm_omega::CrossLapseAirgapFuse::new_armed();
        fuse.audit_and_clamp_junction(1.0e-130, 1.0e-16, 1.0e-15)
            .map_err(|_| "Cross-lapse airgap fuse audit failed")?;
        self.vacuum_pinning_nodes_transient = 0;
        self.causal_horizon_isolated = true;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_mode_is_galactic_synthesis() {
        assert_eq!(SwarmOperationalMode::default(), SwarmOperationalMode::GalacticSynthesis);
        let sim = GalacticSynthesisSimulation::default();
        assert_eq!(sim.mode, SwarmOperationalMode::GalacticSynthesis);
        assert!(sim.verify_galactic_invariants().is_ok());
    }

    #[test]
    fn test_phase5c_transition_folds_vacuum_nodes() {
        let mut sim = GalacticSynthesisSimulation::canonical();
        assert_eq!(sim.vacuum_pinning_nodes_transient, 312);
        assert!(sim
            .execute_phase5c_causal_horizon_isolation(
                1.988_409_870_698_051e39,
                18.26 * 86_400.0,
            )
            .is_ok());
        assert!(sim.causal_horizon_isolated);
        assert_eq!(sim.vacuum_pinning_nodes_transient, 0);
        assert!(sim.verify_galactic_invariants().is_ok());
    }
}
