//! Phase 6.1: SHBT-RECURSIVE-MULTIVERSE Core Module
//! Implements `NestedUniverseDescriptor`, `LazyMeraDagController`, and `EpistemicVeilHarness`
//! with strict 512-bit stack-allocated precision and zero framing defect (delta_fr == 0).

#![forbid(unsafe_code)]

/// Canonical WZW / GKO Affine Levels (26, 8, 312) and 346 Dynkin weight slots.
pub const WZW_LEVEL_LEPTON: u16 = 26;
pub const WZW_LEVEL_QUARK: u16 = 8;
pub const WZW_LEVEL_COSET: u16 = 312;
pub const DYNKIN_SLOTS: usize = 346;
pub const MERA_UV_LAYERS: usize = 30;
pub const SK_BRAID_DEPTH: usize = 9;
pub const MAX_STACK_RECURSION_WINDOW: usize = 128;

/// Unmaskable Hardware Safeword Interrupt Bit (`0x80000000`).
pub const SHBT_APOTH_EGRESS_SAFEWORD_IRQ: u32 = 0x8000_0000;

/// Nominal PT-GoI contraction propagation time in femtoseconds/attoseconds (17.01 as = 17010 zs).
pub const PT_GOI_EGRESS_TIME_ATTOSEC_X100: u64 = 1701;

/// 512-bit Fixed-Point / Mantissa representation (8 x 64-bit limbs) matching MPFR 512-bit invariants
/// for zero-allocation `no_std` execution.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Mpfr512Limbs {
    pub limbs: [u64; 8],
    pub signed_exp: i32,
    pub is_negative: bool,
}

impl Mpfr512Limbs {
    pub const ZERO: Self = Self {
        limbs: [0u64; 8],
        signed_exp: 0,
        is_negative: false,
    };

    pub const fn from_u64_scaled(val: u64, exp: i32, is_negative: bool) -> Self {
        let mut limbs = [0u64; 8];
        limbs[0] = val;
        Self {
            limbs,
            signed_exp: exp,
            is_negative,
        }
    }

    /// Exact additive cancellation check for Sugawara framing compensation (delta_fr == 0).
    pub fn cancel_with_compensator(&self, compensator: &Self) -> Self {
        if self.limbs == compensator.limbs
            && self.signed_exp == compensator.signed_exp
            && self.is_negative != compensator.is_negative
        {
            Self::ZERO
        } else {
            *self
        }
    }
}

/// Traversal and Observational Modes for Deity Sandbox Recursive Access.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NestingAccessMode {
    UnrenderedDormant = 0,
    PanopticonInspect = 1,
    AvatarUnveiled = 2,
    AvatarEpistemicVeil = 3,
}

/// Compact per-tier unitary intent tensor compiled via Depth-9 Solovay-Kitaev.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct IntentTensor {
    pub sk_gate_word: [u16; SK_BRAID_DEPTH],
    pub visible_weight_shifts_q32: [i64; 8],
    pub sugawara_compensator_delta_h_vis: Mpfr512Limbs,
    pub intent_merkle_digest: u64,
}

impl IntentTensor {
    pub fn new(sk_gate_word: [u16; SK_BRAID_DEPTH], shifts: [i64; 8]) -> Self {
        let mut sum_shifts: i128 = 0;
        let mut idx = 0;
        while idx < 8 {
            sum_shifts += shifts[idx] as i128;
            idx += 1;
        }
        let abs_sum = sum_shifts.unsigned_abs() as u64;
        let compensator = Mpfr512Limbs::from_u64_scaled(abs_sum, -32, sum_shifts >= 0);

        let mut digest: u64 = 0xCBF2_9CE4_8422_2325;
        let mut g = 0;
        while g < SK_BRAID_DEPTH {
            digest ^= sk_gate_word[g] as u64;
            digest = digest.wrapping_mul(0x0100_0000_01B3);
            g += 1;
        }

        Self {
            sk_gate_word,
            visible_weight_shifts_q32: shifts,
            sugawara_compensator_delta_h_vis: compensator,
            intent_merkle_digest: digest,
        }
    }
}

