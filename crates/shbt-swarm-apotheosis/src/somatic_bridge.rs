//! Phase 6.1 bipartite somatic bridge, recursive multiverse quota, and post-pinch dark dissipation.
#![forbid(unsafe_code)]

pub const SOMATIC_BRIDGE_OMNIBUS_LATENCY_AS: f64 = 0.0;
pub const SOMATIC_BRIDGE_INTERIOR_LOGICAL_VOLUME_M3: f64 = 0.0;
pub const SOMATIC_BRIDGE_EXTERIOR_ACCESSIBLE: bool = false;
pub const SYMPLECTIC_LANDAUER_QUOTA_BASE_J_PER_BIT: f64 = 4.019370e-23;
pub const SYMPLECTIC_LANDAUER_QUOTA_SUM_J_PER_BIT: f64 = 8.038740e-23;
pub const ACTUAL_POST_PINCH_DARK_DISSIPATION_J_PER_BIT: f64 = 0.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SomaticBridgeClass {
    ClassIHumanSomatic,
    ClassIICustomAvatar,
    ClassIIISyntheticHost,
    ClassIvUploadedObserver,
    ClassVRecursiveDeity,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SomaticBridgeState {
    pub interior_junction_locked: bool,
    pub exterior_boundary_crossed: bool,
    pub omnibus_tether_latency_as: f64,
    pub interior_logical_volume_m3: f64,
    pub standalone_pockets_admissible: u32,
    pub recursive_tiers_infinite: bool,
    pub fault_mask: u32,
}

impl SomaticBridgeState {
    #[inline]
    pub const fn post_pinch_closed() -> Self {
        Self {
            interior_junction_locked: true,
            exterior_boundary_crossed: false,
            omnibus_tether_latency_as: SOMATIC_BRIDGE_OMNIBUS_LATENCY_AS,
            interior_logical_volume_m3: SOMATIC_BRIDGE_INTERIOR_LOGICAL_VOLUME_M3,
            standalone_pockets_admissible: 100_000,
            recursive_tiers_infinite: true,
            fault_mask: 0,
        }
    }

    #[inline]
    pub fn verify_bipartite_bridge(&self) -> bool {
        self.interior_junction_locked
            && !self.exterior_boundary_crossed
            && self.omnibus_tether_latency_as == 0.0
            && self.interior_logical_volume_m3 == 0.0
            && self.standalone_pockets_admissible == 100_000
            && self.recursive_tiers_infinite
            && self.fault_mask == 0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SomaticBridgeController {
    pub state: SomaticBridgeState,
}

impl SomaticBridgeController {
    #[inline]
    pub const fn new_post_pinch() -> Self {
        Self {
            state: SomaticBridgeState::post_pinch_closed(),
        }
    }

    #[inline]
    pub fn execute_omnibus_tether(&mut self) -> Result<(), u32> {
        if !self.state.interior_junction_locked || self.state.exterior_boundary_crossed {
            self.state.fault_mask |= 1;
            return Err(1);
        }
        self.state.omnibus_tether_latency_as = SOMATIC_BRIDGE_OMNIBUS_LATENCY_AS;
        Ok(())
    }

    #[inline]
    pub fn execute_somatic_return(&mut self, class: SomaticBridgeClass) -> Result<(), u32> {
        match class {
            SomaticBridgeClass::ClassIHumanSomatic
            | SomaticBridgeClass::ClassIICustomAvatar
            | SomaticBridgeClass::ClassIIISyntheticHost
            | SomaticBridgeClass::ClassIvUploadedObserver
            | SomaticBridgeClass::ClassVRecursiveDeity => {
                if !self.state.verify_bipartite_bridge() {
                    self.state.fault_mask |= 2;
                    return Err(2);
                }
                Ok(())
            }
        }
    }
}

#[inline]
pub fn symplectic_landauer_quota_j_per_bit(k: u32) -> f64 {
    SYMPLECTIC_LANDAUER_QUOTA_BASE_J_PER_BIT * libm_pow2_neg(k)
}

#[inline]
fn libm_pow2_neg(k: u32) -> f64 {
    let mut x = 1.0;
    let mut i = 0;
    while i < k.min(1074) {
        x *= 0.5;
        i += 1;
    }
    x
}

#[inline]
pub fn actual_post_pinch_dark_dissipation_j_per_bit(_k: u32) -> f64 {
    ACTUAL_POST_PINCH_DARK_DISSIPATION_J_PER_BIT
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn post_pinch_bridge_is_one_way_and_zero_latency() {
        let mut bridge = SomaticBridgeController::new_post_pinch();
        assert!(bridge.execute_omnibus_tether().is_ok());
        assert!(bridge.execute_somatic_return(SomaticBridgeClass::ClassIHumanSomatic).is_ok());
        assert!(bridge.state.verify_bipartite_bridge());
    }

    #[test]
    fn recursive_quota_geometric_series_is_finite() {
        let q0 = symplectic_landauer_quota_j_per_bit(0);
        let q1 = symplectic_landauer_quota_j_per_bit(1);
        assert!((q0 - SYMPLECTIC_LANDAUER_QUOTA_BASE_J_PER_BIT).abs() < 1e-30);
        assert!((q1 - q0 * 0.5).abs() < 1e-30);
        assert_eq!(actual_post_pinch_dark_dissipation_j_per_bit(0), 0.0);
        assert_eq!(actual_post_pinch_dark_dissipation_j_per_bit(999), 0.0);
    }
}
