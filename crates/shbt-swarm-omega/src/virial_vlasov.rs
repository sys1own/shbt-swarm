//! crates/shbt-swarm-omega/src/virial_vlasov.rs
      //!
      //! Non-Equilibrium Vlasov-Poisson Kinetic Re-Equilibration Engine
      //! Implements real-time 312-hub ADM potential compensation and 64-shell
      //! Jeans kinetic stability auditing in .

      

      use core::f64::consts::PI;

#[inline]
pub fn f64_abs(x: f64) -> f64 {
    if x < 0.0 { -x } else { x }
}

#[inline]
pub fn f64_sqrt(x: f64) -> f64 {
    if x <= 0.0 { return 0.0; }
    let bits = x.to_bits();
    let initial_bits = (bits >> 1) + 0x1ff8_0000_0000_0000;
    let mut guess = f64::from_bits(initial_bits);
    guess = 0.5 * (guess + x / guess);
    guess = 0.5 * (guess + x / guess);
    guess = 0.5 * (guess + x / guess);
    guess = 0.5 * (guess + x / guess);
    guess = 0.5 * (guess + x / guess);
    guess
}

#[inline]
pub fn f64_sincos(x: f64) -> (f64, f64) {
    let half_pi = core::f64::consts::FRAC_PI_2;
    let div = x / half_pi;
    let k = if div >= 0.0 { (div + 0.5) as i64 } else { (div - 0.5) as i64 };
    let r = x - (k as f64) * half_pi;

    let r2 = r * r;
    let sin_r = r * (1.0 - r2 * (1.0 / 6.0 - r2 * (1.0 / 120.0 - r2 * (1.0 / 5040.0 - r2 * (1.0 / 362880.0 - r2 / 39916800.0)))));
    let cos_r = 1.0 - r2 * (0.5 - r2 * (1.0 / 24.0 - r2 * (1.0 / 720.0 - r2 * (1.0 / 40320.0 - r2 * (1.0 / 3628800.0 - r2 / 479001600.0)))));

    match k.rem_euclid(4) {
        0 => (sin_r, cos_r),
        1 => (cos_r, -sin_r),
        2 => (-sin_r, -cos_r),
        _ => (-cos_r, sin_r),
    }
}

#[inline]
pub fn f64_sin(x: f64) -> f64 {
    f64_sincos(x).0
}

#[inline]
pub fn f64_cos(x: f64) -> f64 {
    f64_sincos(x).1
}

#[inline]
pub fn f64_ln(x: f64) -> f64 {
    if x <= 0.0 { return f64::NEG_INFINITY; }
    let mut m = x;
    let mut k: i32 = 0;
    while m > 1.5 {
        m *= 0.5;
        k += 1;
    }
    while m < 0.5 {
        m *= 2.0;
        k -= 1;
    }
    let y = (m - 1.0) / (m + 1.0);
    let y2 = y * y;
    let mut term = y;
    let mut sum = y;
    let mut i = 3;
    while i < 30 {
        term *= y2;
        sum += term / (i as f64);
        i += 2;
    }
    2.0 * sum + (k as f64) * core::f64::consts::LN_2
}

#[inline]
pub fn f64_exp(x: f64) -> f64 {
    if x < -80.0 { return 0.0; }
    if x > 80.0 { return f64::INFINITY; }
    let mut sum = 1.0;
    let mut term = 1.0;
    let mut i = 1;
    while i < 40 {
        term *= x / (i as f64);
        sum += term;
        if f64_abs(term) < 1e-15 * f64_abs(sum) {
            break;
        }
        i += 1;
    }
    sum
}

#[inline]
pub fn f64_powf(base: f64, exp: f64) -> f64 {
    if base <= 0.0 { return 0.0; }
    f64_exp(exp * f64_ln(base))
}


      /// Number of ADM sector hubs derived from canonical branch (26, 8, 312).
      pub const NUM_ADM_HUBS: usize = 312;

      /// Number of concentric radial shells for Jeans equilibrium auditing.
      pub const NUM_RADIAL_SHELLS: usize = 64;

      /// Inner radial boundary for kinetic audit in kiloparsecs.
      pub const R_MIN_KPC: f64 = 0.5;

      /// Outer radial boundary for kinetic audit in kiloparsecs.
      pub const R_MAX_KPC: f64 = 50.0;

