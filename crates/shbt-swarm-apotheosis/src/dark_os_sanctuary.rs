//! Dark-OS Class X0 CRT recall and Class X1 Frobenius sanctuary fork engine.
#![forbid(unsafe_code)]

use shbt_swarm_logos::{CrtPrimaryModule, DarkOsKernel, VoaIsaOpcode, DARK_VOA_PRIMARIES};

/// Dark-OS Sugawara/KZ clock period for one sanctuary fork (`17.010000 as`).
pub const DARK_OS_SANCTUARY_FORK_FEMTO_AS: u64 = 17_010_000;
/// Minimum Class X1 streamed-fidelity floor `1 - 10^-122`.
pub const DARK_OS_CLASS_X1_STREAM_FIDELITY_FLOOR: f64 = 1.0 - 1.0e-122;

/// Executes native Dark-OS recall/fork opcodes while preserving CBF sanctuary records.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DarkOsSanctuaryForkEngine {
    pub kernel: DarkOsKernel,
    pub class_x0_crt_recalls: u64,
    pub class_x1_sanctuary_forks: u64,
}

impl Default for DarkOsSanctuaryForkEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl DarkOsSanctuaryForkEngine {
    #[inline]
    pub const fn new() -> Self {
        Self {
            kernel: DarkOsKernel::new(),
            class_x0_crt_recalls: 0,
            class_x1_sanctuary_forks: 0,
        }
    }

    /// Performs a Class X0 exact CRT primary recall through `VoaCrtAddMod` with zero.
    #[inline]
    pub fn execute_class_x0_crt_recall(&mut self, primary_index: u64) -> bool {
        let expected = primary_index % DARK_VOA_PRIMARIES;
        let module = CrtPrimaryModule::from_primary_index(expected);
        let zero = CrtPrimaryModule::zero();
        let (out, ok) = self
            .kernel
            .execute_voa_instruction(VoaIsaOpcode::VoaCrtAddMod, module, zero);
        if ok && out.to_primary_index() == expected {
            self.class_x0_crt_recalls = self.class_x0_crt_recalls.wrapping_add(1);
            true
        } else {
            false
        }
    }

    /// Executes one 17.010000 as Class X1 Frobenius sanctuary fork through `VoaSanctuaryFork`.
    #[inline]
    pub fn execute_class_x1_sanctuary_fork(&mut self) -> bool {
        let zero = CrtPrimaryModule::zero();
        let (_, ok) = self
            .kernel
            .execute_voa_instruction(VoaIsaOpcode::VoaSanctuaryFork, zero, zero);
        if ok && self.kernel.sanctuary_fork_commits > 0 {
            self.class_x1_sanctuary_forks = self.class_x1_sanctuary_forks.wrapping_add(1);
            true
        } else {
            false
        }
    }

    #[inline]
    pub const fn verify_sanctuary_fork_latency(&self) -> bool {
        self.kernel.kz_ticks_elapsed > 0
            && self.class_x0_crt_recalls > 0
            && self.class_x1_sanctuary_forks > 0
            && self.kernel.verify_dark_os_invariants()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dark_os_sanctuary_recall_and_fork() {
        let mut engine = DarkOsSanctuaryForkEngine::new();
        assert!(engine.execute_class_x0_crt_recall(1_234_567));
        assert!(engine.execute_class_x1_sanctuary_fork());
        assert!(engine.verify_sanctuary_fork_latency());
        assert!(engine.kernel.kz_ticks_elapsed >= 2);
    }
}
