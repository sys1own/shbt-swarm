//! Phase 5C one-way nilpotent Dehn-surgery ingress airlock and bipartite cobordism manifold.
#![forbid(unsafe_code)]

pub const BIPARTITE_POCKET_COUNT: u32 = 100_000;
pub const BIPARTITE_POCKET_BOUNDARY_COUNT: u32 = 200_000;
pub const BIPARTITE_COBORDISM_DIMENSION: u32 = 11;
pub const BIPARTITE_INTERIOR_GENERA_COUNT: u32 = 312;
pub const BIPARTITE_ELYSIUM_POCKETS_COUNT: u32 = 100_000;
pub const BIPARTITE_OMNIBUS_TETHER_LATENCY_AS: f64 = 0.0;
pub const BIPARTITE_INTERIOR_GENUS_G: u32 = 156;

pub const DEHN_INGRESS_LOGICAL_VOXELS: u64 = 100_000;
pub const DEHN_INGRESS_CAYLEY_GRAPH_ORDER: u64 = 1 << 20;
pub const DEHN_INGRESS_SPOOF_CYCLES: u64 = 1;
pub const DEHN_INGRESS_ADM_MONOPOLE_ZERO: f64 = 0.0;
pub const DEHN_INGRESS_HASH_MATCH_BITS: u32 = 256;
pub const DEHN_INGRESS_MAX_BRUTE_FORCE_OPS: u64 = 1 << 40;
pub const DEHN_INGRESS_VALIDATION_LATENCY_AS: f64 = 17.010_000;
pub const DEHN_INGRESS_INTERIOR_REPAIR_DELAY_AS: f64 = 0.0;
pub const DEHN_INGRESS_INTERIOR_LOGICAL_VOLUME_M3: f64 = 0.0;
pub const DEHN_INGRESS_INTERIOR_PROPER_TIME_DELAY_S: f64 = 0.0;

pub const SHBT_AEGIS_BIPARTITE_COBORDISM_PINCHED: u32 = 0x0000_0010;
pub const SHBT_AEGIS_DEHN_INGRESS_PASSED: u32 = 0x0000_0020;
pub const SHBT_SWARM_FAULT_BIPARTITE_COBORDISM: u32 = 0x0000_0008;
pub const SHBT_SWARM_FAULT_DEHN_INGRESS: u32 = 0x0000_0009;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BipartiteCobordismManifold {
    pub status_latch: u32,
    pub fault_mask: u32,
    pub pocket_count: u32,
    pub pocket_boundary_count: u32,
    pub cobordism_dimension: u32,
    pub interior_genera_count: u32,
    pub elysium_pockets_count: u32,
    pub omnibus_tether_latency_as: f64,
    pub interior_genus_g: u32,
    pub pinch_executed: bool,
    pub exterior_adm_monopole_kg: f64,
    pub exterior_s_matrix_leak_norm: f64,
    pub exterior_gw_memory_residual_norm: f64,
    pub interior_logical_volume_m3: f64,
    pub interior_symplectic_form_residual: f64,
}

impl BipartiteCobordismManifold {
    #[inline]
    pub const fn new_unpinched() -> Self {
        Self {
            status_latch: 0,
            fault_mask: 0,
            pocket_count: BIPARTITE_POCKET_COUNT,
            pocket_boundary_count: BIPARTITE_POCKET_BOUNDARY_COUNT,
            cobordism_dimension: BIPARTITE_COBORDISM_DIMENSION,
            interior_genera_count: BIPARTITE_INTERIOR_GENERA_COUNT,
            elysium_pockets_count: BIPARTITE_ELYSIUM_POCKETS_COUNT,
            omnibus_tether_latency_as: BIPARTITE_OMNIBUS_TETHER_LATENCY_AS,
            interior_genus_g: BIPARTITE_INTERIOR_GENUS_G,
            pinch_executed: false,
            exterior_adm_monopole_kg: 0.0,
            exterior_s_matrix_leak_norm: 0.0,
            exterior_gw_memory_residual_norm: 0.0,
            interior_logical_volume_m3: 0.0,
            interior_symplectic_form_residual: 0.0,
        }
    }

    #[inline]
    pub fn execute_pinch_off(
        &mut self,
        exterior_adm_monopole_kg: f64,
        exterior_s_matrix_leak_norm: f64,
        exterior_gw_memory_residual_norm: f64,
        interior_symplectic_form_residual: f64,
    ) -> Result<(), u32> {
        self.exterior_adm_monopole_kg = exterior_adm_monopole_kg;
        self.exterior_s_matrix_leak_norm = exterior_s_matrix_leak_norm;
        self.exterior_gw_memory_residual_norm = exterior_gw_memory_residual_norm;
        self.interior_symplectic_form_residual = interior_symplectic_form_residual;
        if exterior_adm_monopole_kg != DEHN_INGRESS_ADM_MONOPOLE_ZERO
            || exterior_s_matrix_leak_norm > 1.0e-120
            || exterior_gw_memory_residual_norm > 1.0e-120
            || interior_symplectic_form_residual > 1.0e-14
        {
            self.fault_mask |= SHBT_SWARM_FAULT_BIPARTITE_COBORDISM;
            return Err(SHBT_SWARM_FAULT_BIPARTITE_COBORDISM);
        }
        self.interior_logical_volume_m3 = 0.0;
        self.pinch_executed = true;
        self.status_latch |= SHBT_AEGIS_BIPARTITE_COBORDISM_PINCHED;
        Ok(())
    }

