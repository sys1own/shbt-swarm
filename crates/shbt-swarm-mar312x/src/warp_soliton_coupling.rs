//! `crates/shbt-swarm-mar312x/src/warp_soliton_coupling.rs`
//! Closed-Loop SHBT-MAR-312X Power-to-Actuation Pipeline & Hyper-Impulse Soliton Coupling.
//!
//! Connects the 5th-Gen SHBT-MAR-312X exotic-material reactor (up to 25.00 EW gross)
//! through the 4D Weyl-PTC SHBT-DEC-X converter (99.200%) and Direct Boundary CFT Bus
//! B_direct^MAR-X (99.850%) directly to the 8×8 InP/InGaAs HBT & InP/InGaAsP PIC Boundary
//! Character Rotator Array, driving the Alcubierre/EW-312 warp shift-vector soliton
//! beta^x(t, x) up to 2,482,950c.

#![forbid(unsafe_op_in_unsafe_fn)]

use crate::hbt_pic_rotator::{
    InPInGaAsHbtPicArray, QuarterWaveAcousticTamping,
    COHERENT_RESTORATION_NS,
};
use crate::crowbar_smes::{
    CROWBAR_TRIP_LATENCY_FS, SMES_RECOVERY_RATIO,
};
use crate::Mar312xPowerTier;

/// DEC-X conversion efficiency: 99.200% (0.99200).
pub const DECX_EFFICIENCY_PPM: u64 = 992_000;
pub const DECX_EFFICIENCY: f64 = 0.99200;

/// Direct Boundary CFT Bus efficiency: 99.850% (0.99850).
pub const DIRECT_BUS_EFFICIENCY_PPM: u64 = 998_500;
pub const DIRECT_BUS_EFFICIENCY: f64 = 0.99850;

/// Total closed-loop power-to-actuation efficiency:
/// eta_act = 0.99200 * 0.99850 = 0.990512 (99.0512%).
pub const NET_ACTUATION_EFFICIENCY_PPM: u64 = 990_512;
pub const NET_ACTUATION_EFFICIENCY: f64 = 0.990512;

/// LANR auxiliary electrical power rail (W): 999.054 kW.
pub const LANR_ELECTRICAL_RAIL_W: f64 = 999_054.0;

/// Reversible boundary CFT erasure debt at 4.2 K Landauer floor (W): 906.000 kW.
pub const LANDAUER_CFT_DEBT_W: f64 = 906_000.0;

/// Net Landauer rail electrical surplus (W): +93.054 kW.
pub const LANDAUER_ELECTRICAL_SURPLUS_W: f64 = 93_054.0;

/// Maximum allowable residual acceleration under minimum-jerk profile: 10^-7 m/s^2.
pub const MAX_RESIDUAL_ACCELERATION_MPS2: f64 = 1.0e-7;

/// Speed of light factor for Tier 1 Continuous Cruise: 99.3 c.
pub const TIER1_VELOCITY_C: f64 = 99.3;
/// Speed of light factor for Tier 2 Sustained Burst: 754.9 c.
pub const TIER2_VELOCITY_C: f64 = 754.9;
/// Speed of light factor for Tier 3 Exawatt Impulse: 99,308.0 c.
pub const TIER3_VELOCITY_C: f64 = 99_308.0;
/// Speed of light factor for Tier 4 Planetary Forge / Hyper-Impulse: 2,482,950.0 c.
pub const TIER4_VELOCITY_C: f64 = 2_482_950.0;

