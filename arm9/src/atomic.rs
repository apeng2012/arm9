//! Software implementation of atomic operations for ARMv5TE.
//!
//! ARMv5TE does not have hardware atomic instructions (like LDREX/STREX),
//! so we implement them using critical sections (interrupt disable/enable).
//!
//! These functions provide the compiler builtins that LLVM expects for atomic
//! operations. They are automatically linked when using `core::sync::atomic`
//! types on ARMv5TE targets.
//!
//! # Safety
//!
//! This implementation is only safe for single-core systems. On multi-core
//! systems, disabling interrupts on one core does not prevent another core
//! from accessing the same memory location.

use core::ffi::c_int;

/// Memory barrier - on single-core ARMv5TE this is a compiler barrier.
///
/// On single-core systems without out-of-order execution, a compiler
/// barrier is sufficient to ensure memory ordering.
#[no_mangle]
pub unsafe extern "C" fn __sync_synchronize() {
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}

// ============================================================================
// 8-bit atomic operations
// ============================================================================

/// Atomic load 8-bit
#[no_mangle]
pub unsafe extern "C" fn __atomic_load_1(ptr: *const u8, _memorder: c_int) -> u8 {
    critical_section::with(|_| core::ptr::read_volatile(ptr))
}

/// Atomic store 8-bit
#[no_mangle]
pub unsafe extern "C" fn __atomic_store_1(ptr: *mut u8, val: u8, _memorder: c_int) {
    critical_section::with(|_| core::ptr::write_volatile(ptr, val))
}

/// Atomic exchange 8-bit
#[no_mangle]
pub unsafe extern "C" fn __atomic_exchange_1(ptr: *mut u8, val: u8, _memorder: c_int) -> u8 {
    critical_section::with(|_| {
        let old = core::ptr::read_volatile(ptr);
        core::ptr::write_volatile(ptr, val);
        old
    })
}

/// Atomic compare and exchange 8-bit
#[no_mangle]
pub unsafe extern "C" fn __atomic_compare_exchange_1(
    ptr: *mut u8,
    expected: *mut u8,
    desired: u8,
    _weak: bool,
    _success_memorder: c_int,
    _failure_memorder: c_int,
) -> bool {
    critical_section::with(|_| {
        let current = core::ptr::read_volatile(ptr);
        if current == core::ptr::read_volatile(expected) {
            core::ptr::write_volatile(ptr, desired);
            true
        } else {
            core::ptr::write_volatile(expected, current);
            false
        }
    })
}


/// Atomic fetch and add 8-bit
#[no_mangle]
pub unsafe extern "C" fn __atomic_fetch_add_1(ptr: *mut u8, val: u8, _memorder: c_int) -> u8 {
    critical_section::with(|_| {
        let old = core::ptr::read_volatile(ptr);
        core::ptr::write_volatile(ptr, old.wrapping_add(val));
        old
    })
}

/// Atomic fetch and sub 8-bit
#[no_mangle]
pub unsafe extern "C" fn __atomic_fetch_sub_1(ptr: *mut u8, val: u8, _memorder: c_int) -> u8 {
    critical_section::with(|_| {
        let old = core::ptr::read_volatile(ptr);
        core::ptr::write_volatile(ptr, old.wrapping_sub(val));
        old
    })
}

/// Atomic fetch and or 8-bit
#[no_mangle]
pub unsafe extern "C" fn __atomic_fetch_or_1(ptr: *mut u8, val: u8, _memorder: c_int) -> u8 {
    critical_section::with(|_| {
        let old = core::ptr::read_volatile(ptr);
        core::ptr::write_volatile(ptr, old | val);
        old
    })
}

/// Atomic fetch and and 8-bit
#[no_mangle]
pub unsafe extern "C" fn __atomic_fetch_and_1(ptr: *mut u8, val: u8, _memorder: c_int) -> u8 {
    critical_section::with(|_| {
        let old = core::ptr::read_volatile(ptr);
        core::ptr::write_volatile(ptr, old & val);
        old
    })
}

