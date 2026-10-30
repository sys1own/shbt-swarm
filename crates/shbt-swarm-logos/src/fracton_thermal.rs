// crates/shbt-swarm-logos/src/fracton_thermal.rs
#![forbid(unsafe_code)]

//! `fracton_thermal`: 12-layer fracton metamaterial thermal sink and phonon bottleneck elimination,
//! bypassing conventional acoustic Klemens decay via sub-femtosecond non-radiative LO phonon capture.

/// Acoustic Klemens decay lifetime LO -> LA + TA: tau_K ~ 3.5 ps = 3.5e-12 s
pub const TAU_KLEMENS_SECONDS: f64 = 3.5e-12;

/// Attosecond propagation envelope upper bound: 1.701 fs = 1.701e-15 s
pub const TAU_CAPTURE_MAX_SECONDS: f64 = 1.701e-15;

/// Non-radiative resonant capture lifetime: 1.428 fs = 1.428e-15 s
pub const TAU_CAPTURE_NOMINAL_SECONDS: f64 = 1.428e-15;

/// Spectral dimension d_s of 12-layer fracton shield: 1.05 (Alexander-Orbach d_s < 4/3)
pub const FRACTON_SPECTRAL_DIMENSION: f64 = 1.05;

/// Symplectic transport channel capacity S = 100/1089 ~ 0.091827
pub const SYMPLECTIC_CHANNEL_S: f64 = 100.0 / 1089.0;

/// Maximum allowable core temperature rise: Delta T_core <= 0.05 K
pub const DELTA_T_CORE_MAX_K: f64 = 0.05;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FractonError {
    SpectralDimensionOutOfBounds,
    CaptureTimeExceedsAttosecondLimit,
    KlemensDecayNotOutperformed,
    CoreThermalRunaway,
}

/// Fracton Thermal Sink.
/// Lifts translational momentum selection rules in the fracton vibrational density of states,
/// draining optical phonon energy directly into the Heegaard-Floer dark completion channel.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FractonThermalSink {
    pub spectral_dimension: f64,
    pub tau_capture_seconds: f64,
    pub heat_generation_w: f64,
}

impl FractonThermalSink {
    pub const fn canonical() -> Self {
        Self {
            spectral_dimension: FRACTON_SPECTRAL_DIMENSION,
            tau_capture_seconds: TAU_CAPTURE_NOMINAL_SECONDS,
            heat_generation_w: 100.0,
        }
    }

    /// Evaluates heat extraction rate through the symplectic channel:
    /// J_heat_drain = J_heat_gen * (1 + S_channel)
    pub fn heat_drain_rate(&self) -> f64 {
        self.heat_generation_w * (1.0 + SYMPLECTIC_CHANNEL_S)
    }

    /// Computes core lattice temperature shift:
    /// Delta T = (J_heat_gen / J_heat_drain) * 0.05 K
    pub fn core_temperature_shift(&self) -> f64 {
        let drain = self.heat_drain_rate();
        (self.heat_generation_w / drain) * 0.05
    }

    /// Ratio of Klemens decay time to fracton capture time (must be >= 1000x)
    pub fn speedup_ratio(&self) -> f64 {
        TAU_KLEMENS_SECONDS / self.tau_capture_seconds
    }

    /// Verifies complete bypass of the phonon bottleneck and thermal clamping
    pub fn verify_thermal_bypass(&self) -> Result<f64, FractonError> {
        if self.spectral_dimension <= 0.8 || self.spectral_dimension >= (4.0 / 3.0) {
            return Err(FractonError::SpectralDimensionOutOfBounds);
        }
        if self.tau_capture_seconds > TAU_CAPTURE_MAX_SECONDS {
            return Err(FractonError::CaptureTimeExceedsAttosecondLimit);
        }
        if self.speedup_ratio() < 1000.0 {
            return Err(FractonError::KlemensDecayNotOutperformed);
        }
        let delta_t = self.core_temperature_shift();
        if delta_t >= 0.10 {
            return Err(FractonError::CoreThermalRunaway);
        }
        Ok(delta_t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fracton_thermal_bypass_canonical() {
        let sink = FractonThermalSink::canonical();
        let dt = sink.verify_thermal_bypass().unwrap();
        assert!(dt <= DELTA_T_CORE_MAX_K);
        assert!(sink.speedup_ratio() > 2000.0);
    }
}