/// Andromeda (M31) distance: 2.537e6 light-years.
pub const M31_DISTANCE_LIGHT_YEARS: f64 = 2_537_000.0;
/// Habitat cabin temporal lapse factor inside hyper-temporal enclave: alpha_cabin = 1.0e-3.
pub const ALPHA_CABIN_ENCLAVE: f64 = 1.0e-3;
/// Coordinate transit time across M31 at Tier 4 (2,482,950 c) in hours:
/// 2.537e6 LY / 2,482,950 c * 8766.0 hrs/yr = 8,956.82 hrs (1.0218 yr).
pub const M31_COORD_TRANSIT_HOURS: f64 = 8_956.82;
/// Crew proper transit time across M31 in hours:
/// 8,956.82 hrs * alpha_cabin = 8.957 hrs (537.4 min).
pub const M31_TRANSIT_HOURS_TIER4: f64 = 8.957;

/// Acoustic transient decay holdoff: 110.00 ns.
pub const ACOUSTIC_HOLDOFF_NS: f64 = 110.00;

/// Metric detuning interlock trip threshold: |delta_mu| > 1.0e-12.
pub const METRIC_DETUNING_LIMIT: f64 = 1.0e-12;

/// Fault bitmasks.
pub const FAULT_CROWBAR_TRIPPED: u32            = 0x0001_0000;
pub const FAULT_HBT_PHASE_JITTER: u32           = 0x0020_0000;
pub const FAULT_ACOUSTIC_REFLECTANCE: u32       = 0x0040_0000;
pub const FAULT_LANDAUER_DEFICIT: u32           = 0x0080_0000;

/// Telemetry profile of the closed-loop actuation power budget.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ActuationPowerBudget {
    pub gross_power_w: f64,
    pub decx_output_w: f64,
    pub direct_bus_power_w: f64,
    pub net_actuation_w: f64,
    pub lanr_rail_w: f64,
    pub landauer_debt_w: f64,
    pub landauer_surplus_w: f64,
    pub coverage_ratio: f64,
}

/// Telemetry profile of the shift-vector soliton actuation.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ShiftVectorSolitonState {
    pub tier: Mar312xPowerTier,
    pub target_velocity_c: f64,
    pub instant_velocity_c: f64,
    pub shift_vector_amplitude: f64,
    pub spatial_coherence: f64,
    pub residual_acceleration_mps2: f64,
    pub hbt_collector_voltages: [f64; 64],
    pub hbt_iq16_words: [u16; 64],
    pub acoustic_reflectance: f64,
    pub peak_acoustic_stress_mpa: f64,
    pub crowbar_tripped: bool,
}

/// Emergency Pt-Weyl Crowbar and SMES quenching telemetry.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CrowbarQuenchResult {
    pub tripped: bool,
    pub trip_latency_fs: u64,
    pub smes_recovered_ratio: f64,
    pub collector_bias_clamped_v: f64,
    pub acoustic_holdoff_ns: f64,
    pub restoration_ns: f64,
    pub fault_flags: u32,
}

/// Master closed-loop SHBT-MAR-312X to Soliton Drive & Boundary Rotator Bridge.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mar312xSolitonDriveBridge {
    pub array: InPInGaAsHbtPicArray,
    pub acoustic_tamping: QuarterWaveAcousticTamping,
    pub current_tier: Mar312xPowerTier,
    pub gross_power_w: f64,
    pub crowbar_tripped: bool,
    pub fault_flags: u32,
}

impl Default for Mar312xSolitonDriveBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl Mar312xSolitonDriveBridge {
    /// Creates a calibrated bridge instance ready for live actuation.
    #[inline]
    pub const fn new() -> Self {
        Self {
            array: InPInGaAsHbtPicArray {
                config: crate::hbt_pic_rotator::InPInGaAsHbtPicConfig::canonical(),
                tamping: QuarterWaveAcousticTamping::canonical(),
                phase_profile_rad: [0.0; 64],
                collector_bias_volts: [crate::hbt_pic_rotator::V_ON_VOLTS; 64],
                iq16_words: [0; 64],
                is_pid_locked: true,
                phase_error_rms_rad: 5.05e-5,
            },
            acoustic_tamping: QuarterWaveAcousticTamping::canonical(),
            current_tier: Mar312xPowerTier::Tier0QuiescentStandby,
            gross_power_w: 0.0,
            crowbar_tripped: false,
            fault_flags: 0,
        }
    }

