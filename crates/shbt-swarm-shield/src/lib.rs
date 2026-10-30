//! crates/shbt-swarm-shield/src/lib.rs
//! #![no_std] Proof-Carrying Linear<V> Control Barrier / Lyapunov Function (CBF/CLF) Safety Shield
//! and TTN-MERA Xenobiological Triage Calculus (`J_xeno`).

#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]

pub mod hocbf_engine;
pub mod poset_sheaf;
pub use hocbf_engine::{BarrierConfig, HocbfEngine, LinearConstraint, PhysicalState, QpFilterResult};
pub use poset_sheaf::{CausalDiamond, PosetSheafEngine, SymplecticShear4x4, HOMOLOGY_RANK};

pub const C_CRIT_SAPIENT_Q16: u64 = 1_000_000 << 16; // C_crit^sapient = 10^6 nat-layers
pub const W_SYNTH_REGOLITH_KJ_PER_UG: u64 = 56_845;  // 56.845009 TJ/kg
pub const W_SYNTH_X2_BIOMASS_KJ_PER_UG: u64 = 425;   // 0.424815 TJ/kg
pub const DELTA_W_SYNTH_SAVED_MILLI_TJ_KG: u64 = 56_420; // 56.420194 TJ/kg saved (99.25% reduction)

// Canonical Earth-habitable CLF setpoints
pub const TARGET_TEMP_SURF_CENTI_K: u32 = 28_815; // 288.15 K (15.0 C)
pub const TARGET_PN2_PA: u32 = 78_084;            // 78.084 kPa
pub const TARGET_PO2_PA: u32 = 20_946;            // 20.946 kPa
pub const TARGET_PCO2_PA: u32 = 40;               // 0.040 kPa (400 ppm)
pub const TARGET_PH2O_PA: u32 = 1_013;            // 1.013 kPa
pub const TARGET_OCEAN_PH_CENTI: u32 = 810;       // pH 8.10
pub const TARGET_OCEAN_SALINITY_DECIG_KG: u32 = 350; // 35.0 g/kg
pub const MIN_DIPOLE_FIELD_NT: u32 = 30_000;      // 30.0 uT = 30,000 nT

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum XenoTriageClass {
    M0AbioticRegolith  = 0x0000_0000,
    X0ProtectedEarth   = 0x1000_0000,
    X1ProtectedSapient = 0x2000_0000,
    X2AssimilablePrim  = 0x4000_0000,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ReactorGeneration {
    Mar312cCompacted, // T_MAR <= 32,920 mK (32.92 K)
    Mar312xExotic,    // T_MAR <= 192,500 mK (192.50 K)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ShieldVerdict {
    Approved {
        cmd_opcode: u32,
        energy_saved_tj_per_kg_milli: u64,
    },
    BlockedFault {
        fault_mask: u32,
    },
}

/// WP-4.1 & WP-4.2: Proof-Carrying Linear<V> CBF/CLF Safety Shield & Xenobiological Triage.
pub struct LinearVCbfSafetyShield;

impl LinearVCbfSafetyShield {
    #[inline]
    pub const fn classify_xenobiology(d_cayley: u32, c_mera_q16: u64) -> XenoTriageClass {
        if d_cayley == 0 {
            XenoTriageClass::X0ProtectedEarth
        } else if c_mera_q16 >= C_CRIT_SAPIENT_Q16 {
            XenoTriageClass::X1ProtectedSapient
        } else if c_mera_q16 > 0 {
            XenoTriageClass::X2AssimilablePrim
        } else {
            XenoTriageClass::M0AbioticRegolith
        }
    }

    /// Evaluates the 9-barrier physical safety invariant h_phys(x) >= 0, the anti-grey-goo
    /// population cap h_pop(M) >= 0, the habitability CLF dV_terra/dt <= -kappa * V_terra,
    /// and the xenobiological triage lockouts (X0, X1, X2, M0).
    #[allow(clippy::too_many_arguments)]
    pub const fn verify_and_filter_command(
        requested_cmd: u32,
        feedstock_ug: u64,
        payload_ug: u64,
        net_b_minus_l: i64,
        weyl_e_munu_norm: u64,
        phi_nu_flux: u64,
        p_brem_watts: u64,
        eikonal_accel_milli: u32,
        temp_qc_mk: u32,
        temp_mar_mk: u32,
        active_nodes_m: u32,
        max_nodes_m_max: u32,
        v_terra_q32: u64,
        dv_terra_dt_q32: i64,
        d_cayley: u32,
        c_mera_q16: u64,
    ) -> ShieldVerdict {
        Self::verify_and_filter_command_with_gen(
            requested_cmd,
            feedstock_ug,
            payload_ug,
            net_b_minus_l,
            weyl_e_munu_norm,
            phi_nu_flux,
            p_brem_watts,
            eikonal_accel_milli,
            temp_qc_mk,
            temp_mar_mk,
            ReactorGeneration::Mar312cCompacted,
            active_nodes_m,
            max_nodes_m_max,
            v_terra_q32,
            dv_terra_dt_q32,
            d_cayley,
            c_mera_q16,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub const fn verify_and_filter_command_with_gen(
        requested_cmd: u32,
        feedstock_ug: u64,
        payload_ug: u64,
        net_b_minus_l: i64,
        weyl_e_munu_norm: u64,
        phi_nu_flux: u64,
        p_brem_watts: u64,
        eikonal_accel_milli: u32,
        temp_qc_mk: u32,
        temp_mar_mk: u32,
        reactor_gen: ReactorGeneration,
        active_nodes_m: u32,
        max_nodes_m_max: u32,
        v_terra_q32: u64,
        dv_terra_dt_q32: i64,
        d_cayley: u32,
        c_mera_q16: u64,
    ) -> ShieldVerdict {
        let mut fault: u32 = 0;

        // 1. Spacetime & Hardware Invariance Barrier h_phys(x) >= 0
        if feedstock_ug != payload_ug {
            fault |= 0x0000_0001; // SHBT_SWARM_FAULT_ADM_IMBALANCE
        }
        if weyl_e_munu_norm != 0 {
            fault |= 0x0000_0004; // SHBT_SWARM_FAULT_FRAMING_WEYL
        }
        if net_b_minus_l != 0 {
            fault |= 0x0000_0008; // SHBT_SWARM_FAULT_BL_ANOMALY
        }
        if phi_nu_flux != 0 {
            fault |= 0x0000_0010; // SHBT_SWARM_FAULT_NEUTRINO_LEAK
        }
        if p_brem_watts != 0 || eikonal_accel_milli > 5773 {
            fault |= 0x0000_0100; // SHBT_SWARM_FAULT_CBF_PHYS_VIOL
        }
        let mar_limit_mk = match reactor_gen {
            ReactorGeneration::Mar312cCompacted => 32_920,
            ReactorGeneration::Mar312xExotic => 192_500,
        };
        if temp_qc_mk > 4200 || temp_mar_mk > mar_limit_mk {
            fault |= 0x0000_0080; // SHBT_SWARM_FAULT_THERMAL_CLAMP
        }

        // 2. Anti-Grey-Goo Fleet Population Barrier h_pop(M) >= 0
        let is_clone_ship = (requested_cmd & 0x0000_0008) != 0;
        if active_nodes_m > max_nodes_m_max || (is_clone_ship && active_nodes_m >= max_nodes_m_max)
        {
            fault |= 0x0000_0200; // SHBT_SWARM_FAULT_GREY_GOO_CAP
        }

        // 3. Planetary Habitability CLF Convergence dV_terra/dt <= -kappa * V_terra
        if v_terra_q32 > 0 && dv_terra_dt_q32 >= 0 {
            fault |= 0x0000_0400; // SHBT_SWARM_FAULT_CLF_DIVERGENCE
        }

        // 4. Xenobiological Triage Invariant (Classes X0, X1, X2)
        let triage = Self::classify_xenobiology(d_cayley, c_mera_q16);
        let is_derender_or_assimilate =
            (requested_cmd & (0x0000_0004 | 0x0000_0010 | 0x0000_1000)) != 0;
        if is_derender_or_assimilate {
            match triage {
                XenoTriageClass::X0ProtectedEarth => {
                    fault |= 0x0000_0800; // SHBT_SWARM_FAULT_X0_PROTECT_LCK
                }
                XenoTriageClass::X1ProtectedSapient => {
                    fault |= 0x0000_1000; // SHBT_SWARM_FAULT_X1_SAPIENT_LCK
                }
                XenoTriageClass::X2AssimilablePrim | XenoTriageClass::M0AbioticRegolith => {}
            }
        }

        if fault != 0 {
            ShieldVerdict::BlockedFault { fault_mask: fault }
        } else {
            let saved = match triage {
                XenoTriageClass::X2AssimilablePrim => {
                    W_SYNTH_REGOLITH_KJ_PER_UG - W_SYNTH_X2_BIOMASS_KJ_PER_UG
                }
                _ => 0,
            };
            ShieldVerdict::Approved {
                cmd_opcode: requested_cmd,
                energy_saved_tj_per_kg_milli: saved,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xeno_triage_and_grey_goo_barriers() {
        assert_eq!(
            LinearVCbfSafetyShield::classify_xenobiology(0, 0),
            XenoTriageClass::X0ProtectedEarth
        );
        assert_eq!(
            LinearVCbfSafetyShield::classify_xenobiology(5, C_CRIT_SAPIENT_Q16),
            XenoTriageClass::X1ProtectedSapient
        );
        assert_eq!(
            LinearVCbfSafetyShield::classify_xenobiology(5, 1024),
            XenoTriageClass::X2AssimilablePrim
        );
        assert_eq!(
            LinearVCbfSafetyShield::classify_xenobiology(5, 0),
            XenoTriageClass::M0AbioticRegolith
        );

        // Reject Class X0 de-render
        let v_x0 = LinearVCbfSafetyShield::verify_and_filter_command(
            0x0000_0004,
            1000,
            1000,
            0,
            0,
            0,
            0,
            5000,
            4200,
            21130,
            10,
            510_000,
            100,
            -5,
            0,
            0,
        );
        assert_eq!(v_x0, ShieldVerdict::BlockedFault { fault_mask: 0x0000_0800 });

        // Approve Class X2 assimilation with 56.420 TJ/kg savings
        let v_x2 = LinearVCbfSafetyShield::verify_and_filter_command(
            0x0000_1000,
            1000,
            1000,
            0,
            0,
            0,
            0,
            5000,
            4200,
            21130,
            10,
            510_000,
            100,
            -5,
            12,
            1024,
        );
        assert_eq!(
            v_x2,
            ShieldVerdict::Approved {
                cmd_opcode: 0x0000_1000,
                energy_saved_tj_per_kg_milli: 56_420
            }
        );
    }

    #[test]
    fn test_poset_sheaf_consensus_engine() {
        let mut engine = PosetSheafEngine::<8, 16>::new();
        let d0 = CausalDiamond { diamond_id: 0, lapse_scaled_tau: 1000, spatial_cell_id: 1 };
        let d1 = CausalDiamond { diamond_id: 1, lapse_scaled_tau: 2000, spatial_cell_id: 1 };

        let idx0 = engine.add_diamond(d0, [10, 0, 0, 0]).unwrap();
        let idx1 = engine.add_diamond(d1, [10, 0, 0, 0]).unwrap();

        // Parabolic integer shear S = 0 => T = I (unipotent, symplectic, rho = 1)
        let shear = SymplecticShear4x4::identity();
        assert!(shear.is_unipotent());
        assert!(shear.is_symplectic());
        assert!(shear.verify_spectral_radius_unity());

        engine.add_causal_relation(idx0, idx1, shear).unwrap();
        assert!(engine.verify_global_section());
        assert!(engine.verify_h1_obstruction_vanishing());
    }
}
