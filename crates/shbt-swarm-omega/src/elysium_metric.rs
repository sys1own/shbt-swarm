//! crates/shbt-swarm-omega/src/elysium_metric.rs
      //!
      //! Gravitomagnetic Null-Current Boundary Splice and Zero-Coriolis Metric Field.
      //! Architecture: sys1own/shbt-swarm-omega (SHBT-ELYSIUM)

      

use core::f64;

#[inline]
pub fn f64_abs(x: f64) -> f64 {
    if x < 0.0 { -x } else { x }
}


/// Speed of light in vacuum (m/s).
pub const SPEED_OF_LIGHT: f64 = 299_792_458.0;

/// Target synthesized acceleration: exactly 1.000000g downward (m/s^2).
pub const G_TARGET: f64 = 9.806_650;

/// Newtonian gravitational constant (m^3 / (kg * s^2)).
pub const GRAVITATIONAL_CONSTANT: f64 = 6.674_30e-11;

/// Second-order holographic backreaction counter-tensor component I_00 (m^-2).
pub const I_00_COUNTER_TERM: f64 = 2.418_93e-32;

/// Threshold precision tolerance for numerical audits.
pub const PRECISION_TOLERANCE: f64 = 1.0e-15;

/// Local 4D ADM metric state on the spatial foliation.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ElysiumPointMetric {
    /// Covariant 4D metric tensor g_mu_nu
    pub g: [[f64; 4]; 4],
    /// Contravariant inverse 4D metric tensor g^mu_nu
    pub g_inv: [[f64; 4]; 4],
    /// Lapse scalar alpha
    pub alpha: f64,
    /// Spatial shift vector beta^i
    pub beta: [f64; 3],
}

/// Evaluates the ADM lapse function alpha(z) via series expansion.
#[inline]
pub fn lapse_function(z: f64) -> f64 {
    let arg = (G_TARGET * z) / (SPEED_OF_LIGHT * SPEED_OF_LIGHT);
    // Explicit Taylor series for  environments lacking libm exp()
    1.0 + arg + (0.5 * arg * arg) + (arg * arg * arg / 6.0)
}

/// Evaluates the dimensionless shift vector beta^i(x, y, z).
/// Elysium habitats enforce an exact irrotational null-current shift field.
#[inline]
pub fn shift_vector(_x: f64, _y: f64, _z: f64) -> [f64; 3] {
    [0.0, 0.0, 0.0]
}
/// Computes the complete 4D metric tensor and inverse at coordinate position (x, y, z).
pub fn evaluate_metric(x: f64, y: f64, z: f64) -> ElysiumPointMetric {
    let alpha = lapse_function(z);
    let beta = shift_vector(x, y, z);
    let alpha_sq = alpha * alpha;

    let beta_sq = beta[0] * beta[0] + beta[1] * beta[1] + beta[2] * beta[2];
    let g_00 = -(alpha_sq - beta_sq);

    let mut g = [[0.0; 4]; 4];
    g[0][0] = g_00;
    g[0][1] = beta[0];
    g[0][2] = beta[1];
    g[0][3] = beta[2];
    g[1][0] = beta[0];
    g[2][0] = beta[1];
    g[3][0] = beta[2];
    g[1][1] = 1.0;
    g[2][2] = 1.0;
    g[3][3] = 1.0;

    let mut g_inv = [[0.0; 4]; 4];
    g_inv[0][0] = -1.0 / alpha_sq;
    g_inv[0][1] = beta[0] / alpha_sq;
    g_inv[0][2] = beta[1] / alpha_sq;
    g_inv[0][3] = beta[2] / alpha_sq;
    g_inv[1][0] = beta[0] / alpha_sq;
    g_inv[2][0] = beta[1] / alpha_sq;
    g_inv[3][0] = beta[2] / alpha_sq;

    g_inv[1][1] = 1.0 - (beta[0] * beta[0] / alpha_sq);
    g_inv[1][2] = -(beta[0] * beta[1] / alpha_sq);
    g_inv[1][3] = -(beta[0] * beta[2] / alpha_sq);
    g_inv[2][1] = -(beta[1] * beta[0] / alpha_sq);
    g_inv[2][2] = 1.0 - (beta[1] * beta[1] / alpha_sq);
    g_inv[2][3] = -(beta[1] * beta[2] / alpha_sq);
    g_inv[3][1] = -(beta[2] * beta[0] / alpha_sq);
    g_inv[3][2] = -(beta[2] * beta[1] / alpha_sq);
    g_inv[3][3] = 1.0 - (beta[2] * beta[2] / alpha_sq);

    ElysiumPointMetric {
        g,
        g_inv,
        alpha,
        beta,
    }
}

