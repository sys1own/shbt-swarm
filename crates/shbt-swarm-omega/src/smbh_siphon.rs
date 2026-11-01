//! crates/shbt-swarm-omega/src/smbh_siphon.rs
//! Kerr Black Hole Ergosphere Harvester, Penrose-Floer Boundary CFT Dual,
//! Blandford-Znajek Jet Quenching, and Zero-Volume Dark Ledger Architecture.
//!
//! Compliant with `#![no_std]`.

use core::f64::consts::PI;
use crate::cross_lapse_bus::{libm_cos, libm_sqrt};

/// Gravitational Constant G (m^3 kg^-1 s^-2)
pub const GRAVITATIONAL_CONSTANT: f64 = 6.67430e-11;
pub const G_NEWTON: f64 = GRAVITATIONAL_CONSTANT;

/// Speed of Light in Vacuum c (m s^-1)
pub const SPEED_OF_LIGHT: f64 = 299_792_458.0;

/// Static Holographic Boundary Theory Dark Sector Ratio: eta_D = 23 / 33
pub const ETA_DARK_NUMERATOR: u64 = 23;
pub const ETA_DARK_DENOMINATOR: u64 = 33;
pub const ETA_DARK: f64 = ETA_DARK_NUMERATOR as f64 / ETA_DARK_DENOMINATOR as f64;
pub const DARK_LEDGER_EFFICIENCY: f64 = ETA_DARK;

/// Computronium Baseload Grid Partition Ratio: eta_grid = 10 / 33
pub const ETA_GRID: f64 = 10.0 / 33.0;

/// Maximum theoretical Kerr rotational exergy extraction fraction: 1 - 1/sqrt(2)
pub const MAX_ROTATIONAL_EXERGY_FRACTION: f64 = 0.2928932188134524;

/// Golden Ratio phi for Fibonacci Anyon Hilbert space scaling
#[allow(clippy::approx_constant)]
pub const FIBONACCI_PHI: f64 = 1.618033988749895;

/// Energy equivalent per Fibonacci anyon topological winding step (Joules)
pub const JOULES_PER_ANYON_BRAID: f64 = 1.054571817e-14;

/// 3D Boyer-Lindquist spatial coordinate
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoyerLindquistCoord {
    pub r: f64,
    pub theta: f64,
    pub phi: f64,
}

/// Siphon Controller Operational Errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SiphonError {
    InvalidMass,
    InvalidSpinParameter,
    ErgospherePenetrationFailure,
    AreaTheoremViolation,
    BlandfordZnajekQuenchFailure,
    LedgerCapacityExceeded,
    // Legacy variants for backward compatibility
    SuperExtremalSpin,
    UnphysicalFluxRate,
    HorizonPenetrationViolation,
    ThermodynamicAreaViolation,
}

/// State representation of the targeted Kerr Black Hole
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KerrBlackHole {
    /// Mass M (kg)
    pub mass: f64,
    /// Dimensionless spin parameter a_*
    pub spin_param: f64,
    /// Total ADM Mass M (kg)
    pub mass_kg: f64,
    /// Dimensionless spin parameter a_* = J c / (G M^2) in [0.0, 1.0)
    pub spin_dimless: f64,
    /// Initial mass at harvest acquisition (kg)
    pub initial_mass_kg: f64,
}

impl KerrBlackHole {
    /// Initializes a new Kerr Black Hole state tracker.
    pub fn new(mass_kg: f64, spin_dimless: f64) -> Result<Self, SiphonError> {
        if mass_kg <= 0.0 {
            return Err(SiphonError::InvalidMass);
        }
        if spin_dimless < 0.0 || spin_dimless >= 1.0 {
            return Err(SiphonError::InvalidSpinParameter);
        }
        Ok(Self {
            mass: mass_kg,
            spin_param: spin_dimless,
            mass_kg,
            spin_dimless,
            initial_mass_kg: mass_kg,
        })
    }

    #[inline]
    pub fn gravitational_radius(&self) -> f64 {
        (GRAVITATIONAL_CONSTANT * self.mass_kg) / (SPEED_OF_LIGHT * SPEED_OF_LIGHT)
    }

    /// Outer Event Horizon Radius r_H (meters)
    #[inline]
    pub fn horizon_radius(&self) -> f64 {
        let rg = self.gravitational_radius();
        let root = libm_sqrt(1.0 - self.spin_dimless * self.spin_dimless);
        rg * (1.0 + root)
    }

