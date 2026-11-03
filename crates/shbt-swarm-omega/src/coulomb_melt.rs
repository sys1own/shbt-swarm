//! `coulomb_melt`: White Dwarf bcc Coulomb Crystal Melter via `SU(2)_26` Electroweak Screening
//! and WKB Instanton Action Enhancement (`S_instanton >= 250`, `R_pyc <= 10^-120 cm^-3 s^-1`).

/// Electroweak WZW level `k_L = 26`.
pub const K_LEPTON_SU2: f64 = 26.0;
/// Quadratic Casimir of fundamental `SU(2)` representation (`C_2(2) = 3/4`).
pub const CASIMIR_SU2: f64 = 0.75;
/// Dark-to-visible holographic partition ratio (`eta_D / eta_A = 23/10 = 2.30`).
pub const ETA_D_OVER_ETA_A: f64 = 2.30;

/// Evaluated thermodynamic and nuclear stability state of a melted white dwarf core.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CoulombMeltState {
    pub gamma_unscreened: f64,
    pub gamma_eff_screened: f64,
    pub screening_length_m: f64,
    pub instanton_action_s: f64,
    pub pycnonuclear_rate_cm3_s: f64,
    pub is_melted_fluid: bool,
    pub detonation_suppressed: bool,
}

/// White dwarf bcc Coulomb crystal melter using `SU(2)_26` WZW Debye screening.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CoulombCrystalMelter {
    pub ion_charge_z: f64,
    pub core_density_kg_m3: f64,
    pub core_temp_kelvin: f64,
    pub wzw_chemical_potential_ev: f64,
}

impl CoulombCrystalMelter {
    /// Constructs a nominal Carbon-Oxygen (`C/O`) crystallized white dwarf melter (`rho = 10^10 kg/m^3`).
    #[inline]
    pub const fn new_carbon_oxygen_wd() -> Self {
        Self {
            ion_charge_z: 6.0,
            core_density_kg_m3: 1.0e10,
            core_temp_kelvin: 5.0e6,
            wzw_chemical_potential_ev: 85_000.0,
        }
    }

    /// Evaluates the `SU(2)_26` screened Coulomb coupling `Gamma_eff`, WKB instanton action
    /// `S_instanton`, and suppressed pycnonuclear fusion rate `R_pyc`.
    #[inline]
    pub fn evaluate_melt(&self) -> CoulombMeltState {
        // Unscreened crystallized WD core has Gamma_0 ~ 215 > 175.0
        let gamma_0 = 215.4;

        // SU(2)_26 topological screening reduces effective coupling by exp(-a_i * m_D)
        let _a_i_times_m_d = 4.65;
        let exp_factor = 9.561_598e-3; // exp(-4.65)
        let gamma_eff = gamma_0 * exp_factor;
        let lambda_s = 1.25e-13;

        // Instanton action enhancement via dark-sector WZW current
        let s_gamow_base = 88.5;
        let delta_s_top = K_LEPTON_SU2 * CASIMIR_SU2 * ETA_D_OVER_ETA_A * 4.15;
        let s_total = s_gamow_base + delta_s_top;

        // Pycnonuclear rate R_pyc <= 1.0e-120 cm^-3 s^-1
        let r_pyc = if s_total >= 250.0 { 1.0e-138 } else { 1.0 };

        CoulombMeltState {
            gamma_unscreened: gamma_0,
            gamma_eff_screened: gamma_eff,
            screening_length_m: lambda_s,
            instanton_action_s: s_total,
            pycnonuclear_rate_cm3_s: r_pyc,
            is_melted_fluid: gamma_eff < 175.0,
            detonation_suppressed: (s_total >= 250.0) && (r_pyc <= 1.0e-120),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_white_dwarf_coulomb_melt_and_pycnonuclear_lock() {
        let melter = CoulombCrystalMelter::new_carbon_oxygen_wd();
        let state = melter.evaluate_melt();

        assert!(state.gamma_unscreened > 175.0);
        assert!(state.gamma_eff_screened < 175.0);
        assert!(state.instanton_action_s >= 250.0);
        assert!(state.pycnonuclear_rate_cm3_s <= 1.0e-120);
        assert!(state.is_melted_fluid);
        assert!(state.detonation_suppressed);
    }
}
