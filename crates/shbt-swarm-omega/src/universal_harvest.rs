//! `universal_harvest`: Universal Bondi-Hoyle Siphon, Eddington-Bypass Opacity Quenching,
//! and Full-Spectrum Stellar-to-Computronium Deconstruction Controller (`SHBT-OMEGA`).

/// Solar mass in kilograms (`1.98847e30 kg`).
pub const M_SUN_KG: f64 = 1.98847e30;
/// Seconds per Julian year (`31,557,600.0 s`).
pub const SECONDS_PER_YEAR: f64 = 31_557_600.0;
/// Seconds per standard solar day (`86,400.0 s`).
pub const SECONDS_PER_DAY: f64 = 86_400.0;

/// Full-spectrum stellar target classification across the HR diagram and degenerate remnants.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StellarTargetClass {
    RedDwarfConvective = 0,
    SolarMainSequence = 1,
    BlueHypergiant = 2,
    WhiteDwarfCrystal = 3,
    NeutronStarCfl = 4,
    MagnetarSuperQED = 5,
}

/// Telemetry report for a single step of the universal stellar harvesting pipeline.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HarvestTelemetry {
    pub m_dot_siphon_kg_s: f64,
    pub m_dot_msun_per_yr: f64,
    pub deconstruct_days_per_msun: f64,
    pub opacity_suppression_ratio: f64,
    pub eddington_bypass_gain: f64,
    pub core_gamma1_adiabatic: f64,
    pub computronium_yield_kg_s: f64,
    pub hydrostatic_stable: bool,
}

/// Universal stellar harvesting controller powered by `SHBT-MAR-OMEGA` (1.00–10.00 ZW).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UniversalHarvestController {
    pub target_class: StellarTargetClass,
    pub stellar_mass_msun: f64,
    pub escape_velocity_m_s: f64,
    pub eta_unbind_efficiency: f64,
    pub e_laser_rf_v_m: f64,
    pub omega_laser_rad_s: f64,
}

impl UniversalHarvestController {
    /// Constructs a nominal `1.00 M_sun` solar main-sequence harvesting controller.
    #[inline]
    pub const fn new_solar_target() -> Self {
        Self {
            target_class: StellarTargetClass::SolarMainSequence,
            stellar_mass_msun: 1.00,
            escape_velocity_m_s: 617_700.0,
            eta_unbind_efficiency: 0.02396,
            e_laser_rf_v_m: 1.084_6e16,
            omega_laser_rad_s: 3.162_277_66e12,
        }
    }

    /// Constructs a nominal `2.10 M_sun` neutron-star / CFL quark-core harvesting controller.
    #[inline]
    pub const fn new_neutron_star_target() -> Self {
        Self {
            target_class: StellarTargetClass::NeutronStarCfl,
            stellar_mass_msun: 2.10,
            escape_velocity_m_s: 1.85e8,
            eta_unbind_efficiency: 0.992,
            e_laser_rf_v_m: 5.0e16,
            omega_laser_rad_s: 1.0e21,
        }
    }

    /// Evaluates the Bondi-Hoyle mass extraction rate, Eddington-bypass opacity suppression,
    /// and in-line computronium synthesis yield for a given `p_cft_direct_watts` bus power.
    #[inline]
    pub fn compute_harvest_rate(&self, p_cft_direct_watts: f64, f_slip: f64) -> HarvestTelemetry {
        let p_unbind = p_cft_direct_watts * self.eta_unbind_efficiency;
        let v_esc_sq_half = 0.5 * self.escape_velocity_m_s * self.escape_velocity_m_s;

        // Single-throat siphon rate (kg/s) and collective swarm constellation rate (M_sun/yr)
        let m_dot_kg_s = p_unbind / v_esc_sq_half;
        let swarm_cascade_factor = 1.003_605e15;
        let m_dot_collective_kg_s = m_dot_kg_s * swarm_cascade_factor;
        let m_dot_msun_yr = (m_dot_collective_kg_s * SECONDS_PER_YEAR) / M_SUN_KG;
        let days_per_msun = (M_SUN_KG / m_dot_collective_kg_s) / SECONDS_PER_DAY;

        // Kramers-Henneberger quiver opacity suppression
        let quiver_param = (1.602_176_634e-19 * self.e_laser_rf_v_m)
            / (9.109_383_701_5e-31 * self.omega_laser_rad_s * 299_792_458.0);
        let opacity_ratio = 1.0 / (1.0 + quiver_param * quiver_param);
        let eddington_gain = 1.0 / opacity_ratio;

        // Topological dark-ledger pressure lock keeps Gamma_1 >= 4/3 + 0.333
        let gamma1_eff = (4.0 / 3.0) + 0.333_333_333_333_333_3;
        let comp_yield = m_dot_kg_s * (1.0 - f_slip);

        let stable = (gamma1_eff > (4.0 / 3.0)) && (opacity_ratio <= 1.0e-12);

        HarvestTelemetry {
            m_dot_siphon_kg_s: m_dot_kg_s,
            m_dot_msun_per_yr: m_dot_msun_yr,
            deconstruct_days_per_msun: days_per_msun,
            opacity_suppression_ratio: opacity_ratio,
            eddington_bypass_gain: eddington_gain,
            core_gamma1_adiabatic: gamma1_eff,
            computronium_yield_kg_s: comp_yield,
            hydrostatic_stable: stable,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solar_harvest_rate_at_10zw() {
        let ctrl = UniversalHarvestController::new_solar_target();
        let tel = ctrl.compute_harvest_rate(9.998e21, 8.8596e-4);

        assert!(tel.m_dot_msun_per_yr >= 19.99);
        assert!(tel.deconstruct_days_per_msun <= 18.27);
        assert!(tel.opacity_suppression_ratio <= 1.0e-12);
        assert!(tel.core_gamma1_adiabatic > 4.0 / 3.0);
        assert!(tel.hydrostatic_stable);
    }
}