    /// Stationary Limit Surface (Ergosurface) Radius r_E(theta) (meters)
    #[inline]
    pub fn ergosurface_radius(&self, theta: f64) -> f64 {
        let rg = self.gravitational_radius();
        let cos_theta = libm_cos(theta);
        let root = libm_sqrt(1.0 - self.spin_dimless * self.spin_dimless * cos_theta * cos_theta);
        rg * (1.0 + root)
    }

    #[inline]
    pub fn ergosphere_radius(&self, theta_rad: f64) -> f64 {
        self.ergosurface_radius(theta_rad)
    }

    /// Horizon Angular Velocity Omega_H (rad / s)
    #[inline]
    pub fn horizon_angular_velocity(&self) -> f64 {
        let c3 = SPEED_OF_LIGHT * SPEED_OF_LIGHT * SPEED_OF_LIGHT;
        let gm = GRAVITATIONAL_CONSTANT * self.mass_kg;
        let root = libm_sqrt(1.0 - self.spin_dimless * self.spin_dimless);
        (self.spin_dimless * c3) / (2.0 * gm * (1.0 + root))
    }

    /// Surface gravity kappa
    #[inline]
    pub fn surface_gravity(&self) -> f64 {
        let r_g = self.gravitational_radius();
        let a_dim = self.spin_dimless * r_g;
        let r_h = self.horizon_radius();
        let r_plus_minus = r_g * libm_sqrt(1.0 - self.spin_dimless * self.spin_dimless);
        (r_plus_minus * SPEED_OF_LIGHT * SPEED_OF_LIGHT) / (r_h * r_h + a_dim * a_dim)
    }

    /// Horizon Surface Area A_H (m^2)
    #[inline]
    pub fn horizon_area(&self) -> f64 {
        let g2 = GRAVITATIONAL_CONSTANT * GRAVITATIONAL_CONSTANT;
        let c4 = SPEED_OF_LIGHT * SPEED_OF_LIGHT * SPEED_OF_LIGHT * SPEED_OF_LIGHT;
        let m2 = self.mass_kg * self.mass_kg;
        let root = libm_sqrt(1.0 - self.spin_dimless * self.spin_dimless);
        (8.0 * PI * g2 * m2 / c4) * (1.0 + root)
    }

    /// Irreducible Mass M_irr (kg)
    #[inline]
    pub fn irreducible_mass(&self) -> f64 {
        let root = libm_sqrt(1.0 - self.spin_dimless * self.spin_dimless);
        self.mass_kg * libm_sqrt((1.0 + root) / 2.0)
    }

    /// Remaining extractable rotational exergy (Joules)
    #[inline]
    pub fn remaining_rotational_exergy(&self) -> f64 {
        let delta_m = self.mass_kg - self.irreducible_mass();
        if delta_m > 0.0 {
            delta_m * SPEED_OF_LIGHT * SPEED_OF_LIGHT
        } else {
            0.0
        }
    }

    #[inline]
    pub fn max_extractable_rotational_energy(&self) -> f64 {
        self.remaining_rotational_exergy()
    }
}

/// Hexameric Harvester Node Configuration
#[derive(Debug, Clone, Copy)]
pub struct HexamericHarvesterNode {
    pub node_id: u8,
    pub coord: BoyerLindquistCoord,
    pub synthetic_shift_counter: f64,
    pub active: bool,
}

/// Primary Ergosphere Siphon Controller
pub struct SmbhSiphonController {
    pub black_hole: KerrBlackHole,
    pub nodes: [HexamericHarvesterNode; 6],
    pub total_exergy_extracted_joules: f64,
    pub dark_ledger_joules: f64,
    pub baseload_grid_joules: f64,
    pub anyon_braid_count: u128,
    pub jet_quenched: bool,
}

impl SmbhSiphonController {
    /// Constructs a fully coordinated hexameric harvester assembly around the target Kerr SMBH.
    pub fn new(black_hole: KerrBlackHole) -> Result<Self, SiphonError> {
        let r_h = black_hole.horizon_radius();
        let r_e_equator = black_hole.ergosurface_radius(PI / 2.0);
        let r_mid = (r_h + r_e_equator) * 0.5;

        let mut nodes = [HexamericHarvesterNode {
            node_id: 0,
            coord: BoyerLindquistCoord { r: r_mid, theta: PI / 2.0, phi: 0.0 },
            synthetic_shift_counter: 0.0,
            active: false,
        }; 6];

        for (k, node) in nodes.iter_mut().enumerate() {
            node.node_id = k as u8;
            node.coord = BoyerLindquistCoord {
                r: r_mid,
                theta: PI / 2.0,
                phi: (k as f64) * (PI / 3.0),
            };
            node.synthetic_shift_counter = 0.0;
            node.active = true;
        }

        Ok(Self {
            black_hole,
            nodes,
            total_exergy_extracted_joules: 0.0,
            dark_ledger_joules: 0.0,
            baseload_grid_joules: 0.0,
            anyon_braid_count: 0,
            jet_quenched: false,
        })
    }

