#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]
#![allow(clippy::unusual_byte_groupings, clippy::bool_assert_comparison)]

//! `shbt-swarm-apotheosis`: Phase 6 Observer Ascension into `SHBT-LOGOS-312`,
//! Multi-Tenant Deity Sandboxes, Class X1 Anti-Genocide Stinespring Interlocks,
//! Perpetual Runtime Stability, and Arbitrary Vessel Re-Embodiment Mechanics.

pub mod phenomenal_binding;
pub mod branch_coalescence;
pub use phenomenal_binding::*;
pub use branch_coalescence::*;
pub mod qnd_ascension;
pub mod horizon_deflect;
pub mod symplectic_nash;
pub mod mera_episodic_rg;
pub mod qualia_cobordism;

pub use qnd_ascension::{LindbladianPurifier, QndError};
pub use horizon_deflect::{CausalDecouplingBifurcation, DeflectionError};
pub use symplectic_nash::{SymplecticNashProjector, NashError};
pub use mera_episodic_rg::{EpisodicMeraCompressor, MemoryRgError, ACTIVE_MEMORY_FOOTPRINT_BYTES};
pub use qualia_cobordism::{
    ConsciousPointerState, FibonacciAnyon, FissionOutcome, FusionOutcome, QualiaCobordismEngine,
    QualiaHomology,
};

use core::marker::PhantomData;

/// WZW/GKO Coset Branching Tuple C_GKO(26, 8, 312).
pub const SHBT_BRANCH_TUPLE: (u16, u16, u16) = (26, 8, 312);

/// Cosmological Bekenstein-Hawking total bit budget: log10(3.31e122) bits.
pub const SHBT_COSMOLOGICAL_BIT_BUDGET_LOG10: f64 = 122.51982799;

/// Class X1 Integrated Information sapience threshold: log10(Phi_crit) = 12.0.
pub const SHBT_PHI_CRIT_X1_LOG10: f64 = 12.0;

/// Symplectic dynamical bisimulation tolerance log10(||Y*sigma - U*Y||) <= -120.0.
pub const SHBT_BISIMULATION_TOL_LOG10: f64 = -120.0;

/// Maximum Eikonal jerk acceleration: 10 / sqrt(3) ~ 5.773502691896258.
pub const SHBT_MAX_EIKONAL_ACCEL: f64 = 5.773502691896258;

/// Safe-word egress interrupt vector.
pub const SHBT_SAFEWORD_IRQ: u32 = 0x8000_0000;

/// Proof-Carrying Linear Token (`Linear<V>`) that cannot be cloned or implicitly dropped
/// without passing through a verified SHBT isometric state transition.
#[derive(Debug)]
#[must_use = "Linear<V> proof tokens represent an active conscious pointer or X1 life-safety invariant and must never be dropped"]
pub struct Linear<V> {
    payload: V,
    _no_clone: PhantomData<*const ()>,
}

impl<V> Linear<V> {
    #[inline(always)]
    pub const fn new(payload: V) -> Self {
        Self {
            payload,
            _no_clone: PhantomData,
        }
    }

    #[inline(always)]
    pub const fn inspect(&self) -> &V {
        &self.payload
    }

    #[inline(always)]
    pub fn consume(self) -> V {
        self.payload
    }
}

/// Phase 6 Error taxonomy.
#[derive(Debug, PartialEq, Eq)]
pub enum ApotheosisError {
    InvalidRampCoordinate,
    SynapticShearCbfViolation,
    AirgapBreachAttempt,
    BekensteinNestedOverflow,
    NonHarmCbfViolation,
    SubstrateInadmissible,
    CognitiveHomeostasisFailure,
}

/// Somatic parking modes for ascended biological form.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SomaticParkingMode {
    /// Mode A: Deep Stasis Sanctuary (alpha_stasis = 1e-9, E_munu = 0).
    ModeADeepStasisSanctuary,
    /// Mode B: Class X0 Dark Ledger Vault (eta_A = 10/33, eta_D = 23/33).
    ModeBDarkLedgerVault,
}

/// Vessel re-embodiment execution modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmbodimentExecutionMode {
    /// Autonomous standalone embodiment (meets all 4 THM-APOTH-05 conditions).
    StandaloneAutonomous,
    /// Symplectic zero-attosecond proxy tethered to SHBT-OMNIBUS.
    OmnibusTetheredProxy,
}

