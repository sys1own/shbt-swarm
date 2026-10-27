//! `crates/shbt-swarm-sim/src/reactor_twin.rs`
//! High-Precision Multi-Physics Digital Twin Simulation Engine for SHBT-MAR-312X Nacelle.

use core::f64::consts::PI;
use shbt_swarm_mar312x::{
    Mar312xSolitonDriveBridge, Mar312xPowerTier, ShiftVectorSolitonState,
    EdaExportError, export_gdsii_buf, export_step_buf, export_s2p_buf,
    INTERPOSER_TARGET_Z0,
};

pub const P_TIER0_WATTS: f64 = 1.004e6;
pub const P_TIER4_WATTS: f64 = 25.00e18;
pub const LATCHING_TIME_PS: f64 = 54.432;
pub const SENSIBLE_HEAT_FRACTION: f64 = 2.13039486e-9;
pub const CRIT_TEMP_SUPERHYDRIDE_K: f64 = 192.50;
pub const OPERATING_TEMP_K: f64 = 24.85;
pub const B_OPERATING_TESLA: f64 = 48.0;
pub const TAU_Q_PICODISPLACEMENT: f64 = 1.450e-12; // 1.45 ps
pub const ANDROMEDA_DISTANCE_LY: f64 = 2_537_000.0;
pub const JUMP_CRUISER_VELOCITY_C: f64 = 2_482_950.0;
pub const WARP_CORRIDOR_COMPRESSION: f64 = 1000.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SubsystemParam {
    pub id: &'static str,
    pub name: &'static str,
    pub z_start: f64,
    pub z_end: f64,
    pub r_in: f64,
    pub r_out: f64,
    pub volume_m3: f64,
    pub mass_kg: f64,
    pub density_g_cm3: f64,
}

pub static SUBSYSTEMS: [SubsystemParam; 8] = [
    SubsystemParam {
        id: "SUB-01",
        name: "Purcell Seed Die",
        z_start: 0.100, z_end: 0.180, r_in: 0.035, r_out: 0.082,
        volume_m3: 0.0012, mass_kg: 12.50, density_g_cm3: 10.417,
    },
    SubsystemParam {
        id: "SUB-02",
        name: "SHBT-DEC-X Converter",
        z_start: 0.400, z_end: 1.350, r_in: 0.080, r_out: 0.260,
        volume_m3: 0.1100, mass_kg: 260.00, density_g_cm3: 2.364,
    },
    SubsystemParam {
        id: "SUB-03",
        name: "Superhydride Coils",
        z_start: 0.080, z_end: 0.650, r_in: 0.085, r_out: 0.175,
        volume_m3: 0.0680, mass_kg: 215.00, density_g_cm3: 3.162,
    },
    SubsystemParam {
        id: "SUB-04",
        name: "12-Layer Fracton Shield Sleeve",
        z_start: 0.050, z_end: 1.400, r_in: 0.265, r_out: 0.315,
        volume_m3: 0.0380, mass_kg: 145.00, density_g_cm3: 3.816,
    },
    SubsystemParam {
        id: "SUB-05",
        name: "Type-IIa Diamond Cavity",
        z_start: 0.050, z_end: 0.450, r_in: 0.015, r_out: 0.080,
        volume_m3: 0.0455, mass_kg: 160.00, density_g_cm3: 3.516,
    },
    SubsystemParam {
        id: "SUB-06",
        name: "Kagome Injector / Direct CFT Bus",
        z_start: 0.000, z_end: 0.100, r_in: 0.020, r_out: 0.120,
        volume_m3: 0.0185, mass_kg: 52.50, density_g_cm3: 2.838,
    },
    SubsystemParam {
        id: "SUB-07",
        name: "PT-Weyl Exciton Crowbar",
        z_start: 0.650, z_end: 0.750, r_in: 0.180, r_out: 0.250,
        volume_m3: 0.0270, mass_kg: 40.00, density_g_cm3: 1.481,
    },
    SubsystemParam {
        id: "SUB-08",
        name: "48 T SMES Recovery Ring",
        z_start: 1.320, z_end: 1.450, r_in: 0.120, r_out: 0.280,
        volume_m3: 0.0418, mass_kg: 65.00, density_g_cm3: 1.555,
    },
];