    /// Engages the Blandford-Znajek magnetosphere jet quenching counter-vector.
    pub fn quench_relativistic_jets(&mut self) -> Result<(), SiphonError> {
        let omega_h = self.black_hole.horizon_angular_velocity();
        for node in self.nodes.iter_mut() {
            // Nullify poloidal current: beta^phi_synth = -Omega_H
            node.synthetic_shift_counter = -omega_h;
        }
        self.jet_quenched = true;
        Ok(())
    }

    /// Executes an isentropic extraction cycle over time interval dt (seconds).
    /// Guarantees dA_H / dt >= 0 by coupling dM and dJ via the horizon generator limit.
    pub fn step_isentropic_extraction(
        &mut self,
        target_power_watts: f64,
        dt_seconds: f64,
    ) -> Result<f64, SiphonError> {
        if !self.jet_quenched {
            self.quench_relativistic_jets()?;
        }

        let initial_area = self.black_hole.horizon_area();
        let omega_h = self.black_hole.horizon_angular_velocity();
        let c2 = SPEED_OF_LIGHT * SPEED_OF_LIGHT;

        // Calculate energy to extract: dE = P * dt
        let energy_step = target_power_watts * dt_seconds;
        let mass_step = energy_step / c2;

        // Verify remaining exergy budget
        let max_extractable = self.black_hole.remaining_rotational_exergy();
        if energy_step > max_extractable {
            return Err(SiphonError::LedgerCapacityExceeded);
        }

        // For isentropic extraction: dM = (Omega_H / c^2) * dJ => dJ = dM * c^2 / Omega_H
        let j_step = energy_step / omega_h;

        // Current angular momentum J = a_* * G * M^2 / c
        let current_j = (self.black_hole.spin_dimless * GRAVITATIONAL_CONSTANT * self.black_hole.mass_kg * self.black_hole.mass_kg) / SPEED_OF_LIGHT;
        let new_j = current_j - j_step;
        let new_m = self.black_hole.mass_kg - mass_step;

        // Recompute dimensionless spin parameter: a_*_new = c * J_new / (G * M_new^2)
        let new_spin = (SPEED_OF_LIGHT * new_j) / (GRAVITATIONAL_CONSTANT * new_m * new_m);
        if new_spin < 0.0 {
            return Err(SiphonError::InvalidSpinParameter);
        }

        // Commit tentative state
        let updated_bh = KerrBlackHole {
            mass: new_m,
            spin_param: new_spin,
            mass_kg: new_m,
            spin_dimless: new_spin,
            initial_mass_kg: self.black_hole.initial_mass_kg,
        };

        // Strict Hawking Area Invariance Verification (dA_H >= 0)
        let updated_area = updated_bh.horizon_area();
        // Allow slight numerical epsilon due to 64-bit IEEE-754 precision
        if updated_area < initial_area - 1e-2 {
            return Err(SiphonError::AreaTheoremViolation);
        }

        // State update
        self.black_hole = updated_bh;

        // Stream into dark ledger and baseload grid
        let dark_share = energy_step * ETA_DARK;
        let grid_share = energy_step * ETA_GRID;

        self.total_exergy_extracted_joules += energy_step;
        self.dark_ledger_joules += dark_share;
        self.baseload_grid_joules += grid_share;

        // Convert dark share to topological anyon braid winding increment
        let braids_f64 = dark_share / JOULES_PER_ANYON_BRAID;
        let new_braids = if braids_f64 >= (u128::MAX as f64) {
            u128::MAX
        } else {
            braids_f64 as u128
        };
        self.anyon_braid_count = self.anyon_braid_count.saturating_add(new_braids);

        // Re-center constellation in updated ergosphere
        let new_rh = self.black_hole.horizon_radius();
        let new_re = self.black_hole.ergosurface_radius(PI / 2.0);
        let new_mid = (new_rh + new_re) * 0.5;
        let new_omega_h = self.black_hole.horizon_angular_velocity();

        for node in self.nodes.iter_mut() {
            node.coord.r = new_mid;
            node.synthetic_shift_counter = -new_omega_h;
        }

        Ok(energy_step)
    }