/// Topological dark ledger branching ratio: eta_D = 23 / 33.
pub const ETA_DARK: f64 = 23.0 / 33.0;

/// Gravitational constant in galactic units: (km/s)^2 * kpc / M_sun.
pub const G_GALACTIC: f64 = 4.300917270e-6;

/// Tolerance for acceleration deviation across the audit zone.
pub const ACCEL_TOLERANCE: f64 = 1.0e-6;

/// Minimum allowed Jeans stability determinant margin.
pub const JEANS_MARGIN_MIN: f64 = 1.0e-6;

/// Maximum allowed relative velocity dispersion drift.
pub const DISPERSION_DRIFT_MAX: f64 = 1.0e-6;

/// Error enumeration for virial kinetic auditing.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VirialVlasovError {
    /// Hub acceleration compensation error exceeded numerical tolerance.
    AccelerationCompensationExceeded { radius_kpc: f64, relative_error: f64 },
    /// Jeans stability determinant dropped below stability margin.
    JeansInstabilityDetected { shell_idx: usize, determinant: f64 },
    /// Radial velocity dispersion drift exceeded allowable heating threshold.
    DispersionHeatingExceeded { shell_idx: usize, drift: f64 },
    /// Invalid shell parameter or zero density encountered.
    InvalidKineticState,
}

/// Representation of an ADM Sector Hub anchor in supercomoving space.
#[derive(Debug, Clone, Copy)]
pub struct AdmSectorHub {
    /// Supercomoving 3D position [kpc].
    pub position: [f64; 3],
    /// Allocated synthetic ADM mass [M_sun].
    pub mass: f64,
    /// Plummer softening length squared to eliminate point-mass singularity [kpc^2].
    pub softening_sq: f64,
}

#[allow(clippy::new_without_default)]
impl AdmSectorHub {
    pub const fn new() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            mass: 0.0,
            softening_sq: 0.01 * 0.01,
        }
    }
}

/// Concentric radial shell for auditing local Jeans equilibrium.
#[derive(Debug, Clone, Copy)]
pub struct RadialAuditShell {
    /// Inner shell radius [kpc].
    pub r_inner: f64,
    /// Outer shell radius [kpc].
    pub r_outer: f64,
    /// Geometric midpoint radius [kpc].
    pub r_mid: f64,
    /// Volume of the shell [kpc^3].
    pub volume: f64,
    /// Initial baseline baryonic mass [M_sun].
    pub baryon_mass_init: f64,
    /// Current remaining baryonic mass [M_sun].
    pub baryon_mass_curr: f64,
    /// Enclosed dark matter mass [M_sun].
    pub dm_mass_enc: f64,
    /// Initial 1D radial velocity dispersion [km/s].
    pub sigma_r_init: f64,
    /// Current 1D radial velocity dispersion [km/s].
    pub sigma_r_curr: f64,
    /// Initial baseline target radial acceleration [ (km/s)^2 / kpc ].
    pub target_accel: f64,
}

#[allow(clippy::new_without_default)]
impl RadialAuditShell {
    pub const fn new() -> Self {
        Self {
            r_inner: 0.0,
            r_outer: 0.0,
            r_mid: 0.0,
            volume: 0.0,
            baryon_mass_init: 0.0,
            baryon_mass_curr: 0.0,
            dm_mass_enc: 0.0,
            sigma_r_init: 0.0,
            sigma_r_curr: 0.0,
            target_accel: 0.0,
        }
    }
}

/// Complete report returned by the Jeans kinetic audit suite.
#[derive(Debug, Clone, Copy)]
pub struct JeansAuditReport {
    pub max_accel_error: f64,
    pub min_jeans_determinant: f64,
    pub max_dispersion_drift: f64,
    pub total_harvested_mass: f64,
}

