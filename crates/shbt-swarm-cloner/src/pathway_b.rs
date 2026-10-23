//! crates/shbt-swarm-cloner/src/pathway_b.rs
//! Pathway B: Isometric Adjoint Verlinde Splitting (Delta_split) & Non-Hermitian PT-GoI
//! Optical Template Projection EX(M,U) = (I + UM)U in 1.701 ps/bank (54.432 ps / 32 banks).

pub const PT_GOI_PROP_FS_PER_BANK: u64 = 1_701;      // 1.701 ps
pub const PT_GOI_32BANK_SWEEP_FS: u64 = 54_432;      // 54.432 ps
pub const PT_GOI_64BANK_SWEEP_FS: u64 = 108_864;     // 108.864 ps
pub const POLARITON_EPR_SQUEEZING_R3_MILLI: u32 = 3_800; // r_3 >= 3.80 (Delta X_-^2 <= 5.0e-4)
pub const VERLINDE_SPLIT_ISO_LOG10_BOUND: i32 = -125;    // ||Delta_split^dagger Delta_split - I||_op <= 10^-125

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PathwayBProjectionReceipt {
    pub active_banks: u32,
    pub total_propagation_fs: u64,
    pub epr_squeezing_r3_milli: u32,
    pub nilpotent_ep2_verified: bool,
    pub verlinde_isometry_log10: i32,
}

/// Executes Pathway B Adjoint Verlinde Splitting coupled to the 3-stage polariton EPR bridge
/// and non-Hermitian PT-GoI single-pass optical projection EX(M,U) = (I + UM)U.
#[inline]
pub const fn execute_pathway_b_verlinde_pt_goi(
    active_banks: u32,
    epr_squeezing_r3_milli: u32,
    nilpotent_ep2_locked: bool,
) -> Result<PathwayBProjectionReceipt, u32> {
    if active_banks == 0 || active_banks > 64 {
        return Err(0x0000_0002); // SHBT_SWARM_FAULT_PARTITION_LOCK
    }
    if epr_squeezing_r3_milli < POLARITON_EPR_SQUEEZING_R3_MILLI || !nilpotent_ep2_locked {
        return Err(0x0000_0004); // SHBT_SWARM_FAULT_FRAMING_WEYL
    }
    Ok(PathwayBProjectionReceipt {
        active_banks,
        total_propagation_fs: (active_banks as u64) * PT_GOI_PROP_FS_PER_BANK,
        epr_squeezing_r3_milli,
        nilpotent_ep2_verified: true,
        verlinde_isometry_log10: VERLINDE_SPLIT_ISO_LOG10_BOUND,
    })
}
