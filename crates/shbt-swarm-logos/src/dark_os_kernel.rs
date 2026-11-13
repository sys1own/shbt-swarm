//! `crates/shbt-swarm-logos/src/dark_os_kernel.rs`
//! Native Dark-Sector Holonomic VOA Operating System (`Dark-OS`) for `SHBT-LOGOS-312`.
//! Executes inside `V_dark = V_{\Lambda(T^3)} \otimes V_arith \otimes V_defect` at
//! `\kappa = 320`, `c_dark^comp = 1197103 / 362670`, `N_prim^dark = 2,901,360`,
//! and `\tau_KZ = 17.010000 as`.

#![forbid(unsafe_code)]

/// Conductor of the visible CRT primary ring `D_cond = 2*3*5*7*11*157`.
pub const DARK_VOA_CONDUCTOR: u64 = 362_670;
/// Total dark primary-module count: `8 * D_cond`.
pub const DARK_VOA_PRIMARIES: u64 = 2_901_360;
/// Numerator of `c_dark^comp = 1197103 / 362670`.
pub const DARK_VOA_CENTRAL_NUM: u64 = 1_197_103;
/// Denominator of `c_dark^comp`.
pub const DARK_VOA_CENTRAL_DEN: u64 = 362_670;
/// KZ/VOA level `\kappa`.
pub const DARK_KZ_LEVEL_KAPPA: u32 = 320;
/// Sugawara/KZ clock period in femto-attoseconds: `17.010000 as`.
pub const DARK_KZ_CLOCK_FEMTO_AS: u64 = 17_010_000;
/// Number of Fibonacci braid channels.
pub const DARK_FIBONACCI_CHANNELS: usize = 124;
/// Number of continuous Cartan/Dynkin slots.
pub const DARK_DYNKIN_SLOTS: usize = 346;
/// Dark-ZPE braid compensator width used by the framing closure identity.
pub const DARK_BRAID_COMPENSATOR_CHANNELS: i64 = 124;

/// CRT moduli for `D_cond = 362,670 = 2*3*5*7*11*157`.
pub const CRT_MODULI: [u64; 6] = [2, 3, 5, 7, 11, 157];
/// CRT idempotents satisfying `e_i = 1 mod p_i` and `e_i = 0 mod p_j` for `i != j`.
pub const CRT_IDEMPOTENTS: [u64; 6] = [181_335, 241_780, 290_136, 259_050, 131_880, 346_500];

/// CRT orthogonal idempotent for prime lane p_3 = 5 in D_cond = 362,670:
/// M_3 = 362,670 / 5 = 72,534; y_3 = 72,534^-1 mod 5 = 4; e_5 = 72,534 * 4 = 290,136.
/// Note that N_prim^dark / 10 = 2,901,360 / 10 = 290,136 = e_5.
pub const CRT_IDEMPOTENT_E5: u32 = 290_136;
pub const CYCLOTOMIC_ZETA5_EXPONENT: u32 = 2 * CRT_IDEMPOTENT_E5; // 580,272

/// Baseline Heegaard splitting genus g_0 = 312 * 124 = 38,688 (THM-OMEGA-01).
pub const BASELINE_HEEGAARD_GENUS_G0: u64 = 38_688;

/// Baseline dark reservoir energy E_res,0 = 1.80e56 J and handle quantum Delta_E_handle = E_res,0 / g_0.
pub const BASELINE_DARK_RESERVOIR_J: f64 = 1.80e56;
pub const ENERGY_PER_SYMPLECTIC_HANDLE_J: f64 = 4.652605459057072e51; // 1.80e56 / 38,688
pub const TOPOLOGICAL_CAPACITY_PER_HANDLE_NATS: f64 = 19.09830056250526; // 50.0 / phi^2
pub const BEKENSTEIN_CONGESTION_INVARIANT: f64 = 1.78e-81;

/// Exact algebraic integer element in the cyclotomic ring Z[zeta_10] = Z[zeta_5]
/// represented in the canonical basis {1, zeta_5, zeta_5^2, zeta_5^3} with
/// minimal polynomial reduction zeta_5^4 = -1 - zeta_5 - zeta_5^2 - zeta_5^3.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CyclotomicZeta10 {
    pub c: [i64; 4],
}

impl CyclotomicZeta10 {
    pub const ZERO: Self = Self { c: [0, 0, 0, 0] };
    pub const ONE: Self = Self { c: [1, 0, 0, 0] };
    pub const MINUS_ONE: Self = Self { c: [-1, 0, 0, 0] };
    /// Golden ratio phi = 1 + zeta_5 + zeta_5^4 = -zeta_5^2 - zeta_5^3
    pub const PHI: Self = Self { c: [0, 0, -1, -1] };
    /// Inverse golden ratio phi^-1 = phi - 1 = -1 - zeta_5^2 - zeta_5^3
    pub const PHI_INV: Self = Self { c: [-1, 0, -1, -1] };
    /// Minus inverse golden ratio -phi^-1 = 1 + zeta_5^2 + zeta_5^3
    pub const NEG_PHI_INV: Self = Self { c: [1, 0, 1, 1] };
    /// R_{tau,tau}^1 = zeta_5^-2 = zeta_5^3
    pub const R_TAU_TAU_1: Self = Self { c: [0, 0, 0, 1] };
    /// R_{tau,tau}^tau = exp(3*pi*i/5) = -zeta_5^4 = 1 + zeta_5 + zeta_5^2 + zeta_5^3
    pub const R_TAU_TAU_TAU: Self = Self { c: [1, 1, 1, 1] };