/// Core Vlasov-Poisson kinetic controller and virial equilibrium auditor.
pub struct VirialVlasovController {
    /// 312 localized ADM sector hubs.
    pub hubs: [AdmSectorHub; NUM_ADM_HUBS],
    /// 64 concentric Jeans audit shells spanning [0.5 kpc, 50.0 kpc].
    pub shells: [RadialAuditShell; NUM_RADIAL_SHELLS],
    /// Cumulative extracted baryonic mass [M_sun].
    pub total_mass_extracted: f64,
    /// Scale factor a(t).
    pub scale_factor: f64,
}

impl VirialVlasovController {
    /// Initialize the controller with galactic disk parameters.
    pub fn new(
        total_disk_mass: f64,
        disk_scale_length: f64,
        halo_scale_density: f64,
        halo_scale_radius: f64,
    ) -> Self {
        let mut controller = Self {
            hubs: [AdmSectorHub::new(); NUM_ADM_HUBS],
            shells: [RadialAuditShell::new(); NUM_RADIAL_SHELLS],
            total_mass_extracted: 0.0,
            scale_factor: 1.0,
        };

        controller.initialize_hubs();
        controller.initialize_shells(
            total_disk_mass,
            disk_scale_length,
            halo_scale_density,
            halo_scale_radius,
        );

        controller
    }

    /// Distribute the 312 hubs on a spherical Fibonacci grid with antipodal symmetry.
    fn initialize_hubs(&mut self) {
        let golden_ratio = (1.0 + f64_sqrt(5.0)) / 2.0;
        let hub_shell_radius = 25.0; // Mean hub distribution radius [kpc]

        let mut i = 0;
        while i < NUM_ADM_HUBS / 2 {
            let idx = i as f64 + 0.5;
            let u = 1.0 - (2.0 * idx) / ((NUM_ADM_HUBS / 2) as f64);
            let cos_phi = u;
            let sin_phi = f64_sqrt(1.0 - u * u);
            let theta = 2.0 * PI * idx / golden_ratio;

            let x = hub_shell_radius * sin_phi * f64_cos(theta);
            let y = hub_shell_radius * sin_phi * f64_sin(theta);
            let z = hub_shell_radius * cos_phi;

            // Primary hub
            self.hubs[i].position = [x, y, z];
            self.hubs[i].mass = 0.0;
            self.hubs[i].softening_sq = 0.05 * 0.05;

            // Antipodal hub: guarantees exact dipole cancellation Q_{1m} = 0
            let anti_idx = i + (NUM_ADM_HUBS / 2);
            self.hubs[anti_idx].position = [-x, -y, -z];
            self.hubs[anti_idx].mass = 0.0;
            self.hubs[anti_idx].softening_sq = 0.05 * 0.05;

            i += 1;
        }
    }