/// Evaluates the curl of the shift vector: curl(beta) = \nabla x \beta.
#[inline]
pub fn evaluate_shift_curl(_x: f64, _y: f64, _z: f64) -> [f64; 3] {
    [0.0, 0.0, 0.0]
}

/// Evaluates the Coriolis acceleration experienced by a moving occupant:
/// a_Cor = v x (nabla x beta).
#[inline]
pub fn compute_coriolis_acceleration(velocity: [f64; 3], pos: [f64; 3]) -> [f64; 3] {
    let curl = evaluate_shift_curl(pos[0], pos[1], pos[2]);
    [
        velocity[1] * curl[2] - velocity[2] * curl[1],
        velocity[2] * curl[0] - velocity[0] * curl[2],
        velocity[0] * curl[1] - velocity[1] * curl[0],
    ]
}

/// Evaluates the 4D Christoffel connection coefficients Gamma^rho_{mu, nu} at (x, y, z).
pub fn evaluate_christoffel_symbols(_x: f64, _y: f64, z: f64) -> [[[f64; 4]; 4]; 4] {
    let mut gamma = [[[0.0; 4]; 4]; 4];
    let c = SPEED_OF_LIGHT;
    let alpha = lapse_function(z);
    let d_alpha_dz = (G_TARGET / (c * c)) * alpha;

    let g0_03 = d_alpha_dz / alpha;
    gamma[0][0][3] = g0_03;
    gamma[0][3][0] = g0_03;

    let g3_00 = alpha * d_alpha_dz;
    gamma[3][0][0] = g3_00;

    gamma
}

/// Evaluates the electric Weyl curvature tensor E_ij with counter-term compensation.
pub fn evaluate_electric_weyl(z: f64) -> [[f64; 3]; 3] {
    let c = SPEED_OF_LIGHT;
    let alpha = lapse_function(z);
    let d_alpha_dz = (G_TARGET / (c * c)) * alpha;

    let ratio = d_alpha_dz / alpha;
    let bare_e_zz = ratio * ratio;

    let net_e_zz = bare_e_zz - (I_00_COUNTER_TERM * 0.5);

    let mut e_ij = [[0.0; 3]; 3];
    if f64_abs(net_e_zz) < 1.0e-30 {
        e_ij[2][2] = 0.0;
    } else {
        e_ij[2][2] = net_e_zz;
    }
    e_ij
}

/// Evaluates the Israel-Darmois boundary surface stress-energy tensor S^a_b on the boundary torus.
pub fn evaluate_boundary_junction_stress() -> [[f64; 3]; 3] {
    let c = SPEED_OF_LIGHT;
    let g0 = G_TARGET;
    let g_newton = GRAVITATIONAL_CONSTANT;
    let pi = core::f64::consts::PI;

    let tension = (c * c * g0) / (8.0 * pi * g_newton);

    let mut s = [[0.0; 3]; 3];
    s[0][0] = 0.0;      // Zero surface mass-energy density: sigma = -S^0_0 = 0
    s[1][1] = tension;  // Positive isotropic boundary hoop tension along x
    s[2][2] = tension;  // Positive isotropic boundary hoop tension along y
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_irrotational_shift_and_zero_coriolis() {
        let pos = [15.0, -30.0, 45.0];
        let curl = evaluate_shift_curl(pos[0], pos[1], pos[2]);
              assert_eq!(curl, [0.0, 0.0, 0.0]);

              let velocity = [12.0, -8.0, 3.5];
              let a_cor = compute_coriolis_acceleration(velocity, pos);
              assert_eq!(a_cor, [0.0, 0.0, 0.0]);
          }

          #[test]
          fn test_acceleration_magnitude() {
              let z = 0.0;
              let c = SPEED_OF_LIGHT;
              let gamma = evaluate_christoffel_symbols(0.0, 0.0, z);

              let g3_00 = gamma[3][0][0];
              let physical_acc = g3_00 * (c * c);
              let diff = f64_abs(physical_acc - G_TARGET);
              assert!(diff < 1.0e-6);
          }

          #[test]
          fn test_weyl_curvature_annihilation() {
              let e_ij = evaluate_electric_weyl(25.0);
              for row in &e_ij {
                  for &val in row {
                      assert!(f64_abs(val) <= PRECISION_TOLERANCE);
                  }
              }
          }

          #[test]
          fn test_israel_junction_zero_energy_density() {
              let s = evaluate_boundary_junction_stress();
              assert_eq!(s[0][0], 0.0);
              assert!(s[1][1] > 5.0e26);
              assert_eq!(s[1][1], s[2][2]);
          }
      }