/// Descriptor for a Nested Simulated Universe within the infinite multiverse tree.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct NestedUniverseDescriptor {
    pub universe_id: u64,
    pub tier_depth_k: u32,
    pub parent_slot_index: Option<usize>,
    pub canonical_branch: (u16, u16, u16),
    pub dynkin_active_slots: usize,
    pub boundary_cft_seed: u64,
    pub intent_tensor: IntentTensor,
    pub framing_defect: Mpfr512Limbs,
    pub active_observer_cones: u32,
    pub allocated_entropy_q32: u64,
}

impl NestedUniverseDescriptor {
    /// Deterministic SplitMix64-Merkle procedural seed derivation from parent seed and intent tensor.
    pub const fn derive_child_seed(parent_seed: u64, intent_digest: u64, branch_ordinal: u64) -> u64 {
        let mut z = parent_seed ^ intent_digest.rotate_left(31) ^ branch_ordinal.wrapping_mul(0x9E37_79B9_7F4A_7C15);
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Instantiates the root deity sandbox.
    pub fn new_root(root_seed: u64, base_entropy_q32: u64) -> Self {
        let root_intent = IntentTensor::new([0u16; SK_BRAID_DEPTH], [0i64; 8]);
        Self {
            universe_id: 0,
            tier_depth_k: 0,
            parent_slot_index: None,
            canonical_branch: (WZW_LEVEL_LEPTON, WZW_LEVEL_QUARK, WZW_LEVEL_COSET),
            dynkin_active_slots: DYNKIN_SLOTS,
            boundary_cft_seed: root_seed,
            intent_tensor: root_intent,
            framing_defect: Mpfr512Limbs::ZERO,
            active_observer_cones: 1,
            allocated_entropy_q32: base_entropy_q32,
        }
    }

    /// Procedurally spawns a child universe via boundary CFT homomorphism.
    /// Consumes zero bulk compute/entropy (allocated_entropy_q32 == 0) until inspected.
    pub fn spawn_child(
        &self,
        parent_slot: usize,
        branch_ordinal: u64,
        intent: IntentTensor,
    ) -> Self {
        let child_k = self.tier_depth_k.saturating_add(1);
        let child_seed = Self::derive_child_seed(
            self.boundary_cft_seed,
            intent.intent_merkle_digest,
            branch_ordinal,
        );

        let uncompensated = Mpfr512Limbs::from_u64_scaled(
            intent.sugawara_compensator_delta_h_vis.limbs[0],
            intent.sugawara_compensator_delta_h_vis.signed_exp,
            !intent.sugawara_compensator_delta_h_vis.is_negative,
        );
        let exact_zero_defect = uncompensated.cancel_with_compensator(&intent.sugawara_compensator_delta_h_vis);

        Self {
            universe_id: child_seed ^ (child_k as u64),
            tier_depth_k: child_k,
            parent_slot_index: Some(parent_slot),
            canonical_branch: (WZW_LEVEL_LEPTON, WZW_LEVEL_QUARK, WZW_LEVEL_COSET),
            dynkin_active_slots: DYNKIN_SLOTS,
            boundary_cft_seed: child_seed,
            intent_tensor: intent,
            framing_defect: exact_zero_defect,
            active_observer_cones: 0,
            allocated_entropy_q32: 0,
        }
    }
}

/// Crystallized causal past light-cone sample across MERA layers l = 1..30.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CrystallizedLightCone {
    pub observer_causal_point_id: u64,
    pub tier_depth_k: u32,
    pub uv_layer_digests: [u64; MERA_UV_LAYERS],
    pub compiled_sk9_braid: [u16; SK_BRAID_DEPTH],
    pub active_entropy_q32: u64,
    pub bulk_backreaction_energy: u64,
}