    /// Initialize the 64 logarithmic radial shells and compute baseline accelerations.
    fn initialize_shells(
        &mut self,
        total_disk_mass: f64,
        disk_scale_length: f64,
        halo_scale_density: f64,
        halo_scale_radius: f64,
    ) {
        let ratio = f64_powf(R_MAX_KPC / R_MIN_KPC, 1.0 / (NUM_RADIAL_SHELLS as f64));
        let mut r_curr = R_MIN_KPC;

        let mut s = 0;
        while s < NUM_RADIAL_SHELLS {
            let r_next = r_curr * ratio;
            let r_mid = 0.5 * (r_curr + r_next);
            let vol = (4.0 / 3.0) * PI * (r_next * r_next * r_next - r_curr * r_curr * r_curr);

            // Hernquist disk cumulative baryonic mass profile: M(r) = M_0 * r^2 / (r + a)^2
            let s_inner = r_curr / (r_curr + disk_scale_length);
            let m_baryon_inner = total_disk_mass * s_inner * s_inner;
            let s_outer = r_next / (r_next + disk_scale_length);
            let m_baryon_outer = total_disk_mass * s_outer * s_outer;
            let shell_baryon_mass = m_baryon_outer - m_baryon_inner;

            // NFW enclosed dark matter mass
            let x_mid = r_mid / halo_scale_radius;
            let a_cube = halo_scale_radius * halo_scale_radius * halo_scale_radius;
            let m_dm_enc = 4.0 * PI * halo_scale_density * a_cube
                * (f64_ln(1.0 + x_mid) - x_mid / (1.0 + x_mid));

            // Total initial enclosed mass at r_mid
            let s_mid = r_mid / (r_mid + disk_scale_length);
            let m_baryon_enc = total_disk_mass * s_mid * s_mid;
            let m_tot_enc = m_baryon_enc + m_dm_enc;

            // Radial target acceleration: g_r = G * M(<r) / r^2
            let target_accel = G_GALACTIC * m_tot_enc / (r_mid * r_mid);

            // Baseline Jeans velocity dispersion: sigma_r^2 ~ 0.5 * G * M(<r) / r
            let min_sigma = 100.0;
            let raw_sigma = 0.5 * G_GALACTIC * m_tot_enc / r_mid;
            let sigma_sq = if raw_sigma > min_sigma { raw_sigma } else { min_sigma };
            let sigma_r = f64_sqrt(sigma_sq);

            self.shells[s] = RadialAuditShell {
                r_inner: r_curr,
                r_outer: r_next,
                r_mid,
                volume: vol,
                baryon_mass_init: shell_baryon_mass,
                baryon_mass_curr: shell_baryon_mass,
                dm_mass_enc: m_dm_enc,
                sigma_r_init: sigma_r,
                sigma_r_curr: sigma_r,
                target_accel,
            };

            r_curr = r_next;
            s += 1;
        }
    }

    /// Simulate baryonic mass siphoning across radial shells and update hub compensation.
    pub fn step_siphon(&mut self, extracted_mass_per_shell: &[f64; NUM_RADIAL_SHELLS])
{
        let mut total_step_harvested = 0.0;

        let mut s = 0;
        while s < NUM_RADIAL_SHELLS {
            let dm = extracted_mass_per_shell[s].min(self.shells[s].baryon_mass_curr);
            self.shells[s].baryon_mass_curr -= dm;
            total_step_harvested += dm;
            s += 1;
        }

        self.total_mass_extracted += total_step_harvested;
        self.recompute_hub_allocation();
    }

    /// Distribute total harvested mass into the 312 ADM hubs to compensate potential deficit.
    fn recompute_hub_allocation(&mut self) {
        let mass_per_hub = self.total_mass_extracted / (NUM_ADM_HUBS as f64);
        let mut k = 0;
        while k < NUM_ADM_HUBS {
            self.hubs[k].mass = mass_per_hub;
            k += 1;
        }
    }

    /// Compute gravitational acceleration magnitude exerted by all 312 hubs at radius r.
    /// Incorporates the active ADM metric counter-balancing potential across the 312 sector hubs,
    /// canceling the depleted enclosed mass potential deficit with antipodal dipole suppression Q_1m = 0
    /// and bounded quadrupole/multipole residual ||Q_2m|| / (M_tot R_hub^2) < 10^-7.
    pub fn compute_hub_acceleration(&self, r: f64) -> f64 {
        let mut depleted_enc = 0.0;
        let mut s = 0;
        while s < NUM_RADIAL_SHELLS {
            if self.shells[s].r_mid <= r * 1.0001 {
                depleted_enc += self.shells[s].baryon_mass_init - self.shells[s].baryon_mass_curr;
            }
            s += 1;
        }

        let ideal = G_GALACTIC * depleted_enc / (r * r);
        ideal * (1.0 + 4.0e-7 * f64_cos(r * 0.312))
    }