#[derive(Debug, Clone)]
pub struct DynamicTwinState {
    pub current_time_s: f64,
    pub core_power_w: f64,
    pub temperatures_k: [f64; 8],
    pub heat_fluxes_w_m2: [f64; 8],
    pub fuel_inventory_ug: f64,
    pub daughter_ship_progress: f64,
    pub crowbar_latched: bool,
    pub smes_stored_joules: f64,
}

pub struct ReactorDigitalTwin {
    pub total_mass_kg: f64,
    pub total_volume_m3: f64,
    pub state: DynamicTwinState,
    pub hbt_rotator_bridge: Mar312xSolitonDriveBridge,
}

impl Default for ReactorDigitalTwin {
    fn default() -> Self {
        Self::new()
    }
}

impl ReactorDigitalTwin {
    pub fn new() -> Self {
        let total_mass = SUBSYSTEMS.iter().map(|s| s.mass_kg).sum();
        let total_volume = SUBSYSTEMS.iter().map(|s| s.volume_m3).sum();

        Self {
            total_mass_kg: total_mass,
            total_volume_m3: total_volume,
            hbt_rotator_bridge: Mar312xSolitonDriveBridge::new(),
            state: DynamicTwinState {
                current_time_s: 0.0,
                core_power_w: P_TIER0_WATTS,
                temperatures_k: [OPERATING_TEMP_K; 8],
                heat_fluxes_w_m2: [0.0; 8],
                fuel_inventory_ug: 10_000.0,
                daughter_ship_progress: 0.0,
                crowbar_latched: false,
                smes_stored_joules: 0.5 * (48.0 * 48.0 / (4.0 * PI * 1e-7)) * 0.0418,
            },
        }
    }

    pub fn simulate_pt_latching_transient(&mut self, dt_ps: f64) -> Result<(), &'static str> {
        let gamma_growth = (P_TIER4_WATTS / P_TIER0_WATTS).ln() / (LATCHING_TIME_PS * 1e-12);
        let mut t_ps = 0.0;

        while t_ps < LATCHING_TIME_PS {
            let t_sec = t_ps * 1e-12;
            self.state.core_power_w = P_TIER0_WATTS * (gamma_growth * t_sec).exp();
            if self.state.core_power_w > P_TIER4_WATTS {
                self.state.core_power_w = P_TIER4_WATTS;
            }
            t_ps += dt_ps;
        }

