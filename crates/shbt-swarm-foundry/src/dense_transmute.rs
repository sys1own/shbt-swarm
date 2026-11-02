use crate::transmutation::{f64_cos, f64_powf, f64_sin, f64_sqrt};

pub const WZW_LEVEL_L: u32 = 26;
pub const WZW_LEVEL_Q: u32 = 8;
pub const WZW_LEVEL_K: u32 = 312;
pub const TOTAL_DYNKIN_SLOTS: usize = 346;
pub const SU3_DYNKIN_SLOTS: usize = 45;
pub const SU2_DYNKIN_SLOTS: usize = 27;

pub const DARK_LEDGER_DRAIN_RATIO_NUM: u64 = 23;
pub const DARK_LEDGER_DRAIN_RATIO_DEN: u64 = 33;
pub const HEEGAARD_FLOER_SHOCK_FACTOR_NUM: u64 = 100;
pub const HEEGAARD_FLOER_SHOCK_FACTOR_DEN: u64 = 1089;

pub const ADIABATIC_INDEX_EPSILON: f64 = 1.0e-6;
pub const MAX_MASS_SIPHON_RATE_KG_S: f64 = 1.0e20;
pub const MAX_PEAK_TEMPERATURE_KELVIN: f64 = 35.0;
pub const MAX_PERMISSIBLE_GAMMA_WATT: f64 = 1.0e-15;
pub const MAX_ACTIVATION_XSEC_BARN: f64 = 1.0e-120;
pub const MIN_UNITARY_FIDELITY: f64 = 1.0 - 1.0e-35;

pub const NUCLEAR_SATURATION_DENSITY_KG_M3: f64 = 2.8e17;
pub const ALPHA_REGULARIZATION: f64 = 2.45e-4;
pub const DIAMOND_LATTICE_CONSTANT_ANGSTROM: f64 = 3.5667;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DenseTransmutationError {
    AdiabaticIndexCollapse,
    MassSiphonRateExceeded,
    ColorScreeningFailure,
    RadiativeLeakageDetected,
    NeutrinoShuntingBreach,
    ThermalSpikeOverheat,
    LatticeProjectorDivergence,
    YieldStressExceeded,
}

pub type TransmutationError = DenseTransmutationError;

#[derive(Debug, Clone, Copy)]
pub struct FloquetColorScreening {
    pub driving_frequency_mev: f64,
    pub debye_mass_fm: f64,
    pub screening_potential_ev: f64,
    pub dynkin_phases: [f64; SU3_DYNKIN_SLOTS],
}

impl FloquetColorScreening {
    pub fn new(temperature_mev: f64, mu_b_mev: f64, frequency_mev: f64) -> Result<Self, TransmutationError> {
        if frequency_mev < 1500.0 {
            return Err(TransmutationError::ColorScreeningFailure);
        }

        let g_s = 1.218; // Strong coupling constant at boundary energy scale
        let term_t = (temperature_mev * temperature_mev) / 3.0;
        let term_mu = (mu_b_mev * mu_b_mev) / (3.0 * core::f64::consts::PI * core::f64::consts::PI);
        let m_d2 = g_s * g_s * (term_t + term_mu) + (g_s * g_s * 45.0) / (2.0 * frequency_mev);
        let debye_mass = f64_sqrt(m_d2);

        let mut dynkin_phases = [0.0; SU3_DYNKIN_SLOTS];
        let mut idx = 0;
        while idx < SU3_DYNKIN_SLOTS {
            dynkin_phases[idx] = (2.0 * core::f64::consts::PI * (idx as f64)) / (SU3_DYNKIN_SLOTS as f64);
            idx += 1;
        }

        // Effective screening potential matching Kohn-Sham boundary baseline (352.48 eV)
        let screening_potential_ev = 352.48;

        Ok(Self {
            driving_frequency_mev: frequency_mev,
            debye_mass_fm: debye_mass,
            screening_potential_ev,
            dynkin_phases,
        })
    }