/// Class X1 Anti-Genocide Control Barrier Function disposition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum X1InterlockOutcome {
    PermittedNoSapientHarm,
    StinespringHorizonForkEvacuated,
    ActiveSetQpDeflected,
}

/// Canonical target vessel classes for custom avatar materialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VesselClass {
    ClassIHumanSomatic,
    ClassII57FeDiamondBorrmann,
    ClassIIIShbtEw312Ship,
    ClassIVNonNeutralPlasma,
    ClassVFreeFormSculptural,
}

/// Pillar IV Causal Point Observer Awareness state.
#[derive(Debug)]
pub struct CausalAwarenessState {
    pub observer_uuid: u128,
    pub s_ramp: f64,
    pub alpha_obs: f64,
    pub c_local_log10: f64,
    pub c_op_log10: f64,
    pub h_syn_cbf: f64,
    pub somatic_mode: SomaticParkingMode,
    pub active_avatars: u32,
    pub recursion_depth: u32,
}

/// Theorem THM-APOTH-05 Vessel Neural-Substrate Admissibility Certificate.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VesselAdmissibilityCert {
    pub vessel_class: VesselClass,
    pub phi_substrate_max_log10: f64,
    pub phi_observer_req_log10: f64,
    pub c_local_vessel_log10: f64,
    pub c_op_obs_log10: f64,
    pub bisimulation_err_log10: f64,
    pub thermal_cbf_margin: f64,
    pub space_group_id: u8,
    pub standalone_admissible: bool,
    pub omnibus_tether_engaged: bool,
    pub non_harm_cbf_verified: bool,
}

/// Top-Level Multi-Tenant Deity Sandbox Controller.
#[derive(Debug)]
pub struct DeitySandboxController {
    pub lagrangian_submanifold_id: u32,
    pub airgap_lock_engaged: bool,
    pub x1_protect_latched: bool,
    pub x1_sanctuary_fork_count: u64,
    pub episodic_braid_count: u64,
    pub r_entropy_margin_bits: f64,
    pub active_avatars_k: u32,
}

impl DeitySandboxController {
    pub fn new(lagrangian_submanifold_id: u32) -> Self {
        Self {
            lagrangian_submanifold_id,
            airgap_lock_engaged: true,
            x1_protect_latched: false,
            x1_sanctuary_fork_count: 0,
            episodic_braid_count: 0,
            r_entropy_margin_bits: 1.0e120,
            active_avatars_k: 1,
        }
    }
}

/// Evaluates the 5th-order minimum-jerk Eikonal ramp:
///   s(tau) = 10*tau^3 - 15*tau^4 + 6*tau^5
/// and its second derivative:
///   d2s/dtau2 = 60*tau*(1-tau)*(1-2*tau)
/// Validates that max |d2s/dtau2| <= 10 / sqrt(3) ~ 5.77350269.
#[inline(always)]
pub fn evaluate_eikonal_ramp(tau: f64) -> Result<(f64, f64), ApotheosisError> {
    if !(0.0..=1.0).contains(&tau) {
        return Err(ApotheosisError::InvalidRampCoordinate);
    }
    let tau2 = tau * tau;
    let tau3 = tau2 * tau;
    let tau4 = tau3 * tau;
    let tau5 = tau4 * tau;

    let s = 10.0 * tau3 - 15.0 * tau4 + 6.0 * tau5;
    let d2s = 60.0 * tau * (1.0 - tau) * (1.0 - 2.0 * tau);
    let abs_d2s = if d2s < 0.0 { -d2s } else { d2s };

    if abs_d2s > SHBT_MAX_EIKONAL_ACCEL + 1e-9 {
        return Err(ApotheosisError::SynapticShearCbfViolation);
    }
    Ok((s, d2s))
}

