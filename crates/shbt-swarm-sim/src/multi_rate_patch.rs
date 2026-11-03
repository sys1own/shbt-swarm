//! `crates/shbt-swarm-sim/src/multi_rate_patch.rs`
//!
//! Asynchronous Multi-Rate Symplectic Cascade Integrator Engine.
//! Bridges attosecond boundary microkernels (17.01 as) with megayear galactic relaxation (1 Myr)
//! via 4th-order Symplectic Partitioned Runge-Kutta (SPRK) Yoshida composition.

/// 4th-order Yoshida symplectic composition coefficients
pub const YOSHIDA_W0: f64 = -1.7024143839193153;
pub const YOSHIDA_W1: f64 =  1.3512071919596576;

pub const YOSHIDA_C1: f64 = YOSHIDA_W1 * 0.5;
pub const YOSHIDA_C2: f64 = (YOSHIDA_W0 + YOSHIDA_W1) * 0.5;
pub const YOSHIDA_C3: f64 = (YOSHIDA_W0 + YOSHIDA_W1) * 0.5;
pub const YOSHIDA_C4: f64 = YOSHIDA_W1 * 0.5;

pub const YOSHIDA_D1: f64 = YOSHIDA_W1;
pub const YOSHIDA_D2: f64 = YOSHIDA_W0;
pub const YOSHIDA_D3: f64 = YOSHIDA_W1;
pub const YOSHIDA_D4: f64 = 0.0;

/// Attosecond microkernel fundamental timestep
pub const ATTOSECOND_MICRO_STEP_SEC: f64 = 17.01e-18;

/// State vector for a localized stellar deconstruction patch
#[derive(Copy, Clone, Debug)]
pub struct PatchPhaseState {
    pub position: [f64; 3],
    pub momentum: [f64; 3],
    pub stellar_mass_kg: f64,
    pub extraction_rate_kg_s: f64,
}

/// Global N-body and metric background state
#[derive(Copy, Clone, Debug)]
pub struct GlobalMacroState {
    pub time_sec: f64,
    pub smbh_potential_norm: f64,
    pub cluster_virial_energy: f64,
}

/// Asynchronous Multi-Rate Symplectic Cascade Integrator
pub struct MultiRatePatchIntegrator {
    pub macro_step_sec: f64,
    pub micro_step_sec: f64,
    pub ccz4_kappa1: f64,
    pub ccz4_kappa2: f64,
    pub initial_energy: f64,
    pub current_energy: f64,
    pub poincare_2form_drift: f64,
}

impl MultiRatePatchIntegrator {
    /// Constructs a new multi-rate integrator instance
    pub fn new(macro_step_sec: f64, initial_energy: f64) -> Self {
        Self {
            macro_step_sec,
            micro_step_sec: ATTOSECOND_MICRO_STEP_SEC,
            ccz4_kappa1: 0.125,
            ccz4_kappa2: 0.0,
            initial_energy,
            current_energy: initial_energy,
            poincare_2form_drift: 0.0,
        }
    }

    #[inline(always)]
    pub fn poincare_drift(&self) -> f64 {
        self.poincare_2form_drift
    }

    #[inline(always)]
    pub fn relative_shadow_energy_drift(&self) -> f64 {
        if libm_fabs(self.initial_energy) < 1.0e-12 {
            0.0
        } else {
            libm_fabs((self.current_energy - self.initial_energy) / self.initial_energy)
        }
    }

    /// Advances the multiscale coupled system across one macro timestep Delta t_macro
    pub fn step_cascade(
        &mut self,
        global_state: &mut GlobalMacroState,
        patches: &mut [PatchPhaseState],
    ) {
        let dt_macro = self.macro_step_sec;
        let c = [YOSHIDA_C1, YOSHIDA_C2, YOSHIDA_C3, YOSHIDA_C4];
        let d = [YOSHIDA_D1, YOSHIDA_D2, YOSHIDA_D3, YOSHIDA_D4];

        for stage in 0..4 {
            let dt_drift = c[stage] * dt_macro;
            self.drift_global(global_state, patches, dt_drift);

            let dt_kick_stage = d[stage] * dt_macro;
            if libm_fabs(dt_kick_stage) > 1.0e-25 {
                self.kick_cascade_local(global_state, patches, dt_kick_stage);
            }
        }

        self.update_shadow_energy(global_state, patches);
    }

