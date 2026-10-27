//! `crates/shbt-swarm-mar312x/src/lib.rs`
//! `#![no_std]` First-Principles Control, Telemetry & Safety Verification Crate
//! for the 5th-Gen Exotic-Material Autopoietic Mass-Annihilation Reactor (`SHBT-MAR-312X`).

#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]

pub mod purcell_seed;
pub mod polariton_decx;
pub mod superhydride_coils;
pub mod fracton_shield;
pub mod autopoiesis;
pub mod crowbar_smes;
pub mod hbt_pic_rotator;
pub mod warp_soliton_coupling;
pub mod eda_export;

pub use purcell_seed::*;
pub use polariton_decx::*;
pub use superhydride_coils::*;
pub use fracton_shield::*;
pub use autopoiesis::*;
pub use crowbar_smes::*;
pub use hbt_pic_rotator::*;
pub use warp_soliton_coupling::*;
pub use eda_export::*;

/// Exact 8-subsystem masses in grams (summing to 950,000 g = 950.00 kg).
pub const MASS_SUB1_SEED_DIE_G: u64           = 12_500;   // 12.50 kg (P6_3/mmc)
pub const MASS_SUB2_DEC_X_G: u64              = 260_000;  // 260.00 kg (I4_1md)
pub const MASS_SUB3_SUPERHYDRIDE_BUS_G: u64   = 215_000;  // 215.00 kg (Fm-3m)
pub const MASS_SUB4_FRACTON_SHIELD_G: u64     = 145_000;  // 145.00 kg (P6/mmm)
pub const MASS_SUB5_MONOLITH_CAVITY_G: u64    = 160_000;  // 160.00 kg (Fd-3m)
pub const MASS_SUB6_KAGOME_CFT_BUS_G: u64     = 52_500;   // 52.50 kg (R-3m)
pub const MASS_SUB7_PT_WEYL_CROWBAR_G: u64    = 40_000;   // 40.00 kg (F-43m)
pub const MASS_SUB8_SMES_RECOVERY_RING_G: u64 = 65_000;   // 65.00 kg (P6_3mc)
pub const MASS_NACELLE_TOTAL_G: u64           = 950_000;  // 950.00 kg (Exact)

/// Backwards compatibility aliases.
pub const MASS_SUB6_DARK_INJECTOR_G: u64      = MASS_SUB6_KAGOME_CFT_BUS_G;
pub const MASS_SUB7_8_SWITCH_SMES_G: u64      = MASS_SUB7_PT_WEYL_CROWBAR_G + MASS_SUB8_SMES_RECOVERY_RING_G; // 105.00 kg

pub const MASS_LEGACY_MAR312C_G: u64          = 2_257_700; // 2,257.70 kg

/// Exact 8-subsystem volumes in millilitres (cm^3) (summing to 350,000 mL = 0.3500 m^3).
pub const VOL_SUB1_SEED_DIE_ML: u64           = 1_200;    // 0.0012 m^3
pub const VOL_SUB2_DEC_X_ML: u64              = 110_000;  // 0.1100 m^3
pub const VOL_SUB3_SUPERHYDRIDE_BUS_ML: u64   = 68_000;   // 0.0680 m^3
pub const VOL_SUB4_FRACTON_SHIELD_ML: u64     = 38_000;   // 0.0380 m^3
pub const VOL_SUB5_MONOLITH_CAVITY_ML: u64    = 45_500;   // 0.0455 m^3
pub const VOL_SUB6_KAGOME_CFT_BUS_ML: u64     = 18_500;   // 0.0185 m^3
pub const VOL_SUB7_PT_WEYL_CROWBAR_ML: u64    = 27_000;   // 0.0270 m^3
pub const VOL_SUB8_SMES_RECOVERY_RING_ML: u64 = 41_800;   // 0.0418 m^3
pub const VOL_NACELLE_TOTAL_ML: u64           = 350_000;  // 0.3500 m^3
pub const VOL_LEGACY_MAR312C_ML: u64          = 858_900;  // 0.8589 m^3

/// Upgraded SHBT-EW-312 spacecraft dry mass with MAR-312X nacelle (`5,892.30 kg` vs `7,200.00 kg`).
pub const UPGRADED_SHIP_DRY_MASS_G: u64     = 5_892_300;
pub const LEGACY_SHIP_DRY_MASS_G: u64       = 7_200_000;

