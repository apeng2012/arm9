//! Interrupt manipulation for ARM9
//!
//! ARM9 uses CPSR I and F bits to control interrupts.

use core::arch::asm;

/// Disables all interrupts, returns previous CPSR I/F bits state
#[inline]
pub fn disable() -> u32 {
    let cpsr: u32;
    unsafe {
        asm!(
            "mrs {0}, cpsr",
            "orr {1}, {0}, #0xC0",
            "msr cpsr_c, {1}",
            out(reg) cpsr,
            out(reg) _,
            options(nomem, nostack)
        );
    }
    cpsr & 0xC0
}

/// Enables all interrupts
///
/// # Safety
/// Enabling interrupts can cause handlers to execute immediately.
#[inline]
pub unsafe fn enable() {
    asm!(
        "mrs {0}, cpsr",
        "bic {0}, {0}, #0xC0",
        "msr cpsr_c, {0}",
        out(reg) _,
        options(nomem, nostack)
    );
}

/// Restores interrupt state
///
/// # Safety
/// May enable interrupts.
#[inline]
pub unsafe fn restore(state: u32) {
    asm!(
        "mrs {0}, cpsr",
        "bic {0}, {0}, #0xC0",
        "orr {0}, {0}, {1}",
        "msr cpsr_c, {0}",
        out(reg) _,
        in(reg) state & 0xC0,
        options(nomem, nostack)
    );
}

/// Execute closure with interrupts disabled
#[inline]
pub fn free<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let state = disable();
    let result = f();
    unsafe { restore(state) };
    result
}

/// Type alias for interrupt state, matches critical_section's RawRestoreState
pub type State = u32;
