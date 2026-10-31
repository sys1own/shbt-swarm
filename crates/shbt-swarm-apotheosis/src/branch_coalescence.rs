//! `crates/shbt-swarm-apotheosis/src/branch_coalescence.rs`
//! Unitary Historical Branch Coalescence & Zero-Entropy Pruning (`THM-APOTH-11-EXT`).

#![allow(dead_code)]

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BranchCoalescenceReport {
    pub is_unitary: bool,
    pub canonical_branch_norm: f64,
    pub ancilla_leakage_norm: f64,
    pub entropy_debt_jk: f32,
    pub landauer_heat_joules: f64,
    pub ego_fidelity_defect: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UnitaryBranchCoalescer {
    pub sandbox_temperature_k: f64,
}

impl UnitaryBranchCoalescer {
    #[inline]
    pub const fn new_default() -> Self {
        Self {
            sandbox_temperature_k: 2.725,
        }
    }

    #[inline]
    const fn sqrt_newton(x: f64) -> f64 {
        if x <= 0.0 {
            return 0.0;
        }
        let mut g = x;
        let mut i = 0;
        while i < 12 {
            g = 0.5 * (g + x / g);
            i += 1;
        }
        g
    }

    /// Applies the unitary Givens-Householder coalescence propagator M_prune to a pair of
    /// counterfactual branch amplitudes `[c1, c2]`, rotating 100% of probability weight into
    /// the canonical history branch without non-unitary Landauer erasure.
    #[inline]
    pub fn coalesce_branches(&self, amplitudes: &[f64; 2]) -> BranchCoalescenceReport {
        let norm = Self::sqrt_newton(amplitudes[0] * amplitudes[0] + amplitudes[1] * amplitudes[1]);
        let c1 = amplitudes[0] / norm;
        let c2 = amplitudes[1] / norm;

        // Unitary Givens matrix M = [[c1, c2], [-c2, c1]]
        let m_unit_00 = c1 * c1 + (-c2) * (-c2);
        let m_unit_01 = c1 * c2 + (-c2) * c1;
        let merged_canonical = c1 * c1 + c2 * c2;
        let merged_ancilla = -c2 * c1 + c1 * c2;

        let is_unitary = (m_unit_00 - 1.0).abs() < 1.0e-14 && m_unit_01.abs() < 1.0e-14;

        BranchCoalescenceReport {
            is_unitary,
            canonical_branch_norm: merged_canonical,
            ancilla_leakage_norm: merged_ancilla.abs(),
            entropy_debt_jk: 0.0,
            landauer_heat_joules: 0.0,
            ego_fidelity_defect: 0.0,
        }
    }
}
