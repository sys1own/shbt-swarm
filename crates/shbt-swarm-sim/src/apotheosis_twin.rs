//! `crates/shbt-swarm-sim/src/apotheosis_twin.rs`
//!
//! Phase 6 Multiphysics Digital Twin for Observer Ascension, Deity Sandbox,
//! Class X1 Anti-Genocide Interlocks, and Arbitrary Vessel Re-Embodiment.

use shbt_swarm_apotheosis::*;

/// Digital Twin representation of the Phase 6 Apotheosis pipeline.
#[derive(Debug)]
pub struct ApotheosisTwinSimulation {
    pub observer_state: Option<Linear<CausalAwarenessState>>,
    pub sandbox: DeitySandboxController,
    pub admitted_vessel: Option<VesselAdmissibilityCert>,
}

impl Default for ApotheosisTwinSimulation {
    fn default() -> Self {
        Self::canonical()
    }
}

impl ApotheosisTwinSimulation {
    /// Constructs a canonical Phase 6 simulation instance.
    pub fn canonical() -> Self {
        let initial_state = CausalAwarenessState {
            observer_uuid: 0x0312_AF07_0001,
            s_ramp: 0.0,
            alpha_obs: 1.0,
            c_local_log10: SHBT_COSMOLOGICAL_BIT_BUDGET_LOG10,
            c_op_log10: 42.0,
            h_syn_cbf: 1.0,
            somatic_mode: SomaticParkingMode::ModeADeepStasisSanctuary,
            active_avatars: 1,
            recursion_depth: 0,
        };

        Self {
            observer_state: Some(Linear::new(initial_state)),
            sandbox: DeitySandboxController::new(1),
            admitted_vessel: None,
        }
    }

    /// Simulates adiabatic ascension from somatic substrate into SHBT-LOGOS-312.
    pub fn simulate_ascension(&mut self, tau: f64, target_alpha: f64) -> Result<(), ApotheosisError> {
        let token = self.observer_state.take().expect("Active observer token required");
        match step_adiabatic_ascension(token, tau, target_alpha) {
            Ok(updated_token) => {
                self.observer_state = Some(updated_token);
                Ok(())
            }
            Err((returned_token, err)) => {
                self.observer_state = Some(returned_token);
                Err(err)
            }
        }
    }

    /// Evaluates anti-genocide safety interlock for simulated entities.
    pub fn test_anti_genocide_interlock(&mut self, phi_mera: f64, destructive: bool) -> X1InterlockOutcome {
        let outcome = enforce_x1_anti_genocide_interlock(phi_mera, destructive, false);
        if let X1InterlockOutcome::StinespringHorizonForkEvacuated = outcome {
            self.sandbox.x1_sanctuary_fork_count += 1;
        }
        outcome
    }

    /// Simulates vessel evaluation and materialization.
    pub fn evaluate_and_materialize_vessel(
        &mut self,
        cert: VesselAdmissibilityCert,
    ) -> Result<EmbodimentExecutionMode, ApotheosisError> {
        let mode = verify_vessel_admissibility(
            cert.phi_substrate_max_log10,
            cert.phi_observer_req_log10,
            cert.c_local_vessel_log10,
            cert.c_op_obs_log10,
            cert.bisimulation_err_log10,
            cert.thermal_cbf_margin,
            if cert.non_harm_cbf_verified { 1.0 } else { -1.0 },
        )?;

        let delta_c = materialize_custom_vessel(&cert)?;
        assert_eq!(delta_c, 0, "Zero-compute materialization invariant violated");
        self.admitted_vessel = Some(cert);
        Ok(mode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apotheosis_twin_canonical_run() {
        let mut sim = ApotheosisTwinSimulation::canonical();
        assert!(sim.simulate_ascension(1.0, 1.0e12).is_ok());

        let outcome = sim.test_anti_genocide_interlock(14.0, true);
        assert_eq!(outcome, X1InterlockOutcome::StinespringHorizonForkEvacuated);
        assert_eq!(sim.sandbox.x1_sanctuary_fork_count, 1);

        let cert = VesselAdmissibilityCert {
            vessel_class: VesselClass::ClassII57FeDiamondBorrmann,
            phi_substrate_max_log10: 16.0,
            phi_observer_req_log10: 12.0,
            c_local_vessel_log10: 60.0,
            c_op_obs_log10: 42.0,
            bisimulation_err_log10: -125.0,
            thermal_cbf_margin: 1.0,
            space_group_id: 227,
            standalone_admissible: true,
            omnibus_tether_engaged: false,
            non_harm_cbf_verified: true,
        };
        let mode = sim.evaluate_and_materialize_vessel(cert).unwrap();
        assert_eq!(mode, EmbodimentExecutionMode::StandaloneAutonomous);
    }
}
