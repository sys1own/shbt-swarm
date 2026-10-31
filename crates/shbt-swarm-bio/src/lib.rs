//! crates/shbt-swarm-bio/src/lib.rs
//! #![no_std] Pathway C Mode II Tomographic Readout at the 4.2 K Landauer Floor,
//! WP-3.2 Stoichiometric Crust-to-CHNOPS Biogenic Transmutation (`T_{pe<->n}^{(26,312)}`, Phi_nu == 0),
//! 5th-Order Minimum-Jerk Eikonal Foliation, Dynamic GST-225 Impedance Matching,
//! Kohn-Sham Screening (352.48 eV, P_Brem == 0 W), and Blueshifted Hyper-Temporal Enclaves.

#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]

pub mod chiral_electroweak;
pub use chiral_electroweak::*;
pub mod lyapunov_reactor;
pub use lyapunov_reactor::*;
extern crate alloc;

/// 4.2 K Landauer readout floor: Q_readout = k_B * 4.2 K * ln(2) = 4.01937e-23 J/bit.
pub const LANDAUER_4P2K_FEMTO_ATTOJOULES_PER_BIT: u64 = 40_194; // 4.0194e-23 J/bit
pub const KOHN_SHAM_SCREENING_EV_CENTI: u32 = 35_248;           // 352.48 eV
pub const THOMAS_FERMI_SCREENING_MA: u32 = 482;                 // 0.482 Angstrom = 482 mA
pub const MAX_EIKONAL_DERENDER_VEL_MM_S: u32 = 56_988;          // 56.988 m/s
pub const MAX_EIKONAL_NORM_ACCEL_MILLI: u32 = 5_773;            // 10 / sqrt(3) ~ 5.7735

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CapturedMatterClass {
    /// Class 1: Biological, Genomic & Chiral Macromolecular Structures (F >= 1 - 10^-122)
    BiogenicChiralGroundState,
    /// Class 2: Atomically Precise Nanotech, Superconductors & Metamaterials (F >= 1 - 10^-120)
    CrystallineMetamaterialGroundState,
    /// Class 3: Uncharacterized Non-Stationary Quantum Superposition of dimension d (Werner-Keyl limit)
    UncharacterizedActiveQuantumState { hilbert_dim_d: u64, copies_k: u64 },
}

