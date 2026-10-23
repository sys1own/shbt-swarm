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