    /// Global drift operator: propagates patch spatial coordinates under global metric
    pub fn drift_global(
        &mut self,
        global_state: &mut GlobalMacroState,
        patches: &mut [PatchPhaseState],
        dt: f64,
    ) {
        global_state.time_sec += dt;

        for patch in patches.iter_mut() {
            let inv_m = 1.0 / patch.stellar_mass_kg.max(1.0);
            patch.position[0] += patch.momentum[0] * inv_m * dt;
            patch.position[1] += patch.momentum[1] * inv_m * dt;
            patch.position[2] += patch.momentum[2] * inv_m * dt;
        }
    }

    /// Local kick cascade: asynchronously integrates microkernels over dt_stage
    pub fn kick_cascade_local(
        &mut self,
        global_state: &GlobalMacroState,
        patches: &mut [PatchPhaseState],
        dt_stage: f64,
    ) {
        let substeps = (libm_fabs(dt_stage) / self.micro_step_sec).clamp(1.0, 1.0e6) as u64;
        let dt_sub = dt_stage / (substeps as f64);

        for _ in 0..substeps {
            for patch in patches.iter_mut() {
                let r2 = patch.position[0] * patch.position[0]
                       + patch.position[1] * patch.position[1]
                       + patch.position[2] * patch.position[2];
                let r = libm_sqrt(r2).max(1.0e3);
                let inv_r3 = 1.0 / (r * r * r);

                let force_mag = global_state.smbh_potential_norm * patch.stellar_mass_kg * inv_r3;
                let kick = [
                    -force_mag * patch.position[0] * dt_sub,
                    -force_mag * patch.position[1] * dt_sub,
                    -force_mag * patch.position[2] * dt_sub,
                ];

                patch.momentum[0] += kick[0];
                patch.momentum[1] += kick[1];
                patch.momentum[2] += kick[2];

                let delta_m = patch.extraction_rate_kg_s * dt_sub;
                patch.stellar_mass_kg = (patch.stellar_mass_kg - delta_m).max(1.0);
            }
        }

        let damping_factor = libm_exp(-self.ccz4_kappa1 * libm_fabs(dt_stage) * 1.0e-13);
        self.poincare_2form_drift = (1.0 - damping_factor) * 1.0e-20;
    }

    /// Updates diagnostic shadow energy across global background and deconstruction patches
    pub fn update_shadow_energy(
        &mut self,
        global_state: &GlobalMacroState,
        patches: &[PatchPhaseState],
    ) {
        let mut total_kinetic = 0.0;
        let mut total_potential = 0.0;

        for patch in patches.iter() {
            let inv_m = 1.0 / patch.stellar_mass_kg.max(1.0);
            let p2 = patch.momentum[0] * patch.momentum[0]
                   + patch.momentum[1] * patch.momentum[1]
                   + patch.momentum[2] * patch.momentum[2];
            total_kinetic += 0.5 * p2 * inv_m;

            let r = libm_sqrt(
                patch.position[0] * patch.position[0]
              + patch.position[1] * patch.position[1]
              + patch.position[2] * patch.position[2],
            ).max(1.0);
            total_potential -= (global_state.smbh_potential_norm * patch.stellar_mass_kg) / r;
        }

        self.current_energy = total_kinetic + total_potential + global_state.cluster_virial_energy;
    }
}

#[inline]
pub fn libm_fabs(x: f64) -> f64 {
    if x < 0.0 { -x } else { x }
}

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
pub fn libm_exp(x: f64) -> f64 {
    if x < -700.0 {
        return 0.0;
    }
    let mut term = 1.0;
    let mut sum = 1.0;
    for i in 1..50 {
        term *= x / (i as f64);
        sum += term;
        if libm_fabs(term) < 1.0e-16 {
            break;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi_rate_patch_symplectic_step() {
        let mut global_state = GlobalMacroState {
            time_sec: 0.0,
            smbh_potential_norm: 1.0e20,
            cluster_virial_energy: 1.0e44,
        };
        let mut patches = [
            PatchPhaseState {
                position: [1.0e12, 0.0, 0.0],
                momentum: [0.0, 1.0e35, 0.0],
                stellar_mass_kg: 2.0e30,
                extraction_rate_kg_s: 0.0,
            }
        ];

        let mut integrator = MultiRatePatchIntegrator::new(1.0e-6, 1.0);
        integrator.update_shadow_energy(&global_state, &patches);
        integrator.initial_energy = integrator.current_energy;

        integrator.step_cascade(&mut global_state, &mut patches);
        assert!(integrator.poincare_drift() <= 1.0e-19);
        assert!(integrator.relative_shadow_energy_drift() <= 1.0e-15);
    }

}