/// Atomic fetch and xor 8-bit
#[no_mangle]
pub unsafe extern "C" fn __atomic_fetch_xor_1(ptr: *mut u8, val: u8, _memorder: c_int) -> u8 {
    critical_section::with(|_| {
        let old = core::ptr::read_volatile(ptr);
        core::ptr::write_volatile(ptr, old ^ val);
        old
    })
}

/// Atomic fetch and nand 8-bit
#[no_mangle]
pub unsafe extern "C" fn __atomic_fetch_nand_1(ptr: *mut u8, val: u8, _memorder: c_int) -> u8 {
    critical_section::with(|_| {
        let old = core::ptr::read_volatile(ptr);
        core::ptr::write_volatile(ptr, !(old & val));
        old
    })
}

// ============================================================================
// 16-bit atomic operations
// ============================================================================

/// Atomic load 16-bit
#[no_mangle]
pub unsafe extern "C" fn __atomic_load_2(ptr: *const u16, _memorder: c_int) -> u16 {
    critical_section::with(|_| core::ptr::read_volatile(ptr))
}

/// Atomic store 16-bit
#[no_mangle]
pub unsafe extern "C" fn __atomic_store_2(ptr: *mut u16, val: u16, _memorder: c_int) {
    critical_section::with(|_| core::ptr::write_volatile(ptr, val))
}

/// Atomic exchange 16-bit
#[no_mangle]
pub unsafe extern "C" fn __atomic_exchange_2(ptr: *mut u16, val: u16, _memorder: c_int) -> u16 {
    critical_section::with(|_| {
        let old = core::ptr::read_volatile(ptr);
        core::ptr::write_volatile(ptr, val);
        old
    })
}

/// Atomic compare and exchange 16-bit
#[no_mangle]
pub unsafe extern "C" fn __atomic_compare_exchange_2(
    ptr: *mut u16,
    expected: *mut u16,
    desired: u16,
    _weak: bool,
    _success_memorder: c_int,
    _failure_memorder: c_int,
) -> bool {
    critical_section::with(|_| {
        let current = core::ptr::read_volatile(ptr);
        if current == core::ptr::read_volatile(expected) {
            core::ptr::write_volatile(ptr, desired);
            true
        } else {
            core::ptr::write_volatile(expected, current);
            false
        }
    })
}

/// Atomic fetch and add 16-bit
#[no_mangle]
pub unsafe extern "C" fn __atomic_fetch_add_2(ptr: *mut u16, val: u16, _memorder: c_int) -> u16 {
    critical_section::with(|_| {
        let old = core::ptr::read_volatile(ptr);
        core::ptr::write_volatile(ptr, old.wrapping_add(val));
        old
    })
}

/// Atomic fetch and sub 16-bit
#[no_mangle]
pub unsafe extern "C" fn __atomic_fetch_sub_2(ptr: *mut u16, val: u16, _memorder: c_int) -> u16 {
    critical_section::with(|_| {
        let old = core::ptr::read_volatile(ptr);
        core::ptr::write_volatile(ptr, old.wrapping_sub(val));
        old
    })
}

/// Atomic fetch and or 16-bit
#[no_mangle]
pub unsafe extern "C" fn __atomic_fetch_or_2(ptr: *mut u16, val: u16, _memorder: c_int) -> u16 {
    critical_section::with(|_| {
        let old = core::ptr::read_volatile(ptr);
        core::ptr::write_volatile(ptr, old | val);
        old
    })
}

/// Atomic fetch and and 16-bit
#[no_mangle]
pub unsafe extern "C" fn __atomic_fetch_and_2(ptr: *mut u16, val: u16, _memorder: c_int) -> u16 {
    critical_section::with(|_| {
        let old = core::ptr::read_volatile(ptr);
        core::ptr::write_volatile(ptr, old & val);
        old
    })
}

/// Atomic fetch and xor 16-bit
#[no_mangle]
pub unsafe extern "C" fn __atomic_fetch_xor_2(ptr: *mut u16, val: u16, _memorder: c_int) -> u16 {
    critical_section::with(|_| {
        let old = core::ptr::read_volatile(ptr);
        core::ptr::write_volatile(ptr, old ^ val);
        old
    })
}