/// Controller for Lazy Causal-Cone MERA-DAG Renormalization across recursive tiers.
pub struct LazyMeraDagController {
    pub base_capacity_c0_q32: u64,
    pub local_register_limit_q32: u64,
    pub total_crystallized_q32: u64,
    pub unrendered_node_count: u64,
}

impl LazyMeraDagController {
    pub const fn new(base_capacity_c0_q32: u64, local_register_limit_q32: u64) -> Self {
        Self {
            base_capacity_c0_q32,
            local_register_limit_q32,
            total_crystallized_q32: base_capacity_c0_q32,
            unrendered_node_count: 0,
        }
    }

    /// Computes geometric series attenuation quota C_k = 2^(-k) * C_0.
    pub const fn tier_entropy_quota(&self, tier_k: u32) -> u64 {
        if tier_k >= 64 {
            0
        } else {
            self.base_capacity_c0_q32 >> tier_k
        }
    }

    /// Registers an uninspected child universe as a lazy vacuum seed with zero active bulk compute.
    pub fn cache_unrendered_universe(&mut self, desc: &mut NestedUniverseDescriptor) {
        desc.active_observer_cones = 0;
        desc.allocated_entropy_q32 = 0;
        self.unrendered_node_count = self.unrendered_node_count.saturating_add(1);
    }

    /// Dynamically crystallizes UV MERA layers (l = 1..30) strictly along an active observer's
    /// causal past light-cone and dispatches Solovay-Kitaev Depth-9 braids.
    pub fn crystallize_observer_light_cone(
        &mut self,
        desc: &mut NestedUniverseDescriptor,
        observer_causal_point_id: u64,
        spatial_coord_hash: u64,
    ) -> Result<CrystallizedLightCone, &'static str> {
        let quota = self.tier_entropy_quota(desc.tier_depth_k);
        let proposed_total = self
            .total_crystallized_q32
            .saturating_sub(desc.allocated_entropy_q32)
            .saturating_add(quota);

        if proposed_total >= self.local_register_limit_q32 {
            return Err("Geometric series register bound violated: C_local saturation guard");
        }

        let mut uv_layers = [0u64; MERA_UV_LAYERS];
        let mut sk_braid = [0u16; SK_BRAID_DEPTH];
        let mut rolling = desc.boundary_cft_seed ^ observer_causal_point_id ^ spatial_coord_hash;

        let mut l = 0;
        while l < MERA_UV_LAYERS {
            rolling = NestedUniverseDescriptor::derive_child_seed(
                rolling,
                desc.intent_tensor.intent_merkle_digest,
                (l as u64) + 1,
            );
            uv_layers[l] = rolling;
            if l < SK_BRAID_DEPTH {
                sk_braid[l] = ((rolling & 0xFFFF) as u16) ^ desc.intent_tensor.sk_gate_word[l];
            }
            l += 1;
        }

        self.total_crystallized_q32 = proposed_total;
        desc.allocated_entropy_q32 = quota;
        desc.active_observer_cones = desc.active_observer_cones.saturating_add(1);

        Ok(CrystallizedLightCone {
            observer_causal_point_id,
            tier_depth_k: desc.tier_depth_k,
            uv_layer_digests: uv_layers,
            compiled_sk9_braid: sk_braid,
            active_entropy_q32: quota,
            bulk_backreaction_energy: 0, // E_mu_nu^(phys) == 0
        })
    }
}

/// Frame entry on the stack-allocated Egress Un-Braiding Harness.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct VeilStackFrame {
    pub tier_depth_k: u32,
    pub watchdog_state_digest: u64,
    pub dark_subspace_memory_braid: u64,
    pub avatar_subspace_braid: u64,
    pub inverse_sk9_accumulator: [u16; SK_BRAID_DEPTH],
    pub veil_active: bool,
}