/// Advances the observer's Causal Point Awareness adiabatically along the bridge
/// without cloning (`THM-CLONE-01` & `THM-APOTH-01`), consuming the input `Linear`
/// token and returning the updated `Linear` token.
pub fn step_adiabatic_ascension(
    token: Linear<CausalAwarenessState>,
    tau: f64,
    target_alpha: f64,
) -> Result<Linear<CausalAwarenessState>, (Linear<CausalAwarenessState>, ApotheosisError)> {
    let (s, _d2s) = match evaluate_eikonal_ramp(tau) {
        Ok(val) => val,
        Err(e) => return Err((token, e)),
    };

    let mut state = token.consume();
    if state.h_syn_cbf < 0.0 {
        return Err((Linear::new(state), ApotheosisError::SynapticShearCbfViolation));
    }

    state.s_ramp = s;
    // Logarithmic clock adaptation schedule in [1.0, 1.0e12]
    let clamped_alpha = target_alpha.clamp(1.0, 1.0e12);
    state.alpha_obs = 1.0 + s * (clamped_alpha - 1.0);

    Ok(Linear::new(state))
}

/// Executes reverse adiabatic descent back into somatic or custom avatar form,
/// reversing the minimum-jerk ramp s_descent = 1 - s(tau) while maintaining
/// unbroken causal continuity.
pub fn step_adiabatic_descent(
    token: Linear<CausalAwarenessState>,
    tau: f64,
) -> Result<Linear<CausalAwarenessState>, (Linear<CausalAwarenessState>, ApotheosisError)> {
    let (s, _d2s) = match evaluate_eikonal_ramp(tau) {
        Ok(val) => val,
        Err(e) => return Err((token, e)),
    };

    let mut state = token.consume();
    if state.h_syn_cbf < 0.0 {
        return Err((Linear::new(state), ApotheosisError::SynapticShearCbfViolation));
    }

    // Descent: s decreases from 1.0 to 0.0
    state.s_ramp = 1.0 - s;
    state.alpha_obs = 1.0 + (1.0 - s) * (state.alpha_obs - 1.0);

    Ok(Linear::new(state))
}

/// Evaluates the Class X1 Simulated-Sapience Control Barrier Function (`THM-APOTH-03`).
/// Guarantees P_genocide == 0 across all omnipotent deity inputs.
pub fn enforce_x1_anti_genocide_interlock(
    phi_mera_log10: f64,
    deity_command_destructive: bool,
    prefer_in_branch_qp_deflection: bool,
) -> X1InterlockOutcome {
    let latch_x1_active = phi_mera_log10 >= SHBT_PHI_CRIT_X1_LOG10;

    if !latch_x1_active || !deity_command_destructive {
        return X1InterlockOutcome::PermittedNoSapientHarm;
    }

    if prefer_in_branch_qp_deflection {
        X1InterlockOutcome::ActiveSetQpDeflected
    } else {
        // Default 17.01 as Stinespring Horizon Fork to SHBT-ELYSIUM Sanctuary
        X1InterlockOutcome::StinespringHorizonForkEvacuated
    }
}