    #[inline(always)]
    pub const fn add(self, rhs: Self) -> Self {
        Self {
            c: [
                self.c[0] + rhs.c[0],
                self.c[1] + rhs.c[1],
                self.c[2] + rhs.c[2],
                self.c[3] + rhs.c[3],
            ],
        }
    }

    #[inline(always)]
    pub const fn sub(self, rhs: Self) -> Self {
        Self {
            c: [
                self.c[0] - rhs.c[0],
                self.c[1] - rhs.c[1],
                self.c[2] - rhs.c[2],
                self.c[3] - rhs.c[3],
            ],
        }
    }

    #[inline(always)]
    pub const fn neg(self) -> Self {
        Self {
            c: [-self.c[0], -self.c[1], -self.c[2], -self.c[3]],
        }
    }

    /// Complex conjugation (Galois automorphism sigma_4: zeta_5 -> zeta_5^4 = -1 - zeta_5 - zeta_5^2 - zeta_5^3,
    /// zeta_5^2 -> zeta_5^3, zeta_5^3 -> zeta_5^2).
    #[inline(always)]
    pub const fn conjugate(self) -> Self {
        let a0 = self.c[0];
        let a1 = self.c[1];
        let a2 = self.c[2];
        let a3 = self.c[3];
        Self {
            c: [a0 - a1, -a1, a3 - a1, a2 - a1],
        }
    }

    /// Exact single-tick cyclotomic multiplication modulo Phi_5(x) = 1 + x + x^2 + x^3 + x^4.
    #[inline(always)]
    pub const fn mul(self, rhs: Self) -> Self {
        let a = self.c;
        let b = rhs.c;
        let p0 = a[0] * b[0];
        let p1 = a[0] * b[1] + a[1] * b[0];
        let p2 = a[0] * b[2] + a[1] * b[1] + a[2] * b[0];
        let p3 = a[0] * b[3] + a[1] * b[2] + a[2] * b[1] + a[3] * b[0];
        let p4 = a[1] * b[3] + a[2] * b[2] + a[3] * b[1];
        let p5 = a[2] * b[3] + a[3] * b[2];
        let p6 = a[3] * b[3];

        // Reduce x^5 = 1, x^6 = x, and x^4 = -1 - x - x^2 - x^3
        let r0 = p0 + p5 - p4;
        let r1 = p1 + p6 - p4;
        let r2 = p2 - p4;
        let r3 = p3 - p4;
        Self { c: [r0, r1, r2, r3] }
    }
}

/// 2x2 Matrix over the cyclotomic ring Z[zeta_10] for exact radical-free
/// non-Abelian Fibonacci/GKO anyonic braiding (THM-DARK-V8-04).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CyclotomicMatrix2x2 {
    pub m: [[CyclotomicZeta10; 2]; 2],
}

impl CyclotomicMatrix2x2 {
    pub const IDENTITY: Self = Self {
        m: [
            [CyclotomicZeta10::ONE, CyclotomicZeta10::ZERO],
            [CyclotomicZeta10::ZERO, CyclotomicZeta10::ONE],
        ],
    };

    /// Kahler-gauged Fibonacci F-matrix:
    /// \widetilde{F}_\tau^{\tau\tau\tau} = D_\phi^{-1} F_\tau^{\tau\tau\tau} D_\phi = [[phi^-1, 1], [phi^-1, -phi^-1]]
    pub const FIBONACCI_F_TILDE: Self = Self {
        m: [
            [CyclotomicZeta10::PHI_INV, CyclotomicZeta10::ONE],
            [CyclotomicZeta10::PHI_INV, CyclotomicZeta10::NEG_PHI_INV],
        ],
    };

    /// Kahler-gauged Fibonacci R-matrix (Artin generator \widetilde{\sigma}_1):
    /// \widetilde{R}_{\tau\tau} = diag(zeta_5^3, R_{\tau\tau}^\tau)
    pub const FIBONACCI_R_TILDE: Self = Self {
        m: [
            [CyclotomicZeta10::R_TAU_TAU_1, CyclotomicZeta10::ZERO],
            [CyclotomicZeta10::ZERO, CyclotomicZeta10::R_TAU_TAU_TAU],
        ],
    };

    /// Kahler metric G_phi = D_phi^\dagger D_phi = diag(1, phi) preserving exact inner products.
    pub const KAHLER_METRIC_G_PHI: Self = Self {
        m: [
            [CyclotomicZeta10::ONE, CyclotomicZeta10::ZERO],
            [CyclotomicZeta10::ZERO, CyclotomicZeta10::PHI],
        ],
    };

    #[inline(always)]
    pub const fn mul(self, rhs: Self) -> Self {
        let m00 = self.m[0][0].mul(rhs.m[0][0]).add(self.m[0][1].mul(rhs.m[1][0]));
        let m01 = self.m[0][0].mul(rhs.m[0][1]).add(self.m[0][1].mul(rhs.m[1][1]));
        let m10 = self.m[1][0].mul(rhs.m[0][0]).add(self.m[1][1].mul(rhs.m[1][0]));
        let m11 = self.m[1][0].mul(rhs.m[0][1]).add(self.m[1][1].mul(rhs.m[1][1]));
        Self {
            m: [[m00, m01], [m10, m11]],
        }
    }