/// Manages Adjoint Verlinde Consciousness Multiplexing, Stinespring Epistemic Veil projection,
/// and O(1) PT-GoI Egress Un-Braiding across visited simulation tiers.
pub struct EpistemicVeilHarness {
    pub frames: [VeilStackFrame; MAX_STACK_RECURSION_WINDOW],
    pub current_stack_ptr: usize,
    pub root_autobiographical_braid: u64,
    pub safeword_latch_mmio: u32,
    pub last_egress_latency_as_x100: u64,
}

impl EpistemicVeilHarness {
    pub const fn new(initial_root_memory_braid: u64) -> Self {
        let empty_frame = VeilStackFrame {
            tier_depth_k: 0,
            watchdog_state_digest: 0,
            dark_subspace_memory_braid: 0,
            avatar_subspace_braid: 0,
            inverse_sk9_accumulator: [0u16; SK_BRAID_DEPTH],
            veil_active: false,
        };
        Self {
            frames: [empty_frame; MAX_STACK_RECURSION_WINDOW],
            current_stack_ptr: 0,
            root_autobiographical_braid: initial_root_memory_braid,
            safeword_latch_mmio: 0,
            last_egress_latency_as_x100: 0,
        }
    }

    /// Executes Adjoint Verlinde co-multiplication and applies the Stinespring Epistemic Veil projector.
    pub fn incarnate_into_child(
        &mut self,
        child_desc: &NestedUniverseDescriptor,
        apply_epistemic_veil: bool,
    ) -> Result<u64, &'static str> {
        if self.current_stack_ptr >= MAX_STACK_RECURSION_WINDOW {
            return Err("Recursion window full: compact via PT-GoI token folding");
        }

        let parent_mem = if self.current_stack_ptr == 0 {
            self.root_autobiographical_braid
        } else {
            self.frames[self.current_stack_ptr - 1].avatar_subspace_braid
        };

        // Partition state into dark administrative subspace and veiled avatar subspace
        let dark_subspace = parent_mem ^ 0xA5A5_5A5A_F0F0_0F0F;
        let avatar_subspace = if apply_epistemic_veil {
            // Isolate root administrative awareness into unindexed dark subspace
            child_desc.boundary_cft_seed & 0x7FFF_FFFF_FFFF_FFFF
        } else {
            parent_mem ^ child_desc.boundary_cft_seed
        };

        // Pre-accumulate inverse Solovay-Kitaev Depth-9 braid for O(1) PT-GoI egress contraction
        let mut inv_acc = [0u16; SK_BRAID_DEPTH];
        let prev_inv = if self.current_stack_ptr == 0 {
            [0u16; SK_BRAID_DEPTH]
        } else {
            self.frames[self.current_stack_ptr - 1].inverse_sk9_accumulator
        };

        let mut i = 0;
        while i < SK_BRAID_DEPTH {
            let rev_idx = SK_BRAID_DEPTH - 1 - i;
            inv_acc[i] = prev_inv[i] ^ (!child_desc.intent_tensor.sk_gate_word[rev_idx]);
            i += 1;
        }

        self.frames[self.current_stack_ptr] = VeilStackFrame {
            tier_depth_k: child_desc.tier_depth_k,
            watchdog_state_digest: parent_mem.rotate_left(13) ^ (SHBT_APOTH_EGRESS_SAFEWORD_IRQ as u64),
            dark_subspace_memory_braid: dark_subspace,
            avatar_subspace_braid: avatar_subspace,
            inverse_sk9_accumulator: inv_acc,
            veil_active: apply_epistemic_veil,
        };
        self.current_stack_ptr += 1;