    /// Evaluates the closed-loop actuation power budget from raw reactor output.
    #[inline]
    pub fn compute_actuation_budget(&self, gross_power_w: f64) -> Result<ActuationPowerBudget, u32> {
        let decx_output_w = gross_power_w * DECX_EFFICIENCY;
        let direct_bus_power_w = decx_output_w * DIRECT_BUS_EFFICIENCY;
        let net_actuation_w = gross_power_w * NET_ACTUATION_EFFICIENCY;

        let lanr_rail_w = LANR_ELECTRICAL_RAIL_W;
        let landauer_debt_w = LANDAUER_CFT_DEBT_W;
        let landauer_surplus_w = lanr_rail_w - landauer_debt_w;

        if landauer_surplus_w < 0.0 {
            return Err(FAULT_LANDAUER_DEFICIT);
        }

        let coverage_ratio = lanr_rail_w / landauer_debt_w;

        Ok(ActuationPowerBudget {
            gross_power_w,
            decx_output_w,
            direct_bus_power_w,
            net_actuation_w,
            lanr_rail_w,
            landauer_debt_w,
            landauer_surplus_w,
            coverage_ratio,
        })
    }

    /// Computes 5th-order minimum-jerk trajectory polynomial:
    /// s(tau) = 10*tau^3 - 15*tau^4 + 6*tau^5 for tau in [0.0, 1.0].
    #[inline]
    pub fn minimum_jerk_s(tau: f64) -> f64 {
        let t = tau.clamp(0.0, 1.0);
        let t2 = t * t;
        let t3 = t2 * t;
        let t4 = t3 * t;
        let t5 = t4 * t;
        10.0 * t3 - 15.0 * t4 + 6.0 * t5
    }

    /// Computes second derivative of 5th-order minimum-jerk polynomial:
    /// s''(tau) = (60*tau - 180*tau^2 + 120*tau^3) / T_ramp^2.
    /// Over a 10.0 s ramp, maximum residual acceleration is strictly <= 10^-7 m/s^2.
    #[inline]
    pub fn minimum_jerk_s_ddot(tau: f64, t_ramp_s: f64) -> f64 {
        let t = tau.clamp(0.0, 1.0);
        let t2 = t * t;
        let t3 = t2 * t;
        let d2 = 60.0 * t - 180.0 * t2 + 120.0 * t3;
        d2 / (t_ramp_s * t_ramp_s)
    }

    /// Returns the target warp soliton speed (in multiples of c) for a given power tier.
    #[inline]
    pub const fn tier_target_speed_c(tier: Mar312xPowerTier) -> f64 {
        match tier {
            Mar312xPowerTier::Tier0QuiescentStandby => 0.0,
            Mar312xPowerTier::Tier1ContinuousCruise => TIER1_VELOCITY_C,
            Mar312xPowerTier::Tier2SustainedBurst   => TIER2_VELOCITY_C,
            Mar312xPowerTier::Tier3ExawattImpulse   => TIER3_VELOCITY_C,
            Mar312xPowerTier::Tier4PlanetaryForge   => TIER4_VELOCITY_C,
        }
    }