/// Single-frame ADM mass limit and fuel core parameters in grams.
pub const MASS_FUEL_CORE_G: u64             = 376_990; // 376.99 kg per core
pub const MAX_SINGLE_FRAME_CORES: u64       = 41;      // 15,456.59 kg fuel + 950.00 kg = 16,406.59 kg
pub const ADM_SINGLE_FRAME_CAP_G: u64       = 16_600_000; // 16,600.00 kg

/// Phase 4 fault bitmasks matching `include/shbt_swarm_mmio.h` v4.
pub const FAULT_ADM_IMBALANCE: u32            = 0x0000_0001;
pub const FAULT_BL_ANOMALY: u32               = 0x0000_0008;
pub const FAULT_CAYLEY_HASH: u32              = 0x0000_0040;
pub const FAULT_THERMAL_CLAMP: u32            = 0x0000_0080;
pub const FAULT_SUPERHYDRIDE_TETHER_LOSS: u32 = 0x0002_0000;
pub const FAULT_PURCELL_BANDGAP_DRIFT: u32    = 0x0004_0000;
pub const FAULT_DECX_CHERN_BACKSCATTER: u32   = 0x0008_0000;
pub const FAULT_FORGE_SHEAR_EXCEEDED: u32     = 0x0010_0000;

/// Operational Power Tiers of SHBT-MAR-312X.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mar312xPowerTier {
    Tier0QuiescentStandby = 0, // 0 W boil-off (t_1/2 -> inf), 999.05 kW LANR aux
    Tier1ContinuousCruise = 1, // 1.000 PW gross -> 992.00 TW net elec (v_eff = 99.3 c)
    Tier2SustainedBurst   = 2, // 10.00 PW gross -> 9.920 PW net elec (v_eff = 993.2 c)
    Tier3ExawattImpulse   = 3, // 1.000 EW gross -> 992.00 PW net elec (v_eff = 99,318 c)
    Tier4PlanetaryForge   = 4, // 25.00 EW gross -> 24.80 EW elec / 24.9625 EW direct bus
}

impl Mar312xPowerTier {
    /// Returns `(gross_power_tw, net_elec_power_tw, operating_temp_mk)` for each tier.
    #[inline]
    pub const fn telemetry_profile(self) -> (u64, u64, u32) {
        match self {
            Self::Tier0QuiescentStandby => (0, 0, 21_130),
            Self::Tier1ContinuousCruise => (1_000, 992, 21_130),
            Self::Tier2SustainedBurst   => (10_000, 9_920, 21_131),
            Self::Tier3ExawattImpulse   => (1_000_000, 992_000, 21_280),
            Self::Tier4PlanetaryForge   => (25_000_000, 24_800_000, 24_850),
        }
    }
}

/// Master Nacelle Mass, Volume & 41-Core Single-Frame ADM Configuration.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Mar312xExoticNacelleConfig {
    pub loaded_cores: u64,
}

impl Mar312xExoticNacelleConfig {
    #[inline]
    pub const fn total_nacelle_mass_g() -> u64 {
        MASS_SUB1_SEED_DIE_G
            + MASS_SUB2_DEC_X_G
            + MASS_SUB3_SUPERHYDRIDE_BUS_G
            + MASS_SUB4_FRACTON_SHIELD_G
            + MASS_SUB5_MONOLITH_CAVITY_G
            + MASS_SUB6_KAGOME_CFT_BUS_G
            + MASS_SUB7_PT_WEYL_CROWBAR_G
            + MASS_SUB8_SMES_RECOVERY_RING_G
    }

    #[inline]
    pub const fn total_nacelle_volume_ml() -> u64 {
        VOL_SUB1_SEED_DIE_ML
            + VOL_SUB2_DEC_X_ML
            + VOL_SUB3_SUPERHYDRIDE_BUS_ML
            + VOL_SUB4_FRACTON_SHIELD_ML
            + VOL_SUB5_MONOLITH_CAVITY_ML
            + VOL_SUB6_KAGOME_CFT_BUS_ML
            + VOL_SUB7_PT_WEYL_CROWBAR_ML
            + VOL_SUB8_SMES_RECOVERY_RING_ML
    }