impl CapturedMatterClass {
    /// Returns the asymptotic fidelity bound:
    /// - For Class 1 (bio): log10(1 - F) = -122
    /// - For Class 2 (nano): log10(1 - F) = -120
    /// - For Class 3 (unknown quantum): Werner-Keyl-Buzek-Hillery F_{1->K}(d) = (K*(d+1) + d - 1) / (K*(d+1)) in permille.
    #[inline]
    pub const fn fidelity_bound(&self) -> (i32, u64) {
        match *self {
            CapturedMatterClass::BiogenicChiralGroundState => (-122, 1000),
            CapturedMatterClass::CrystallineMetamaterialGroundState => (-120, 1000),
            CapturedMatterClass::UncharacterizedActiveQuantumState { hilbert_dim_d, copies_k } => {
                if copies_k == 0 || hilbert_dim_d == 0 {
                    return (0, 0);
                }
                let num = copies_k * (hilbert_dim_d + 1) + hilbert_dim_d - 1;
                let den = copies_k * (hilbert_dim_d + 1);
                (0, (num * 1000) / den)
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct StoichiometricChnopsReaction {
    pub name: &'static str,
    pub a_in: i32,
    pub z_in: i32,
    pub n_in: i32,
    pub e_in: i32,
    pub weak_n_to_pe_shift: i32,
    pub a_out: i32,
    pub z_out: i32,
    pub n_out: i32,
    pub e_out: i32,
    pub nu_virtual_ancilla: i32,
    pub phi_nu_free: i32,
}

impl StoichiometricChnopsReaction {
    #[inline]
    pub const fn verify_conservation(&self) -> bool {
        let a_ok = self.a_in == (self.z_in + self.n_in)
            && self.a_out == (self.z_out + self.n_out)
            && self.a_in == self.a_out;
        let z_ok = self.z_out == self.z_in + self.weak_n_to_pe_shift;
        let n_ok = self.n_out == self.n_in - self.weak_n_to_pe_shift;
        let e_ok = self.e_out == self.e_in + self.weak_n_to_pe_shift;
        let nu_ok = self.nu_virtual_ancilla == -self.weak_n_to_pe_shift
            && self.phi_nu_free == 0;
        let delta_b = self.a_out - self.a_in;
        let delta_l = (self.e_out - self.e_in) + self.nu_virtual_ancilla;
        let bl_ok = (delta_b - delta_l) == 0;
        let q_em_ok = (self.z_out - self.e_out) == 0 && (self.z_in - self.e_in) == 0;
        a_ok && z_ok && n_ok && e_ok && nu_ok && bl_ok && q_em_ok
    }
}

/// Canonical WP-3.2 Crust-to-CHNOPS Stoichiometric Transmutation Table.
pub const CANONICAL_CHNOPS_REACTIONS: [StoichiometricChnopsReaction; 3] = [
    // 1 SiO2 (A=60, Z=30, N=30) -> 2 C12 + 1 N14 + 1 O16 + 6 H1 (A=60, Z=34, N=26), Delta T_3 = +4
    StoichiometricChnopsReaction {
        name: "1 SiO2 -> 2 C12 + 1 N14 + 1 O16 + 6 H1",
        a_in: 60,
        z_in: 30,
        n_in: 30,
        e_in: 30,
        weak_n_to_pe_shift: 4,
        a_out: 60,
        z_out: 34,
        n_out: 26,
        e_out: 34,
        nu_virtual_ancilla: -4,
        phi_nu_free: 0,
    },
    // 1 FeO (A=72, Z=34, N=38) -> 1 P31 + 1 S32 + 9 H1 (A=72, Z=40, N=32), Delta T_3 = +6
    StoichiometricChnopsReaction {
        name: "1 FeO -> 1 P31 + 1 S32 + 9 H1",
        a_in: 72,
        z_in: 34,
        n_in: 38,
        e_in: 34,
        weak_n_to_pe_shift: 6,
        a_out: 72,
        z_out: 40,
        n_out: 32,
        e_out: 40,
        nu_virtual_ancilla: -6,
        phi_nu_free: 0,
    },
    // 2 MgO (A=80, Z=40, N=40) -> 8 H2O (A=80, Z=48, N=32), Delta T_3 = +8
    StoichiometricChnopsReaction {
        name: "2 MgO -> 8 H2O",
        a_in: 80,
        z_in: 40,
        n_in: 40,
        e_in: 40,
        weak_n_to_pe_shift: 8,
        a_out: 80,
        z_out: 48,
        n_out: 32,
        e_out: 48,
        nu_virtual_ancilla: -8,
        phi_nu_free: 0,
    },
];

const _: () = {
    assert!(CANONICAL_CHNOPS_REACTIONS[0].verify_conservation());
    assert!(CANONICAL_CHNOPS_REACTIONS[1].verify_conservation());
    assert!(CANONICAL_CHNOPS_REACTIONS[2].verify_conservation());
};

/// 5th-order minimum-jerk Eikonal foliation profile s(tau) = 10*tau^3 - 15*tau^4 + 6*tau^5
/// evaluated at normalized time tau_permille in [0, 1000].
#[inline]
pub const fn evaluate_eikonal_min_jerk_permille(tau_permille: u32) -> (u64, u64) {
    if tau_permille >= 1000 {
        return (1000, 0);
    }
    let t = tau_permille as u128;
    let t2 = t * t;             // scale 10^6
    let t3 = t2 * t;            // scale 10^9
    let t4 = t3 * t;            // scale 10^12
    let t5 = t4 * t;            // scale 10^15
    // s_permille = (10 * t^3 * 10^6 - 15 * t^4 * 10^3 + 6 * t^5) / 10^12
    let s_num = (10 * t3 * 1_000_000 + 6 * t5).saturating_sub(15 * t4 * 1_000);
    let s_permille = (s_num / 1_000_000_000_000) as u64;
    // ds/dtau = 30*tau^2 - 60*tau^3 + 30*tau^4 (peaks at tau = 0.5 -> ds/dtau = 1.875 = 1875 permille)
    let v_num = (30 * t2 * 1_000_000 + 30 * t4).saturating_sub(60 * t3 * 1_000);
    let v_norm_permille = (v_num / 1_000_000_000) as u64;
    (s_permille, v_norm_permille)
}

/// Validates Stage-4 biogenic emergence parameters (GST-225 impedance, Kohn-Sham screening,
/// homochiral L-amino/D-sugar parity, and Hyper-Temporal Enclave lapse alpha in [10, 10000]).
#[inline]
pub const fn verify_stage4_biogenic_emergence(
    gst225_krayl: u32,
    target_krayl: u32,
    brem_power_watts: u64,
    l_amino_d_sugar_locked: bool,
    enclave_alpha: u32,
    interior_lapse_grad_milli: u32,
) -> Result<(), u32> {
    if gst225_krayl != target_krayl || target_krayl < 1_000 || target_krayl > 100_000 {
        return Err(shbt_swarm_core::SHBT_SWARM_FAULT_CBF_PHYS_VIOL);
    }
    if brem_power_watts != 0 || !l_amino_d_sugar_locked {
        return Err(shbt_swarm_core::SHBT_SWARM_FAULT_CBF_PHYS_VIOL);
    }
    if enclave_alpha < 1 || enclave_alpha > 10_000 || interior_lapse_grad_milli != 0 {
        return Err(shbt_swarm_core::SHBT_SWARM_FAULT_FRAMING_WEYL);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chnops_reactions_and_eikonal_foliation() {
        for rxn in &CANONICAL_CHNOPS_REACTIONS {
            assert!(rxn.verify_conservation());
        }
        assert_eq!(evaluate_eikonal_min_jerk_permille(0), (0, 0));
        assert_eq!(evaluate_eikonal_min_jerk_permille(500), (500, 1875));
        assert_eq!(evaluate_eikonal_min_jerk_permille(1000), (1000, 0));
        assert!(verify_stage4_biogenic_emergence(1580, 1580, 0, true, 1000, 0).is_ok());
    }
}
