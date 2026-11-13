//! Dark-ZPE void-loiter/warp magazine recharge and void-quine bus for `SHBT-MAR-312X`.
#![forbid(unsafe_code)]

/// Channel-1 ship void-loiter power (`187.75 MW = 187,750,000 W`).
pub const DARK_ZPE_VOID_LOITER_W: u64 = 187_750_000;
/// Channel-1 ship void-warp corridor power in petawatts (`15.510 PW = 15.510 EW`).
pub const DARK_ZPE_VOID_WARP_PW: u64 = 15_510;
/// Ship Landauer debt (`906.00 kW = 906,000 W`).
pub const DARK_ZPE_LANDAUER_DEBT_W: u64 = 906_000;
/// Void recharge power in exawatts (`23.93 EW`).
pub const DARK_ZPE_VOID_RECHARGE_EW_F64: f64 = 23.93;
/// Void recharge duration in milliseconds (`57.59 s`).
pub const DARK_ZPE_VOID_RECHARGE_MS: u64 = 57_590;
/// Void harvest duration before quine in milliseconds (`61.62 s`).
pub const DARK_ZPE_VOID_HARVEST_MS: u64 = 61_620;
/// Void quine replication latency in microseconds (`13.418 ms`).
pub const DARK_ZPE_VOID_QUINE_US: u64 = 13_418;

/// Tracks Dark-ZPE magazine recharge and void-quine timing on the MAR-312X bus.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DarkZpeMagazineBus {
    pub void_loiter_w: u64,
    pub void_warp_pw: u64,
    pub landauer_debt_w: u64,
    pub recharge_ew: f64,
    pub recharge_ms: u64,
    pub harvest_ms: u64,
    pub quine_us: u64,
    pub magazine_cycles: u64,
}

impl Default for DarkZpeMagazineBus {
    fn default() -> Self {
        Self::new()
    }
}

impl DarkZpeMagazineBus {
    #[inline]
    pub const fn new() -> Self {
        Self {
            void_loiter_w: DARK_ZPE_VOID_LOITER_W,
            void_warp_pw: DARK_ZPE_VOID_WARP_PW,
            landauer_debt_w: DARK_ZPE_LANDAUER_DEBT_W,
            recharge_ew: DARK_ZPE_VOID_RECHARGE_EW_F64,
            recharge_ms: DARK_ZPE_VOID_RECHARGE_MS,
            harvest_ms: DARK_ZPE_VOID_HARVEST_MS,
            quine_us: DARK_ZPE_VOID_QUINE_US,
            magazine_cycles: 0,
        }
    }

    #[inline]
    pub fn void_warp_power_w(&self) -> u64 {
        self.void_warp_pw.saturating_mul(1_000_000_000_000_000)
    }

    #[inline]
    pub fn recharge_power_w(&self) -> f64 {
        self.recharge_ew * 1.0e18
    }

    #[inline]
    pub const fn verify_magazine_recharge(&self) -> bool {
        self.void_loiter_w > 207 * self.landauer_debt_w
            && self.void_warp_pw == DARK_ZPE_VOID_WARP_PW
            && self.recharge_ms == DARK_ZPE_VOID_RECHARGE_MS
            && self.harvest_ms == DARK_ZPE_VOID_HARVEST_MS
    }

    #[inline]
    pub const fn verify_void_quine(&self) -> bool {
        self.quine_us == DARK_ZPE_VOID_QUINE_US && self.verify_magazine_recharge()
    }

    #[inline]
    pub fn step_magazine_cycle(&mut self) -> bool {
        self.magazine_cycles = self.magazine_cycles.wrapping_add(1);
        self.verify_void_quine()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dark_zpe_magazine_recharge_and_quine() {
        let mut bus = DarkZpeMagazineBus::new();
        assert!(bus.verify_magazine_recharge());
        assert!(bus.step_magazine_cycle());
        assert_eq!(bus.void_warp_power_w(), 15_510_000_000_000_000_000);
        assert_eq!(bus.quine_us, 13_418);
    }
}
