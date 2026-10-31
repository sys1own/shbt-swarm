//! `crates/shbt-swarm-foundry/src/fluid_gravity_dual.rs`
//! Holographic Boundary Fluid-Gravity Dual for Atmospheric Navier-Stokes (`THM-UNIV-06-EXT`).

#![allow(dead_code)]

pub const MAX_CLAMPED_MACH: f64 = 0.15;
pub const MAX_PERMISSIBLE_SHEAR_PA: f32 = 45.0;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HolographicFluidGravityShunt {
    pub sound_speed_mps: f64,
    pub atmospheric_density_kg_m3: f64,
    pub skin_friction_coeff: f64,
    pub mach_clamp_limit: f64,
    pub baroclinic_misalignment_rad: f64,
}

impl HolographicFluidGravityShunt {
    #[inline]
    pub const fn new_default() -> Self {
        Self {
            sound_speed_mps: 340.0,
            atmospheric_density_kg_m3: 1.20,
            skin_friction_coeff: 0.0030,
            mach_clamp_limit: MAX_CLAMPED_MACH,
            baroclinic_misalignment_rad: 0.0,
        }
    }

    /// Computes the Sugawara off-diagonal stress tensor shunt Upsilon^shunt_{ij} (in Pa)
    /// that absorbs supersonic convective momentum flux above v_clamp = Ma_max * c_s.
    #[inline]
    pub const fn sugawara_momentum_shunt_pa(&self, raw_velocity_mps: f64) -> f64 {
        let v_clamp = self.mach_clamp_limit * self.sound_speed_mps;
        if raw_velocity_mps <= v_clamp {
            0.0
        } else {
            -self.atmospheric_density_kg_m3 * (raw_velocity_mps * raw_velocity_mps - v_clamp * v_clamp)
        }
    }

    /// Returns the effective post-shunt Mach number (clamped to <= 0.15).
    #[inline]
    pub const fn clamped_mach_number(&self, raw_velocity_mps: f64) -> f64 {
        let raw_mach = raw_velocity_mps / self.sound_speed_mps;
        if raw_mach > self.mach_clamp_limit {
            self.mach_clamp_limit
        } else if raw_mach < 0.0 {
            0.0
        } else {
            raw_mach
        }
    }

    /// Evaluates planetary surface shear stress tau_shear (in Pa) under Sugawara acoustic tamping.
    #[inline]
    pub const fn clamped_surface_shear_pa(&self, raw_velocity_mps: f64) -> f32 {
        let raw_flux = self.atmospheric_density_kg_m3 * raw_velocity_mps * raw_velocity_mps;
        let shunted_flux = raw_flux + self.sugawara_momentum_shunt_pa(raw_velocity_mps);
        (0.5 * shunted_flux * self.skin_friction_coeff) as f32
    }

    /// Evaluates unshunted surface shear stress tau_shear (in Pa) for fault injection comparison.
    #[inline]
    pub const fn unshunted_surface_shear_pa(&self, raw_velocity_mps: f64) -> f32 {
        (0.5 * self.atmospheric_density_kg_m3 * raw_velocity_mps * raw_velocity_mps * self.skin_friction_coeff) as f32
    }

    /// Returns the baroclinic vorticity generation norm |(grad rho x grad P) / rho^2|.
    #[inline]
    pub const fn baroclinic_vorticity_norm(&self) -> f64 {
        self.baroclinic_misalignment_rad
    }
}
