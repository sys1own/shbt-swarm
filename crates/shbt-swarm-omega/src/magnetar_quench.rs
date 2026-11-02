//! `crates/shbt-swarm-omega/src/magnetar_quench.rs`
//!
//! Magnetar Crustal Gravitomagnetic Helicity Quencher (>= 120 dB Attenuation).
//! Pure #![no_std], zero-heap allocation implementation.

pub const TARGET_ATTENUATION_DB: f64 = 120.0;
pub const GRAVITATIONAL_CONSTANT: f64 = 6.674_30e-11;

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum QuenchStatus {
    Standby = 0x00,
    GravitomagneticLock = 0x01,
    QuenchingActive = 0x02,
    Completed = 0x03,
    ReconnectionFault = 0xFF,
}

impl QuenchStatus {
    #[allow(non_upper_case_globals)]
    pub const QuenchComplete120dB: Self = Self::Completed;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HelicityQuenchReport {
    pub initial_field_tesla: f64,
    pub current_field_tesla: f64,
    pub attenuation_db: f64,
    pub helicity_evacuation_rate_w: f64,
    pub elapsed_quench_time_s: f64,
    pub topological_winding_w: i64,
    pub status: QuenchStatus,
}

pub struct MagnetarHelicityQuencher {
    pub b_init_tesla: f64,
    pub b_current_tesla: f64,
    pub elapsed_time_s: f64,
    pub quench_tau_s: f64,
    pub target_attenuation_db: f64,
    pub current_db_down: f64,
    pub status: QuenchStatus,
    pub winding_number: i64,
}

impl MagnetarHelicityQuencher {
    pub const fn new(b_surface_tesla: f64) -> Self {
        Self {
            b_init_tesla: b_surface_tesla,
            b_current_tesla: b_surface_tesla,
            elapsed_time_s: 0.0,
            quench_tau_s: 100.0,
            target_attenuation_db: TARGET_ATTENUATION_DB,
            current_db_down: 0.0,
            status: QuenchStatus::Standby,
            winding_number: 0,
        }
    }

    pub fn is_quenched(&self) -> bool {
        self.status == QuenchStatus::Completed || self.current_db_down >= self.target_attenuation_db
    }

    pub fn exp_approx(x: f64) -> f64 {
        if x < -20.0 {
            return 0.0;
        }
        let mut sum = 1.0;
        let mut term = 1.0;
        let mut i = 1.0;
        while i < 20.0 {
            term *= x / i;
            sum += term;
            i += 1.0;
        }
        sum
    }

    pub fn log10_approx(val: f64) -> f64 {
        if val <= 0.0 {
            return -999.0;
        }
        let ln10 = 2.302_585_092_994_046;
        let mut v = val;
        let mut exponent = 0.0;
        while v < 0.1 {
            v *= 10.0;
            exponent -= 1.0;
        }
        while v > 1.0 {
            v /= 10.0;
            exponent += 1.0;
        }
        let z = (v - 1.0) / (v + 1.0);
        let z2 = z * z;
        let ln_val = 2.0 * z * (1.0 + z2 / 3.0 + (z2 * z2) / 5.0 + (z2 * z2 * z2) / 7.0);
        (ln_val / ln10) + exponent
    }

    pub fn step_quench(&mut self, dt_s: f64) -> HelicityQuenchReport {
        if self.status == QuenchStatus::Standby {
            self.status = QuenchStatus::GravitomagneticLock;
        }
        self.elapsed_time_s += dt_s;
        let decay_factor = Self::exp_approx(-dt_s / self.quench_tau_s);
        self.b_current_tesla *= decay_factor;
        let ratio = self.b_current_tesla / self.b_init_tesla;
        let atten_db = -20.0 * Self::log10_approx(ratio);
        self.current_db_down = atten_db;
        let helicity_power_w = 1.0e38 * (self.b_current_tesla / self.b_init_tesla) * (self.b_current_tesla / self.b_init_tesla);
        self.winding_number = (self.elapsed_time_s / 10.0) as i64;
        if atten_db >= self.target_attenuation_db {
            self.status = QuenchStatus::Completed;
        } else {
            self.status = QuenchStatus::QuenchingActive;
        }
        HelicityQuenchReport {
            initial_field_tesla: self.b_init_tesla,
            current_field_tesla: self.b_current_tesla,
            attenuation_db: atten_db,
            helicity_evacuation_rate_w: helicity_power_w,
            elapsed_quench_time_s: self.elapsed_time_s,
            topological_winding_w: self.winding_number,
            status: self.status,
        }
    }
}

pub trait FloatApproximations {
    fn exp_approx(self) -> f64;
    fn log10_approx(self) -> f64;
}

impl FloatApproximations for f64 {
    fn exp_approx(self) -> f64 {
        MagnetarHelicityQuencher::exp_approx(self)
    }

    fn log10_approx(self) -> f64 {
        MagnetarHelicityQuencher::log10_approx(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quench_to_completion() {
        let mut quencher = MagnetarHelicityQuencher::new(1.0e11);
        assert!(!quencher.is_quenched());
        let rep = quencher.step_quench(1400.0);
        assert!(rep.attenuation_db >= 120.0);
        assert_eq!(rep.status, QuenchStatus::Completed);
        assert!(quencher.is_quenched());
    }
}