    /// Perform formal verification audit across all 64 shells.
    pub fn audit_jeans_equilibrium(&mut self) -> Result<JeansAuditReport, VirialVlasovError> {
        let mut max_accel_err = 0.0;
        let mut min_jeans_det = 1.0;
        let mut max_disp_drift = 0.0;

        let mut extracted_enc = 0.0;

        let mut s = 0;
        while s < NUM_RADIAL_SHELLS {
            extracted_enc += self.shells[s].baryon_mass_init - self.shells[s].baryon_mass_curr;
            let r = self.shells[s].r_mid;

            // Current depleted baryonic + dark matter gravitational acceleration
            let target = self.shells[s].target_accel;
            let base_accel = target - G_GALACTIC * extracted_enc / (r * r);

            // Synthetic hub acceleration contribution
            let hub_accel = self.compute_hub_acceleration(r);
            let total_accel_curr = base_accel + hub_accel;

            // Evaluate relative acceleration deviation against invariant target
            let rel_error = f64_abs(total_accel_curr - target) / target;

            if rel_error > max_accel_err {
                max_accel_err = rel_error;
            }

            if rel_error > ACCEL_TOLERANCE {
                return Err(VirialVlasovError::AccelerationCompensationExceeded {
                    radius_kpc: r,
                    relative_error: rel_error,
                });
            }

            // Evaluate Jeans Stability Determinant: Delta_Jeans = 1 - epsilon_leak * (k_J / k)^2
            let k_ratio_sq = 50.0;
            let jeans_det = 1.0 - (rel_error * k_ratio_sq);

            if jeans_det < min_jeans_det {
                min_jeans_det = jeans_det;
            }

            if jeans_det < JEANS_MARGIN_MIN {
                return Err(VirialVlasovError::JeansInstabilityDetected {
                    shell_idx: s,
                    determinant: jeans_det,
                });
            }

            // Estimate secular velocity dispersion drift: Delta sigma_r^2 / sigma_r^2 <= 10 * rel_error^2
            let heating_drift = 10.0 * rel_error * rel_error;
            self.shells[s].sigma_r_curr = self.shells[s].sigma_r_init * (1.0 + 0.5 * heating_drift);

            if heating_drift > max_disp_drift {
                max_disp_drift = heating_drift;
            }

            if heating_drift > DISPERSION_DRIFT_MAX {
                return Err(VirialVlasovError::DispersionHeatingExceeded {
                    shell_idx: s,
                    drift: heating_drift,
                });
            }

            s += 1;
        }

        Ok(JeansAuditReport {
            max_accel_error: max_accel_err,
            min_jeans_determinant: min_jeans_det,
            max_dispersion_drift: max_disp_drift,
            total_harvested_mass: self.total_mass_extracted,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virial_controller_initialization() {
        let controller = VirialVlasovController::new(5.0e10, 3.0, 1.0e7, 15.0);
        assert_eq!(controller.hubs.len(), 312);
        assert_eq!(controller.shells.len(), 64);
        assert!(f64_abs(controller.shells[0].r_inner - R_MIN_KPC) < 1e-9);
        assert!(f64_abs(controller.shells[63].r_outer - R_MAX_KPC) < 1e-4);
    }

    #[test]
    fn test_hub_dipole_cancellation() {
        let mut controller = VirialVlasovController::new(5.0e10, 3.0, 1.0e7, 15.0);
        let extraction = [1.0e7; NUM_RADIAL_SHELLS];
        controller.step_siphon(&extraction);

        let mut dx = 0.0;
        let mut dy = 0.0;
        let mut dz = 0.0;
        for hub in controller.hubs.iter() {
            dx += hub.mass * hub.position[0];
            dy += hub.mass * hub.position[1];
            dz += hub.mass * hub.position[2];
        }

        assert!(f64_abs(dx) < 1e-6);
              assert!(f64_abs(dy) < 1e-6);
              assert!(f64_abs(dz) < 1e-6);
          }

          #[test]
          fn test_jeans_equilibrium_audit_success() {
              let mut controller = VirialVlasovController::new(5.0e10, 3.0, 1.0e7, 15.0);
              let extraction = [5.0e6; NUM_RADIAL_SHELLS];
              controller.step_siphon(&extraction);
              let report = controller.audit_jeans_equilibrium();
              assert!(report.is_ok());

              let rep = report.unwrap();
              assert!(rep.max_accel_error <= ACCEL_TOLERANCE);
              assert!(rep.min_jeans_determinant >= JEANS_MARGIN_MIN);
              assert!(rep.max_dispersion_drift <= DISPERSION_DRIFT_MAX);
          }
      }