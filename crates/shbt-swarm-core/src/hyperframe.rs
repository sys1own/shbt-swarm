//! crates/shbt-swarm-core/src/hyperframe.rs
//! Exact #[repr(C)] hardware layout equivalents of SHBT-MMIO-SWARM v4 (include/shbt_swarm_mmio.h).

pub const SHBT_SWARM_MMIO_BASE_ADDR: usize = 0x7000_0000;
pub const SHBT_SWARM_HYPERFRAME_BASE_ADDR: usize = 0x7002_0000;

pub const SHBT_FRAME_BYTES: usize = 2112;
pub const SHBT_VISIBLE_ANCHOR_BYTES: usize = 640; // eta_A = 10/33 * 2112 B
pub const SHBT_DARK_LEDGER_BYTES: usize = 1472;   // eta_D = 23/33 * 2112 B
pub const SHBT_HYPERFRAME_BANKS: usize = 64;
pub const SHBT_HYPERFRAME_TOTAL_BYTES: usize = 135_168;
pub const SHBT_BRAIDS_PER_FRAME: usize = 124;
pub const SHBT_DYNKIN_WEIGHT_SLOTS: usize = 346;
pub const SHBT_MERA_MAX_LAYERS: u8 = 101;
pub const SHBT_MAX_SPACE_GROUP_ID: u16 = 230;

// Phase 1..4 Command Opcodes
pub const SHBT_SWARM_CMD_NOP: u32                     = 0x0000_0000;
pub const SHBT_SWARM_CMD_TOMITA_QUINE_CPY: u32        = 0x0000_0001;
pub const SHBT_SWARM_CMD_PT_GOI_PROJECT: u32          = 0x0000_0002;
pub const SHBT_SWARM_CMD_TOMO_CAPTURE: u32            = 0x0000_0004;
pub const SHBT_SWARM_CMD_EXEC_CLONE_SHIP: u32         = 0x0000_0008;
pub const SHBT_SWARM_CMD_EXEC_CLONE_BIO: u32          = 0x0000_0010;
pub const SHBT_SWARM_CMD_LANDAUER_SHUNT: u32          = 0x0000_0020;
pub const SHBT_SWARM_CMD_MERA_PINN_SYNC: u32          = 0x0000_0040;
pub const SHBT_SWARM_CMD_PT_GOI_EQUIL: u32            = 0x0000_0080;
pub const SHBT_SWARM_CMD_TOMITA_LEARN: u32            = 0x0000_0100;
pub const SHBT_SWARM_CMD_VERLINDE_PLAN: u32           = 0x0000_0200;
pub const SHBT_SWARM_CMD_SP2G_HIVE_SYNC: u32          = 0x0000_0400;
pub const SHBT_SWARM_CMD_AP_MPC_ENCLAVE: u32          = 0x0000_0800;
pub const SHBT_SWARM_CMD_XENO_ASSIMILATE: u32         = 0x0000_1000;
pub const SHBT_SWARM_CMD_EXEC_UNIV_SYNTH: u32         = 0x0000_2000;
pub const SHBT_SWARM_CMD_ZERO_COMP_DIRECT_RENDER: u32 = 0x0000_4000;
pub const SHBT_SWARM_CMD_MERA_DAG_INSTANCE: u32       = 0x0000_8000;
pub const SHBT_SWARM_CMD_MAR312X_FORGE_TIER4: u32     = 0x0001_0000;
pub const SHBT_SWARM_CMD_DIRECT_BOUNDARY_SYNTH: u32   = 0x0002_0000;
pub const SHBT_SWARM_CMD_AUTOPOIETIC_SELF_HEAL: u32   = 0x0004_0000;
pub const SHBT_SWARM_CMD_WEYL_EXCITON_CROWBAR: u32    = 0x8000_0000;
pub const SHBT_SWARM_CMD_PCSS_CROWBAR: u32            = 0x8000_0000;

// Phase 1..4 Fault Bitmasks
pub const SHBT_SWARM_FAULT_NONE: u32                        = 0x0000_0000;
pub const SHBT_SWARM_FAULT_ADM_IMBALANCE: u32               = 0x0000_0001;
pub const SHBT_SWARM_FAULT_PARTITION_LOCK: u32              = 0x0000_0002;
pub const SHBT_SWARM_FAULT_FRAMING_WEYL: u32                = 0x0000_0004;
pub const SHBT_SWARM_FAULT_BL_ANOMALY: u32                  = 0x0000_0008;
pub const SHBT_SWARM_FAULT_NEUTRINO_LEAK: u32               = 0x0000_0010;
pub const SHBT_SWARM_FAULT_SECDED_UNCORR: u32               = 0x0000_0020;
pub const SHBT_SWARM_FAULT_CAYLEY_HASH: u32                 = 0x0000_0040;
pub const SHBT_SWARM_FAULT_THERMAL_CLAMP: u32               = 0x0000_0080;
pub const SHBT_SWARM_FAULT_CBF_PHYS_VIOL: u32               = 0x0000_0100;
pub const SHBT_SWARM_FAULT_GREY_GOO_CAP: u32                = 0x0000_0200;
pub const SHBT_SWARM_FAULT_CLF_DIVERGENCE: u32              = 0x0000_0400;
pub const SHBT_SWARM_FAULT_X0_PROTECT_LCK: u32              = 0x0000_0800;
pub const SHBT_SWARM_FAULT_X1_SAPIENT_LCK: u32              = 0x0000_1000;
pub const SHBT_SWARM_FAULT_SUGAWARA_CAP_EXCEEDED: u32       = 0x0000_2000;
pub const SHBT_SWARM_FAULT_DRIPLINE_UNSTABLE: u32           = 0x0000_4000;
pub const SHBT_SWARM_FAULT_SIM_TO_RENDER_HASH_MISMATCH: u32 = 0x0000_8000;
pub const SHBT_SWARM_FAULT_HABITAT_GRAV_DEVIATION: u32      = 0x0001_0000;
pub const SHBT_SWARM_FAULT_SUPERHYDRIDE_TETHER_LOSS: u32    = 0x0002_0000;
pub const SHBT_SWARM_FAULT_PURCELL_BANDGAP_DRIFT: u32       = 0x0004_0000;
pub const SHBT_SWARM_FAULT_DECX_CHERN_BACKSCATTER: u32      = 0x0008_0000;
pub const SHBT_SWARM_FAULT_FORGE_SHEAR_EXCEEDED: u32        = 0x0010_0000;