    #[inline]
    pub const fn verify_single_frame_adm_closure(&self) -> Result<u64, u32> {
        let nacelle_g = Self::total_nacelle_mass_g();
        if nacelle_g != MASS_NACELLE_TOTAL_G {
            return Err(FAULT_ADM_IMBALANCE);
        }
        if self.loaded_cores > MAX_SINGLE_FRAME_CORES {
            return Err(FAULT_ADM_IMBALANCE);
        }
        let total_mass_g = nacelle_g + (self.loaded_cores * MASS_FUEL_CORE_G);
        if total_mass_g > ADM_SINGLE_FRAME_CAP_G {
            return Err(FAULT_ADM_IMBALANCE);
        }
        Ok(total_mass_g)
    }
}

const _: () = assert!(Mar312xExoticNacelleConfig::total_nacelle_mass_g() == MASS_NACELLE_TOTAL_G);
const _: () = assert!(Mar312xExoticNacelleConfig::total_nacelle_volume_ml() == VOL_NACELLE_TOTAL_ML);

/// WP-1.1: Subsystem 1 — Purcell-Inhibited Hyper-Diamond / Lonsdaleite-c-BN Seed Die.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PurcellHyperDiamondSeedDie {
    pub spontaneous_ldos_milli: u32,    // Strictly 0 (complete 3D photonic/phononic bandgap)
    pub purcell_factor_milli: u32,      // Strictly 0 => Gamma_spont == 0 => t_1/2 -> inf
    pub isomer_gateway_gain_milli: u32, // >= 85,400 (85.40 > 61.15 threshold)
    pub mossbauer_f_m_ppm: u32,         // 994,200 ppm (0.9942 at Debye temp 2,480 K)
    pub borrmann_eps_b_ppm: u32,        // 999,500 ppm (0.9995, tau_Borrmann = 1,542 > 1,500)
    pub active_shear_stress_mpa: u32,   // <= 4,800 MPa Frenkel tether limit (785 MPa at 25 EW)
}

impl PurcellHyperDiamondSeedDie {
    pub const MAX_ALLOWABLE_SHEAR_MPA: u32 = 4_800;
    pub const MIN_ISOMER_GAIN_MILLI: u32   = 61_150;

    #[inline]
    pub const fn canonical_tier4_forge() -> Self {
        Self {
            spontaneous_ldos_milli: 0,
            purcell_factor_milli: 0,
            isomer_gateway_gain_milli: 85_400,
            mossbauer_f_m_ppm: 994_200,
            borrmann_eps_b_ppm: 999_500,
            active_shear_stress_mpa: 785,
        }
    }

    #[inline]
    pub const fn validate_seed_die(&self) -> Result<(), u32> {
        if self.spontaneous_ldos_milli != 0 || self.purcell_factor_milli != 0 {
            return Err(FAULT_PURCELL_BANDGAP_DRIFT);
        }
        if self.isomer_gateway_gain_milli <= Self::MIN_ISOMER_GAIN_MILLI
            || self.mossbauer_f_m_ppm < 994_200
            || self.borrmann_eps_b_ppm < 999_500
        {
            return Err(FAULT_PURCELL_BANDGAP_DRIFT);
        }
        if self.active_shear_stress_mpa > Self::MAX_ALLOWABLE_SHEAR_MPA {
            return Err(FAULT_FORGE_SHEAR_EXCEEDED);
        }
        Ok(())
    }
}

/// WP-1.2: Subsystem 2 — Solid-State 4D Weyl-Polariton & PTC Direct Energy Converter (`SHBT-DEC-X`).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct WeylPtcDecXConverter {
    pub second_chern_number: i32,     // C_2 != 0 (quantized topological protection)
    pub backscatter_coeff_ppm: u32,   // Strictly 0 ppm
    pub brem_power_watts: u64,        // Strictly 0 W (E_KS = 352.48 eV PTC k-gap deceleration)
    pub zone_losses_ppm: [u32; 5],    // [2200, 1800, 1500, 1500, 1000] ppm => 8000 ppm (0.800%)
}

impl WeylPtcDecXConverter {
    pub const TARGET_EFFICIENCY_PPM: u32 = 992_000; // 99.200%
    /// 4-stage sensible heating ratio * 10^17: 2.13039486e-9 * 10^17 = 213_039_486
    pub const SENSIBLE_HEATING_RATIO_E17: u64 = 213_039_486;