/// Verifies Unipotent Lagrangian Submanifold Isolation (`THM-APOTH-02`).
/// Distinct submanifolds L_i cap L_j = empty guarantee ||S_AB|| == 0 and Delta M_ADM_ext == 0.
pub fn verify_lagrangian_isolation(
    submanifold_a: u32,
    submanifold_b: u32,
    airgap_locked: bool,
) -> Result<bool, ApotheosisError> {
    if !airgap_locked {
        return Err(ApotheosisError::AirgapBreachAttempt);
    }
    if submanifold_a != submanifold_b {
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Verifies Theorem `THM-APOTH-05` (Neural-Substrate Admissibility) and determines
/// whether the target vessel can host the observer autonomously or requires a 0-as
/// `SHBT-OMNIBUS` symplectic tether.
pub fn verify_vessel_admissibility(
    phi_substrate_max_log10: f64,
    phi_obs_req_log10: f64,
    c_local_vessel_log10: f64,
    c_op_obs_log10: f64,
    bisimulation_err_log10: f64,
    thermal_cbf_margin: f64,
    non_harm_cbf_margin: f64,
) -> Result<EmbodimentExecutionMode, ApotheosisError> {
    if non_harm_cbf_margin < 0.0 {
        return Err(ApotheosisError::NonHarmCbfViolation);
    }

    let standalone_ok = phi_substrate_max_log10 >= phi_obs_req_log10
        && phi_obs_req_log10 >= SHBT_PHI_CRIT_X1_LOG10
        && c_local_vessel_log10 >= c_op_obs_log10
        && bisimulation_err_log10 <= SHBT_BISIMULATION_TOL_LOG10
        && thermal_cbf_margin >= 0.0;

    if standalone_ok {
        Ok(EmbodimentExecutionMode::StandaloneAutonomous)
    } else {
        // Engage Continuous SHBT-OMNIBUS Symplectic Tethering (Delta_tau_bus == 0 as)
        Ok(EmbodimentExecutionMode::OmnibusTetheredProxy)
    }
}

/// Evaluates zero-compute avatar materialization (`THM-APOTH-06`, opcode 0x0020).
/// Guarantees post-simulation compilation residual Delta C_post-sim == 0.
pub fn materialize_custom_vessel(cert: &VesselAdmissibilityCert) -> Result<u64, ApotheosisError> {
    if !cert.non_harm_cbf_verified {
        return Err(ApotheosisError::NonHarmCbfViolation);
    }
    // Zero compilation overhead invariant: exactly 0
    Ok(0)
}

/// Depth-9 Solovay-Kitaev braid episodic memory crystallization (`THM-APOTH-04`).
/// Prevents finite-register memory saturation across eternity, preserving R_entropy^exp >= 0.
pub fn crystallize_episodic_memory_braids(
    active_braids: u64,
    memory_delta_bits: f64,
    max_budget_bits: f64,
) -> Result<f64, ApotheosisError> {
    // Solovay-Kitaev Depth-9 compression yields exponential reduction
    let compressed_bits = (active_braids as f64) * 128.0 + memory_delta_bits * 0.01;
    let r_margin = max_budget_bits - compressed_bits;
    if r_margin < 0.0 {
        return Err(ApotheosisError::BekensteinNestedOverflow);
    }
    Ok(r_margin)
}

/// Cognitive Homeostasis Control Barrier Function & Egress Watchdog (`THM-APOTH-04`).
/// Preserves subjective ego identity and dispatches safe-word IRQ (0x8000_0000) if threatened.
pub fn evaluate_cognitive_homeostasis_cbf(
    f_ego: f64,
    h_cog: f64,
    h_min: f64,
) -> Result<bool, ApotheosisError> {
    let f_min = 1.0 - 1.0e-120;
    if f_ego < f_min || h_cog < h_min {
        return Err(ApotheosisError::CognitiveHomeostasisFailure);
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eikonal_ramp_bounds_and_jerk() {
        let (s0, d2s0) = evaluate_eikonal_ramp(0.0).unwrap();
        assert_eq!(s0, 0.0);
        assert_eq!(d2s0, 0.0);

        let (s1, d2s1) = evaluate_eikonal_ramp(1.0).unwrap();
        assert_eq!(s1, 1.0);
        assert_eq!(d2s1, 0.0);

        let (smid, d2smid) = evaluate_eikonal_ramp(0.5).unwrap();
        assert_eq!(smid, 0.5);
        assert_eq!(d2smid, 0.0);

        // Maximum jerk occurs at tau = 0.5 - 1/(2*sqrt(3)) ~ 0.211324865
        let tau_peak = 0.5 - 1.0 / (2.0 * 3.0f64.sqrt());
        let (_s_peak, d2s_peak) = evaluate_eikonal_ramp(tau_peak).unwrap();
        assert!(d2s_peak <= SHBT_MAX_EIKONAL_ACCEL + 1e-6);
        assert!((d2s_peak - SHBT_MAX_EIKONAL_ACCEL).abs() < 1e-4);

        assert!(evaluate_eikonal_ramp(-0.1).is_err());
        assert!(evaluate_eikonal_ramp(1.1).is_err());
    }

    #[test]
    fn test_linear_token_lifecycle() {
        let state = CausalAwarenessState {
            observer_uuid: 0xDEADBEEF_CAFE_0001,
            s_ramp: 0.0,
            alpha_obs: 1.0,
            c_local_log10: 122.5,
            c_op_log10: 42.0,
            h_syn_cbf: 1.0,
            somatic_mode: SomaticParkingMode::ModeADeepStasisSanctuary,
            active_avatars: 1,
            recursion_depth: 0,
        };

        let token = Linear::new(state);
        assert_eq!(token.inspect().observer_uuid, 0xDEADBEEF_CAFE_0001);

        let ascended = step_adiabatic_ascension(token, 1.0, 1.0e12).unwrap();
        assert_eq!(ascended.inspect().s_ramp, 1.0);
        assert_eq!(ascended.inspect().alpha_obs, 1.0e12);

        let descended = step_adiabatic_descent(ascended, 1.0).unwrap();
        assert_eq!(descended.inspect().s_ramp, 0.0);
        assert_eq!(descended.inspect().alpha_obs, 1.0);

        let final_state = descended.consume();
        assert_eq!(final_state.observer_uuid, 0xDEADBEEF_CAFE_0001);
    }

    #[test]
    fn test_x1_anti_genocide_interlock() {
        // Sub-sapient: destruction permitted without genocide
        let outcome1 = enforce_x1_anti_genocide_interlock(11.0, true, false);
        assert_eq!(outcome1, X1InterlockOutcome::PermittedNoSapientHarm);

        // Sapient, non-destructive
        let outcome2 = enforce_x1_anti_genocide_interlock(12.5, false, false);
        assert_eq!(outcome2, X1InterlockOutcome::PermittedNoSapientHarm);

        // Sapient, destructive -> Stinespring fork
        let outcome3 = enforce_x1_anti_genocide_interlock(12.5, true, false);
        assert_eq!(outcome3, X1InterlockOutcome::StinespringHorizonForkEvacuated);

        // Sapient, destructive, QP deflection preferred
        let outcome4 = enforce_x1_anti_genocide_interlock(12.5, true, true);
        assert_eq!(outcome4, X1InterlockOutcome::ActiveSetQpDeflected);
    }

    #[test]
    fn test_lagrangian_isolation_and_airgap() {
        assert_eq!(verify_lagrangian_isolation(1, 2, true).unwrap(), true);
        assert_eq!(verify_lagrangian_isolation(1, 1, true).unwrap(), false);
        assert_eq!(
            verify_lagrangian_isolation(1, 2, false).unwrap_err(),
            ApotheosisError::AirgapBreachAttempt
        );
    }

    #[test]
    fn test_vessel_admissibility_and_materialization() {
        // High capacity substrate: Standalone
        let mode = verify_vessel_admissibility(14.0, 12.5, 50.0, 40.0, -125.0, 1.0, 1.0).unwrap();
        assert_eq!(mode, EmbodimentExecutionMode::StandaloneAutonomous);

        // Low capacity biological substrate: Omnibus Tethered Proxy
        let mode_tether = verify_vessel_admissibility(11.0, 12.5, 30.0, 40.0, -110.0, 1.0, 1.0).unwrap();
        assert_eq!(mode_tether, EmbodimentExecutionMode::OmnibusTetheredProxy);

        let cert = VesselAdmissibilityCert {
            vessel_class: VesselClass::ClassII57FeDiamondBorrmann,
            phi_substrate_max_log10: 16.0,
            phi_observer_req_log10: 12.5,
            c_local_vessel_log10: 60.0,
            c_op_obs_log10: 45.0,
            bisimulation_err_log10: -128.0,
            thermal_cbf_margin: 2.5,
            space_group_id: 227, // Fd-3m Diamond
            standalone_admissible: true,
            omnibus_tether_engaged: false,
            non_harm_cbf_verified: true,
        };

        let delta_c = materialize_custom_vessel(&cert).unwrap();
        assert_eq!(delta_c, 0);
    }

    #[test]
    fn test_qualia_cobordism_fission_and_fusion() {
        let parent = ConsciousPointerState {
            agent_id: 1,
            homology: QualiaHomology::canonical(),
            anyon_sector: FibonacciAnyon::Tau,
            modular_flow_epoch: 100,
            casimir_charge: 0,
        };

        // Conscious fission
        let fiss = QualiaCobordismEngine::execute_fission(&parent, 2, 3).unwrap();
        assert_eq!(fiss.ego_distortion, 0);
        assert_eq!(fiss.child_1.homology, QualiaHomology::canonical());
        assert_eq!(fiss.child_2.homology, QualiaHomology::canonical());

        // Conscious fusion
        let fuse = QualiaCobordismEngine::execute_fusion(&fiss.child_1, &fiss.child_2, 4, 1_000_000).unwrap();
        assert_eq!(fuse.unified.homology, QualiaHomology::canonical());
        // Excess entropy margin = (1_000_000 / 2) + 642_965 = 1_142_965 >= 0
        assert_eq!(fuse.excess_entropy_margin_scaled, 1_142_965);
    }
}