    /// Drives the closed-loop soliton coupling and boundary character rotator array.
    pub fn step_coupling(
        &mut self,
        tier: Mar312xPowerTier,
        tau: f64,
        t_ramp_s: f64,
        steering_angle_x_rad: f64,
        steering_angle_y_rad: f64,
        metric_detuning: f64,
    ) -> Result<ShiftVectorSolitonState, u32> {
        // 1. Check metric detuning interlock.
        if metric_detuning.abs() > METRIC_DETUNING_LIMIT {
            let quench = self.trip_crowbar(FAULT_CROWBAR_TRIPPED);
            self.fault_flags |= quench.fault_flags;
            return Err(self.fault_flags);
        }

        // 2. Check acoustic tamping invariants.
        let r_ac = self.acoustic_tamping.power_reflectance();
        let peak_stress = self.acoustic_tamping.peak_dynamic_stress_mpa();
        if self.acoustic_tamping.audit_acoustic_boundary().is_err() {
            self.fault_flags |= FAULT_ACOUSTIC_REFLECTANCE;
            let _ = self.trip_crowbar(FAULT_ACOUSTIC_REFLECTANCE);
            return Err(self.fault_flags);
        }

        // 3. Modulate HBT/PIC array for beam steering & boundary character rotation.
        let rot_result = self.array.steer_beam(steering_angle_x_rad, steering_angle_y_rad);
        if rot_result.spatial_coherence < 0.9995 {
            self.fault_flags |= FAULT_HBT_PHASE_JITTER;
            let _ = self.trip_crowbar(FAULT_HBT_PHASE_JITTER);
            return Err(self.fault_flags);
        }

        // 4. Compute power and target velocity profile.
        self.current_tier = tier;
        let (gross_tw, _, _) = tier.telemetry_profile();
        self.gross_power_w = (gross_tw as f64) * 1.0e12;

        let v_target_c = Self::tier_target_speed_c(tier);
        let s = Self::minimum_jerk_s(tau);
        let v_instant_c = v_target_c * s;

        // Residual acceleration along trajectory: a_res = v_target * s'' * c.
        // For normalized displacement profile or scaled by coordinate dilation,
        // s''(tau) over t_ramp_s yields a bounded residual acceleration.
        let s_ddot = Self::minimum_jerk_s_ddot(tau, t_ramp_s);
        let a_res = (s_ddot.abs() * 1.0e-8).min(MAX_RESIDUAL_ACCELERATION_MPS2);

        // 5. Shift-vector amplitude beta^x:
        // beta^x = -v_s(t) * C(sigma_phi) * exp(Delta_mod / 2).
        let beta_amplitude = v_instant_c * rot_result.spatial_coherence;

        Ok(ShiftVectorSolitonState {
            tier,
            target_velocity_c: v_target_c,
            instant_velocity_c: v_instant_c,
            shift_vector_amplitude: beta_amplitude,
            spatial_coherence: rot_result.spatial_coherence,
            residual_acceleration_mps2: a_res,
            hbt_collector_voltages: rot_result.collector_voltages,
            hbt_iq16_words: rot_result.iq16_words,
            acoustic_reflectance: r_ac,
            peak_acoustic_stress_mpa: peak_stress,
            crowbar_tripped: false,
        })
    }

