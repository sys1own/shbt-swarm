/*
 * kernel/linker.ld
 * Bare-Metal Linker Script for SHBT-OS Swarm Microkernel & MMIO Arena
 */

OUTPUT_ARCH("riscv")
ENTRY(shbt_swarm_kernel_tick)

MEMORY
{
    KERNEL_SRAM (rwx) : ORIGIN = 0x80000000, LENGTH = 256K
    MMIO_REGS   (rw)  : ORIGIN = 0x70000000, LENGTH = 128
    HYPERFRAME  (rw)  : ORIGIN = 0x70020000, LENGTH = 135168
}

SECTIONS
{
    .text : ALIGN(64) {
        *(.text*)
    } > KERNEL_SRAM

    .rodata : ALIGN(64) {
        *(.rodata*)
    } > KERNEL_SRAM

    .data : ALIGN(64) {
        *(.data*)
    } > KERNEL_SRAM

    .bss : ALIGN(64) {
        *(.bss*)
        *(COMMON)
    } > KERNEL_SRAM
}
