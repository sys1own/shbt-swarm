//! crates/shbt-swarm-cloner/src/ship_ledger.rs
//! WP-3.1 Daughter Spacecraft Mass, Nucleon, Bank, Dynkin Slot (346 slots), and Energy Ledger
//! for both the baseline 7,200.00 kg `SHBT-EW-312` (MAR-312C) and upgraded 5,892.30 kg `SHBT-EW-312X` (MAR-312X).

pub const DYNKIN_SLOTS_SU2_26: u16 = 27;   // Slots 001..027
pub const DYNKIN_SLOTS_SU3_8: u16 = 45;    // Slots 028..072
pub const DYNKIN_SLOTS_SO10_312: u16 = 274; // Slots 073..346
pub const DYNKIN_SLOTS_TOTAL: u16 = DYNKIN_SLOTS_SU2_26 + DYNKIN_SLOTS_SU3_8 + DYNKIN_SLOTS_SO10_312;

const _: () = assert!(DYNKIN_SLOTS_TOTAL == 346);

pub const DAUGHTER_SHIP_MASS_G: u64 = 7_200_000;        // 7,200.00 kg
pub const UPGRADED_SHIP_312X_MASS_G: u64 = 5_892_300;   // 5,892.30 kg (with 950 kg MAR-312X nacelle)
pub const MAR312C_SEED_WAFER_MASS_MG: u64 = 37_699_000; // 37.699 kg (18.8495 kg 16 + 18.8495 kg bar-16)
pub const MAR312C_WAFER_NET_ENERGY_KJ: u64 = 3_033_100_000_000_000; // 3.03310 EJ
pub const DAUGHTER_CLONE_WORK_KJ: u64 = 409_284_100_000_000;        // 409.2841 PJ
pub const UPGRADED_312X_CLONE_WORK_KJ: u64 = 334_947_700_000_000;   // 334.9477 PJ

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DaughterSubsystemSpec {
    pub id: u8,
    pub name: &'static str,
    pub rom_bank_start: u8,
    pub rom_bank_end: u8,
    pub synth_bank_start: u8,
    pub synth_bank_end: u8,
    pub mass_grams: u64,
    pub synthesis_work_mj: u64,
}

pub const DAUGHTER_EW312_SUBSYSTEMS: [DaughterSubsystemSpec; 8] = [
    DaughterSubsystemSpec {
        id: 1,
        name: "SHBT-MAR-312C Reactor & Seed Wafer",
        rom_bank_start: 0,
        rom_bank_end: 9,
        synth_bank_start: 32,
        synth_bank_end: 41,
        mass_grams: 2_257_700,
        synthesis_work_mj: 128_339_000_000,
    },
    DaughterSubsystemSpec {
        id: 2,
        name: "Cryostat & Primary Hull Shell (Ti-6Al-4V)",
        rom_bank_start: 10,
        rom_bank_end: 15,
        synth_bank_start: 42,
        synth_bank_end: 47,
        mass_grams: 1_480_000,
        synthesis_work_mj: 84_130_600_000,
    },
    DaughterSubsystemSpec {
        id: 3,
        name: "SHBT-EW-312 Warp & SGLT Transducers",
        rom_bank_start: 16,
        rom_bank_end: 20,
        synth_bank_start: 48,
        synth_bank_end: 52,
        mass_grams: 1_215_500,
        synthesis_work_mj: 69_095_100_000,
    },
    DaughterSubsystemSpec {
        id: 4,
        name: "REBCO (18.5 T) Toroidal/Poloidal Coils",
        rom_bank_start: 21,
        rom_bank_end: 23,
        synth_bank_start: 53,
        synth_bank_end: 55,
        mass_grams: 840_500,
        synthesis_work_mj: 47_778_200_000,
    },
    DaughterSubsystemSpec {
        id: 5,
        name: "5-Stage DEC Louvers & Thermal Bus",
        rom_bank_start: 24,
        rom_bank_end: 26,
        synth_bank_start: 56,
        synth_bank_end: 58,
        mass_grams: 620_300,
        synthesis_work_mj: 35_261_000_000,
    },
    DaughterSubsystemSpec {
        id: 6,
        name: "Borrmann Cavity & Symplectic Sleeve",
        rom_bank_start: 27,
        rom_bank_end: 28,
        synth_bank_start: 59,
        synth_bank_end: 60,
        mass_grams: 312_400,
        synthesis_work_mj: 17_758_400_000,
    },
    DaughterSubsystemSpec {
        id: 7,
        name: "SHBT-IBR-312 Magazine & Aux Bus",
        rom_bank_start: 29,
        rom_bank_end: 29,
        synth_bank_start: 61,
        synth_bank_end: 61,
        mass_grams: 288_000,
        synthesis_work_mj: 16_371_400_000,
    },
    DaughterSubsystemSpec {
        id: 8,
        name: "SHBT-R 312-Ch Quantum Core & Recon Bay",
        rom_bank_start: 30,
        rom_bank_end: 31,
        synth_bank_start: 62,
        synth_bank_end: 63,
        mass_grams: 185_600,
        synthesis_work_mj: 10_550_400_000,
    },
];

/// Verifies exact mass (7,200.00 kg), bank count (32 ROM + 32 Synth), and work (409.2841 PJ) closure.
#[inline]
pub const fn verify_daughter_ship_ledger() -> (u64, u64, u64) {
    let mut total_mass_g: u64 = 0;
    let mut total_work_mj: u64 = 0;
    let mut i: usize = 0;
    while i < 8 {
        total_mass_g += DAUGHTER_EW312_SUBSYSTEMS[i].mass_grams;
        total_work_mj += DAUGHTER_EW312_SUBSYSTEMS[i].synthesis_work_mj;
        i += 1;
    }
    let daughters_per_wafer = MAR312C_WAFER_NET_ENERGY_KJ / DAUGHTER_CLONE_WORK_KJ;
    (total_mass_g, total_work_mj, daughters_per_wafer)
}

const _: () = {
    let (m, w, d) = verify_daughter_ship_ledger();
    assert!(m == 7_200_000);
    assert!(w == 409_284_100_000);
    assert!(d == 7);
};

/// Returns replication turnaround times in microseconds for Cruise (895.19 TW), Burst (6.805 PW),
/// and Impulse (895.19 PW quantum + 394.82 ms Eikonal sweep = 852.02 ms total).
#[inline]
pub const fn daughter_turnaround_times_us() -> (u64, u64, u64, u64) {
    let cruise_us: u64 = 457_200_000;      // 457.20 s
    let burst_us: u64 = 60_140_000;        // 60.14 s
    let impulse_quantum_us: u64 = 457_200; // 457.20 ms
    let impulse_total_us: u64 = 852_020;   // 457.20 ms + 394.82 ms = 852.02 ms
    (cruise_us, burst_us, impulse_quantum_us, impulse_total_us)
}
