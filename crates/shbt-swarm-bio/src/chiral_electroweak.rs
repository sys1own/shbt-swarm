//! `crates/shbt-swarm-bio/src/chiral_electroweak.rs`
//! Microscopic Electroweak Parity Selection & Floquet-Amplified Homochiral Condensation (`THM-BIO-05-EXT`).

#![allow(dead_code)]

pub const WEAK_MIXING_SIN2_THETA_W: f64 = 0.23122;
pub const RAW_ELECTROWEAK_PV_GAP_EV: f64 = 1.00e-17;
pub const FLOQUET_312_AMPLIFICATION: f64 = 1.00e16;
pub const BOLTZMANN_EV_PER_K: f64 = 8.617_333_262e-5;
pub const FOLDING_WINDOW_SECONDS: f64 = 11.40e-6;
pub const Q16_ONE: u32 = 0x0001_0000;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChiralElectroweakSynthesizer {
    pub raw_pv_gap_ev: f64,
    pub floquet_gain: f64,
    pub autocatalytic_rate_s: f64,
    pub cubic_saturation_s: f64,
    pub electroweak_bias_drift_s: f64,
}

impl ChiralElectroweakSynthesizer {
    #[inline]
    pub const fn new_default() -> Self {
        Self {
            raw_pv_gap_ev: RAW_ELECTROWEAK_PV_GAP_EV,
            floquet_gain: FLOQUET_312_AMPLIFICATION,
            autocatalytic_rate_s: 2.50e6,
            cubic_saturation_s: 2.50e6,
            electroweak_bias_drift_s: 1.00e5,
        }
    }

    /// Computes the weak nuclear charge Q_{W,A} = Z_A * (1 - 4 sin^2 theta_W) - N_A.
    #[inline]
    pub const fn weak_nuclear_charge(protons_z: u16, neutrons_n: u16) -> f64 {
        (protons_z as f64) * (1.0 - 4.0 * WEAK_MIXING_SIN2_THETA_W) - (neutrons_n as f64)
    }

    /// Returns the 312-channel Floquet-amplified parity-violating energy difference in eV.
    #[inline]
    pub const fn amplified_pv_gap_ev(&self) -> f64 {
        self.raw_pv_gap_ev * self.floquet_gain
    }

    /// Computes the super-thermal chiral selection ratio Delta_E_PV^eff / (k_B T).
    #[inline]
    pub const fn thermal_snr_ratio(&self, temp_kelvin: f64) -> f64 {
        let kb_t = BOLTZMANN_EV_PER_K * temp_kelvin;
        if kb_t <= 0.0 {
            0.0
        } else {
            self.amplified_pv_gap_ev() / kb_t
        }
    }

    /// Integrates the deterministic Kondepudi-Nelson autocatalytic condensation ODE
    /// d(alpha)/dt = lambda * alpha - B * alpha^3 + C_EW over `duration_s` using RK4.
    #[inline]
    pub fn integrate_kondepudi_nelson(&self, duration_s: f64) -> f64 {
        let steps: usize = 1140;
        let dt = duration_s / (steps as f64);
        let mut alpha = 0.0_f64;

        let drift = |a: f64| -> f64 {
            self.autocatalytic_rate_s * a - self.cubic_saturation_s * (a * a * a) + self.electroweak_bias_drift_s
        };

        let mut i = 0;
        while i < steps {
            let k1 = drift(alpha);
            let k2 = drift(alpha + 0.5 * dt * k1);
            let k3 = drift(alpha + 0.5 * dt * k2);
            let k4 = drift(alpha + dt * k3);
            alpha += (dt / 6.0) * (k1 + 2.0 * k2 + 2.0 * k3 + k4);
            if alpha >= 1.0 {
                alpha = 1.0;
                break;
            }
            i += 1;
        }
        alpha
    }

    /// Returns the Q16.16 fixed-point enantiomeric excess register value (`0x00010000` == 1.000000).
    #[inline]
    pub fn integrate_kondepudi_nelson_q16(&self, duration_s: f64) -> u32 {
        let ee = self.integrate_kondepudi_nelson(duration_s);
        if ee >= 0.999_992 {
            Q16_ONE
        } else if ee <= 0.0 {
            0
        } else {
            (ee * (Q16_ONE as f64)) as u32
        }
    }
}
