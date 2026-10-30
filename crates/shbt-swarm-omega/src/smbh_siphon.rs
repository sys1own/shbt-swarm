//! crates/shbt-swarm-omega/src/smbh_siphon.rs
//! Supermassive Black Hole Ergosphere Siphoning and Area Non-Decrease Engine.
//! Implements THM-OMEGA-08-EXT in `#![no_std]`.

use core::f64::consts::PI;

pub const G_NEWTON: f64 = 6.674_30e-11; // m^3 kg^-1 s^-2
pub const SPEED_OF_LIGHT: f64 = 299_792_458.0; // m s^-1
pub const DARK_LEDGER_EFFICIENCY: f64 = 23.0 / 33.0; // eta_D

#[inline]
pub fn f64_sqrt(x: f64) -> f64 {
    if x <= 0.0 { return 0.0; }
    let bits = x.to_bits();
    let initial_bits = (bits >> 1) + 0x1ff8_0000_0000_0000;
    let mut guess = f64::from_bits(initial_bits);
    guess = 0.5 * (guess + x / guess);
    guess = 0.5 * (guess + x / guess);
    guess = 0.5 * (guess + x / guess);
    guess = 0.5 * (guess + x / guess);
    guess = 0.5 * (guess + x / guess);
    guess
}

#[inline]
pub fn f64_cos(x: f64) -> f64 {
    let half_pi = core::f64::consts::FRAC_PI_2;
    let div = x / half_pi;
    let k = if div >= 0.0 { (div + 0.5) as i64 } else { (div - 0.5) as i64 };
    let r = x - (k as f64) * half_pi;
    let r2 = r * r;
    let sin_r = r * (1.0 - r2 * (1.0 / 6.0 - r2 * (1.0 / 120.0 - r2 * (1.0 / 5040.0 - r2 * (1.0 / 362880.0 - r2 / 39916800.0)))));
    let cos_r = 1.0 - r2 * (0.5 - r2 * (1.0 / 24.0 - r2 * (1.0 / 720.0 - r2 * (1.0 / 40320.0 - r2 * (1.0 / 3628800.0 - r2 / 479001600.0)))));
    match k.rem_euclid(4) {
        0 => cos_r,
        1 => -sin_r,
        2 => -cos_r,
        _ => sin_r,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SiphonError {
    SuperExtremalSpin,
    UnphysicalFluxRate,
    HorizonPenetrationViolation,
    ThermodynamicAreaViolation,
}

#[derive(Debug, Clone, Copy)]
pub struct KerrBlackHole {
    pub mass: f64,              // Mass M in kg
    pub spin_param: f64,        // Dimensionless spin a* = a / (G M / c^2) in [0, 1)
}

impl KerrBlackHole {
    pub fn new(mass: f64, spin_param: f64) -> Result<Self, SiphonError> {
        if spin_param < 0.0 || spin_param >= 1.0 {
            return Err(SiphonError::SuperExtremalSpin);
        }
        Ok(Self { mass, spin_param })
    }

    #[inline]
    pub fn gravitational_radius(&self) -> f64 {
        (G_NEWTON * self.mass) / (SPEED_OF_LIGHT * SPEED_OF_LIGHT)
    }

    #[inline]
    pub fn horizon_radius(&self) -> f64 {
        let r_g = self.gravitational_radius();
        let term = f64_sqrt(1.0 - self.spin_param * self.spin_param);
        r_g * (1.0 + term)
    }

    #[inline]
    pub fn ergosphere_radius(&self, theta_rad: f64) -> f64 {
        let r_g = self.gravitational_radius();
        let cos_theta = f64_cos(theta_rad);
        let term = f64_sqrt(1.0 - self.spin_param * self.spin_param * cos_theta * cos_theta);
        r_g * (1.0 + term)
    }

    #[inline]
    pub fn horizon_angular_velocity(&self) -> f64 {
        let r_g = self.gravitational_radius();
        let a_dim = self.spin_param * r_g;
        let r_h = self.horizon_radius();
        (a_dim * SPEED_OF_LIGHT) / (r_h * r_h + a_dim * a_dim)
    }

    #[inline]
    pub fn surface_gravity(&self) -> f64 {
        let r_g = self.gravitational_radius();
        let a_dim = self.spin_param * r_g;
        let r_h = self.horizon_radius();
        let r_plus_minus = r_g * f64_sqrt(1.0 - self.spin_param * self.spin_param);
        (r_plus_minus * SPEED_OF_LIGHT * SPEED_OF_LIGHT) / (r_h * r_h + a_dim * a_dim)
    }

    #[inline]
    pub fn horizon_area(&self) -> f64 {
        let r_g = self.gravitational_radius();
        let a_dim = self.spin_param * r_g;
        let r_h = self.horizon_radius();
        4.0 * PI * (r_h * r_h + a_dim * a_dim)
    }

    #[inline]
    pub fn irreducible_mass(&self) -> f64 {
        let area = self.horizon_area();
        let factor = SPEED_OF_LIGHT * SPEED_OF_LIGHT / G_NEWTON;
        f64_sqrt((area / (16.0 * PI)) * (factor * factor))
    }

    #[inline]
    pub fn max_extractable_rotational_energy(&self) -> f64 {
        let m_irr = self.irreducible_mass();
        (self.mass - m_irr) * SPEED_OF_LIGHT * SPEED_OF_LIGHT
    }
}

/// Hexameric Bondi-Hoyle Siphoning Constellation
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
        // Standoff radius: 1.25 times the equatorial ergosphere radius
        let node_radius = 1.25 * black_hole.ergosphere_radius(PI / 2.0);
        Ok(Self {
            black_hole,
            node_radius,
            dark_ledger_reserve_joules: 0.0,
            total_extracted_joules: 0.0,
        })
    }

    /// Executes an infinitesimal siphoning cycle across all 6 hexameric funnels.
    /// Infalling negative mode energy: dm_infalling_kg (< 0)
    /// Infalling retrograde angular momentum: dj_infalling_kg_m2_s1 (< 0)
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

        // Fundamental null-energy condition: dm - (omega_h / c^2) * dj >= 0
        let effective_flux = dm_infalling_kg * SPEED_OF_LIGHT * SPEED_OF_LIGHT
            - omega_h * dj_infalling_kg_m2_s1;

        if effective_flux < -1e-12 {
            return Err(SiphonError::HorizonPenetrationViolation);
        }

        // Area differential: dA = (8 * PI * G / (c^2 * kappa)) * effective_flux
        let d_area = ((8.0 * PI * G_NEWTON) / (SPEED_OF_LIGHT * SPEED_OF_LIGHT * kappa))
            * effective_flux;

        if d_area < -1e-15 {
            return Err(SiphonError::ThermodynamicAreaViolation);
        }

        // Extracted rotational energy from the 6 funnels
        let energy_extracted_joules = (-dm_infalling_kg) * SPEED_OF_LIGHT * SPEED_OF_LIGHT;
        let energy_to_ledger = energy_extracted_joules * DARK_LEDGER_EFFICIENCY;

        // Update angular momentum based on loss, using pre-step mass
        let j_current = self.black_hole.spin_param
            * (G_NEWTON * self.black_hole.mass * self.black_hole.mass / SPEED_OF_LIGHT);
        let j_next = (j_current + dj_infalling_kg_m2_s1).max(0.0);

        // Update mass and resulting dimensionless spin parameter
        self.black_hole.mass += dm_infalling_kg;
        self.black_hole.spin_param = (j_next * SPEED_OF_LIGHT)
            / (G_NEWTON * self.black_hole.mass * self.black_hole.mass);

        let area_after = self.black_hole.horizon_area();
        let area_tol = 1e-12 * area_before.max(1.0);
        if area_after + area_tol < area_before {
            return Err(SiphonError::ThermodynamicAreaViolation);
        }

        self.dark_ledger_reserve_joules += energy_to_ledger;
        self.total_extracted_joules += energy_extracted_joules;

        Ok(energy_to_ledger)
    }
}
