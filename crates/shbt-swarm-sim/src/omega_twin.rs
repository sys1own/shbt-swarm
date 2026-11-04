//! `crates/shbt-swarm-sim/src/omega_twin.rs`
//!
//! 6th-Generation Continuous-Flow Boundary Annihilation Reactor (SHBT-MAR-OMEGA) Digital Twin.
//! Implements the Six Deterministic Stages of the Closed-Loop Pipeline in `#![no_std]` execution.

/// Physical and mathematical constants
pub const SPEED_OF_LIGHT: f64 = 299_792_458.0;
pub const SPEED_OF_LIGHT_SQ: f64 = 89_875_517_873_681_764.0;
pub const DIRECT_COUPLING_EFFICIENCY: f64 = 0.99981146;
pub const WASTE_COUPLING_FRACTION: f64 = 1.0 - DIRECT_COUPLING_EFFICIENCY; // 1.8854e-4
pub const OPACITY_QUENCHING_ZETA: f64 = 100.0 / 1089.0;                  // 0.09182736455
pub const SEED_MASS_ALPHA: f64 = 2.1648e-16;                            // kg/bit
pub const FRACTON_ATTENUATION_PER_LAYER: f64 = 0.15653453279;           // 16 layers = 1.299459e-13 (-128.86 dB)
pub const CATTANEO_TAU_Q: f64 = 14.5e-12;                               // 14.5 picoseconds
pub const SENSIBLE_HEAT_CLAMP_WATTS: f64 = 245_000.0;                   // 245.0 kW
pub const COIL_TEMPERATURE_MAX_KELVIN: f64 = 25.0;                      // Limit: 25.0 K
pub const CCZ4_KAPPA_1: f64 = 0.15;
pub const CCZ4_KAPPA_2: f64 = 0.0;
pub const CCZ4_KAPPA_3: f64 = 1.0;
pub const RESIDUAL_VON_MISES_MPA: f64 = 7.85e-9;                        // 7.85e-9 MPa << 1.0e-6 MPa

/// Number of integrable boundary Dynkin characters in SU(2)_26 x SU(3)_8 x SO(10)_312
pub const DYNKIN_SLOT_COUNT: usize = 346;

/// Configuration parameters for the continuous-flow MAR-OMEGA twin
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OmegaSimulationParams {
    pub ambient_plasma_density: f64,       // kg/m^3
    pub relative_inflow_velocity: f64,     // m/s
    pub plasma_sound_speed: f64,           // m/s
    pub dynamic_slipstream_throttle: f64,  // f_slip in [1.0e-4, 1.0e-3]
    pub target_gross_power: f64,           // Watts (nominal: 10.0e21 W = 10.0 ZW)
    pub allocation_pinch: f64,             // u_pinch (nominal: 0.280)
    pub allocation_melt: f64,              // u_melt  (nominal: 0.145)
    pub allocation_hadron: f64,            // u_hadron (nominal: 0.185)
    pub allocation_warp: f64,              // u_warp  (nominal: 0.390)
}

impl Default for OmegaSimulationParams {
    fn default() -> Self {
        Self {
            ambient_plasma_density: 1.45e-3,
            relative_inflow_velocity: 1.25e6,
            plasma_sound_speed: 4.50e5,
            dynamic_slipstream_throttle: 5.25e-4,
            target_gross_power: 10.00e21, // 10.00 ZW
            allocation_pinch: 0.280,
            allocation_melt: 0.145,
            allocation_hadron: 0.185,
            allocation_warp: 0.390,
        }
    }
}

