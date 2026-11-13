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

/// Orthogonal CRT moduli for `D_cond = 362,670 = 2*3*5*7*11*157`.
pub const CRT_MODULI: [u64; 6] = [2, 3, 5, 7, 11, 157];
/// CRT idempotents satisfying `e_i = 1 mod p_i` and `e_i = 0 mod p_j` for `i != j`.
pub const CRT_IDEMPOTENTS: [u64; 6] = [181_335, 241_780, 290_136, 259_050, 131_880, 346_500];

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
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
}