    #[inline]
    pub const fn canonical() -> Self {
        Self {
            second_chern_number: 1,
            backscatter_coeff_ppm: 0,
            brem_power_watts: 0,
            zone_losses_ppm: [2200, 1800, 1500, 1500, 1000],
        }
    }

    #[inline]
    pub const fn compute_net_efficiency_ppm(&self) -> Result<u32, u32> {
        if self.second_chern_number == 0 || self.backscatter_coeff_ppm != 0 || self.brem_power_watts != 0 {
            return Err(FAULT_DECX_CHERN_BACKSCATTER);
        }
        let total_loss_ppm = self.zone_losses_ppm[0]
            + self.zone_losses_ppm[1]
            + self.zone_losses_ppm[2]
            + self.zone_losses_ppm[3]
            + self.zone_losses_ppm[4];
        let eff_ppm = 1_000_000u32.saturating_sub(total_loss_ppm);
        if eff_ppm < Self::TARGET_EFFICIENCY_PPM {
            return Err(FAULT_DECX_CHERN_BACKSCATTER);
        }
        Ok(eff_ppm)
    }
}

/// WP-1.3 & WP-1.5: Subsystem 3, 7 & 8 — Tethered Clathrate Superhydride Coils, Thermal Bus & SMES.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TetheredSuperhydrideCoilSystem {
    pub wyckoff_jitter_max_fm: u32,  // <= 10 fm (10^-4 Angstrom) at k_tether = 5.1295e10 N/m
    pub operating_field_mt: u32,     // 48,000 mT (48.0 T, B_c2 > 95 T)
    pub operating_temp_mk: u32,      // 21,130 mK (Tier 1..3) to 24,850 mK (Tier 4)
    pub critical_temp_mk: u32,       // 192,500 mK (192.50 K)
    pub weyl_crowbar_trip_fs: u32,   // 1,701 fs (1.701 ps <= 2.50 ns)
    pub smes_recovery_ppm: u32,      // 998,800 ppm (99.88%)
}

impl TetheredSuperhydrideCoilSystem {
    #[inline]
    pub const fn canonical_tier4() -> Self {
        Self {
            wyckoff_jitter_max_fm: 10,
            operating_field_mt: 48_000,
            operating_temp_mk: 24_850,
            critical_temp_mk: 192_500,
            weyl_crowbar_trip_fs: 1_701,
            smes_recovery_ppm: 998_800,
        }
    }

    #[inline]
    pub const fn validate_coils_and_thermal(&self) -> Result<u32, u32> {
        if self.wyckoff_jitter_max_fm > 10 || self.operating_field_mt < 48_000 {
            return Err(FAULT_SUPERHYDRIDE_TETHER_LOSS);
        }
        if self.critical_temp_mk != 192_500 || self.operating_temp_mk >= self.critical_temp_mk {
            return Err(FAULT_THERMAL_CLAMP);
        }
        if self.weyl_crowbar_trip_fs > 2_500_000 || self.smes_recovery_ppm < 998_800 {
            return Err(FAULT_SUPERHYDRIDE_TETHER_LOSS);
        }
        Ok(self.critical_temp_mk - self.operating_temp_mk)
    }
}

/// WP-3.2: Direct Boundary-Conformal Annihilation-to-Synthesis Bus (`B_direct^MAR-X`).
pub struct DirectBoundaryTransmutationBus;

impl DirectBoundaryTransmutationBus {
    pub const DIRECT_BUS_EFF_PPM: u64          = 998_500; // 99.850%
    pub const UPGRADED_SHIP_SYNTH_WORK_PJ: u64 = 334_948; // 334.9477 PJ for 5,892.30 kg ship
    pub const LEGACY_SHIP_SYNTH_WORK_PJ: u64   = 409_284; // 409.2841 PJ for 7,200.00 kg ship