// Xenobiological Triage Bitmasks
pub const SHBT_SWARM_TRIAGE_SHIFT: u32         = 28;
pub const SHBT_SWARM_TRIAGE_MASK: u32          = 0xF000_0000;
pub const SHBT_SWARM_TRIAGE_M0_REGOLITH: u32   = 0x0000_0000;
pub const SHBT_SWARM_TRIAGE_X0_PROTECT: u32    = 0x1000_0000;
pub const SHBT_SWARM_TRIAGE_X1_QUARANTINE: u32 = 0x2000_0000;
pub const SHBT_SWARM_TRIAGE_X2_ASSIMILATE: u32 = 0x4000_0000;

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct ShbtStenDarkTelemetry {
    pub v_terra_lyapunov_q32: u64,
    pub dv_terra_dt_q32: i64,
    pub c_mera_complexity_q16: u64,
    pub d_cayley_distance: u32,
    pub active_swarm_nodes_m: u32,
    pub max_swarm_nodes_m_max: u32,
    pub sp2g_symplectic_crc32: u32,
    pub delta_h_vis_q32: i64,
    pub delta_h_braid_per_ch_q32: i64,
    pub mera_dag_tile_instance_cnt: u64,
    pub dark_folding_eta_b_ppm: u32,
    pub space_group_id: u16,
    pub ghost_hab_grav_mms2: u16,
    pub autopoietic_heal_cycle_cnt: u64,
    pub superhydride_field_mt: u32,
    pub superhydride_tcrit_margin_mk: u32,
    pub decx_conv_efficiency_ppm: u32,
    pub seed_die_shear_stress_mpa: u16,
    pub loaded_fuel_cores_cnt: u16,
    pub dark_purification_state: [u8; 256],
    pub dark_holonomy_crc32: u32,
}

const _: () = assert!(core::mem::size_of::<ShbtStenDarkTelemetry>() == 356);

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct ShbtFrameCtrlView {
    pub visible_anchor: [u8; SHBT_VISIBLE_ANCHOR_BYTES],
    pub braid_descriptors: [u64; SHBT_BRAIDS_PER_FRAME],
    pub secded_parity: [u8; SHBT_BRAIDS_PER_FRAME],
    pub dark_sten: ShbtStenDarkTelemetry,
}

const _: () = assert!(core::mem::size_of::<ShbtFrameCtrlView>() == 2112);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ShbtFrameMpfrView {
    pub mpfr_limbs: [[u64; 8]; 33],
}

const _: () = assert!(core::mem::size_of::<ShbtFrameMpfrView>() == 2112);

#[repr(C, align(64))]
#[derive(Copy, Clone)]
pub union UnifiedStinespringFrame {
    pub ctrl_view: ShbtFrameCtrlView,
    pub mpfr_view: ShbtFrameMpfrView,
    pub raw_bytes: [u8; SHBT_FRAME_BYTES],
}

const _: () = assert!(core::mem::size_of::<UnifiedStinespringFrame>() == 2112);

impl Default for UnifiedStinespringFrame {
    fn default() -> Self {
        Self {
            raw_bytes: [0u8; SHBT_FRAME_BYTES],
        }
    }
}

#[repr(C, align(4096))]
#[derive(Copy, Clone, Default)]
pub struct UnifiedStinespringHyperFrame {
    pub parent_quine_rom: [UnifiedStinespringFrame; 32],
    pub daughter_synth_ram: [UnifiedStinespringFrame; 32],
}

const _: () = assert!(core::mem::size_of::<UnifiedStinespringHyperFrame>() == 135_168);

#[repr(C, align(64))]
#[derive(Copy, Clone, Debug, Default)]
pub struct ShbtSwarmMmioRegs {
    pub magic_id: u32,
    pub abi_version: u32,
    pub cmd_reg: u32,
    pub status_reg: u32,
    pub fault_mask_reg: u32,
    pub rom_bank_mask: u32,
    pub synth_bank_mask: u32,
    pub power_tier_sel: u32,
    pub tomita_flow_phase_q64: u64,
    pub pt_goi_prop_ps_q32: u64,
    pub secded_syndrome_cnt: u64,
    pub generation_counter: u64,

    pub feedstock_mass_ug: u64,
    pub target_payload_mass_ug: u64,
    pub net_b_minus_l_balance: i64,
    pub gst225_impedance_krayl: u32,
    pub enclave_lapse_milli: u32,
    pub temp_qc_mk: u32,
    pub temp_nacelle_mk: u32,
    pub cayley_hash_lo: u64,
    pub cayley_hash_mid0: u64,
    pub cayley_hash_mid1: u64,
}

const _: () = assert!(core::mem::size_of::<ShbtSwarmMmioRegs>() == 128);