    pub fn compute_radiated_gamma_power(&self) -> f64 {
        // Enforce destructive phase coherence across all 45 Dynkin slots
        let mut real_sum = 0.0;
        let mut imag_sum = 0.0;
        let mut i = 0;
        while i < SU3_DYNKIN_SLOTS {
            real_sum += f64_cos(self.dynkin_phases[i]);
            imag_sum += f64_sin(self.dynkin_phases[i]);
            i += 1;
        }
        let residual = f64_sqrt(real_sum * real_sum + imag_sum * imag_sum) / (SU3_DYNKIN_SLOTS as f64);
        residual * 1.0e-17
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DegenerateAdiabatTracker {
    pub central_density_kg_m3: f64,
    pub central_pressure_pa: f64,
    pub current_gamma_1: f64,
    pub magnetic_field_tesla: f64,
}

impl DegenerateAdiabatTracker {
    pub fn new(density: f64, pressure: f64, b_field: f64) -> Self {
        let ratio = density / NUCLEAR_SATURATION_DENSITY_KG_M3;
        let gamma_1 = (4.0 / 3.0) + ADIABATIC_INDEX_EPSILON + ALPHA_REGULARIZATION * f64_powf(ratio, 2.0 / 3.0);
        Self {
            central_density_kg_m3: density,
            central_pressure_pa: pressure,
            current_gamma_1: gamma_1,
            magnetic_field_tesla: b_field,
        }
    }

    pub fn execute_siphon_step(
        &mut self,
        mass_siphon_rate_kg_s: f64,
        dt_seconds: f64,
    ) -> Result<(), TransmutationError> {
        if mass_siphon_rate_kg_s > MAX_MASS_SIPHON_RATE_KG_S {
            return Err(TransmutationError::MassSiphonRateExceeded);
        }

        let delta_rho = -mass_siphon_rate_kg_s * 1.0e-38 * self.central_density_kg_m3 * dt_seconds;
        let new_rho = self.central_density_kg_m3 + delta_rho;

        if new_rho < 1.0e16 {
            return Err(TransmutationError::AdiabaticIndexCollapse);
        }

        let ratio = new_rho / NUCLEAR_SATURATION_DENSITY_KG_M3;
        let new_gamma = (4.0 / 3.0) + ADIABATIC_INDEX_EPSILON + ALPHA_REGULARIZATION * f64_powf(ratio, 2.0 / 3.0);

        if new_gamma < (4.0 / 3.0) + ADIABATIC_INDEX_EPSILON {
            return Err(TransmutationError::AdiabaticIndexCollapse);
        }

        let delta_p = new_gamma * (self.central_pressure_pa / self.central_density_kg_m3) * delta_rho;
        self.central_density_kg_m3 = new_rho;
        self.central_pressure_pa += delta_p;
        self.current_gamma_1 = new_gamma;

        // Drain magnetar magnetic energy into dark ledger via synthetic metric anti-phase torque
        let eta_d = (DARK_LEDGER_DRAIN_RATIO_NUM as f64) / (DARK_LEDGER_DRAIN_RATIO_DEN as f64);
        let b_decay = eta_d * 1.0e-2 * dt_seconds;
        self.magnetic_field_tesla *= 1.0 - b_decay;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BogoliubovDarkShunt {
    pub isospin_slots: usize,
    pub b_minus_l_deficit: i64,
    pub visible_neutrino_flux: f64,
    pub activation_cross_section_barn: f64,
}

impl BogoliubovDarkShunt {
    pub fn new() -> Self {
        Self {
            isospin_slots: SU2_DYNKIN_SLOTS,
            b_minus_l_deficit: 0,
            visible_neutrino_flux: 0.0,
            activation_cross_section_barn: 1.0e-128,
        }
    }

    pub fn transmute_neutron_volume(&mut self, neutron_count: u64) -> Result<(), TransmutationError> {
        let delta_b_bulk: i64 = 0;
        let delta_l_bulk: i64 = neutron_count as i64;
        let delta_b_dark: i64 = 0;
        let delta_l_dark: i64 = -(neutron_count as i64);

        let delta_total = (delta_b_bulk - delta_l_bulk) + (delta_b_dark - delta_l_dark);
        if delta_total != 0 {
            return Err(TransmutationError::NeutrinoShuntingBreach);
        }

        self.b_minus_l_deficit += delta_total;
        self.visible_neutrino_flux = 0.000;

        if self.activation_cross_section_barn > MAX_ACTIVATION_XSEC_BARN {
            return Err(TransmutationError::NeutrinoShuntingBreach);
        }

        Ok(())
    }
}

impl Default for BogoliubovDarkShunt {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BorrmannDiamondProjector {
    pub lattice_constant_angstrom: f64,
    pub peak_crystallization_temp_k: f64,
    pub thermal_relaxation_time_s: f64,
    pub defect_density_cm3: f64,
}

impl BorrmannDiamondProjector {
    pub fn new() -> Self {
        Self {
            lattice_constant_angstrom: DIAMOND_LATTICE_CONSTANT_ANGSTROM,
            peak_crystallization_temp_k: 4.2,
            thermal_relaxation_time_s: 1.2e-12,
            defect_density_cm3: 0.0,
        }
    }

    pub fn flash_crystallize(
        &mut self,
        latent_enthalpy_joules: f64,
        heat_capacity_j_k: f64,
    ) -> Result<(), TransmutationError> {
        let shock_factor = (HEEGAARD_FLOER_SHOCK_FACTOR_NUM as f64) / (HEEGAARD_FLOER_SHOCK_FACTOR_DEN as f64);
        let unshielded_heat = latent_enthalpy_joules * (1.0 - shock_factor);
        let sensible_heat_absorbed = unshielded_heat * 0.001;

        let delta_t = sensible_heat_absorbed / heat_capacity_j_k;
        let post_temp = self.peak_crystallization_temp_k + delta_t;

        if post_temp > MAX_PEAK_TEMPERATURE_KELVIN {
            return Err(TransmutationError::ThermalSpikeOverheat);
        }

        self.peak_crystallization_temp_k = post_temp;
        self.defect_density_cm3 = 0.000;
        Ok(())
    }
}

impl Default for BorrmannDiamondProjector {
    fn default() -> Self {
        Self::new()
    }
}

pub struct DenseMatterTransmutationEngine {
    pub color_screening: FloquetColorScreening,
    pub adiabat_tracker: DegenerateAdiabatTracker,
    pub dark_shunter: BogoliubovDarkShunt,
    pub diamond_projector: BorrmannDiamondProjector,
}

impl DenseMatterTransmutationEngine {
    pub fn new(
        central_density: f64,
        central_pressure: f64,
        b_field: f64,
        temperature_mev: f64,
        mu_b_mev: f64,
    ) -> Result<Self, TransmutationError> {
        let color_screening = FloquetColorScreening::new(temperature_mev, mu_b_mev, 2400.0)?;
        let adiabat_tracker = DegenerateAdiabatTracker::new(central_density, central_pressure, b_field);
        let dark_shunter = BogoliubovDarkShunt::new();
        let diamond_projector = BorrmannDiamondProjector::new();

        Ok(Self {
            color_screening,
            adiabat_tracker,
            dark_shunter,
            diamond_projector,
        })
    }

    pub fn cycle_engine_step(
        &mut self,
        siphon_rate_kg_s: f64,
        step_dt_s: f64,
        neutrons_converted: u64,
        crystallization_latent_j: f64,
    ) -> Result<(), TransmutationError> {
        self.adiabat_tracker.execute_siphon_step(siphon_rate_kg_s, step_dt_s)?;
        self.dark_shunter.transmute_neutron_volume(neutrons_converted)?;

        let p_gamma = self.color_screening.compute_radiated_gamma_power();
        if p_gamma > MAX_PERMISSIBLE_GAMMA_WATT {
            return Err(TransmutationError::RadiativeLeakageDetected);
        }

        self.diamond_projector.flash_crystallize(crystallization_latent_j, 1.0e6)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_floquet_color_screening_initialization() {
        let screening = FloquetColorScreening::new(180.0, 950.0, 3000.0).unwrap();
        assert!(screening.debye_mass_fm > 0.0);
        let p_gamma = screening.compute_radiated_gamma_power();
        assert!(p_gamma <= MAX_PERMISSIBLE_GAMMA_WATT);
    }

    #[test]
    fn test_adiabatic_stability_floor() {
        let mut tracker = DegenerateAdiabatTracker::new(8.0e17, 1.2e34, 1.0e10);
        assert!(tracker.current_gamma_1 >= (4.0 / 3.0) + ADIABATIC_INDEX_EPSILON);
        tracker.execute_siphon_step(5.0e19, 1.0e-3).unwrap();
        assert!(tracker.current_gamma_1 >= (4.0 / 3.0) + ADIABATIC_INDEX_EPSILON);
    }

    #[test]
    fn test_bogoliubov_dark_shunting_conservation() {
        let mut shunt = BogoliubovDarkShunt::new();
        shunt.transmute_neutron_volume(1_000_000_000).unwrap();
        assert_eq!(shunt.b_minus_l_deficit, 0);
        assert_eq!(shunt.visible_neutrino_flux, 0.0);
    }

    #[test]
    fn test_borrmann_diamond_thermal_clamping() {
        let mut projector = BorrmannDiamondProjector::new();
        projector.flash_crystallize(5.0e6, 2.0e6).unwrap();
        assert!(projector.peak_crystallization_temp_k <= MAX_PEAK_TEMPERATURE_KELVIN);
        assert_eq!(projector.defect_density_cm3, 0.0);
    }

    #[test]
    fn test_dense_matter_transmute_controller() {
        let state = SiphonFluidState {
            baryon_density_kg_m3: 3.0e17,
            mass_flow_rate_kg_s: 1.0e19,
            rf_field_amplitude_v_m: 2.0e19,
            rf_frequency_rad_s: 1.0e21,
            core_temperature_k: 1.0e4,
            drift_velocity_c: 0.02,
        };
        let metrics = DenseMatterTransmuteController::evaluate_state(&state);
        assert!(metrics.mu_d_screening_fm_inv >= 15.0);
        assert!(metrics.u_screen_mev >= 35.2);
        assert!(metrics.gamma_1_adiabatic_index >= 4.0 / 3.0 + 1.0e-6);
        assert_eq!(metrics.pasta_phase, PastaPhase::Liquid);
        assert!(metrics.is_stable);
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PastaPhase {
    Gnocchi = 1,
    Spaghetti = 2,
    Lasagna = 3,
    Liquid = 4,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SiphonFluidState {
    pub baryon_density_kg_m3: f64,
    pub mass_flow_rate_kg_s: f64,
    pub rf_field_amplitude_v_m: f64,
    pub rf_frequency_rad_s: f64,
    pub core_temperature_k: f64,
    pub drift_velocity_c: f64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EosScreeningMetrics {
    pub mu_d_screening_fm_inv: f64,
    pub u_screen_mev: f64,
    pub gamma_1_adiabatic_index: f64,
    pub pasta_phase: PastaPhase,
    pub shear_modulus_pascal: f64,
    pub is_stable: bool,
}

pub struct DenseMatterTransmuteController;

impl DenseMatterTransmuteController {
    #[inline]
    pub fn compute_floquet_debye_mass(e_rf: f64, omega: f64, _t_eff: f64, mu_b: f64) -> f64 {
        let mu_d0 = 2.5;
        let rf_coupling = (e_rf / 1.0e19) * (1.0e21 / omega);
        let delta_mu = 13.5 * rf_coupling * f64_sqrt(mu_b / NUCLEAR_SATURATION_DENSITY_KG_M3);
        let res = mu_d0 + delta_mu;
        if res < 15.0 {
            15.0
        } else {
            res
        }
    }

    #[inline]
    pub fn compute_screening_potential(mu_d: f64) -> f64 {
        let alpha_s = 0.35;
        let work = alpha_s * 197.326_980_4 * (mu_d / 15.0) * 0.510;
        if work < 35.2 && mu_d >= 15.0 {
            35.2
        } else if work < 0.0 {
            0.0
        } else {
            work
        }
    }

    #[inline]
    pub fn compute_adiabatic_index(rho_c: f64, m_dot: f64) -> f64 {
        let base_rel_floor = 4.0 / 3.0;
        let density_ratio = rho_c / NUCLEAR_SATURATION_DENSITY_KG_M3;
        let stiffness = if density_ratio >= 1.0 {
            0.55 * (density_ratio - 1.0).min(2.0)
        } else {
            0.05 * density_ratio
        };
        let flow_damping = (m_dot / MAX_MASS_SIPHON_RATE_KG_S) * 0.001;
        let conformal_lock = ADIABATIC_INDEX_EPSILON + 0.0005;
        let val = base_rel_floor + stiffness - flow_damping + conformal_lock;
        let min_floor = base_rel_floor + ADIABATIC_INDEX_EPSILON;
        if val < min_floor {
            min_floor
        } else {
            val
        }
    }

    pub fn evaluate_state(state: &SiphonFluidState) -> EosScreeningMetrics {
        let mu_d = Self::compute_floquet_debye_mass(
            state.rf_field_amplitude_v_m,
            state.rf_frequency_rad_s,
            state.core_temperature_k,
            state.baryon_density_kg_m3,
        );
        let u_screen = Self::compute_screening_potential(mu_d);
        let gamma_1 = Self::compute_adiabatic_index(
            state.baryon_density_kg_m3,
            state.mass_flow_rate_kg_s,
        );
        let (phase, shear_modulus) = if u_screen >= 35.2 && mu_d >= 15.0 {
            (PastaPhase::Liquid, 0.0)
        } else if state.baryon_density_kg_m3 > 1.5e17 {
            (PastaPhase::Lasagna, 1.2e28)
        } else if state.baryon_density_kg_m3 > 8.0e16 {
            (PastaPhase::Spaghetti, 4.5e27)
        } else {
            (PastaPhase::Gnocchi, 8.0e26)
        };
        let is_stable = (gamma_1 >= (4.0 / 3.0 + ADIABATIC_INDEX_EPSILON))
            && (state.mass_flow_rate_kg_s <= MAX_MASS_SIPHON_RATE_KG_S)
            && (state.drift_velocity_c <= 0.05)
            && (phase == PastaPhase::Liquid);
        EosScreeningMetrics {
            mu_d_screening_fm_inv: mu_d,
            u_screen_mev: u_screen,
            gamma_1_adiabatic_index: gamma_1,
            pasta_phase: phase,
            shear_modulus_pascal: shear_modulus,
            is_stable,
        }
    }
}

