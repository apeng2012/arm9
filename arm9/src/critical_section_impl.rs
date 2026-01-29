//! Critical section implementation for ARM9

use critical_section::{set_impl, Impl, RawRestoreState};
use crate::interrupt;

struct Arm9CriticalSection;
set_impl!(Arm9CriticalSection);

unsafe impl Impl for Arm9CriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        // Returns the I and F bits (bits 6-7) of CPSR
        interrupt::disable()
    }

    unsafe fn release(state: RawRestoreState) {
        interrupt::restore(state);
    }
}