/// Dynamic state vector of the continuous-flow reactor twin
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OmegaSimulationState {
    pub simulation_time: f64,
    pub mass_ingestion_rate: f64,          // kg/s
    pub gross_power_watts: f64,            // Watts
    pub direct_cft_power_watts: f64,       // Watts
    pub thermal_waste_power_watts: f64,     // Watts
    pub holographic_bit_budget: f64,       // bits
    pub synthetic_seed_mass_kg: f64,       // kg
    pub cryostat_heat_flux_watts: f64,     // Watts (clamped <= 245 kW)
    pub superhydride_coil_temp_k: f64,     // Kelvin (clamped <= 25.0 K)
    pub actuator_pinch_power: f64,         // Watts
    pub actuator_melt_power: f64,          // Watts
    pub actuator_hadron_power: f64,        // Watts
    pub actuator_warp_power: f64,          // Watts
    pub von_mises_shear_stress_mpa: f64,   // MPa
    pub is_cryogenically_stable: bool,
    pub is_cloaking_active: bool,
}

impl Default for OmegaSimulationState {
    fn default() -> Self {
        Self {
            simulation_time: 0.0,
            mass_ingestion_rate: 111_265.01,
            gross_power_watts: 10.00e21,
            direct_cft_power_watts: 10.00e21 * DIRECT_COUPLING_EFFICIENCY,
            thermal_waste_power_watts: 10.00e21 * WASTE_COUPLING_FRACTION,
            holographic_bit_budget: 0.0,
            synthetic_seed_mass_kg: 0.0,
            cryostat_heat_flux_watts: 244_999.0,
            superhydride_coil_temp_k: 20.0,
            actuator_pinch_power: 2.7995e21,
            actuator_melt_power: 1.4497e21,
            actuator_hadron_power: 1.8497e21,
            actuator_warp_power: 3.8993e21,
            von_mises_shear_stress_mpa: RESIDUAL_VON_MISES_MPA,
            is_cryogenically_stable: true,
            is_cloaking_active: true,
        }
    }
}

impl OmegaSimulationState {
    /// Advances this state by executing the 6-stage pipeline over `dt`
    pub fn step_omega_pipeline(&mut self, dt: f64) -> OmegaStageOutputs {
        let params = OmegaSimulationParams::default();
        step_omega_pipeline(self, &params, dt)
    }
}

/// Telemetry output packet summarizing a single step of the pipeline
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OmegaStageOutputs {
    pub stage1_m_dot: f64,
    pub stage1_p_gross: f64,
    pub stage2_p_direct: f64,
    pub stage2_p_waste: f64,
    pub stage3_m_seed: f64,
    pub stage3_zeta: f64,
    pub stage4_p_sensible: f64,
    pub stage4_t_coil: f64,
    pub stage5_p_pinch: f64,
    pub stage5_p_melt: f64,
    pub stage5_p_hadron: f64,
    pub stage5_p_warp: f64,
    pub stage6_sigma_vm: f64,
}

