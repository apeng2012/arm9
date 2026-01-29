//! Miscellaneous assembly instructions for ARM9

use core::arch::asm;

/// No operation
#[inline(always)]
pub fn nop() {
    unsafe {
        asm!("nop", options(nomem, nostack, preserves_flags));
    }
}

/// Wait for interrupt (ARMv5+)
#[inline(always)]
pub fn wfi() {
    unsafe {
        asm!(
            "mcr p15, 0, {0}, c7, c0, 4",
            in(reg) 0u32,
            options(nomem, nostack, preserves_flags)
        );
    }
}

/// Data Synchronization Barrier
#[inline(always)]
pub fn dsb() {
    unsafe {
        asm!(
            "mcr p15, 0, {0}, c7, c10, 4",
            in(reg) 0u32,
            options(nomem, nostack, preserves_flags)
        );
    }
}

/// Data Memory Barrier
#[inline(always)]
pub fn dmb() {
    unsafe {
        asm!(
            "mcr p15, 0, {0}, c7, c10, 5",
            in(reg) 0u32,
            options(nomem, nostack, preserves_flags)
        );
    }
}

/// Instruction Synchronization Barrier
#[inline(always)]
pub fn isb() {
    unsafe {
        asm!(
            "mcr p15, 0, {0}, c7, c5, 4",
            in(reg) 0u32,
            options(nomem, nostack, preserves_flags)
        );
    }
}

/// Software breakpoint
#[inline(always)]
pub fn bkpt() {
    unsafe {
        asm!("bkpt #0", options(nomem, nostack));
    }
}

/// Delay loop
#[inline]
pub fn delay(cycles: u32) {
    let iterations = cycles / 4;
    for _ in 0..iterations {
        nop();
    }
}

/// Invalidate instruction cache
#[inline(always)]
pub fn invalidate_icache() {
    unsafe {
        asm!(
            "mcr p15, 0, {0}, c7, c5, 0",
            in(reg) 0u32,
            options(nomem, nostack)
        );
    }
}

/// Invalidate data cache
#[inline(always)]
pub fn invalidate_dcache() {
    unsafe {
        asm!(
            "mcr p15, 0, {0}, c7, c6, 0",
            in(reg) 0u32,
            options(nomem, nostack)
        );
    }
}
