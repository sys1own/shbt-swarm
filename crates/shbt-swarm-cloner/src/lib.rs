//! crates/shbt-swarm-cloner/src/lib.rs
//! #![no_std] Phase 1 Macroscopic Anyon-Braid Payload Cloner (`R_clone^SWARM`) vs.
//! Formal SHBT No-Cloning Theorems (THM-CLONE-01..04).

#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]

pub mod pathway_a;
pub mod pathway_b;
pub mod ship_ledger;

pub use pathway_a::*;
pub use pathway_b::*;
pub use ship_ledger::*;
use shbt_swarm_core::*;

/// Theorem 1.1 & Theorem 1.2 Hardware Guards:
/// - Rejects unknown quantum superpositions with inner-product overlap |<psi_1|psi_2>| not in {0, 1}
///   (e.g., golden-ratio inverse phi^-1 ~ 0.618034, which induces isometric defect >= 0.381966 >> 10^-120 and E_munu != 0).
/// - Rejects vacuum duplication (M_feedstock == 0 or M_feedstock != M_payload), rational partition
///   violation (eta_A : eta_D != 10 : 23), framing anomaly (Delta_fr != 0), or (B-L) violation.
#[inline]
pub const fn verify_no_cloning_and_adm_guards(
    overlap_permille: u32,
    feedstock_mass_ug: u64,
    payload_mass_ug: u64,
    vis_anchor_bytes: usize,
    dark_ledger_bytes: usize,
    net_b_minus_l: i64,
    framing_anomaly_milli: i64,
) -> Result<(), u32> {
    let mut fault: u32 = 0;

    // Theorem 1.1: Wootters-Zurek / MTC fusion linearity requires overlap in {0, 1000} permille
    if overlap_permille != 0 && overlap_permille != 1000 {
        fault |= SHBT_SWARM_FAULT_FRAMING_WEYL;
    }

    // Theorem 1.2 (a): Asymptotic ADM mass conservation
    if feedstock_mass_ug == 0 || feedstock_mass_ug != payload_mass_ug {
        fault |= SHBT_SWARM_FAULT_ADM_IMBALANCE;
    }

    // Theorem 1.2 (b): Rational capacity partition eta_A : eta_D == 10 : 23 (640 B : 1472 B)
    if vis_anchor_bytes != SHBT_VISIBLE_ANCHOR_BYTES
        || dark_ledger_bytes != SHBT_DARK_LEDGER_BYTES
        || (vis_anchor_bytes * 23 != dark_ledger_bytes * 10)
    {
        fault |= SHBT_SWARM_FAULT_PARTITION_LOCK;
    }

    // Theorem 1.2 (c): Exact modular framing cancellation Delta_fr == 0 <=> E_munu == 0
    if framing_anomaly_milli != 0 {
        fault |= SHBT_SWARM_FAULT_FRAMING_WEYL;
    }

    // Theorem 1.2 (d): Global SO(10) (B-L) gauge superselection
    if net_b_minus_l != 0 {
        fault |= SHBT_SWARM_FAULT_BL_ANOMALY;
    }

    if fault != 0 {
        Err(fault)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thm_1_1_and_1_2_no_cloning_guards() {
        // Reject golden-ratio superposition overlap phi^-1 ~ 618 permille
        assert_eq!(
            verify_no_cloning_and_adm_guards(618, 7_200_000_000, 7_200_000_000, 640, 1472, 0, 0),
            Err(SHBT_SWARM_FAULT_FRAMING_WEYL)
        );
        // Reject vacuum duplication (M_feedstock = 0)
        assert_eq!(
            verify_no_cloning_and_adm_guards(0, 0, 7_200_000_000, 640, 1472, 0, 0),
            Err(SHBT_SWARM_FAULT_ADM_IMBALANCE)
        );
        // Reject partition ratio 20:13
        assert_eq!(
            verify_no_cloning_and_adm_guards(0, 1000, 1000, 1280, 832, 0, 0),
            Err(SHBT_SWARM_FAULT_PARTITION_LOCK)
        );
        // Approve orthogonal computational basis state (overlap = 0) with balanced feedstock
        assert_eq!(
            verify_no_cloning_and_adm_guards(0, 7_200_000_000, 7_200_000_000, 640, 1472, 0, 0),
            Ok(())
        );
    }

    #[test]
    fn test_pathway_a_and_b_execution() {
        let mut hyper = UnifiedStinespringHyperFrame::default();
        for b in 0..32usize {
            let mut ctrl = unsafe { hyper.parent_quine_rom[b].ctrl_view };
            for k in 0..124usize {
                let w = 0x55AA_F00F_1234_5678u64 ^ ((b as u64) << 32) ^ (k as u64);
                ctrl.braid_descriptors[k] = w;
                ctrl.secded_parity[k] = encode_hamming72_64(w);
            }
            hyper.parent_quine_rom[b].ctrl_view = ctrl;
        }
        let receipt_a = execute_pathway_a_quine_copy(&mut hyper).unwrap();
        assert_eq!(receipt_a.banks_copied, 32);
        assert_eq!(receipt_a.braids_verified, 3968);
        assert_eq!(receipt_a.q_internal_joules_per_bit, 0);

        let receipt_b = execute_pathway_b_verlinde_pt_goi(32, 3820, true).unwrap();
        assert_eq!(receipt_b.total_propagation_fs, 54_432);
    }
}