    #[inline]
    pub fn verify_isolated_metrics(&self) -> bool {
        self.pinch_executed
            && self.exterior_adm_monopole_kg == 0.0
            && self.exterior_s_matrix_leak_norm <= 1.0e-120
            && self.exterior_gw_memory_residual_norm <= 1.0e-120
            && self.interior_logical_volume_m3 == 0.0
            && self.interior_symplectic_form_residual <= 1.0e-14
            && self.omnibus_tether_latency_as == 0.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NilpotentDehnIngressAirlock {
    pub status_latch: u32,
    pub fault_mask: u32,
    pub logical_voxels: u64,
    pub cayley_graph_order: u64,
    pub trusted_cayley_hash: [u8; 32],
    pub inbound_digest: [u8; 32],
    pub nilpotent_order: u32,
    pub cbf_valid: bool,
    pub interior_repair_delay_as: f64,
    pub validation_latency_as: f64,
    pub spoof_cycles_observed: u64,
    pub admitted: bool,
}

impl NilpotentDehnIngressAirlock {
    #[inline]
    pub const fn new_ep2_diode() -> Self {
        Self {
            status_latch: SHBT_AEGIS_DEHN_INGRESS_PASSED,
            fault_mask: 0,
            logical_voxels: DEHN_INGRESS_LOGICAL_VOXELS,
            cayley_graph_order: DEHN_INGRESS_CAYLEY_GRAPH_ORDER,
            trusted_cayley_hash: [0; 32],
            inbound_digest: [0; 32],
            nilpotent_order: 2,
            cbf_valid: true,
            interior_repair_delay_as: DEHN_INGRESS_INTERIOR_REPAIR_DELAY_AS,
            validation_latency_as: DEHN_INGRESS_VALIDATION_LATENCY_AS,
            spoof_cycles_observed: 0,
            admitted: true,
        }
    }

    #[inline]
    pub fn evaluate_ingress(
        &mut self,
        cbf_valid: bool,
        causal_consistent: bool,
        digest_match: bool,
        lethal_intent: bool,
        exterior_reach_attempt: bool,
    ) -> Result<(), u32> {
        self.cbf_valid = cbf_valid;
        if !cbf_valid {
            self.fault_mask |= SHBT_SWARM_FAULT_DEHN_INGRESS;
            self.admitted = false;
            return Err(SHBT_SWARM_FAULT_DEHN_INGRESS);
        }
        if lethal_intent {
            self.fault_mask |= SHBT_SWARM_FAULT_DEHN_INGRESS;
            self.admitted = false;
            return Err(SHBT_SWARM_FAULT_DEHN_INGRESS);
        }
        if exterior_reach_attempt {
            self.spoof_cycles_observed = DEHN_INGRESS_SPOOF_CYCLES;
            self.fault_mask |= SHBT_SWARM_FAULT_DEHN_INGRESS;
            self.admitted = false;
            return Err(SHBT_SWARM_FAULT_DEHN_INGRESS);
        }
        if !causal_consistent || !digest_match {
            self.spoof_cycles_observed = DEHN_INGRESS_SPOOF_CYCLES;
            self.fault_mask |= SHBT_SWARM_FAULT_DEHN_INGRESS;
            self.admitted = false;
            return Err(SHBT_SWARM_FAULT_DEHN_INGRESS);
        }
        self.interior_repair_delay_as = DEHN_INGRESS_INTERIOR_REPAIR_DELAY_AS;
        self.admitted = true;
        self.status_latch |= SHBT_AEGIS_DEHN_INGRESS_PASSED;
        Ok(())
    }

    #[inline]
    pub fn verify_one_way_nilpotency(&self) -> bool {
        self.nilpotent_order == 2
            && self.spoof_cycles_observed == 0
            && self.interior_repair_delay_as == 0.0
            && self.validation_latency_as == DEHN_INGRESS_VALIDATION_LATENCY_AS
            && self.admitted
            && self.fault_mask == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bipartite_pinch_locks_interior_bus() {
        let mut m = BipartiteCobordismManifold::new_unpinched();
        assert!(m.execute_pinch_off(0.0, 1e-130, 1e-130, 1e-15).is_ok());
        assert!(m.verify_isolated_metrics());
    }

    #[test]
    fn dehn_airlock_admits_clean_inbound_only() {
        let mut airlock = NilpotentDehnIngressAirlock::new_ep2_diode();
        assert!(airlock.evaluate_ingress(true, true, true, false, false).is_ok());
        assert!(airlock.verify_one_way_nilpotency());
        assert!(airlock.evaluate_ingress(true, true, true, true, false).is_err());
        assert!(airlock.evaluate_ingress(true, true, true, false, true).is_err());
    }
}