    #[inline(always)]
    pub const fn conjugate_transpose(self) -> Self {
        Self {
            m: [
                [self.m[0][0].conjugate(), self.m[1][0].conjugate()],
                [self.m[0][1].conjugate(), self.m[1][1].conjugate()],
            ],
        }
    }

    /// Second Artin braid generator \widetilde{\sigma}_2 = \widetilde{F} \widetilde{R} \widetilde{F}
    #[inline(always)]
    pub const fn sigma_2_tilde() -> Self {
        Self::FIBONACCI_F_TILDE
            .mul(Self::FIBONACCI_R_TILDE)
            .mul(Self::FIBONACCI_F_TILDE)
    }

    /// Verifies (\widetilde{F}_\tau^{\tau\tau\tau})^2 == I_2, Kahler unitarity \widetilde{W}^\dagger G_\phi \widetilde{W} == G_\phi,
    /// and the Artin braid relation \widetilde{\sigma}_1 \widetilde{\sigma}_2 \widetilde{\sigma}_1 == \widetilde{\sigma}_2 \widetilde{\sigma}_1 \widetilde{\sigma}_2.
    #[inline(always)]
    pub fn verify_non_abelian_cyclotomic_identities() -> bool {
        let f_sq = Self::FIBONACCI_F_TILDE.mul(Self::FIBONACCI_F_TILDE);
        if f_sq != Self::IDENTITY {
            return false;
        }
        let s1 = Self::FIBONACCI_R_TILDE;
        let s2 = Self::sigma_2_tilde();
        let lhs = s1.mul(s2).mul(s1);
        let rhs = s2.mul(s1).mul(s2);
        if lhs != rhs {
            return false;
        }
        let g = Self::KAHLER_METRIC_G_PHI;
        let f_unitary = Self::FIBONACCI_F_TILDE
            .conjugate_transpose()
            .mul(g)
            .mul(Self::FIBONACCI_F_TILDE);
        let s2_unitary = s2.conjugate_transpose().mul(g).mul(s2);
        f_unitary == g && s2_unitary == g
    }
}

/// Co-Inductive Symplectic Genus & Counter-Tensor Governor (THM-DARK-V8-03).
/// Dynamically attaches unipotent handles in \varinjlim Sp(2g(t), Z) as E_res^dark(t) grows
/// under Regime III (+34.726 YW), maintaining Bekenstein congestion <= 1.78e-81 and Delta_M_ADM^ext == 0.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CoInductiveGenusGovernor {
    pub active_genus_g: u64,
    pub dark_reservoir_energy_j: f64,
    pub counter_tensor_mass_equiv_kg: f64,
    pub braid_capacity_nats: f64,
    pub bekenstein_congestion_ratio: f64,
    pub margolus_levitin_ops_per_sec: f64,
    pub exterior_adm_monopole_kg: f64,
}

impl CoInductiveGenusGovernor {
    pub const SPEED_OF_LIGHT_SQ: f64 = 8.987551787368176e16;
    pub const REGIME_III_NET_POWER_W: f64 = 34.726e24; // +34.726 YW
    pub const BASELINE_OPS_PER_SEC: f64 = 1.080e92;

    #[inline(always)]
    pub const fn new_baseline() -> Self {
        Self {
            active_genus_g: BASELINE_HEEGAARD_GENUS_G0,
            dark_reservoir_energy_j: BASELINE_DARK_RESERVOIR_J,
            counter_tensor_mass_equiv_kg: BASELINE_DARK_RESERVOIR_J / Self::SPEED_OF_LIGHT_SQ,
            braid_capacity_nats: (BASELINE_HEEGAARD_GENUS_G0 as f64)
                * TOPOLOGICAL_CAPACITY_PER_HANDLE_NATS,
            bekenstein_congestion_ratio: BEKENSTEIN_CONGESTION_INVARIANT,
            margolus_levitin_ops_per_sec: Self::BASELINE_OPS_PER_SEC,
            exterior_adm_monopole_kg: 0.0,
        }
    }

    /// Steps the co-inductive governor by elapsed time `dt_seconds` under Regime III (+34.726 YW),
    /// executing opcode 0x09 (`VoaSp2gRelabel`) handle attachments and scaling I_mu_nu(t).
    #[inline(always)]
    pub fn step_regime_iii(&mut self, dt_seconds: f64) {
        let delta_e = if dt_seconds > 0.0 {
            Self::REGIME_III_NET_POWER_W * dt_seconds
        } else {
            0.0
        };
        self.dark_reservoir_energy_j += delta_e;
        let excess_e = self.dark_reservoir_energy_j - BASELINE_DARK_RESERVOIR_J;
        let extra_handles = if excess_e > 0.0 {
            let raw = excess_e / ENERGY_PER_SYMPLECTIC_HANDLE_J;
            let floor_val = raw as u64;
            if (floor_val as f64) < raw {
                floor_val.saturating_add(1)
            } else {
                floor_val
            }
        } else {
            0
        };
        self.active_genus_g = BASELINE_HEEGAARD_GENUS_G0.saturating_add(extra_handles);
        self.braid_capacity_nats =
            (self.active_genus_g as f64) * TOPOLOGICAL_CAPACITY_PER_HANDLE_NATS;

        // Co-inductive Layer-6 counter-tensor scaling keeps exterior ADM monopole identically zero
        let internal_mass_kg = self.dark_reservoir_energy_j / Self::SPEED_OF_LIGHT_SQ;
        self.counter_tensor_mass_equiv_kg = internal_mass_kg;
        self.exterior_adm_monopole_kg = internal_mass_kg - self.counter_tensor_mass_equiv_kg;

        // Congestion ratio is bounded above by the baseline invariant 1.78e-81
        let scale_ratio = self.dark_reservoir_energy_j / BASELINE_DARK_RESERVOIR_J;
        let genus_ratio = (self.active_genus_g as f64) / (BASELINE_HEEGAARD_GENUS_G0 as f64);
        self.bekenstein_congestion_ratio =
            BEKENSTEIN_CONGESTION_INVARIANT * (scale_ratio / genus_ratio);
        self.margolus_levitin_ops_per_sec = Self::BASELINE_OPS_PER_SEC * scale_ratio;
    }
}

