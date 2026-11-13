#![no_std]
#![forbid(unsafe_code)]
#![allow(unused_parens)]

//! Phase 5A/5C `shbt-swarm-aegis`: 6-Layer Kardashev Type III Perimeter Defense, Bipartite Cobordism & One-Way Dehn Ingress
//! Strictly grounded in C_SHBT = Fib (x) C_GKO(26, 8, 312), eta_A = 10/33, eta_D = 23/33.

pub mod vacuum_pinning;
pub mod cobordism_pinch;
pub mod dehn_ingress_airlock;
pub use cobordism_pinch::{
    CobordismPinchController, MAX_GW_MEMORY_RESIDUAL_NORM, SHBT_AEGIS_ADM_MONOPOLE_NULL_LOCK,
};
pub use vacuum_pinning::{
    VacuumPinningState, MAX_FRAMING_DEFECT_TOL, MAX_HARVEST_EXPOSURE_SEC_PER_MSUN,
    MIN_CDL_INSTANTON_ACTION_SE, SHBT_AEGIS_VACUUM_DECAY_LOCK,
};
pub use dehn_ingress_airlock::{
    BipartiteCobordismManifold, NilpotentDehnIngressAirlock, BIPARTITE_POCKET_COUNT,
    BIPARTITE_INTERIOR_GENERA_COUNT, DEHN_INGRESS_VALIDATION_LATENCY_AS,
};