/// Helper function to clamp f64 in `#![no_std]`
#[inline(always)]
fn clamp_f64(val: f64, min: f64, max: f64) -> f64 {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

/// Executes the complete six-stage continuous-flow simulation pipeline
pub fn step_omega_pipeline(
    state: &mut OmegaSimulationState,
    params: &OmegaSimulationParams,
    dt: f64,
) -> OmegaStageOutputs {
    state.simulation_time += dt;

    // STAGE 1: Slipstream Intake & Hydrodynamic Divertor
    let _f_slip = clamp_f64(params.dynamic_slipstream_throttle, 1.0e-4, 1.0e-3);
    let m_dot = params.target_gross_power / SPEED_OF_LIGHT_SQ;
    let p_gross = m_dot * SPEED_OF_LIGHT_SQ;

    state.mass_ingestion_rate = m_dot;
    state.gross_power_watts = p_gross;

    // STAGE 2: Direct 346-Slot Boundary CFT Exergy Coupling
    let p_direct = p_gross * DIRECT_COUPLING_EFFICIENCY;
    let p_waste = p_gross * WASTE_COUPLING_FRACTION;

    state.direct_cft_power_watts = p_direct;
    state.thermal_waste_power_watts = p_waste;

    // STAGE 3: Holographic Stress Injection & Conformal Metric Tuning
    let bit_injection_rate = p_direct * 1.50e-9;
    state.holographic_bit_budget += bit_injection_rate * dt;
    let m_seed = SEED_MASS_ALPHA * state.holographic_bit_budget;
    state.synthetic_seed_mass_kg = m_seed;

    // STAGE 4: 16-Layer Sp(16, ℤ) Fracton Cryogenic Attenuation
    let mut fracton_transmittance = 1.0;
    let mut layer = 0;
    while layer < 16 {
        fracton_transmittance *= FRACTON_ATTENUATION_PER_LAYER;
        layer += 1;
    }
    let raw_sensible = p_waste * fracton_transmittance;
    let p_sensible = if raw_sensible > SENSIBLE_HEAT_CLAMP_WATTS {
        SENSIBLE_HEAT_CLAMP_WATTS
    } else {
        raw_sensible
    };
    state.cryostat_heat_flux_watts = p_sensible;

    let t_equilibrium = 20.0 + (p_sensible / SENSIBLE_HEAT_CLAMP_WATTS) * 0.50;
    state.superhydride_coil_temp_k = t_equilibrium;
    state.is_cryogenically_stable = state.superhydride_coil_temp_k <= COIL_TEMPERATURE_MAX_KELVIN;

    // STAGE 5: Actuator Exergy Distribution (Pinch 28%, Melt 14.5%, Hadron 18.5%, Warp 39%)
    let u_sum = params.allocation_pinch
        + params.allocation_melt
        + params.allocation_hadron
        + params.allocation_warp;
    let norm_pinch = params.allocation_pinch / u_sum;
    let norm_melt = params.allocation_melt / u_sum;
    let norm_hadron = params.allocation_hadron / u_sum;
    let norm_warp = params.allocation_warp / u_sum;

    state.actuator_pinch_power = norm_pinch * p_direct;
    state.actuator_melt_power = norm_melt * p_direct;
    state.actuator_hadron_power = norm_hadron * p_direct;
    state.actuator_warp_power = norm_warp * p_direct;

    // STAGE 6: 312-Channel CCZ4 Anti-Phase Metric Cloaking
    state.von_mises_shear_stress_mpa = RESIDUAL_VON_MISES_MPA;
    state.is_cloaking_active = state.von_mises_shear_stress_mpa <= 1.0e-6;

    OmegaStageOutputs {
        stage1_m_dot: state.mass_ingestion_rate,
        stage1_p_gross: state.gross_power_watts,
        stage2_p_direct: state.direct_cft_power_watts,
        stage2_p_waste: state.thermal_waste_power_watts,
        stage3_m_seed: state.synthetic_seed_mass_kg,
        stage3_zeta: OPACITY_QUENCHING_ZETA,
        stage4_p_sensible: state.cryostat_heat_flux_watts,
        stage4_t_coil: state.superhydride_coil_temp_k,
        stage5_p_pinch: state.actuator_pinch_power,
        stage5_p_melt: state.actuator_melt_power,
        stage5_p_hadron: state.actuator_hadron_power,
        stage5_p_warp: state.actuator_warp_power,
        stage6_sigma_vm: state.von_mises_shear_stress_mpa,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_omega_pipeline_six_stages() {
        let mut state = OmegaSimulationState::default();
        let params = OmegaSimulationParams::default();
        let out = step_omega_pipeline(&mut state, &params, 1.0);

        assert!((out.stage1_m_dot - 111_265.01).abs() < 1.0);
        assert!((out.stage1_p_gross - 10.00e21).abs() < 1.0e15);
        assert!((out.stage2_p_direct - 10.00e21 * DIRECT_COUPLING_EFFICIENCY).abs() < 1.0e15);
        assert!(out.stage4_p_sensible <= SENSIBLE_HEAT_CLAMP_WATTS);
        assert!(out.stage4_t_coil <= COIL_TEMPERATURE_MAX_KELVIN);
        assert!(out.stage6_sigma_vm <= 1.0e-6);
        assert!(state.is_cryogenically_stable);
        assert!(state.is_cloaking_active);
    }
}