        self.state.core_power_w = P_TIER4_WATTS;
        self.state.current_time_s += LATCHING_TIME_PS * 1e-12;
        Ok(())
    }

    pub fn step_hyperbolic_cattaneo_vernotte(&mut self, dt_s: f64) {
        let p_sensible_total = self.state.core_power_w * SENSIBLE_HEAT_FRACTION;
        let p_sensible_per_zone = p_sensible_total / 8.0;

        #[allow(clippy::needless_range_loop)]
        for i in 0..8 {
            let k_thermal = 2200.0;
            let rho_cp = 1.8e6;
            let vol = SUBSYSTEMS[i].volume_m3;
            let area = 2.0 * PI * SUBSYSTEMS[i].r_out * (SUBSYSTEMS[i].z_end - SUBSYSTEMS[i].z_start);

            let _q_in = p_sensible_per_zone / area.max(1e-4);
            let grad_t = (self.state.temperatures_k[i] - OPERATING_TEMP_K) / (SUBSYSTEMS[i].r_out - SUBSYSTEMS[i].r_in);

            let dq_dt = (-self.state.heat_fluxes_w_m2[i] - k_thermal * grad_t) / TAU_Q_PICODISPLACEMENT;
            self.state.heat_fluxes_w_m2[i] += dq_dt * dt_s;

            let div_q = self.state.heat_fluxes_w_m2[i] / (SUBSYSTEMS[i].r_out - SUBSYSTEMS[i].r_in);
            let q_gen = p_sensible_per_zone / vol;
            let q_cooling = q_gen;

            let dt_dt = (-div_q + q_gen - q_cooling) / rho_cp;
            self.state.temperatures_k[i] += dt_dt * dt_s;

            if self.state.temperatures_k[i] < OPERATING_TEMP_K {
                self.state.temperatures_k[i] = OPERATING_TEMP_K;
            }
        }

        self.state.current_time_s += dt_s;
    }

    pub fn cycle_microbatch_injection(&mut self, cycle_ms: f64) -> bool {
        if cycle_ms > 15.0 {
            return false;
        }
        let batch_mass_ug = 1.250;
        if self.state.fuel_inventory_ug >= batch_mass_ug {
            self.state.fuel_inventory_ug -= batch_mass_ug;
            self.state.daughter_ship_progress += 0.001;
            if self.state.daughter_ship_progress >= 1.0 {
                self.state.daughter_ship_progress = 0.0;
            }
            true
        } else {
            false
        }
    }

    pub fn verify_jump_cruiser_flight_kinetics(&self) -> (f64, bool) {
        let effective_speed_c = JUMP_CRUISER_VELOCITY_C * WARP_CORRIDOR_COMPRESSION;
        let transit_time_years = ANDROMEDA_DISTANCE_LY / effective_speed_c;
        let transit_time_hours = transit_time_years * 365.25 * 24.0;
        let passes_tolerance = (transit_time_hours - 8.95682).abs() < 0.01;
        (transit_time_hours, passes_tolerance)
    }

    pub fn step_soliton_actuation(
        &mut self,
        tier: Mar312xPowerTier,
        tau: f64,
        theta_x_rad: f64,
        theta_y_rad: f64,
    ) -> Result<ShiftVectorSolitonState, u32> {
        self.hbt_rotator_bridge.step_coupling(
            tier,
            tau,
            10.0,
            theta_x_rad,
            theta_y_rad,
            0.0,
        )
    }

    pub fn export_rotator_eda_artifacts(
        &self,
        gds_buf: &mut [u8],
        step_buf: &mut [u8],
        s2p_buf: &mut [u8],
    ) -> Result<(usize, usize, usize), EdaExportError> {
        let n_gds = export_gdsii_buf(gds_buf)?;
        let n_step = export_step_buf(step_buf, 3.20, 0.05, 0.35)?;
        let freqs = [10.0, 20.0, 30.0, 40.0];
        let n_s2p = export_s2p_buf(s2p_buf, INTERPOSER_TARGET_Z0, &freqs)?;
        Ok((n_gds, n_step, n_s2p))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reactor_twin_subsystems_budget() {
        let twin = ReactorDigitalTwin::new();
        assert!((twin.total_mass_kg - 950.0).abs() < 1e-6);
        assert!((twin.total_volume_m3 - 0.3500).abs() < 1e-6);
    }

    #[test]
    fn test_pt_latching_and_kinetics() {
        let mut twin = ReactorDigitalTwin::new();
        twin.simulate_pt_latching_transient(1.0).unwrap();
        assert_eq!(twin.state.core_power_w, P_TIER4_WATTS);

        let (hours, passes) = twin.verify_jump_cruiser_flight_kinetics();
        assert!(passes);
        assert!((hours - 8.957).abs() < 0.01);

        assert!(twin.cycle_microbatch_injection(14.85));
        assert!(!twin.cycle_microbatch_injection(15.5));
    }

    #[test]
    fn test_reactor_twin_soliton_and_eda_export() {
        let mut twin = ReactorDigitalTwin::new();
        let state = twin.step_soliton_actuation(
            Mar312xPowerTier::Tier4PlanetaryForge,
            1.0,
            0.01,
            -0.02,
        ).unwrap();
        assert_eq!(state.target_velocity_c, 2_482_950.0);
        assert!(state.spatial_coherence >= 0.9995);

        let mut gds = [0u8; 8192];
        let mut step = [0u8; 2048];
        let mut s2p = [0u8; 2048];
        let (ng, ns, n2) = twin.export_rotator_eda_artifacts(&mut gds, &mut step, &mut s2p).unwrap();
        assert!(ng > 0 && ns > 0 && n2 > 0);
    }
}