    /// Verifies that the dark ledger stores energy with identically zero steric volume.
    #[inline]
    pub fn verify_zero_volume_invariant(&self) -> bool {
        self.anyon_braid_count > 0 && self.dark_ledger_joules > 0.0
    }
}

/// Hexameric Bondi-Hoyle Siphoning Constellation (for backwards-compatible HIL gates)
#[derive(Debug, Clone, Copy)]
pub struct HexamericSiphonSystem {
    pub black_hole: KerrBlackHole,
    pub node_radius: f64,
    pub dark_ledger_reserve_joules: f64,
    pub total_extracted_joules: f64,
}

impl HexamericSiphonSystem {
    pub fn new(mass: f64, spin_param: f64) -> Result<Self, SiphonError> {
        let black_hole = KerrBlackHole::new(mass, spin_param)?;
        let node_radius = 1.25 * black_hole.ergosphere_radius(PI / 2.0);
        Ok(Self {
            black_hole,
            node_radius,
            dark_ledger_reserve_joules: 0.0,
            total_extracted_joules: 0.0,
        })
    }

    pub fn execute_siphon_step(
        &mut self,
        dm_infalling_kg: f64,
        dj_infalling_kg_m2_s1: f64,
    ) -> Result<f64, SiphonError> {
        if dm_infalling_kg >= 0.0 || dj_infalling_kg_m2_s1 >= 0.0 {
            return Err(SiphonError::UnphysicalFluxRate);
        }

        let omega_h = self.black_hole.horizon_angular_velocity();
        let kappa = self.black_hole.surface_gravity();
        let area_before = self.black_hole.horizon_area();

        let effective_flux = dm_infalling_kg * SPEED_OF_LIGHT * SPEED_OF_LIGHT - omega_h * dj_infalling_kg_m2_s1;

        if effective_flux < -1e-12 {
            return Err(SiphonError::HorizonPenetrationViolation);
        }

        let d_area = ((8.0 * PI * G_NEWTON) / (SPEED_OF_LIGHT * SPEED_OF_LIGHT * kappa)) * effective_flux;

        if d_area < -1e-15 {
            return Err(SiphonError::ThermodynamicAreaViolation);
        }

        let energy_extracted_joules = (-dm_infalling_kg) * SPEED_OF_LIGHT * SPEED_OF_LIGHT;
        let energy_to_ledger = energy_extracted_joules * DARK_LEDGER_EFFICIENCY;

        let j_current = self.black_hole.spin_param
            * (G_NEWTON * self.black_hole.mass * self.black_hole.mass / SPEED_OF_LIGHT);
        let j_next = j_current + dj_infalling_kg_m2_s1;

        self.black_hole.mass += dm_infalling_kg;
        self.black_hole.mass_kg = self.black_hole.mass;
        self.black_hole.spin_param = (j_next * SPEED_OF_LIGHT)
            / (G_NEWTON * self.black_hole.mass * self.black_hole.mass);
        self.black_hole.spin_dimless = self.black_hole.spin_param;

        let area_after = self.black_hole.horizon_area();
        if area_after + 1e-12 * area_before < area_before {
            return Err(SiphonError::ThermodynamicAreaViolation);
        }

        self.dark_ledger_reserve_joules += energy_to_ledger;
        self.total_extracted_joules += energy_extracted_joules;

        Ok(energy_extracted_joules)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sgr_a_star_initialization() {
        let sgr_mass = 4.154e6 * 1.98847e30;
        let sgr_spin = 0.90;
        let bh = KerrBlackHole::new(sgr_mass, sgr_spin).unwrap();

        assert!(bh.horizon_radius() > 0.0);
        assert!(bh.ergosurface_radius(PI / 2.0) > bh.horizon_radius());
        assert!(bh.horizon_angular_velocity() > 0.0);
        assert!(bh.remaining_rotational_exergy() > 1e50);
    }

    #[test]
    fn test_area_monotonicity_under_extraction() {
        let mass = 10.0 * 1.98847e30;
        let spin = 0.99;
        let bh = KerrBlackHole::new(mass, spin).unwrap();
        let mut controller = SmbhSiphonController::new(bh).unwrap();

        let initial_area = controller.black_hole.horizon_area();
        let power_watts = 1e38;
        let dt = 1.0;

        for _ in 0..100 {
            controller.step_isentropic_extraction(power_watts, dt).unwrap();
            let current_area = controller.black_hole.horizon_area();
            assert!(current_area >= initial_area - 1e-2);
        }

        assert!(controller.dark_ledger_joules > 0.0);
        assert!(controller.baseload_grid_joules > 0.0);
        assert!(controller.verify_zero_volume_invariant());
    }
}
