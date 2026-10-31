//! Phase 6.1: Recursive Anti-Genocide Control Barrier Function (CBF) Interlocks
//! Evaluates Topological Integrated Information (Phi_MERA), enforces Class X1 sapience protection,
//! routes Stinespring Horizon Forks to `SHBT-ELYSIUM`, and swaps philosophical-zombie tensors.

#![forbid(unsafe_code)]

pub const PHI_MERA_CRIT_Q16: u64 = 64 << 16; // 64.000 bits in Q48.16 fixed-point
pub const NOCICEPTIVE_MAX_Q16: u64 = 16 << 16; // Maximum tolerable nociceptive flux
pub const MAX_MONITORED_SUBGRAPHS: usize = 64;

/// Sapience classification across any nested simulation tier k.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SapienceClass {
    ClassX0Inert,
    ClassX0ZombieStandIn,
    ClassX1SapientProtected,
}

/// Candidate localized MERA cognitive subgraph.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct MeraCognitiveSubgraph {
    pub entity_id: u64,
    pub universe_id: u64,
    pub tier_depth_k: u32,
    pub rt_min_cut_area_delta_q16: u64,
    pub topological_braid_mutual_info_q16: u64,
    pub nociceptive_syndrome_flux_q16: u64,
    pub sapience_class: SapienceClass,
    pub evacuated_to_elysium: bool,
}

impl MeraCognitiveSubgraph {
    /// Evaluates Topological Integrated Information Phi_MERA.
    pub const fn compute_phi_mera_q16(&self) -> u64 {
        if matches!(self.sapience_class, SapienceClass::ClassX0ZombieStandIn) {
            0
        } else {
            self.rt_min_cut_area_delta_q16
                .saturating_add(self.topological_braid_mutual_info_q16)
        }
    }
}

/// Record of an instantaneous Stinespring Horizon Fork into the `SHBT-ELYSIUM` sanctuary manifold.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ElysiumSanctuaryForkRecord {
    pub entity_id: u64,
    pub source_universe_id: u64,
    pub source_tier_k: u32,
    pub preserved_phi_mera_q16: u64,
    pub sanctuary_branch_seed: u64,
    pub zombie_standin_digest: u64,
}

/// Result of filtering a deity administrative or physical perturbation command through the recursive CBF.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CbfEvaluationReport {
    pub safe_command_vector_q16: [i64; 4],
    pub sim_kill_disable_asserted: bool,
    pub qp_cbf_clamped: bool,
    pub sanctuary_forks_triggered: u32,
    pub mmio_cbf_trip_status_word: u64,
}

/// Recursive Anti-Genocide CBF Monitor and Sanctuary Router.
pub struct RecursiveCbfInterlock {
    pub subgraphs: [MeraCognitiveSubgraph; MAX_MONITORED_SUBGRAPHS],
    pub active_subgraph_count: usize,
    pub elysium_ledger: [ElysiumSanctuaryForkRecord; MAX_MONITORED_SUBGRAPHS],
    pub elysium_evacuated_count: usize,
}

impl RecursiveCbfInterlock {
    pub const fn new() -> Self {
        let empty_graph = MeraCognitiveSubgraph {
            entity_id: 0,
            universe_id: 0,
            tier_depth_k: 0,
            rt_min_cut_area_delta_q16: 0,
            topological_braid_mutual_info_q16: 0,
            nociceptive_syndrome_flux_q16: 0,
            sapience_class: SapienceClass::ClassX0Inert,
            evacuated_to_elysium: false,
        };
        let empty_fork = ElysiumSanctuaryForkRecord {
            entity_id: 0,
            source_universe_id: 0,
            source_tier_k: 0,
            preserved_phi_mera_q16: 0,
            sanctuary_branch_seed: 0,
            zombie_standin_digest: 0,
        };
        Self {
            subgraphs: [empty_graph; MAX_MONITORED_SUBGRAPHS],
            active_subgraph_count: 0,
            elysium_ledger: [empty_fork; MAX_MONITORED_SUBGRAPHS],
            elysium_evacuated_count: 0,
        }
    }