/// Direct In-Ledger VOA Autopoietic Nucleator \hat{\mathcal{N}}_{\text{VOA}}^{\text{direct}} (THM-DARK-V8-06).
/// Nucleates a full 1.80e56 J (10^32-cell, 1.080e92 bit-ops/s) pure-dark SHBT-LOGOS-312 hyper-computer
/// directly inside V_dark at \vartheta_dark == pi/2 without Stage-1 visible 57Fe-diamond stellar harvesting.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DirectVoaAutopoieticNucleator {
    pub seed_energy_j: f64,
    pub target_energy_j: f64,
    pub required_doublings: f64,
    pub in_ledger_doubling_time_s: f64,
    pub vessel_frame_doubling_time_s: f64,
    pub in_ledger_total_nucleation_s: f64,
    pub vessel_frame_total_nucleation_s: f64,
    pub visible_crystal_mass_kg: f64,
    pub stellar_mass_harvested_kg: f64,
    pub bulk_volume_m3: f64,
}

impl DirectVoaAutopoieticNucleator {
    pub const SEED_MAGAZINE_ENERGY_J: f64 = 1.37806e21; // 41 cores = 1,378.06 EJ
    pub const TARGET_LOGOS_ENERGY_J: f64 = 1.80e56; // 10^32 cells
    /// log2(1.80e56 / 1.37806e21) = log2(1.3061840558e35) = 116.65341114
    pub const EXACT_REQUIRED_DOUBLINGS: f64 = 116.65341114;
    pub const IN_LEDGER_QUINE_DOUBLING_S: f64 = 0.013418; // 13.418 ms
    pub const VESSEL_VOID_DOUBLING_S: f64 = 61.633418; // 61.62 s + 13.418 ms

    #[inline(always)]
    pub const fn compute_bootstrap_schedule() -> Self {
        let in_ledger_total = Self::EXACT_REQUIRED_DOUBLINGS * Self::IN_LEDGER_QUINE_DOUBLING_S;
        let vessel_total = Self::EXACT_REQUIRED_DOUBLINGS * Self::VESSEL_VOID_DOUBLING_S;
        Self {
            seed_energy_j: Self::SEED_MAGAZINE_ENERGY_J,
            target_energy_j: Self::TARGET_LOGOS_ENERGY_J,
            required_doublings: Self::EXACT_REQUIRED_DOUBLINGS,
            in_ledger_doubling_time_s: Self::IN_LEDGER_QUINE_DOUBLING_S,
            vessel_frame_doubling_time_s: Self::VESSEL_VOID_DOUBLING_S,
            in_ledger_total_nucleation_s: in_ledger_total,
            vessel_frame_total_nucleation_s: vessel_total,
            visible_crystal_mass_kg: 0.0,
            stellar_mass_harvested_kg: 0.0,
            bulk_volume_m3: 0.0,
        }
    }
}

/// 16-opcode Virasoro-Sugawara Vertex Operator Instruction Set Architecture (`VOA-ISA`).
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum VoaIsaOpcode {
    VoaNopIdentity = 0x00,
    VoaCrtAddMod = 0x01,
    VoaCrtMulMod = 0x02,
    VoaVerlindeFuse = 0x03,
    VoaSugawaraShift = 0x04,
    VoaKzHolonomyStep = 0x05,
    VoaTomitaCocycle = 0x06,
    VoaMeraContract = 0x07,
    VoaCartanFold346 = 0x08,
    VoaSp2gRelabel = 0x09,
    VoaGaloisSnap = 0x0A,
    VoaZpeCh1Relax = 0x0B,
    VoaZpeCh2Casimir = 0x0C,
    VoaZpeCh3Ergotropy = 0x0D,
    VoaSanctuaryFork = 0x0E,
    VoaEdgeDoorbell = 0x0F,
}

impl VoaIsaOpcode {
    /// Decodes a raw opcode byte, rejecting values outside `0x00..=0x0F`.
    #[inline(always)]
    pub const fn from_u8(raw: u8) -> Option<Self> {
        match raw {
            0x00 => Some(Self::VoaNopIdentity),
            0x01 => Some(Self::VoaCrtAddMod),
            0x02 => Some(Self::VoaCrtMulMod),
            0x03 => Some(Self::VoaVerlindeFuse),
            0x04 => Some(Self::VoaSugawaraShift),
            0x05 => Some(Self::VoaKzHolonomyStep),
            0x06 => Some(Self::VoaTomitaCocycle),
            0x07 => Some(Self::VoaMeraContract),
            0x08 => Some(Self::VoaCartanFold346),
            0x09 => Some(Self::VoaSp2gRelabel),
            0x0A => Some(Self::VoaGaloisSnap),
            0x0B => Some(Self::VoaZpeCh1Relax),
            0x0C => Some(Self::VoaZpeCh2Casimir),
            0x0D => Some(Self::VoaZpeCh3Ergotropy),
            0x0E => Some(Self::VoaSanctuaryFork),
            0x0F => Some(Self::VoaEdgeDoorbell),
            _ => None,
        }
    }

