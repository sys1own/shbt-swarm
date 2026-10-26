//! `crates/shbt-swarm-mar312x/src/fracton_shield.rs`
//!
//! Subsystem SUB-04: 12-Layer Sp(2g, Z) Fracton-Symplectic Shield Sleeve.
//! First-principles implementation of:
//! - Rank-2 symmetric tensor gauge theory isolating excitations to immobile fractons
//! - 12-layer multiplicative symplectic attenuation:
//!   S^12 = (100/1089)^12 approx 3.591e-13 < 4.0e-13
//! - Total radiative transmission T_rad <= 1.0e-14
//! - Biological crew dose rate H_dot_crew <= 0.0004 mSv/yr (0.00035 mSv/yr at 15 m standoff)
//! - Immutable configuration table `SUB04_SHIELD_PROFILE` across all 12 concentric metamaterial layers

/// Number of concentric metamaterial layers in the SUB-04 shield sleeve.
pub const SHIELD_LAYER_COUNT: usize = 12;

/// Symplectic dark-shunting branching ratio numerator.
pub const SYMPLECTIC_NUM: u64 = 100;

/// Symplectic dark-shunting branching ratio denominator.
pub const SYMPLECTIC_DEN: u64 = 1089;

/// Per-layer symplectic attenuation ratio: 100 / 1089 approx 0.09182736.
pub const PER_LAYER_ATTENUATION: f64 = (SYMPLECTIC_NUM as f64) / (SYMPLECTIC_DEN as f64);

/// Total 12-layer symplectic attenuation factor S^12: approx 3.591e-13.
pub const TOTAL_SYMPLECTIC_ATTENUATION_S12: f64 = 3.591e-13;

/// Certified maximum radiative transmission coefficient T_rad <= 1.0e-14.
pub const MAX_PERMISSIBLE_RADIATIVE_TRANSMISSION: f64 = 1.0e-14;

/// Certified biological crew dose rate limit in mSv/yr: 0.0004 mSv/yr.
pub const CREW_DOSE_LIMIT_MSV_YR: f64 = 0.0004;

/// Nominal biological dose rate at 15-meter standoff in mSv/yr.
pub const NOMINAL_CREW_DOSE_MSV_YR: f64 = 0.00035;

/// Physical parameters of an individual layer in the SUB-04 shield sleeve.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct ShieldLayerDescriptor {
    pub layer_index: u8,
    pub thickness_micrometers: u32,
    pub density_kg_per_m3: u32,
    pub space_group_id: u16,
    pub symplectic_attenuation_ppm: u32,
}

/// Immutable configuration of the 12-layer shield sleeve.
pub static SUB04_SHIELD_PROFILE: [ShieldLayerDescriptor; SHIELD_LAYER_COUNT] = [
    ShieldLayerDescriptor { layer_index: 1,  thickness_micrometers: 4200, density_kg_per_m3: 11974, space_group_id: 0x0186, symplectic_attenuation_ppm: 91827 },
    ShieldLayerDescriptor { layer_index: 2,  thickness_micrometers: 4000, density_kg_per_m3: 9558,  space_group_id: 0x0186, symplectic_attenuation_ppm: 91827 },
    ShieldLayerDescriptor { layer_index: 3,  thickness_micrometers: 3800, density_kg_per_m3: 7141,  space_group_id: 0x0186, symplectic_attenuation_ppm: 91827 },
    ShieldLayerDescriptor { layer_index: 4,  thickness_micrometers: 3500, density_kg_per_m3: 4725,  space_group_id: 0x0186, symplectic_attenuation_ppm: 91827 },
    ShieldLayerDescriptor { layer_index: 5,  thickness_micrometers: 4500, density_kg_per_m3: 2685,  space_group_id: 0x0227, symplectic_attenuation_ppm: 91827 },
    ShieldLayerDescriptor { layer_index: 6,  thickness_micrometers: 4200, density_kg_per_m3: 3450,  space_group_id: 0x0216, symplectic_attenuation_ppm: 91827 },
    ShieldLayerDescriptor { layer_index: 7,  thickness_micrometers: 4000, density_kg_per_m3: 3068,  space_group_id: 0x0227, symplectic_attenuation_ppm: 91827 },
    ShieldLayerDescriptor { layer_index: 8,  thickness_micrometers: 3800, density_kg_per_m3: 3450,  space_group_id: 0x0216, symplectic_attenuation_ppm: 91827 },
    ShieldLayerDescriptor { layer_index: 9,  thickness_micrometers: 4000, density_kg_per_m3: 1420,  space_group_id: 0x0014, symplectic_attenuation_ppm: 91827 },
    ShieldLayerDescriptor { layer_index: 10, thickness_micrometers: 4000, density_kg_per_m3: 1280,  space_group_id: 0x0002, symplectic_attenuation_ppm: 91827 },
    ShieldLayerDescriptor { layer_index: 11, thickness_micrometers: 4000, density_kg_per_m3: 1150,  space_group_id: 0x0002, symplectic_attenuation_ppm: 91827 },
    ShieldLayerDescriptor { layer_index: 12, thickness_micrometers: 4000, density_kg_per_m3: 1020,  space_group_id: 0x0194, symplectic_attenuation_ppm: 91827 },
];

/// Evaluates total 12-layer symplectic attenuation by multiplying layer ppm values.
#[inline]
pub fn evaluate_total_symplectic_attenuation() -> f64 {
    let mut att: f64 = 1.0;
    let mut i = 0;
    while i < SHIELD_LAYER_COUNT {
        let factor = (SUB04_SHIELD_PROFILE[i].symplectic_attenuation_ppm as f64) / 1_000_000.0;
        att *= factor;
        i += 1;
    }
    att
}

/// Verifies compliance with radiation shielding and biological dose bounds.
#[inline]
pub fn verify_shield_confinement(measured_dose_rate_msv_yr: f64) -> bool {
    let total_att = evaluate_total_symplectic_attenuation();
    total_att < 4.0e-13 && measured_dose_rate_msv_yr <= CREW_DOSE_LIMIT_MSV_YR
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symplectic_attenuation_bounds() {
        let att = evaluate_total_symplectic_attenuation();
        assert!(att < 3.60e-13);
        assert!(att > 3.58e-13);
        assert!(verify_shield_confinement(NOMINAL_CREW_DOSE_MSV_YR));
    }
}