pub const BYZANTINE_LOCKOUT_FEMTO_S: u64 = 1_701;   // 1.701 ps
pub const INTERCEPT_TIME_MICRO_S: u64 = 508;        // 0.508 ms
pub const TMSV_SQUEEZE_MILLI_DB: u32 = 21_715;      // 21.715 dB (r = 2.50)
pub const MAX_RANGE_NOISE_FM_RT_HZ: u32 = 144;      // 0.144 pm/sqrt(Hz)

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ThreatClassification {
    ClassX0UnmannedHardware,
    ClassX1SapientCrew,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DefenseAction {
    DarkBraidFuelRecycle { intercept_us: u64, adm_delta: i64 },
    NonLethalDeepStasis { intercept_us: u64, stasis_lapse_inv: u64, weyl_shear: u64 },
    ByzantineFirewallLockout { reject_fs: u64 },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AegisError {
    PhaseCloakCrossSectionNonZero,
    CausalCurtainBreach,
    UnethicalLethalActionBlockedByCbf,
    SensorSqueezingDegraded,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AegisOmegaController {
    pub l1_dark_cloak_sigma_coll: u64, // Must be 0 m^2
    pub l2_causal_ds2_2pn: i64,        // Must be <= 0
    pub l2_shift_vector_collapsed: bool,
    pub l3_trusted_cayley_hash: [u64; 4],
    pub l4_tmsv_squeeze_mdb: u32,
    pub l4_range_noise_fm: u32,
    pub layer5_vacuum_pinning: VacuumPinningState,
    pub layer6_cobordism_pinch: CobordismPinchController,
    pub bipartite_cobordism_manifold: BipartiteCobordismManifold,
    pub dehn_ingress_airlock: NilpotentDehnIngressAirlock,
}

pub type AegisAbsoluteController = AegisOmegaController;

impl AegisOmegaController {
    /// Canonical baseline configuration for SHBT-AEGIS-OMEGA perimeter
    pub const fn canonical() -> Self {
        Self {
            l1_dark_cloak_sigma_coll: 0,
            l2_causal_ds2_2pn: 0,
            l2_shift_vector_collapsed: true,
            l3_trusted_cayley_hash: [
                0x312A_04B1_C0DE_0001,
                0x312B_04B2_C0DE_0002,
                0x312C_04B3_C0DE_0003,
                0x312D_04B4_C0DE_0004,
            ],
            l4_tmsv_squeeze_mdb: TMSV_SQUEEZE_MILLI_DB,
            l4_range_noise_fm: MAX_RANGE_NOISE_FM_RT_HZ,
            layer5_vacuum_pinning: VacuumPinningState::new_pinned(),
            layer6_cobordism_pinch: CobordismPinchController::new_isolated(1.0e30),
            bipartite_cobordism_manifold: BipartiteCobordismManifold::new_unpinched(),
            dehn_ingress_airlock: NilpotentDehnIngressAirlock::new_ep2_diode(),
        }
    }

    /// Verifies all 4 perimeter layers and executes Proof-Carrying CBF interception
    pub const fn evaluate_and_intercept(
        &self,
        incoming_cayley_hash: &[u64; 4],
        threat_class: ThreatClassification,
        request_de_render: bool,
    ) -> Result<DefenseAction, AegisError> {
        // Layer 1: Gauge-decoupled dark-ledger phase cloak invariant
        if self.l1_dark_cloak_sigma_coll != 0 {
            return Err(AegisError::PhaseCloakCrossSectionNonZero);
        }
        // Layer 2: CCZ4 shift nullifier and 2PN causal gate invariant
        if self.l2_causal_ds2_2pn > 0 || !self.l2_shift_vector_collapsed {
            return Err(AegisError::CausalCurtainBreach);
        }
        // Layer 4 sensor health verification
        if self.l4_tmsv_squeeze_mdb < TMSV_SQUEEZE_MILLI_DB
            || self.l4_range_noise_fm > MAX_RANGE_NOISE_FM_RT_HZ
        {
            return Err(AegisError::SensorSqueezingDegraded);
        }
        // Layer 3: 256-Bit Burau-Cayley cryptographic braid authentication
        let auth_match = (self.l3_trusted_cayley_hash[0] == incoming_cayley_hash[0])
            & (self.l3_trusted_cayley_hash[1] == incoming_cayley_hash[1])
            & (self.l3_trusted_cayley_hash[2] == incoming_cayley_hash[2])
            & (self.l3_trusted_cayley_hash[3] == incoming_cayley_hash[3]);

        if !auth_match {
            // Foreign intruder detected: execute Layer 4 Bondi-Hoyle Harvester with CBF triage
            match threat_class {
                ThreatClassification::ClassX1SapientCrew => {
                    if request_de_render {
                        // Proof-Carrying CBF strictly blocks lethal de-rendering of Class X1 sapients
                        return Err(AegisError::UnethicalLethalActionBlockedByCbf);
                    }
                    Ok(DefenseAction::NonLethalDeepStasis {
                        intercept_us: INTERCEPT_TIME_MICRO_S,
                        stasis_lapse_inv: 1_000_000_000, // alpha = 10^-9 deep stasis
                        weyl_shear: 0,                   // E_uv == 0 non-harm guarantee
                    })
                }
                ThreatClassification::ClassX0UnmannedHardware => {
                    Ok(DefenseAction::DarkBraidFuelRecycle {
                        intercept_us: INTERCEPT_TIME_MICRO_S,
                        adm_delta: 0,
                    })
                }
            }
        } else {
            Ok(DefenseAction::ByzantineFirewallLockout {
                reject_fs: BYZANTINE_LOCKOUT_FEMTO_S,
            })
        }
    }

    /// Unified Phase 5A->5C transition: verify Layers 1-5 during bounded harvest,
    /// then engage Layer 6 ADM monopole nullification and 17.01 as pinch-off.
    pub fn execute_harvest_to_isolation_transition(
        &mut self,
        harvest_mass_kg: f64,
        harvest_window_sec: f64,
    ) -> Result<(), u32> {
        if self.l1_dark_cloak_sigma_coll != 0
            || self.l2_causal_ds2_2pn > 0
            || !self.l2_shift_vector_collapsed
        {
            return Err(vacuum_pinning::SHBT_SWARM_FAULT_CBF_VIOLATION);
        }
        if self.l4_tmsv_squeeze_mdb < TMSV_SQUEEZE_MILLI_DB
            || self.l4_range_noise_fm > MAX_RANGE_NOISE_FM_RT_HZ
        {
            return Err(vacuum_pinning::SHBT_SWARM_FAULT_CBF_BREACH);
        }
        self.layer5_vacuum_pinning.enforce_sugawara_framing_closure(
            0.0,
            vacuum_pinning::MIN_CDL_INSTANTON_ACTION_SE,
            harvest_mass_kg,
            harvest_window_sec,
        )?;
        if !self.layer6_cobordism_pinch.nullify_exterior_adm_monopole(
            0.0,
            cobordism_pinch::CCZ4_GUNDLACH_KAPPA_1,
        ) {
            return Err(cobordism_pinch::SHBT_SWARM_FAULT_COBORDISM_LEAK);
        }
        self.layer6_cobordism_pinch.execute_macroscopic_cobordism_pinch(
            cobordism_pinch::MAX_GW_MEMORY_RESIDUAL_NORM,
        )?;
        self.bipartite_cobordism_manifold.execute_pinch_off(
            0.0,
            1.0e-130,
            cobordism_pinch::MAX_GW_MEMORY_RESIDUAL_NORM,
            1.0e-15,
        )?;
        self.dehn_ingress_airlock.evaluate_ingress(true, true, true, false, false)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trusted_auth_lockout() {
        let aegis = AegisOmegaController::canonical();
        let hash = aegis.l3_trusted_cayley_hash;
        let action = aegis
            .evaluate_and_intercept(&hash, ThreatClassification::ClassX0UnmannedHardware, false)
            .unwrap();
        assert_eq!(
            action,
            DefenseAction::ByzantineFirewallLockout {
                reject_fs: BYZANTINE_LOCKOUT_FEMTO_S
            }
        );
    }

    #[test]
    fn test_class_x0_recycle() {
        let aegis = AegisOmegaController::canonical();
        let foreign_hash = [0, 0, 0, 0];
        let action = aegis
            .evaluate_and_intercept(&foreign_hash, ThreatClassification::ClassX0UnmannedHardware, true)
            .unwrap();
        assert_eq!(
            action,
            DefenseAction::DarkBraidFuelRecycle {
                intercept_us: INTERCEPT_TIME_MICRO_S,
                adm_delta: 0
            }
        );
    }

    #[test]
    fn test_aegis_absolute_transition_engages_layer6() {
        let mut aegis = AegisOmegaController::canonical();
        assert!(aegis
            .execute_harvest_to_isolation_transition(1.988_409_870_698_051e39, 18.26 * 86_400.0)
            .is_ok());
        assert!(aegis.layer6_cobordism_pinch.cobordism_pinch_executed);
        assert_ne!(
            aegis.layer6_cobordism_pinch.status_latch & cobordism_pinch::SHBT_AEGIS_COBORDISM_PINCH_LATCH,
            0
        );
    }

    #[test]
    fn test_class_x1_cbf_protection() {
        let aegis = AegisOmegaController::canonical();
        let foreign_hash = [0, 0, 0, 0];
        // De-rendering Class X1 is strictly blocked by CBF
        assert_eq!(
            aegis.evaluate_and_intercept(&foreign_hash, ThreatClassification::ClassX1SapientCrew, true),
            Err(AegisError::UnethicalLethalActionBlockedByCbf)
        );

        // Safe stasis is permitted
        let action = aegis
            .evaluate_and_intercept(&foreign_hash, ThreatClassification::ClassX1SapientCrew, false)
            .unwrap();
        assert_eq!(
            action,
            DefenseAction::NonLethalDeepStasis {
                intercept_us: INTERCEPT_TIME_MICRO_S,
                stasis_lapse_inv: 1_000_000_000,
                weyl_shear: 0,
            }
        );
    }
}
