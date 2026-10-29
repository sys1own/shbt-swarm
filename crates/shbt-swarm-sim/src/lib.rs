//! `crates/shbt-swarm-sim/src/lib.rs`
//! Multiphysics Digital Twin simulating the 5th-order minimum-jerk Eikonal foliation
//! (`s(tau) = 10*tau^3 - 15*tau^4 + 6*tau^5`, `v_derender <= 56.988 m/s`,
//! `max|s''(tau)| = 10/sqrt(3) = 5.77350269`), dynamic GST-225 acoustic impedance matching
//! (`R_acoustic <= 10^-6`), blueshifted hyper-temporal biological enclave evolution
//! (`alpha in [10, 10^4]`), and MERA-DAG planetary construction timing across fleet sizes
//! (`M = 1, 1000, 10000, 100000, 510000`), and Phase 4 MAR-312X multi-physics digital twin.

pub mod reactor_twin;
pub mod galactic_twin;

pub use reactor_twin::*;
pub use galactic_twin::*;

/// Maximum de-render front velocity in m/s (`56.988 m/s`).
pub const MAX_DERENDER_VELOCITY_MPS: f64 = 56.988;

/// Theoretical maximum normalized acceleration of the 5th-order minimum-jerk profile (`10 / sqrt(3)`).
pub const MAX_NORMALIZED_ACCEL: f64 = 5.773502691896258;

/// Evaluates the 5th-order minimum-jerk Eikonal foliation profile `s(tau) = 10*tau^3 - 15*tau^4 + 6*tau^5`
/// along with its first derivative `s'(tau) = 30*tau^2 - 60*tau^3 + 30*tau^4` and second derivative
/// `s''(tau) = 60*tau - 180*tau^2 + 120*tau^3` for `tau in [0.0, 1.0]`.
#[inline]
pub fn evaluate_eikonal_minimum_jerk(tau: f64) -> (f64, f64, f64) {
    let t = tau.clamp(0.0, 1.0);
    let t2 = t * t;
    let t3 = t2 * t;
    let t4 = t3 * t;
    let t5 = t4 * t;
    let s = 10.0 * t3 - 15.0 * t4 + 6.0 * t5;
    let ds = 30.0 * t2 - 60.0 * t3 + 30.0 * t4;
    let dds = 60.0 * t - 180.0 * t2 + 120.0 * t3;
    (s, ds, dds)
}

/// Computes the ultrasonic power reflection coefficient `R_acoustic = |(Z_gst - Z_bulk)/(Z_gst + Z_bulk)|^2`
/// across the emerging GST-225 metamaterial interface for `Z in [1.0, 100.0] MRayl`.
#[inline]
pub fn compute_gst225_acoustic_reflection(z_gst_mrayl: f64, z_bulk_mrayl: f64) -> f64 {
    let num = z_gst_mrayl - z_bulk_mrayl;
    let den = z_gst_mrayl + z_bulk_mrayl;
    if den.abs() < 1e-15 {
        return 1.0;
    }
    let r = num / den;
    r * r
}

/// Simulates blueshifted hyper-temporal enclave maturation for biological payloads
/// (`alpha = d_tau_bio / dt_planet in [10.0, 10000.0]`), returning `(coord_time_hours, interior_grad_alpha)`.
#[inline]
pub fn simulate_hypertemporal_enclave(
    proper_bio_years: f64,
    alpha_lapse: f64,
) -> Result<(f64, f64), &'static str> {
    if !(10.0..=10_000.0).contains(&alpha_lapse) {
        return Err("Lapse factor alpha outside admissible [10, 10^4] enclave envelope");
    }
    let total_proper_hours = proper_bio_years * 365.25 * 24.0;
    let coord_time_hours = total_proper_hours / alpha_lapse;
    let interior_grad_alpha = 0.0; // Strictly flat interior lapse gradient ||grad alpha|| == 0
    Ok((coord_time_hours, interior_grad_alpha))
}

/// Fleet replication and planetary settlement turnaround summary for a given fleet size `M`.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FleetTurnaroundMetrics {
    pub fleet_nodes_m: u64,
    pub burst_build_m0_seconds: f64,
    pub burst_build_x2_seconds: f64,
    pub tier4_forge_m0_seconds: f64,
    pub tier4_forge_x2_seconds: f64,
}

/// Computes the exact 202.55-Mt Turnkey Planetary Settlement synthesis turnaround time
/// for a swarm fleet of `M` ships under both legacy Burst power (`6.805 PW/ship` net) and
/// Phase 4 SHBT-MAR-312X Tier 4 Planetary-Forge Direct Bus (`24.9625 EW/ship` net).
#[inline]
pub fn simulate_fleet_settlement_turnaround(fleet_nodes_m: u64) -> FleetTurnaroundMetrics {
    let m = (fleet_nodes_m.max(1)) as f64;
    // Total settlement synthesis work:
    // Pure M0 regolith: 11,513,956.57 EJ = 11,513,956,570 PJ
    // M0 + Class X2 bio: 3,050,927.47 EJ = 3,050,927,470 PJ
    let w_m0_pj = 11_513_956_570.0;
    let w_x2_pj = 3_050_927_470.0;

    let p_burst_pw_per_ship = 6.805;
    let p_tier4_direct_pw_per_ship = 24_962.5; // 24.9625 EW = 24,962.5 PW

    FleetTurnaroundMetrics {
        fleet_nodes_m,
        burst_build_m0_seconds: w_m0_pj / (m * p_burst_pw_per_ship),
        burst_build_x2_seconds: w_x2_pj / (m * p_burst_pw_per_ship),
        tier4_forge_m0_seconds: w_m0_pj / (m * p_tier4_direct_pw_per_ship),
        tier4_forge_x2_seconds: w_x2_pj / (m * p_tier4_direct_pw_per_ship),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eikonal_minimum_jerk_bounds() {
        let (s0, ds0, dds0) = evaluate_eikonal_minimum_jerk(0.0);
        let (s1, ds1, dds1) = evaluate_eikonal_minimum_jerk(1.0);
        assert!((s0 - 0.0).abs() < 1e-12 && ds0.abs() < 1e-12 && dds0.abs() < 1e-12);
        assert!((s1 - 1.0).abs() < 1e-12 && ds1.abs() < 1e-12 && dds1.abs() < 1e-12);

        // Peak acceleration occurs at tau = (3 - sqrt(3))/6 ~ 0.2113248654
        let tau_peak = (3.0 - 3.0_f64.sqrt()) / 6.0;
        let (_, _, dds_peak) = evaluate_eikonal_minimum_jerk(tau_peak);
        assert!((dds_peak - MAX_NORMALIZED_ACCEL).abs() < 1e-9);
    }

    #[test]
    fn test_gst225_and_enclave_and_fleet() {
        let r = compute_gst225_acoustic_reflection(27.3, 27.3);
        assert!(r <= 1e-6);

        let (hrs, grad) = simulate_hypertemporal_enclave(5.0, 1000.0).unwrap();
        assert!((hrs - 43.83).abs() < 0.01);
        assert_eq!(grad, 0.0);

        let m10k = simulate_fleet_settlement_turnaround(10_000);
        assert!((m10k.tier4_forge_m0_seconds - 46.1249).abs() < 0.01);
        assert!((m10k.tier4_forge_x2_seconds - 12.2220).abs() < 0.01);
    }
}
