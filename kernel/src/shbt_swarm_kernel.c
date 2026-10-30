/*
 * kernel/src/shbt_swarm_kernel.c
 * Freestanding C11 50.518 kHz (tau_0 = 19.795 us) Microkernel Tick Handler
 * Enforces ADM Mass Balance, B-L Conservation, Thermal Clamping, SECDED Scrubbing,
 * and the 1.701 ps Weyl-Exciton / PCSS Hardware Crowbar Interlock.
 */

#include "../include/shbt_ecc.h"

void shbt_swarm_kernel_tick(shbt_swarm_mmio_t *mmio, shbt_unified_stinespring_hyperframe_t *hyperframe) {
    if (mmio == NULL) {
        return;
    }

    uint32_t faults = mmio->fault_mask_reg;

    /* 1. Verify ADM Mass Balance during active clone or synthesis commands */
    uint32_t cmd = mmio->cmd_reg;
    uint32_t synth_cmds = SHBT_SWARM_CMD_EXEC_CLONE_SHIP
                        | SHBT_SWARM_CMD_EXEC_CLONE_BIO
                        | SHBT_SWARM_CMD_EXEC_UNIV_SYNTH
                        | SHBT_SWARM_CMD_DIRECT_BOUNDARY_SYNTH;
    if ((cmd & synth_cmds) != 0U) {
        if (mmio->feedstock_mass_ug != mmio->target_payload_mass_ug || mmio->feedstock_mass_ug == 0ULL) {
            faults |= SHBT_SWARM_FAULT_ADM_IMBALANCE;
        }
        if (mmio->net_b_minus_l_balance != 0LL) {
            faults |= SHBT_SWARM_FAULT_BL_ANOMALY;
        }
    }

    /* 2. Verify Cryogenic & Superhydride Thermal Clamping (T_QC <= 4.2 K, T_MAR <= 192.50 K) */
    if (mmio->temp_qc_mk > 4200U || mmio->temp_nacelle_mk > SHBT_MAR312X_TCRIT_SUPERHYDRIDE_MK) {
        faults |= SHBT_SWARM_FAULT_THERMAL_CLAMP;
    }

    /* 3. Scrub active synthesis banks with SECDED Hamming(72,64) if hyperframe is mapped */
    if (hyperframe != NULL && mmio->synth_bank_mask != 0U) {
        for (uint32_t b = 0U; b < 32U; b++) {
            if (((mmio->synth_bank_mask >> b) & 1U) != 0U) {
                shbt_unified_stinespring_frame_t *frame = &hyperframe->daughter_synth_ram[b];
                for (uint32_t k = 0U; k < SHBT_BRAIDS_PER_FRAME; k++) {
                    uint64_t word = frame->ctrl_view.braid_descriptors[k];
                    uint8_t par = frame->ctrl_view.secded_parity[k];
                    shbt_ecc_status_t st = shbt_ecc_decode_hamming72_64(&word, &par);
                    if (st == SHBT_ECC_SINGLE_CORRECTED) {
                        frame->ctrl_view.braid_descriptors[k] = word;
                        frame->ctrl_view.secded_parity[k] = par;
                        mmio->secded_syndrome_cnt++;
                    } else if (st == SHBT_ECC_DOUBLE_DETECTED) {
                        faults |= SHBT_SWARM_FAULT_SECDED_UNCORR;
                    }
                }
                if ((cmd & SHBT_SWARM_CMD_AUTOPOIETIC_SELF_HEAL) != 0U) {
                    frame->ctrl_view.dark_sten.autopoietic_heal_cycle_cnt++;
                }
            }
        }
    }

    mmio->fault_mask_reg = faults;

    /* 4. Hardware Fault Interlock: Trigger 1.701 ps Weyl-Exciton / PCSS Crowbar if any fault is active */
    if (mmio->fault_mask_reg != 0U) {
        mmio->cmd_reg = SHBT_SWARM_CMD_WEYL_EXCITON_CROWBAR;
    }
}

#include "../include/shbt_mmio_apotheosis.h"

/* Compile-Time Static Asserts for Extended MMIO Registers, Commands & Faults */
_Static_assert(SHBT_OMEGA_REG_OPACITY_QUENCH == 0x00008030U, "SHBT_OMEGA_REG_OPACITY_QUENCH");
_Static_assert(SHBT_OMEGA_REG_ADIABATIC_GAMMA == 0x00008034U, "SHBT_OMEGA_REG_ADIABATIC_GAMMA");
_Static_assert(SHBT_OMEGA_REG_DYNAMO_BRAKE_TENSOR == 0x00008038U, "SHBT_OMEGA_REG_DYNAMO_BRAKE_TENSOR");
_Static_assert(SHBT_OMEGA_REG_DECONSTRUCT_M_DOT == 0x0000803CU, "SHBT_OMEGA_REG_DECONSTRUCT_M_DOT");