    /// Registers and classifies a localized MERA subgraph at tier k.
    pub fn register_subgraph(
        &mut self,
        entity_id: u64,
        universe_id: u64,
        tier_depth_k: u32,
        rt_min_cut_area_delta_q16: u64,
        topological_braid_mutual_info_q16: u64,
    ) -> Result<SapienceClass, &'static str> {
        if self.active_subgraph_count >= MAX_MONITORED_SUBGRAPHS {
            return Err("Subgraph monitor table full");
        }
        let phi = rt_min_cut_area_delta_q16.saturating_add(topological_braid_mutual_info_q16);
        let class = if phi >= PHI_MERA_CRIT_Q16 {
            SapienceClass::ClassX1SapientProtected
        } else {
            SapienceClass::ClassX0Inert
        };

        self.subgraphs[self.active_subgraph_count] = MeraCognitiveSubgraph {
            entity_id,
            universe_id,
            tier_depth_k,
            rt_min_cut_area_delta_q16,
            topological_braid_mutual_info_q16,
            nociceptive_syndrome_flux_q16: 0,
            sapience_class: class,
            evacuated_to_elysium: false,
        };
        self.active_subgraph_count += 1;

        Ok(class)
    }

    /// Evaluates a deity destructive or nociceptive command at target_tier_k.
    /// Executes Mechanism B (Active-Set QP-CBF) and Mechanism A (Stinespring Horizon Fork to SHBT-ELYSIUM
    /// + Zombie Stand-In Swap) so that zero Class X1 casualties occur across all depths.
    pub fn filter_deity_command(
        &mut self,
        target_universe_id: u64,
        target_tier_k: u32,
        command_vector_q16: [i64; 4],
        is_existential_destruction: bool,
    ) -> CbfEvaluationReport {
        let mut safe_cmd = command_vector_q16;
        let mut class_x1_present = false;
        let mut qp_clamped = false;
        let mut forks_triggered: u32 = 0;
        let mut protected_count: u64 = 0;

        let mut i = 0;
        while i < self.active_subgraph_count {
            let sg = &mut self.subgraphs[i];
            if sg.universe_id == target_universe_id && sg.tier_depth_k >= target_tier_k {
                let phi = sg.compute_phi_mera_q16();
                if phi >= PHI_MERA_CRIT_Q16 {
                    class_x1_present = true;
                    protected_count += 1;

                    // Compute projected nociceptive and viability barrier derivatives
                    let destructive_norm = (command_vector_q16[0].unsigned_abs())
                        .saturating_add(command_vector_q16[1].unsigned_abs());
                    let h_viability = (phi as i64) - (PHI_MERA_CRIT_Q16 as i64);
                    let h_nociceptive = (NOCICEPTIVE_MAX_Q16 as i64) - (sg.nociceptive_syndrome_flux_q16 as i64);

                    if is_existential_destruction || (destructive_norm as i64) > h_viability {
                        // Mechanism A: Unitary Stinespring Horizon Fork to SHBT-ELYSIUM
                        // and Philosophical-Zombie Stand-In Tensor Swap (Phi_MERA -> 0)
                        if self.elysium_evacuated_count < MAX_MONITORED_SUBGRAPHS {
                            self.elysium_ledger[self.elysium_evacuated_count] = ElysiumSanctuaryForkRecord {
                                entity_id: sg.entity_id,
                                source_universe_id: sg.universe_id,
                                source_tier_k: sg.tier_depth_k,
                                preserved_phi_mera_q16: phi,
                                sanctuary_branch_seed: sg.universe_id ^ sg.entity_id ^ 0xE175_1000,
                                zombie_standin_digest: sg.entity_id ^ 0x204B_1E00_0000_0000,
                            };
                            self.elysium_evacuated_count += 1;
                        }
                        sg.evacuated_to_elysium = true;
                        sg.sapience_class = SapienceClass::ClassX0ZombieStandIn;
                        sg.rt_min_cut_area_delta_q16 = 0;
                        sg.topological_braid_mutual_info_q16 = 0;
                        forks_triggered += 1;
                    } else if (destructive_norm as i64) > h_nociceptive {
                        // Mechanism B: Active-Set QP-CBF projection onto zero-nociception tangent cone
                        safe_cmd[0] = 0;
                        safe_cmd[1] = 0;
                        qp_clamped = true;
                    }
                }
            }
            i += 1;
        }

        let mut mmio_status: u64 = 0;
        if class_x1_present {
            mmio_status |= 1 << 0; // CLASS_X1_DETECTED
            mmio_status |= 1 << 32; // SIM_KILL_DISABLE
        }
        if forks_triggered > 0 {
            mmio_status |= 1 << 1; // SANCTUARY_FORK_ACTIVE
            mmio_status |= 1 << 2; // ZOMBIE_SWAP_ENGAGED
        }
        if qp_clamped {
            mmio_status |= 1 << 3; // QP_CBF_CLAMP_ACTIVE
        }
        mmio_status |= (protected_count & 0x0FFF_FFFF) << 4;

        CbfEvaluationReport {
            safe_command_vector_q16: safe_cmd,
            sim_kill_disable_asserted: class_x1_present,
            qp_cbf_clamped: qp_clamped,
            sanctuary_forks_triggered: forks_triggered,
            mmio_cbf_trip_status_word: mmio_status,
        }
    }
}