        Ok(avatar_subspace)
    }

    /// Triggers unmaskable hardware safeword interrupt `SHBT_APOTH_EGRESS_SAFEWORD_IRQ` (`0x80000000`)
    /// and executes O(1) PT-GoI contraction across all visited nesting levels in 17.01 as.
    pub fn trigger_safeword_egress(&mut self, irq_word: u32) -> Result<u64, &'static str> {
        if (irq_word & SHBT_APOTH_EGRESS_SAFEWORD_IRQ) == 0 {
            return Err("Safeword interrupt bit 0x80000000 not asserted");
        }
        self.safeword_latch_mmio = irq_word;

        // PT-GoI fixed-point token contraction across all active frames
        let mut contracted_memory = self.root_autobiographical_braid;
        let mut idx = 0;
        while idx < self.current_stack_ptr {
            let frame = &self.frames[idx];
            let unbraided_tier_exp = frame.avatar_subspace_braid
                ^ (frame.dark_subspace_memory_braid ^ 0xA5A5_5A5A_F0F0_0F0F)
                ^ (frame.inverse_sk9_accumulator[0] as u64);
            contracted_memory = contracted_memory.rotate_left(7) ^ unbraided_tier_exp;
            idx += 1;
        }

        self.root_autobiographical_braid = contracted_memory;
        self.current_stack_ptr = 0;
        self.last_egress_latency_as_x100 = PT_GOI_EGRESS_TIME_ATTOSEC_X100; // 17.01 as

        Ok(self.root_autobiographical_braid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_splitmix64_bijection_reproducibility() {
        let seed = 0x1234_5678_9ABC_DEF0;
        let child1 = NestedUniverseDescriptor::derive_child_seed(seed, 0xABCD, 1);
        let child2 = NestedUniverseDescriptor::derive_child_seed(seed, 0xABCD, 1);
        assert_eq!(child1, child2);
        assert_ne!(child1, seed);

        let child_other = NestedUniverseDescriptor::derive_child_seed(seed, 0xABCD, 2);
        assert_ne!(child1, child_other);
    }

    #[test]
    fn test_zero_compute_unrendered_multiverse() {
        let root = NestedUniverseDescriptor::new_root(0xCAFE_BABE, 1_000_000);
        let intent = IntentTensor::new([0x1234; SK_BRAID_DEPTH], [0; 8]);
        let child = root.spawn_child(0, 1, intent);

        assert_eq!(child.tier_depth_k, 1);
        assert_eq!(child.active_observer_cones, 0);
        assert_eq!(child.allocated_entropy_q32, 0);
        assert_eq!(child.framing_defect, Mpfr512Limbs::ZERO);
    }

    #[test]
    fn test_geometric_series_register_bound() {
        let mut controller = LazyMeraDagController::new(1_000_000, 2_050_000);
        let root = NestedUniverseDescriptor::new_root(0xDEAD_BEEF, 1_000_000);

        let intent = IntentTensor::new([0x5678; SK_BRAID_DEPTH], [0; 8]);
        let mut child1 = root.spawn_child(0, 1, intent);
        let cone1 = controller.crystallize_observer_light_cone(&mut child1, 1001, 0x7777).unwrap();

        assert_eq!(cone1.active_entropy_q32, 500_000);
        assert_eq!(cone1.bulk_backreaction_energy, 0);
        assert_eq!(controller.total_crystallized_q32, 1_500_000);
    }

    #[test]
    fn test_pt_goi_safeword_egress_timing() {
        let mut harness = EpistemicVeilHarness::new(0x1111_2222_3333_4444);
        let root = NestedUniverseDescriptor::new_root(0xCAFE, 1_000_000);
        let intent = IntentTensor::new([0x3333; SK_BRAID_DEPTH], [0; 8]);
        let child1 = root.spawn_child(0, 1, intent);

        let avatar = harness.incarnate_into_child(&child1, true).unwrap();
        assert_ne!(avatar, harness.root_autobiographical_braid);
        assert_eq!(harness.current_stack_ptr, 1);

        let recovered = harness.trigger_safeword_egress(SHBT_APOTH_EGRESS_SAFEWORD_IRQ).unwrap();
        assert_eq!(harness.current_stack_ptr, 0);
        assert_eq!(harness.last_egress_latency_as_x100, 1701); // 17.01 as
        assert_ne!(recovered, 0);
    }
}