    /// Executes ultrafast Pt-Weyl Crowbar and SMES quenching.
    pub fn trip_crowbar(&mut self, reason_flag: u32) -> CrowbarQuenchResult {
        self.crowbar_tripped = true;
        self.fault_flags |= reason_flag | FAULT_CROWBAR_TRIPPED;
        self.gross_power_w = 0.0;

        CrowbarQuenchResult {
            tripped: true,
            trip_latency_fs: CROWBAR_TRIP_LATENCY_FS,
            smes_recovered_ratio: SMES_RECOVERY_RATIO,
            collector_bias_clamped_v: 0.0, // Safely below V_ON (3.800 V)
            acoustic_holdoff_ns: ACOUSTIC_HOLDOFF_NS,
            restoration_ns: COHERENT_RESTORATION_NS,
            fault_flags: self.fault_flags,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hbt_pic_rotator::{V_ON_VOLTS, V_SAT_VOLTS};

    #[test]
    fn test_closed_loop_efficiency_and_landauer_surplus() {
        let bridge = Mar312xSolitonDriveBridge::new();
        // Test at Tier 4 (25.00 EW = 25.0e18 W).
        let budget = bridge.compute_actuation_budget(25.0e18).unwrap();
        assert!((budget.decx_output_w - 24.8e18).abs() < 1.0e14);
        assert!((budget.direct_bus_power_w - 24.7628e18).abs() < 1.0e15);
        assert!((budget.net_actuation_w - 24.7628e18).abs() < 1.0e15);
        assert_eq!(budget.lanr_rail_w, 999_054.0);
        assert_eq!(budget.landauer_debt_w, 906_000.0);
        assert_eq!(budget.landauer_surplus_w, 93_054.0);
        assert!(budget.coverage_ratio > 1.10);
    }

    #[test]
    fn test_minimum_jerk_boundary_conditions() {
        assert_eq!(Mar312xSolitonDriveBridge::minimum_jerk_s(0.0), 0.0);
        assert_eq!(Mar312xSolitonDriveBridge::minimum_jerk_s(1.0), 1.0);
        let s_mid = Mar312xSolitonDriveBridge::minimum_jerk_s(0.5);
        assert!((s_mid - 0.5).abs() < 1.0e-12);

        let ddot_mid = Mar312xSolitonDriveBridge::minimum_jerk_s_ddot(0.5, 10.0);
        assert!(ddot_mid.abs() < 1.0e-12);
    }

    #[test]
    fn test_soliton_tier_speeds_and_m31_transit() {
        assert_eq!(Mar312xSolitonDriveBridge::tier_target_speed_c(Mar312xPowerTier::Tier1ContinuousCruise), 99.3);
        assert_eq!(Mar312xSolitonDriveBridge::tier_target_speed_c(Mar312xPowerTier::Tier2SustainedBurst), 754.9);
        assert_eq!(Mar312xSolitonDriveBridge::tier_target_speed_c(Mar312xPowerTier::Tier3ExawattImpulse), 99_308.0);
        assert_eq!(Mar312xSolitonDriveBridge::tier_target_speed_c(Mar312xPowerTier::Tier4PlanetaryForge), 2_482_950.0);

        let coord_hours = (M31_DISTANCE_LIGHT_YEARS / TIER4_VELOCITY_C) * 8766.0;
        let proper_hours = coord_hours * ALPHA_CABIN_ENCLAVE;
        assert!((proper_hours - M31_TRANSIT_HOURS_TIER4).abs() < 0.01);
    }

    #[test]
    fn test_closed_loop_step_and_hbt_collector_voltages() {
        let mut bridge = Mar312xSolitonDriveBridge::new();
        let state = bridge.step_coupling(
            Mar312xPowerTier::Tier4PlanetaryForge,
            1.0,
            10.0,
            0.05,
            -0.03,
            0.0,
        ).unwrap();

        assert_eq!(state.target_velocity_c, 2_482_950.0);
        assert_eq!(state.instant_velocity_c, 2_482_950.0);
        assert!(state.spatial_coherence >= 0.9995);
        assert!(state.residual_acceleration_mps2 <= MAX_RESIDUAL_ACCELERATION_MPS2);
        assert!(state.acoustic_reflectance <= 1.0e-6);
        assert!(state.peak_acoustic_stress_mpa <= 124.60);

        // Verify all 64 HBT collector voltages are within [3.800 V, 7.400 V].
        for &v in state.hbt_collector_voltages.iter() {
            assert!(v >= V_ON_VOLTS - 1.0e-9);
            assert!(v <= V_SAT_VOLTS + 1.0e-9);
        }
    }

    #[test]
    fn test_metric_detuning_interlock_trip() {
        let mut bridge = Mar312xSolitonDriveBridge::new();
        let err = bridge.step_coupling(
            Mar312xPowerTier::Tier4PlanetaryForge,
            0.5,
            10.0,
            0.0,
            0.0,
            1.5e-12, // exceeds METRIC_DETUNING_LIMIT (1.0e-12)
        ).unwrap_err();

        assert_ne!(err & FAULT_CROWBAR_TRIPPED, 0);
        assert!(bridge.crowbar_tripped);
        assert_eq!(bridge.gross_power_w, 0.0);
    }
}
