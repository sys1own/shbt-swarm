//! `crates/shbt-swarm-omega/src/blandford_znajek_null.rs`
//! Blandford-Znajek Jet Extinction in Kerr SMBH Ergosphere Siphoning (`THM-OMEGA-10-EXT`).

#![allow(dead_code)]

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KerrErgosphereSiphon {
    pub g_0phi: f64,
    pub g_phiphi: f64,
    pub lapse_alpha: f64,
    pub radial_b_field_tesla: f64,
    pub omega_field_rad_s: f64,
    pub area_derivative_m2_s: f64,
}

impl KerrErgosphereSiphon {
    #[inline]
    pub const fn new_default() -> Self {
        let g_0phi = -1.85e9;
        let g_phiphi = 2.50e12;
        let omega_drag = -g_0phi / g_phiphi;
        Self {
            g_0phi,
            g_phiphi,
            lapse_alpha: 0.42,
            radial_b_field_tesla: 1.00e4,
            omega_field_rad_s: omega_drag, // Metric-conformal boundary phase lock
            area_derivative_m2_s: 1.25e-4,
        }
    }

    /// Computes the local Lense-Thirring frame-dragging angular velocity omega = -g_{0phi} / g_{phiphi}.
    #[inline]
    pub const fn frame_dragging_omega(&self) -> f64 {
        -self.g_0phi / self.g_phiphi
    }

    #[inline]
    pub const fn field_angular_velocity(&self) -> f64 {
        self.omega_field_rad_s
    }

    /// Evaluates outward radial Blandford-Znajek Poynting flux S_EM^r (in Watts).
    /// Under metric-conformal phase lock Omega_F == omega, S_EM^r is identically 0.0 W.
    #[inline]
    pub const fn radial_poynting_flux_watts(&self) -> f64 {
        let slip = self.omega_field_rad_s - self.frame_dragging_omega();
        self.lapse_alpha * self.g_phiphi * self.omega_field_rad_s * slip * (self.radial_b_field_tesla * self.radial_b_field_tesla)
    }

    #[inline]
    pub const fn horizon_area_derivative(&self) -> f64 {
        self.area_derivative_m2_s
    }
}
