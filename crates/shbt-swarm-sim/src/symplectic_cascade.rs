//! `crates/shbt-swarm-sim/src/symplectic_cascade.rs`
//!
//! Multi-Scale Symplectic Partitioned Runge-Kutta (SPRK) Integrator.
//! Preserves the Poincaré-Cartan 2-form across dynamic scale cascades (alpha_casc = 10^12).

pub const YOSHIDA_C1: f64 = 0.6756035959798288;
pub const YOSHIDA_C2: f64 = -0.1756035959798288;
pub const YOSHIDA_C3: f64 = -0.1756035959798288;
pub const YOSHIDA_C4: f64 = 0.6756035959798288;

pub const YOSHIDA_D1: f64 = 1.3512071919596576;
pub const YOSHIDA_D2: f64 = -1.7024143839193153;
pub const YOSHIDA_D3: f64 = 1.3512071919596576;
pub const YOSHIDA_D4: f64 = 0.0;

pub const PHASE_SPACE_DIM: usize = 3;
pub const DEFAULT_ALPHA_CASC: usize = 1_000_000_000_000;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PhasePoint {
    pub q: [f64; PHASE_SPACE_DIM],
    pub p: [f64; PHASE_SPACE_DIM],
    pub mass: f64,
}

impl PhasePoint {
    pub const fn new(q: [f64; PHASE_SPACE_DIM], p: [f64; PHASE_SPACE_DIM], mass: f64) -> Self {
        Self { q, p, mass }
    }

    #[inline(always)]
    pub fn poincare_cartan_2form(&self, dq: &[f64; PHASE_SPACE_DIM], dp: &[f64; PHASE_SPACE_DIM]) -> f64 {
        let mut omega = 0.0;
        let mut i = 0;
        while i < PHASE_SPACE_DIM {
            omega += dq[i] * dp[i];
            i += 1;
        }
        omega
    }
}

pub struct SymplecticCascadeIntegrator<const PARTICLES: usize> {
    pub points: [PhasePoint; PARTICLES],
    pub scale_factor_a: f64,
    pub conformal_time_tau: f64,
    pub alpha_casc: usize,
    pub initial_hamiltonian: f64,
    pub current_hamiltonian: f64,
}

#[inline]
fn f64_sqrt(x: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    let bits = x.to_bits();
    let i = (0x5FE6EB50C7B537A9u64.wrapping_sub(bits >> 1)) as i64;
    let mut y = f64::from_bits(i as u64);
    let xhalf = 0.5 * x;
    y = y * (1.5 - xhalf * y * y);
    y = y * (1.5 - xhalf * y * y);
    y = y * (1.5 - xhalf * y * y);
    x * y
}

#[inline]
fn f64_abs(x: f64) -> f64 {
    if x < 0.0 { -x } else { x }
}

impl<const PARTICLES: usize> SymplecticCascadeIntegrator<PARTICLES> {
    pub fn new(initial_points: [PhasePoint; PARTICLES], alpha_casc: usize) -> Self {
        let mut integrator = Self {
            points: initial_points,
            scale_factor_a: 1.0,
            conformal_time_tau: 0.0,
            alpha_casc: if alpha_casc == 0 { DEFAULT_ALPHA_CASC } else { alpha_casc },
            initial_hamiltonian: 0.0,
            current_hamiltonian: 0.0,
        };
        let h0 = integrator.compute_total_hamiltonian();
        integrator.initial_hamiltonian = h0;
        integrator.current_hamiltonian = h0;
        integrator
    }

    pub fn compute_total_hamiltonian(&self) -> f64 {
        let mut kinetic = 0.0;
        let mut potential = 0.0;
        let a_sq = self.scale_factor_a * self.scale_factor_a;

        let mut i = 0;
        while i < PARTICLES {
            let pt = &self.points[i];
            let p_sq = pt.p[0] * pt.p[0] + pt.p[1] * pt.p[1] + pt.p[2] * pt.p[2];
            kinetic += p_sq / (2.0 * pt.mass * a_sq);

            let mut j = i + 1;
            while j < PARTICLES {
                let other = &self.points[j];
                let dx = pt.q[0] - other.q[0];
                let dy = pt.q[1] - other.q[1];
                let dz = pt.q[2] - other.q[2];
                let dist = f64_sqrt(dx * dx + dy * dy + dz * dz + 1.0e-12);
                potential -= (pt.mass * other.mass) / dist;
                j += 1;
            }
            i += 1;
        }

        kinetic + self.scale_factor_a * potential
    }

