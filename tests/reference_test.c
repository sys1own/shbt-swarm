/*
 * tests/reference_test.c
 * C11 Static & Runtime Verification Suite for SHBT-MMIO-SWARM v4,
 * 64-Bank UnifiedStinespringHyperFrame, SECDED Hamming(72,64), and Crowbar Interlocks.
 */

#include <stdio.h>
#include <stddef.h>
#include <string.h>
#include <assert.h>
#include "../include/shbt_swarm_mmio.h"
#include "../kernel/include/shbt_ecc.h"

static shbt_unified_stinespring_hyperframe_t g_test_hyperframe;
static shbt_swarm_mmio_t g_test_mmio;

int main(void) {
    /* 1. Verify struct sizes and field offsets */
    assert(sizeof(shbt_sten_dark_telemetry_t) == 356U);
    assert(sizeof(shbt_unified_stinespring_frame_t) == 2112U);
    assert(sizeof(shbt_unified_stinespring_hyperframe_t) == 135168U);
    assert(sizeof(shbt_swarm_mmio_t) == 128U);
    assert(sizeof(shbt_mera_gal_addr_t) == 64U);
    assert(sizeof(shbt_elysium_hab_ctrl_t) == 64U);
    assert(sizeof(shbt_aegis_ctrl_t) == 128U);
    assert(sizeof(shbt_omega_ctrl_t) == 256U);
    assert(sizeof(shbt_logos_core_ctrl_t) == 64U);
    assert(sizeof(shbt_logos_therm_shunt_t) == 64U);
    assert(sizeof(shbt_logos_omnibus_ctrl_t) == 128U);
    assert(sizeof(shbt_logos_ctrl_t) == 256U);
    assert(sizeof(shbt_causal_observer_frame_t) == 128U);
    assert(sizeof(shbt_vessel_admissibility_cert_t) == 64U);
    assert(sizeof(shbt_apotheosis_ctrl_t) == 256U);

    /* Verify Phase 5B & Phase 6 ABI & control definitions */
    assert(SHBT_SWARM_ABI_VERSION == 0x00060312U);
    assert(SHBT_APOTH_MMIO_BASE == 0x4000C000UL);
    assert(SHBT_OMNIBUS_HYPERFRAME_DMA_BASE == 0x70020000UL);
    assert(SHBT_APOTH_EGRESS_SAFEWORD_IRQ == 0x80000000UL);
    assert(SHBT_APOTH_CMD_ASCEND == 0x00000001UL);
    assert(SHBT_APOTH_CMD_DESCEND_SOMATIC == 0x00000002UL);
    assert(SHBT_APOTH_CMD_DEITY_LAW_REWRITE == 0x00000004UL);
    assert(SHBT_APOTH_CMD_VERLINDE_MULTIPLEX == 0x00000008UL);
    assert(SHBT_APOTH_CMD_SANDBOX_FORK_X1 == 0x00000010UL);
    assert(SHBT_APOTH_CMD_ZERO_COMP_RENDER == 0x00000020UL);
    assert(SHBT_APOTH_CMD_SYNTH_AVATAR == 0x00000040UL);
    assert(SHBT_APOTH_CMD_BRAID_CRYSTALLIZE == 0x00000080UL);

    assert(SHBT_LOGOS_MODULAR_Q_ZERO_LOCK == (1ULL << 0));
    assert(SHBT_LOGOS_COHERENT_UNCOMPUTE_EN == (1ULL << 1));
    assert(SHBT_LOGOS_SIM_X1_PROTECT_LATCH == 0x00000002UL);
    assert(SHBT_LOGOS_SIM_KILL_DISABLE_LOCK == (1ULL << 9));

    assert(offsetof(shbt_sten_dark_telemetry_t, v_terra_lyapunov_q32) == 0x00U);
    assert(offsetof(shbt_sten_dark_telemetry_t, delta_h_vis_q32) == 0x28U);
    assert(offsetof(shbt_sten_dark_telemetry_t, space_group_id) == 0x44U);
    assert(offsetof(shbt_sten_dark_telemetry_t, autopoietic_heal_cycle_cnt) == 0x48U);
    assert(offsetof(shbt_sten_dark_telemetry_t, dark_purification_state) == 0x60U);

    assert(offsetof(shbt_swarm_mmio_t, magic_id) == 0x00U);
    assert(offsetof(shbt_swarm_mmio_t, abi_version) == 0x04U);
    assert(offsetof(shbt_swarm_mmio_t, cmd_reg) == 0x08U);
    assert(offsetof(shbt_swarm_mmio_t, status_reg) == 0x0CU);
    assert(offsetof(shbt_swarm_mmio_t, fault_mask_reg) == 0x10U);
    assert(offsetof(shbt_swarm_mmio_t, feedstock_mass_ug) == 0x40U);
    assert(offsetof(shbt_swarm_mmio_t, target_payload_mass_ug) == 0x48U);
    assert(offsetof(shbt_swarm_mmio_t, net_b_minus_l_balance) == 0x50U);
    assert(offsetof(shbt_swarm_mmio_t, temp_qc_mk) == 0x60U);
    assert(offsetof(shbt_swarm_mmio_t, temp_nacelle_mk) == 0x64U);
    assert(offsetof(shbt_swarm_mmio_t, cayley_hash_mid1) == 0x78U);

    /* 2. Verify SECDED Hamming(72,64) single-error correction and double-error detection */
    uint64_t test_word = 0xA55A3C69F00FF00FULL;
    uint8_t parity = shbt_ecc_encode_hamming72_64(test_word);

    uint64_t clean_w = test_word;
    uint8_t clean_p = parity;
    assert(shbt_ecc_decode_hamming72_64(&clean_w, &clean_p) == SHBT_ECC_OK);
    assert(clean_w == test_word);

    /* Inject single-bit flip across all 64 bit positions */
    for (uint8_t bit = 0U; bit < 64U; bit++) {
        uint64_t corrupted_w = test_word ^ (1ULL << bit);
        uint8_t corrupted_p = parity;
        shbt_ecc_status_t st = shbt_ecc_decode_hamming72_64(&corrupted_w, &corrupted_p);
        assert(st == SHBT_ECC_SINGLE_CORRECTED);
        assert(corrupted_w == test_word);
    }

    /* Inject double-bit flip */
    uint64_t double_flip_w = test_word ^ (1ULL << 7U) ^ (1ULL << 42U);
    uint8_t double_flip_p = parity;
    assert(shbt_ecc_decode_hamming72_64(&double_flip_w, &double_flip_p) == SHBT_ECC_DOUBLE_DETECTED);

    /* 3. Verify 50.518 kHz Microkernel Tick & Weyl-Exciton Crowbar Interlock */
    memset(&g_test_mmio, 0, sizeof(g_test_mmio));
    memset(&g_test_hyperframe, 0, sizeof(g_test_hyperframe));

    g_test_mmio.magic_id = 0x5357524DU;
    g_test_mmio.abi_version = SHBT_SWARM_ABI_VERSION;
    g_test_mmio.feedstock_mass_ug = 5892300000ULL;
    g_test_mmio.target_payload_mass_ug = 5892300000ULL;
    g_test_mmio.net_b_minus_l_balance = 0LL;
    g_test_mmio.temp_qc_mk = 4200U;
    g_test_mmio.temp_nacelle_mk = 21130U;
    g_test_mmio.synth_bank_mask = 0x00000001U;
    g_test_mmio.cmd_reg = SHBT_SWARM_CMD_EXEC_CLONE_SHIP | SHBT_SWARM_CMD_AUTOPOIETIC_SELF_HEAL;

    /* Populate bank 0 of daughter_synth_ram with valid SECDED words and one single-bit error */
    for (uint32_t k = 0U; k < SHBT_BRAIDS_PER_FRAME; k++) {
        uint64_t w = 0x123456789ABCDEF0ULL ^ (uint64_t)k;
        uint8_t p = shbt_ecc_encode_hamming72_64(w);
        if (k == 17U) {
            w ^= (1ULL << 19U); /* single-bit flip */
        }
        g_test_hyperframe.daughter_synth_ram[0].ctrl_view.braid_descriptors[k] = w;
        g_test_hyperframe.daughter_synth_ram[0].ctrl_view.secded_parity[k] = p;
    }

    shbt_swarm_kernel_tick(&g_test_mmio, &g_test_hyperframe);
    assert(g_test_mmio.fault_mask_reg == SHBT_SWARM_FAULT_NONE);
    assert(g_test_mmio.secded_syndrome_cnt == 1ULL);
    assert(g_test_hyperframe.daughter_synth_ram[0].ctrl_view.dark_sten.autopoietic_heal_cycle_cnt == 1ULL);

    /* Trigger ADM mass imbalance fault and verify immediate crowbar abort */
    g_test_mmio.feedstock_mass_ug = 0ULL;
    shbt_swarm_kernel_tick(&g_test_mmio, &g_test_hyperframe);
    assert((g_test_mmio.fault_mask_reg & SHBT_SWARM_FAULT_ADM_IMBALANCE) != 0U);
    assert(g_test_mmio.cmd_reg == SHBT_SWARM_CMD_WEYL_EXCITON_CROWBAR);

    /* 4. Verify 8x8 InP/InGaAs HBT & PIC Phase Words and Bias Voltages across boundary angles */
    const double pi_val = 3.14159265358979323846;
    uint16_t w_0 = shbt_swarm_emitter_phase_word(0.0);
    assert(w_0 == 0U);
    assert(shbt_swarm_hbt_bias_mv(w_0) == 3800U);

    uint16_t w_pi_2 = shbt_swarm_emitter_phase_word(pi_val / 2.0);
    assert(w_pi_2 == 16384U);
    assert(shbt_swarm_hbt_bias_mv(w_pi_2) == 4700U);

    uint16_t w_pi = shbt_swarm_emitter_phase_word(pi_val);
    assert(w_pi == 32768U);
    assert(shbt_swarm_hbt_bias_mv(w_pi) == 5600U);

    uint16_t w_3pi_2 = shbt_swarm_emitter_phase_word(3.0 * pi_val / 2.0);
    assert(w_3pi_2 == 49152U);
    assert(shbt_swarm_hbt_bias_mv(w_3pi_2) == 6500U);

    uint16_t w_2pi = shbt_swarm_emitter_phase_word(2.0 * pi_val);
    assert(w_2pi == 0U);
    assert(shbt_swarm_hbt_bias_mv(w_2pi) == 3800U);

    printf("[PASS] C11 static asserts, struct offsets, Hamming(72,64) SECDED, 8x8 HBT/PIC bias, and 50.518 kHz kernel tick verified.\n");
    return 0;
}