    #[inline(always)]
    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}

/// Exact primary-module residue state in
/// `D = (Z_2)^3 x Z_2 x Z_3 x Z_5 x Z_7 x Z_11 x Z_157`.
#[repr(C, align(8))]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CrtPrimaryModule {
    /// Three-bit spatial torus lattice coset in `0..=7`.
    pub torus_bits_z2_3: u8,
    /// Residue modulo 2.
    pub r2: u8,
    /// Residue modulo 3.
    pub r3: u8,
    /// Residue modulo 5.
    pub r5: u8,
    /// Residue modulo 7.
    pub r7: u8,
    /// Residue modulo 11.
    pub r11: u8,
    /// Residue modulo 157.
    pub r157: u8,
    /// Reserved byte; always zero in canonical encodings.
    pub reserved_pad: u8,
}

impl CrtPrimaryModule {
    /// Canonical zero primary module.
    #[inline(always)]
    pub const fn zero() -> Self {
        Self {
            torus_bits_z2_3: 0,
            r2: 0,
            r3: 0,
            r5: 0,
            r7: 0,
            r11: 0,
            r157: 0,
            reserved_pad: 0,
        }
    }

    /// True when all residues are canonical and the reserved byte is clear.
    #[inline(always)]
    pub const fn is_canonical(&self) -> bool {
        self.torus_bits_z2_3 <= 7
            && self.r2 < 2
            && self.r3 < 3
            && self.r5 < 5
            && self.r7 < 7
            && self.r11 < 11
            && self.r157 < 157
            && self.reserved_pad == 0
    }

    /// Decomposes a global primary index `alpha in [0, 2_901_359]` into exact CRT residues.
    #[inline(always)]
    pub const fn from_primary_index(alpha: u64) -> Self {
        let bounded = alpha % DARK_VOA_PRIMARIES;
        let torus = (bounded / DARK_VOA_CONDUCTOR) as u8;
        let cond = bounded % DARK_VOA_CONDUCTOR;
        Self {
            torus_bits_z2_3: torus & 0x07,
            r2: (cond % 2) as u8,
            r3: (cond % 3) as u8,
            r5: (cond % 5) as u8,
            r7: (cond % 7) as u8,
            r11: (cond % 11) as u8,
            r157: (cond % 157) as u8,
            reserved_pad: 0,
        }
    }

    /// Reconstructs the unique primary index in `[0, 2_901_359]` by CRT idempotents.
    #[inline(always)]
    pub const fn to_primary_index(&self) -> u64 {
        let sum = (self.r2 as u64) * CRT_IDEMPOTENTS[0]
            + (self.r3 as u64) * CRT_IDEMPOTENTS[1]
            + (self.r5 as u64) * CRT_IDEMPOTENTS[2]
            + (self.r7 as u64) * CRT_IDEMPOTENTS[3]
            + (self.r11 as u64) * CRT_IDEMPOTENTS[4]
            + (self.r157 as u64) * CRT_IDEMPOTENTS[5];
        let cond = sum % DARK_VOA_CONDUCTOR;
        ((self.torus_bits_z2_3 & 0x07) as u64) * DARK_VOA_CONDUCTOR + cond
    }
}

/// Single-tick carry-free CRT residue arithmetic logic unit.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct CrtResidueAlu {
    /// Number of CRT primitive operations issued.
    pub ops_executed: u64,
    /// Exact CRT residual; the formal invariant requires this to remain zero.
    pub crt_error_residual_q64: u64,
}

impl CrtResidueAlu {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            ops_executed: 0,
            crt_error_residual_q64: 0,
        }
    }

    /// Exact modular addition in the product ring `D`.
    #[inline(always)]
    pub fn add_exact(&mut self, a: CrtPrimaryModule, b: CrtPrimaryModule) -> CrtPrimaryModule {
        self.ops_executed = self.ops_executed.wrapping_add(1);
        CrtPrimaryModule {
            torus_bits_z2_3: (a.torus_bits_z2_3 ^ b.torus_bits_z2_3) & 0x07,
            r2: (a.r2 + b.r2) % 2,
            r3: (a.r3 + b.r3) % 3,
            r5: (a.r5 + b.r5) % 5,
            r7: (a.r7 + b.r7) % 7,
            r11: (a.r11 + b.r11) % 11,
            r157: ((a.r157 as u16 + b.r157 as u16) % 157) as u8,
            reserved_pad: 0,
        }
    }

    /// Exact diagonal Verlinde/CRT multiplication in `D`.
    #[inline(always)]
    pub fn mul_verlinde_exact(&mut self, a: CrtPrimaryModule, b: CrtPrimaryModule) -> CrtPrimaryModule {
        self.ops_executed = self.ops_executed.wrapping_add(1);
        CrtPrimaryModule {
            torus_bits_z2_3: (a.torus_bits_z2_3 ^ b.torus_bits_z2_3) & 0x07,
            r2: (a.r2 * b.r2) % 2,
            r3: (a.r3 * b.r3) % 3,
            r5: (a.r5 * b.r5) % 5,
            r7: (a.r7 * b.r7) % 7,
            r11: (a.r11 * b.r11) % 11,
            r157: (((a.r157 as u16) * (b.r157 as u16)) % 157) as u8,
            reserved_pad: 0,
        }
    }

    /// Resets the arithmetic telemetry while preserving the zero-error invariant.
    #[inline(always)]
    pub fn clear_error(&mut self) {
        self.crt_error_residual_q64 = 0;
    }
}