    /// Computes the daughter spacecraft replication time in microseconds at Tier 4 (25.00 EW gross)
    /// and verifies ADM mass and (B-L) conservation.
    #[inline]
    pub const fn verify_tier4_replication_us(
        feedstock_mass_ug: u64,
        target_ship_mass_ug: u64,
        net_b_minus_l: i64,
        use_upgraded_5892kg_hull: bool,
    ) -> Result<u64, u32> {
        if feedstock_mass_ug != target_ship_mass_ug {
            return Err(FAULT_ADM_IMBALANCE);
        }
        if net_b_minus_l != 0 {
            return Err(FAULT_BL_ANOMALY);
        }
        let work_pj = if use_upgraded_5892kg_hull {
            Self::UPGRADED_SHIP_SYNTH_WORK_PJ
        } else {
            Self::LEGACY_SHIP_SYNTH_WORK_PJ
        };
        let t_rep_us = (work_pj * 1_000) / 24_962;
        Ok(t_rep_us) // 13,418 us (13.42 ms) for 5,892.30 kg; 16,396 us (16.40 ms) for 7,200 kg
    }
}

/// WP-2.2: Autopoietic Self-Healing Engine (`A_heal` at 50.518 kHz).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct AutopoieticSelfHealingEngine {
    pub cumulative_heal_cycles: u64,
}

impl AutopoieticSelfHealingEngine {
    /// Executes a single 19.795 us tick scrub against Read-Only Parent Quine ROM (Banks 00..09),
    /// applying zero-heat Tomita-Takesaki inverse modular flow (Q_internal == 0 J/bit) when needed.
    #[inline]
    pub const fn execute_tick_scrub(
        &mut self,
        live_reactor_cayley_hash: [u64; 3],
        rom_quine_cayley_hash: [u64; 3],
        wyckoff_excursion_fm: u32,
    ) -> Result<(u64, u64), u32> {
        if wyckoff_excursion_fm > 10 {
            return Err(FAULT_SUPERHYDRIDE_TETHER_LOSS);
        }
        let hash_match = live_reactor_cayley_hash[0] == rom_quine_cayley_hash[0]
            && live_reactor_cayley_hash[1] == rom_quine_cayley_hash[1]
            && live_reactor_cayley_hash[2] == rom_quine_cayley_hash[2];

        self.cumulative_heal_cycles = self.cumulative_heal_cycles.saturating_add(1);
        let q_internal_joules_per_bit = 0u64;
        if !hash_match {
            return Err(FAULT_CAYLEY_HASH);
        }
        Ok((self.cumulative_heal_cycles, q_internal_joules_per_bit))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nacelle_mass_and_41_core_closure() {
        let cfg = Mar312xExoticNacelleConfig { loaded_cores: 41 };
        assert_eq!(cfg.verify_single_frame_adm_closure(), Ok(16_406_590));
        let bad_cfg = Mar312xExoticNacelleConfig { loaded_cores: 42 };
        assert_eq!(bad_cfg.verify_single_frame_adm_closure(), Err(FAULT_ADM_IMBALANCE));
        assert_eq!(Mar312xExoticNacelleConfig::total_nacelle_mass_g(), 950_000);
        assert_eq!(Mar312xExoticNacelleConfig::total_nacelle_volume_ml(), 350_000);
    }

    #[test]
    fn test_seed_die_decx_and_coils() {
        let die = PurcellHyperDiamondSeedDie::canonical_tier4_forge();
        assert_eq!(die.validate_seed_die(), Ok(()));

        let decx = WeylPtcDecXConverter::canonical();
        assert_eq!(decx.compute_net_efficiency_ppm(), Ok(992_000));

        let coils = TetheredSuperhydrideCoilSystem::canonical_tier4();
        assert_eq!(coils.validate_coils_and_thermal(), Ok(167_650));
    }

    #[test]
    fn test_direct_bus_and_autopoietic_healing() {
        let t_5892 = DirectBoundaryTransmutationBus::verify_tier4_replication_us(
            5_892_300_000_000,
            5_892_300_000_000,
            0,
            true,
        )
        .unwrap();
        assert_eq!(t_5892, 13_418); // 13.42 ms < 15 ms

        let t_7200 = DirectBoundaryTransmutationBus::verify_tier4_replication_us(
            7_200_000_000_000,
            7_200_000_000_000,
            0,
            false,
        )
        .unwrap();
        assert_eq!(t_7200, 16_396); // 16.40 ms < 20 ms

        let mut engine = AutopoieticSelfHealingEngine { cumulative_heal_cycles: 0 };
        let h = [1, 2, 3];
        assert_eq!(engine.execute_tick_scrub(h, h, 10), Ok((1, 0)));
    }
}
