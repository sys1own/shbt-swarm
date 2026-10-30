//! crates/shbt-swarm-omega/src/halo_clamp.rs
//! Galactic Dark Matter Halo Virial Clamping Engine via Synthetic CCZ4 Counter-Terms.
//! Implements THM-OMEGA-09-EXT in `#![no_std]`.

use core::f64::consts::PI;

pub const G_NEWTON: f64 = 6.674_30e-11; // m^3 kg^-1 s^-2
pub const SOLAR_MASS: f64 = 1.988_47e30; // kg
pub const KILOPARSEC: f64 = 3.085_677_581e19; // m
pub const VIRIAL_TOLERANCE: f64 = 1e-6; // Maximum allowed dispersion deviation

#[inline]
pub fn f64_abs(x: f64) -> f64 {
    if x < 0.0 { -x } else { x }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VirialClampError {
    InvalidHaloParameters,
    ExcessiveHarvestMass,
    VirialEquilibriumBroken,
}

/// Spherically Symmetric Hernquist Dark Matter Halo Profile
#[derive(Debug, Clone, Copy)]
pub struct HernquistDMHalo {
    pub total_dm_mass: f64, // kg
    pub scale_radius: f64,  // m
    pub anisotropy_beta: f64,
}

impl HernquistDMHalo {
    pub fn new(
        total_dm_mass: f64,
        scale_radius: f64,
        anisotropy_beta: f64,
    ) -> Result<Self, VirialClampError> {
        if total_dm_mass <= 0.0 || scale_radius <= 0.0 || anisotropy_beta < -1.0 || anisotropy_beta > 1.0 {
            return Err(VirialClampError::InvalidHaloParameters);
        }
        Ok(Self { total_dm_mass, scale_radius, anisotropy_beta })
    }

    #[inline]
    pub fn enclosed_dm_mass(&self, r: f64) -> f64 {
        let s = r / (r + self.scale_radius);
        self.total_dm_mass * s * s
    }

    #[inline]
    pub fn density(&self, r: f64) -> f64 {
        let a = self.scale_radius;
        let ra = r + a;
        (self.total_dm_mass * a) / (2.0 * PI * r * ra * ra * ra)
    }
}

/// Galactic Baryonic Harvest Profile and Synthetic CCZ4 Clamping Array
#[derive(Debug, Clone, Copy)]
pub struct GalacticVirialClampEngine {
    pub halo: HernquistDMHalo,
    pub initial_baryon_mass: f64,   // Initial 1e9 M_sun in kg
    pub baryon_scale_radius: f64,   // m
    pub harvested_mass: f64,        // Accumulated mass de-rendered to dark ledger
    pub aegis_perimeter_radius: f64,// Outer array perimeter
}

impl GalacticVirialClampEngine {
    pub fn new(
        halo_mass: f64,
        halo_scale: f64,
        baryon_mass: f64,
        baryon_scale: f64,
    ) -> Result<Self, VirialClampError> {
        let halo = HernquistDMHalo::new(halo_mass, halo_scale, 0.0)?;
        let aegis_perimeter_radius = 50.0 * KILOPARSEC;
        Ok(Self {
            halo,
            initial_baryon_mass: baryon_mass,
            baryon_scale_radius: baryon_scale,
            harvested_mass: 0.0,
            aegis_perimeter_radius,
        })
    }

    #[inline]
    pub fn enclosed_initial_baryon_mass(&self, r: f64) -> f64 {
        let s = r / (r + self.baryon_scale_radius);
        self.initial_baryon_mass * s * s
    }

    #[inline]
    pub fn enclosed_remaining_baryon_mass(&self, r: f64) -> f64 {
        let s = r / (r + self.baryon_scale_radius);
        (self.initial_baryon_mass - self.harvested_mass) * s * s
    }

    /// Computes the exact synthetic CCZ4 counter-tensor gradient required to lock the halo
    #[inline]
    pub fn synthetic_ccz4_potential_gradient(&self, r: f64) -> f64 {
        let s = r / (r + self.baryon_scale_radius);
        let m_synthetic_enc = self.harvested_mass * s * s;
        (G_NEWTON * m_synthetic_enc) / (r * r)
    }

    /// Evaluates the total effective potential gradient dPhi_tot / dr
    #[inline]
    pub fn total_effective_potential_gradient(&self, r: f64) -> f64 {
        let m_dm = self.halo.enclosed_dm_mass(r);
        let m_bar_rem = self.enclosed_remaining_baryon_mass(r);
        let grad_uncompensated = (G_NEWTON * (m_dm + m_bar_rem)) / (r * r);
        let grad_synthetic = self.synthetic_ccz4_potential_gradient(r);
        grad_uncompensated + grad_synthetic
    }

    /// Harvests an increment of baryonic mass into the dark ledger and verifies virial stability
    pub fn harvest_and_clamp(&mut self, delta_mass_kg: f64) -> Result<f64, VirialClampError> {
        if self.harvested_mass + delta_mass_kg > self.initial_baryon_mass {
            return Err(VirialClampError::ExcessiveHarvestMass);
        }

        self.harvested_mass += delta_mass_kg;

        // Audit radial velocity dispersion across a 64-point halo grid
        const SAMPLES: usize = 64;
        let r_min = 0.5 * KILOPARSEC;
        let r_max = 30.0 * KILOPARSEC;
        let dr = (r_max - r_min) / (SAMPLES as f64);

        for i in 0..SAMPLES {
            let r = r_min + (i as f64) * dr;
            let initial_m_tot = self.halo.enclosed_dm_mass(r) + self.enclosed_initial_baryon_mass(r);
            let grad_initial = (G_NEWTON * initial_m_tot) / (r * r);
            let grad_clamped = self.total_effective_potential_gradient(r);

            let rel_error = f64_abs((grad_clamped - grad_initial) / grad_initial);
            if rel_error > VIRIAL_TOLERANCE {
                return Err(VirialClampError::VirialEquilibriumBroken);
            }
        }

        Ok(self.harvested_mass / self.initial_baryon_mass)
    }
}