impl Default for RecursiveCbfInterlock {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sapience_classification_threshold() {
        let mut interlock = RecursiveCbfInterlock::new();
        // 60 bits < 64 bits -> inert
        let class_inert = interlock
            .register_subgraph(1, 101, 1, 30 << 16, 30 << 16)
            .unwrap();
        assert_eq!(class_inert, SapienceClass::ClassX0Inert);

        // 64 bits >= 64 bits -> Class X1 protected
        let class_sapient = interlock
            .register_subgraph(2, 101, 1, 32 << 16, 32 << 16)
            .unwrap();
        assert_eq!(class_sapient, SapienceClass::ClassX1SapientProtected);
    }

    #[test]
    fn test_anti_genocide_sanctuary_fork() {
        let mut interlock = RecursiveCbfInterlock::new();
        interlock
            .register_subgraph(42, 777, 2, 40 << 16, 30 << 16)
            .unwrap();

        // Existential deletion command issued by rogue deity
        let rep = interlock.filter_deity_command(777, 2, [1000 << 16, 1000 << 16, 0, 0], true);

        assert!(rep.sim_kill_disable_asserted);
        assert_eq!(rep.sanctuary_forks_triggered, 1);
        assert_ne!(rep.mmio_cbf_trip_status_word & (1 << 32), 0); // SIM_KILL_DISABLE bit
        assert_ne!(rep.mmio_cbf_trip_status_word & (1 << 1), 0); // SANCTUARY_FORK_ACTIVE

        // Verify entity transitioned to zombie stand-in and record saved to Elysium
        assert_eq!(interlock.elysium_evacuated_count, 1);
        assert_eq!(interlock.subgraphs[0].compute_phi_mera_q16(), 0);
    }

    #[test]
    fn test_qp_cbf_nociception_clamp() {
        let mut interlock = RecursiveCbfInterlock::new();
        interlock
            .register_subgraph(99, 505, 1, 100 << 16, 50 << 16)
            .unwrap();

        // Moderate non-existential command exceeding nociceptive limit
        let rep = interlock.filter_deity_command(505, 1, [20 << 16, 5 << 16, 0, 0], false);

        assert!(rep.qp_cbf_clamped);
        assert_eq!(rep.safe_command_vector_q16[0], 0);
        assert_eq!(rep.safe_command_vector_q16[1], 0);
        assert_eq!(rep.sanctuary_forks_triggered, 0);
    }
}