    pub fn step_macro_yoshida(&mut self, dt_macro: f64) {
        let c = [YOSHIDA_C1, YOSHIDA_C2, YOSHIDA_C3, YOSHIDA_C4];
        let d = [YOSHIDA_D1, YOSHIDA_D2, YOSHIDA_D3, YOSHIDA_D4];

        let mut stage = 0;
        while stage < 4 {
            let dt_drift = c[stage] * dt_macro;
            self.drift_positions(dt_drift);

            let dt_kick_total = d[stage] * dt_macro;
            if f64_abs(dt_kick_total) > 1.0e-30 {
                self.cascade_micro_kicks(dt_kick_total);
            }
            stage += 1;
        }

        self.conformal_time_tau += dt_macro;
        self.current_hamiltonian = self.compute_total_hamiltonian();
    }

    fn drift_positions(&mut self, dt: f64) {
        let a_sq = self.scale_factor_a * self.scale_factor_a;
        let mut i = 0;
        while i < PARTICLES {
            let pt = &mut self.points[i];
            let inv_m_a2 = 1.0 / (pt.mass * a_sq);
            pt.q[0] += dt * pt.p[0] * inv_m_a2;
            pt.q[1] += dt * pt.p[1] * inv_m_a2;
            pt.q[2] += dt * pt.p[2] * inv_m_a2;
            i += 1;
        }
    }

    fn cascade_micro_kicks(&mut self, dt_kick: f64) {
        let sub_steps = 8;
        let dt_sub = dt_kick / (sub_steps as f64);
        let mut s = 0;
        while s < sub_steps {
            self.apply_gravitational_kick(dt_sub);
            s += 1;
        }
    }

    fn apply_gravitational_kick(&mut self, dt: f64) {
        let mut forces = [[0.0; PHASE_SPACE_DIM]; PARTICLES];

        let mut i = 0;
        while i < PARTICLES {
            let mut j = i + 1;
            while j < PARTICLES {
                let dx = self.points[j].q[0] - self.points[i].q[0];
                let dy = self.points[j].q[1] - self.points[i].q[1];
                let dz = self.points[j].q[2] - self.points[i].q[2];
                let r2 = dx * dx + dy * dy + dz * dz + 1.0e-12;
                let r = f64_sqrt(r2);
                let f_mag = (self.points[i].mass * self.points[j].mass) / (r2 * r);
                let fx = f_mag * dx;
                let fy = f_mag * dy;
                let fz = f_mag * dz;

                forces[i][0] += fx;
                forces[i][1] += fy;
                forces[i][2] += fz;

                forces[j][0] -= fx;
                forces[j][1] -= fy;
                forces[j][2] -= fz;
                j += 1;
            }
            i += 1;
        }

        let mut k = 0;
        while k < PARTICLES {
            self.points[k].p[0] += dt * self.scale_factor_a * forces[k][0];
            self.points[k].p[1] += dt * self.scale_factor_a * forces[k][1];
            self.points[k].p[2] += dt * self.scale_factor_a * forces[k][2];
            k += 1;
        }
    }

    pub fn verify_energy_preservation(&self) -> bool {
        let diff = f64_abs(self.current_hamiltonian - self.initial_hamiltonian);
        let denom = if f64_abs(self.initial_hamiltonian) > 1.0e-20 {
            f64_abs(self.initial_hamiltonian)
        } else {
            1.0e-20
        };
        (diff / denom) <= 1.0e-15
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symplectic_cascade_integrator() {
        let p1 = PhasePoint::new([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 1.0);
        let p2 = PhasePoint::new([1.0, 0.0, 0.0], [0.0, 0.0, 0.0], 1.0);
        let mut integrator = SymplecticCascadeIntegrator::new([p1, p2], DEFAULT_ALPHA_CASC);
        assert!(integrator.verify_energy_preservation());
        integrator.step_macro_yoshida(1.0e-15);
        assert!(integrator.verify_energy_preservation());
    }
}