/// Atomic fetch and nand 16-bit
#[no_mangle]
pub unsafe extern "C" fn __atomic_fetch_nand_2(ptr: *mut u16, val: u16, _memorder: c_int) -> u16 {
    critical_section::with(|_| {
        let old = core::ptr::read_volatile(ptr);
        core::ptr::write_volatile(ptr, !(old & val));
        old
    })
}


// ============================================================================
// 32-bit atomic operations
// ============================================================================

/// Atomic load 32-bit
#[no_mangle]
pub unsafe extern "C" fn __atomic_load_4(ptr: *const u32, _memorder: c_int) -> u32 {
    critical_section::with(|_| core::ptr::read_volatile(ptr))
}

/// Atomic store 32-bit
#[no_mangle]
pub unsafe extern "C" fn __atomic_store_4(ptr: *mut u32, val: u32, _memorder: c_int) {
    critical_section::with(|_| core::ptr::write_volatile(ptr, val))
}

/// Atomic exchange 32-bit
#[no_mangle]
pub unsafe extern "C" fn __atomic_exchange_4(ptr: *mut u32, val: u32, _memorder: c_int) -> u32 {
    critical_section::with(|_| {
        let old = core::ptr::read_volatile(ptr);
        core::ptr::write_volatile(ptr, val);
        old
    })
}

/// Atomic compare and exchange 32-bit
#[no_mangle]
pub unsafe extern "C" fn __atomic_compare_exchange_4(
    ptr: *mut u32,
    expected: *mut u32,
    desired: u32,
    _weak: bool,
    _success_memorder: c_int,
    _failure_memorder: c_int,
) -> bool {
    critical_section::with(|_| {
        let current = core::ptr::read_volatile(ptr);
        if current == core::ptr::read_volatile(expected) {
            core::ptr::write_volatile(ptr, desired);
            true
        } else {
            core::ptr::write_volatile(expected, current);
            false
        }
    })
}

/// Atomic fetch and add 32-bit
#[no_mangle]
pub unsafe extern "C" fn __atomic_fetch_add_4(ptr: *mut u32, val: u32, _memorder: c_int) -> u32 {
    critical_section::with(|_| {
        let old = core::ptr::read_volatile(ptr);
        core::ptr::write_volatile(ptr, old.wrapping_add(val));
        old
    })
}

/// Atomic fetch and sub 32-bit
#[no_mangle]
pub unsafe extern "C" fn __atomic_fetch_sub_4(ptr: *mut u32, val: u32, _memorder: c_int) -> u32 {
    critical_section::with(|_| {
        let old = core::ptr::read_volatile(ptr);
        core::ptr::write_volatile(ptr, old.wrapping_sub(val));
        old
    })
}

/// Atomic fetch and or 32-bit
#[no_mangle]
pub unsafe extern "C" fn __atomic_fetch_or_4(ptr: *mut u32, val: u32, _memorder: c_int) -> u32 {
    critical_section::with(|_| {
        let old = core::ptr::read_volatile(ptr);
        core::ptr::write_volatile(ptr, old | val);
        old
    })
}

/// Atomic fetch and and 32-bit
#[no_mangle]
pub unsafe extern "C" fn __atomic_fetch_and_4(ptr: *mut u32, val: u32, _memorder: c_int) -> u32 {
    critical_section::with(|_| {
        let old = core::ptr::read_volatile(ptr);
        core::ptr::write_volatile(ptr, old & val);
        old
    })
}

/// Atomic fetch and xor 32-bit
#[no_mangle]
pub unsafe extern "C" fn __atomic_fetch_xor_4(ptr: *mut u32, val: u32, _memorder: c_int) -> u32 {
    critical_section::with(|_| {
        let old = core::ptr::read_volatile(ptr);
        core::ptr::write_volatile(ptr, old ^ val);
        old
    })
}

/// Atomic fetch and nand 32-bit
#[no_mangle]
pub unsafe extern "C" fn __atomic_fetch_nand_4(ptr: *mut u32, val: u32, _memorder: c_int) -> u32 {
    critical_section::with(|_| {
        let old = core::ptr::read_volatile(ptr);
        core::ptr::write_volatile(ptr, !(old & val));
        old
    })
}