_Static_assert(SHBT_OMEGA_CMD_BYPASS_EDDINGTON == 0x00000002U, "SHBT_OMEGA_CMD_BYPASS_EDDINGTON");
_Static_assert(SHBT_OMEGA_CMD_ENGAGE_DYNAMO_BRAKE == 0x00000004U, "SHBT_OMEGA_CMD_ENGAGE_DYNAMO_BRAKE");
_Static_assert(SHBT_OMEGA_FAULT_RADIATION_PRESSURE == 0x00000020U, "SHBT_OMEGA_FAULT_RADIATION_PRESSURE");
_Static_assert(SHBT_OMEGA_FAULT_CORE_COLLAPSE_RISK == 0x00000040U, "SHBT_OMEGA_FAULT_CORE_COLLAPSE_RISK");

_Static_assert(SHBT_LOGOS_REG_BORRMANN_SUSCEPT == 0x0000A030U, "SHBT_LOGOS_REG_BORRMANN_SUSCEPT");
_Static_assert(SHBT_LOGOS_REG_FRACTON_DENSITY == 0x0000A034U, "SHBT_LOGOS_REG_FRACTON_DENSITY");
_Static_assert(SHBT_LOGOS_REG_ISRAEL_LAPSE_GRAD == 0x0000A038U, "SHBT_LOGOS_REG_ISRAEL_LAPSE_GRAD");
_Static_assert(SHBT_LOGOS_REG_TIDAL_SHEAR_MAX == 0x0000A03CU, "SHBT_LOGOS_REG_TIDAL_SHEAR_MAX");

_Static_assert(SHBT_LOGOS_CMD_BORRMANN_TUNE == 0x00000001U, "SHBT_LOGOS_CMD_BORRMANN_TUNE");
_Static_assert(SHBT_LOGOS_CMD_FRACTON_SHUNT_ENGAGE == 0x00000002U, "SHBT_LOGOS_CMD_FRACTON_SHUNT_ENGAGE");
_Static_assert(SHBT_LOGOS_FAULT_HOT_PHONON_TRIP == 0x00000040U, "SHBT_LOGOS_FAULT_HOT_PHONON_TRIP");
_Static_assert(SHBT_LOGOS_FAULT_TIDAL_FRACTURE == 0x00000080U, "SHBT_LOGOS_FAULT_TIDAL_FRACTURE");

_Static_assert(SHBT_APOTH_REG_QND_EGO_COMMUTATOR == 0x0000C030U, "SHBT_APOTH_REG_QND_EGO_COMMUTATOR");
_Static_assert(SHBT_APOTH_REG_LIND_PURIFY_RATE == 0x0000C034U, "SHBT_APOTH_REG_LIND_PURIFY_RATE");
_Static_assert(SHBT_APOTH_REG_HORIZON_DEFLECT_STAT == 0x0000C038U, "SHBT_APOTH_REG_HORIZON_DEFLECT_STAT");
_Static_assert(SHBT_APOTH_REG_MULTI_DEITY_COMMUT == 0x0000C03CU, "SHBT_APOTH_REG_MULTI_DEITY_COMMUT");

_Static_assert(SHBT_APOTH_CMD_QND_HANDSHAKE == 0x00000001U, "SHBT_APOTH_CMD_QND_HANDSHAKE");
_Static_assert(SHBT_APOTH_CMD_HORIZON_DEFLECT_ARM == 0x00000002U, "SHBT_APOTH_CMD_HORIZON_DEFLECT_ARM");
_Static_assert(SHBT_APOTH_CMD_MERA_MEM_COMPRESS == 0x00000004U, "SHBT_APOTH_CMD_MERA_MEM_COMPRESS");
_Static_assert(SHBT_APOTH_FAULT_QND_COLLAPSE == 0x00000040U, "SHBT_APOTH_FAULT_QND_COLLAPSE");
_Static_assert(SHBT_APOTH_FAULT_P_ZOMBIE_COLLAPSE == 0x00000080U, "SHBT_APOTH_FAULT_P_ZOMBIE_COLLAPSE");