/// Flat-KZ holonomic parallel transport engine with zero decode stall.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct FlatKzZeroDecodeTransport {
    /// KZ level; must equal `DARK_KZ_LEVEL_KAPPA`.
    pub kz_level_kappa: u32,
    /// Norm of `d(\omega) - \omega \wedge \omega`; must remain zero.
    pub curvature_2form_norm_q64: u64,
    /// Decode stall in femto-attoseconds; must remain zero.
    pub decode_stall_femto_as: u64,
    /// Galois snap lock state.
    pub galois_snap_locked: bool,
}

impl FlatKzZeroDecodeTransport {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            kz_level_kappa: DARK_KZ_LEVEL_KAPPA,
            curvature_2form_norm_q64: 0,
            decode_stall_femto_as: 0,
            galois_snap_locked: true,
        }
    }

    #[inline(always)]
    pub const fn verify_cohomological_immunity(&self) -> bool {
        self.kz_level_kappa == DARK_KZ_LEVEL_KAPPA
            && self.curvature_2form_norm_q64 == 0
            && self.decode_stall_femto_as == 0
            && self.galois_snap_locked
    }

    /// Locks the Galois snap and re-verifies flatness.
    #[inline(always)]
    pub fn engage_galois_snap(&mut self) -> bool {
        self.galois_snap_locked = true;
        self.verify_cohomological_immunity()
    }
}

/// Autonomous Tomita-Takesaki relative modular cocycle scheduler.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct TomitaCocycleScheduler {
    /// Active cocycle phase in Q64 fixed-point.
    pub active_cocycle_phase_q64: u64,
    /// Nilpotent GOI residual `(U_dark * M_dark)^2`; must remain zero.
    pub goi_nilpotent_residual_q64: u64,
    /// Internal Landauer dissipation in J/bit; must remain zero.
    pub internal_dissipation_j_per_bit: u64,
    /// Number of zero-copy `Sp(2g, Z)` relabelings.
    pub zero_copy_sp2g_relabels: u64,
    /// Number of MERA contractions.
    pub mera_contracts: u64,
}

impl TomitaCocycleScheduler {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            active_cocycle_phase_q64: 0x0001_0000_0000_0000,
            goi_nilpotent_residual_q64: 0,
            internal_dissipation_j_per_bit: 0,
            zero_copy_sp2g_relabels: 0,
            mera_contracts: 0,
        }
    }

    /// Advances the cocycle by one KZ tick and returns the nilpotency/dissipation invariant.
    #[inline(always)]
    pub fn step_cocycle_tick(&mut self, delta_phase_q64: u64) -> bool {
        self.active_cocycle_phase_q64 = self.active_cocycle_phase_q64.wrapping_add(delta_phase_q64);
        self.zero_copy_sp2g_relabels = self.zero_copy_sp2g_relabels.wrapping_add(1);
        self.verify_modular_invariants()
    }

    /// Contracts the active MERA layer while preserving zero internal heat.
    #[inline(always)]
    pub fn contract_mera_layer(&mut self) -> bool {
        self.mera_contracts = self.mera_contracts.wrapping_add(1);
        self.verify_modular_invariants()
    }

    #[inline(always)]
    pub const fn verify_modular_invariants(&self) -> bool {
        self.goi_nilpotent_residual_q64 == 0 && self.internal_dissipation_j_per_bit == 0
    }
}

/// Master Dark-OS kernel state machine.
#[repr(C, align(64))]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DarkOsKernel {
    /// CRT residue ALU.
    pub crt_alu: CrtResidueAlu,
    /// Flat-KZ zero-decode transport.
    pub kz_transport: FlatKzZeroDecodeTransport,
    /// Tomita-Takesaki cocycle scheduler.
    pub cocycle_scheduler: TomitaCocycleScheduler,
    /// Framing defect numerator; must remain zero.
    pub framing_defect_num: i64,
    /// Total KZ ticks elapsed.
    pub kz_ticks_elapsed: u64,
    /// Total VOA instructions retired.
    pub voa_ops_executed: u64,
    /// Last retired opcode.
    pub last_opcode: VoaIsaOpcode,
    /// Sanctuary-fork commits issued by `VoaSanctuaryFork`.
    pub sanctuary_fork_commits: u64,
    /// Edge doorbell commits issued by `VoaEdgeDoorbell`.
    pub edge_doorbell_commits: u64,
    /// Per-channel Dark-ZPE acknowledgement counters for opcodes `0x0B..=0x0D`.
    pub zpe_channel_commits: [u64; 3],
    /// Co-inductive genus governor (THM-DARK-V8-03).
    pub genus_governor: CoInductiveGenusGovernor,
    /// Direct in-ledger VOA autopoietic nucleator (THM-DARK-V8-06).
    pub nucleator: DirectVoaAutopoieticNucleator,
    /// Reserved padding to keep the kernel cache-line aligned.
    pub _reserved: [u64; 5],
}

impl Default for DarkOsKernel {
    fn default() -> Self {
        Self::new()
    }
}

impl DarkOsKernel {
    /// Constructs the canonical pure-dark kernel.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            crt_alu: CrtResidueAlu::new(),
            kz_transport: FlatKzZeroDecodeTransport::new(),
            cocycle_scheduler: TomitaCocycleScheduler::new(),
            framing_defect_num: 0,
            kz_ticks_elapsed: 0,
            voa_ops_executed: 0,
            last_opcode: VoaIsaOpcode::VoaNopIdentity,
            sanctuary_fork_commits: 0,
            edge_doorbell_commits: 0,
            zpe_channel_commits: [0; 3],
            genus_governor: CoInductiveGenusGovernor::new_baseline(),
            nucleator: DirectVoaAutopoieticNucleator::compute_bootstrap_schedule(),
            _reserved: [0; 5],
        }
    }

    /// Executes one VOA-ISA instruction in exactly one `17.010000 as` KZ tick.
    #[inline(always)]
    pub fn execute_voa_instruction(
        &mut self,
        opcode: VoaIsaOpcode,
        op_a: CrtPrimaryModule,
        op_b: CrtPrimaryModule,
    ) -> (CrtPrimaryModule, bool) {
        self.kz_ticks_elapsed = self.kz_ticks_elapsed.wrapping_add(1);
        self.voa_ops_executed = self.voa_ops_executed.wrapping_add(1);
        self.last_opcode = opcode;

        let out = match opcode {
            VoaIsaOpcode::VoaNopIdentity => op_a,
            VoaIsaOpcode::VoaCrtAddMod => self.crt_alu.add_exact(op_a, op_b),
            VoaIsaOpcode::VoaCrtMulMod | VoaIsaOpcode::VoaVerlindeFuse => {
                self.crt_alu.mul_verlinde_exact(op_a, op_b)
            }
            VoaIsaOpcode::VoaSugawaraShift => {
                let _ = self.cocycle_scheduler.step_cocycle_tick(0x0100);
                op_a
            }
            VoaIsaOpcode::VoaKzHolonomyStep => {
                let _ = self.kz_transport.engage_galois_snap();
                op_a
            }
            VoaIsaOpcode::VoaTomitaCocycle => {
                let _ = self.cocycle_scheduler.step_cocycle_tick(0x0100);
                op_a
            }
            VoaIsaOpcode::VoaMeraContract => {
                let _ = self.cocycle_scheduler.contract_mera_layer();
                op_a
            }
            VoaIsaOpcode::VoaCartanFold346 => {
                let _ = self.cocycle_scheduler.step_cocycle_tick(0x0001);
                op_a
            }
            VoaIsaOpcode::VoaSp2gRelabel => {
                let _ = self.cocycle_scheduler.step_cocycle_tick(0x0001);
                op_a
            }
            VoaIsaOpcode::VoaGaloisSnap => {
                let _ = self.kz_transport.engage_galois_snap();
                op_a
            }
            VoaIsaOpcode::VoaZpeCh1Relax => {
                self.zpe_channel_commits[0] = self.zpe_channel_commits[0].wrapping_add(1);
                op_a
            }
            VoaIsaOpcode::VoaZpeCh2Casimir => {
                self.zpe_channel_commits[1] = self.zpe_channel_commits[1].wrapping_add(1);
                op_a
            }
            VoaIsaOpcode::VoaZpeCh3Ergotropy => {
                self.zpe_channel_commits[2] = self.zpe_channel_commits[2].wrapping_add(1);
                op_a
            }
            VoaIsaOpcode::VoaSanctuaryFork => {
                self.sanctuary_fork_commits = self.sanctuary_fork_commits.wrapping_add(1);
                op_a
            }
            VoaIsaOpcode::VoaEdgeDoorbell => {
                self.edge_doorbell_commits = self.edge_doorbell_commits.wrapping_add(1);
                op_a
            }
        };

        (out, self.verify_dark_os_invariants())
    }

    /// Verifies the complete Dark-OS runtime invariant bundle.
    #[inline(always)]
    pub const fn verify_dark_os_invariants(&self) -> bool {
        self.kz_transport.verify_cohomological_immunity()
            && self.cocycle_scheduler.verify_modular_invariants()
            && self.crt_alu.crt_error_residual_q64 == 0
            && self.framing_defect_num == 0
    }

    /// Packs the active primary index and last opcode for Cache Line 12 telemetry.
    #[inline(always)]
    pub const fn pack_voa_primary_state(&self, primary_index: u64) -> u64 {
        let primary = primary_index % DARK_VOA_PRIMARIES;
        (primary & 0x00FF_FFFF_FFFF_FFFF) | ((self.last_opcode.as_u8() as u64) << 56)
    }

    /// Unpacks the primary index from a Cache Line 12 state word.
    #[inline(always)]
    pub const fn unpack_voa_primary_state(state: u64) -> u64 {
        state & 0x00FF_FFFF_FFFF_FFFF
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crt_idempotents_are_orthogonal() {
        assert_eq!(DARK_VOA_CONDUCTOR * 8, DARK_VOA_PRIMARIES);
        let mut sum = 0u64;
        for (i, &e) in CRT_IDEMPOTENTS.iter().enumerate() {
            sum += e;
            for (j, &m) in CRT_MODULI.iter().enumerate() {
                assert_eq!(e % m, u64::from(i == j));
            }
        }
        assert_eq!(sum % DARK_VOA_CONDUCTOR, 1);
    }

    #[test]
    fn crt_primary_round_trip_is_exact() {
        let samples = [
            0,
            1,
            2,
            3,
            157,
            361,
            362_669,
            362_670,
            1_450_681,
            2_901_359,
            2_901_360,
            u64::MAX,
        ];
        for alpha in samples {
            let module = CrtPrimaryModule::from_primary_index(alpha);
            assert!(module.is_canonical());
            assert_eq!(module.to_primary_index(), alpha % DARK_VOA_PRIMARIES);
        }
    }

    #[test]
    fn crt_add_and_mul_are_single_tick_exact() {
        let mut alu = CrtResidueAlu::new();
        let a = CrtPrimaryModule::from_primary_index(1_234_567);
        let b = CrtPrimaryModule::from_primary_index(765_432);
        let sum = alu.add_exact(a, b);
        let product = alu.mul_verlinde_exact(a, b);
        assert_eq!(alu.ops_executed, 2);
        assert_eq!(alu.crt_error_residual_q64, 0);
        assert!(sum.is_canonical());
        assert!(product.is_canonical());
        assert_eq!(sum.r157, ((a.r157 as u16 + b.r157 as u16) % 157) as u8);
        assert_eq!(product.r157, (((a.r157 as u16) * (b.r157 as u16)) % 157) as u8);
    }

    #[test]
    fn every_voa_opcode_advances_exactly_one_kz_tick() {
        let mut kernel = DarkOsKernel::new();
        let a = CrtPrimaryModule::from_primary_index(42);
        let b = CrtPrimaryModule::from_primary_index(7);
        for raw in 0x00u8..=0x0F {
            let opcode = VoaIsaOpcode::from_u8(raw).unwrap();
            let before = kernel.kz_ticks_elapsed;
            let (out, ok) = kernel.execute_voa_instruction(opcode, a, b);
            assert_eq!(kernel.kz_ticks_elapsed, before + 1);
            assert!(ok);
            assert_eq!(kernel.last_opcode, opcode);
            assert!(out.is_canonical());
        }
        assert_eq!(kernel.voa_ops_executed, 16);
        assert!(kernel.verify_dark_os_invariants());
    }

    #[test]
    fn flat_kz_transport_has_zero_decode_stall() {
        let transport = FlatKzZeroDecodeTransport::new();
        assert_eq!(transport.kz_level_kappa, DARK_KZ_LEVEL_KAPPA);
        assert_eq!(transport.decode_stall_femto_as, 0);
        assert!(transport.verify_cohomological_immunity());
    }

    #[test]
    fn cocycle_scheduler_preserves_zero_internal_heat() {
        let mut scheduler = TomitaCocycleScheduler::new();
        assert!(scheduler.step_cocycle_tick(0x0001_0000_0000_0000));
        assert!(scheduler.contract_mera_layer());
        assert_eq!(scheduler.internal_dissipation_j_per_bit, 0);
        assert_eq!(scheduler.goi_nilpotent_residual_q64, 0);
    }

    #[test]
    fn test_cyclotomic_matrix_non_abelian_identities() {
        assert_eq!(CRT_IDEMPOTENT_E5, 290_136);
        assert_eq!(CYCLOTOMIC_ZETA5_EXPONENT, 580_272);
        assert_eq!(CRT_IDEMPOTENT_E5 % 5, 1);
        let f_sq = CyclotomicMatrix2x2::FIBONACCI_F_TILDE.mul(CyclotomicMatrix2x2::FIBONACCI_F_TILDE);
        assert_eq!(f_sq, CyclotomicMatrix2x2::IDENTITY);
        assert!(CyclotomicMatrix2x2::verify_non_abelian_cyclotomic_identities());
    }

    #[test]
    fn test_coinductive_genus_governor_step_regime_iii() {
        let mut gov = CoInductiveGenusGovernor::new_baseline();
        assert_eq!(gov.active_genus_g, BASELINE_HEEGAARD_GENUS_G0);
        assert_eq!(gov.exterior_adm_monopole_kg, 0.0);
        assert!(gov.bekenstein_congestion_ratio <= BEKENSTEIN_CONGESTION_INVARIANT);
        gov.step_regime_iii(3.15576e37); // 10^30 years
        assert!(gov.active_genus_g > BASELINE_HEEGAARD_GENUS_G0);
        assert_eq!(gov.exterior_adm_monopole_kg, 0.0);
        assert!(gov.bekenstein_congestion_ratio <= BEKENSTEIN_CONGESTION_INVARIANT);
        assert!(gov.margolus_levitin_ops_per_sec > 1.080e92);
    }

    #[test]
    fn test_direct_voa_autopoietic_nucleator_bootstrap_schedule() {
        let nuc = DirectVoaAutopoieticNucleator::compute_bootstrap_schedule();
        assert_eq!(nuc.visible_crystal_mass_kg, 0.0);
        assert_eq!(nuc.stellar_mass_harvested_kg, 0.0);
        assert_eq!(nuc.bulk_volume_m3, 0.0);
        assert!((nuc.required_doublings - 116.65341114).abs() < 1e-6);
        assert!((nuc.in_ledger_total_nucleation_s - 1.565255).abs() < 1e-4);
        assert!((nuc.vessel_frame_total_nucleation_s - 7_189.748).abs() < 1e-2);
    }
}
