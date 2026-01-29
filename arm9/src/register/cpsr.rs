//! Current Program Status Register (CPSR)

use core::arch::asm;

/// Processor modes
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    /// User mode
    User = 0b10000,
    /// FIQ mode
    Fiq = 0b10001,
    /// IRQ mode
    Irq = 0b10010,
    /// Supervisor mode
    Supervisor = 0b10011,
    /// Abort mode
    Abort = 0b10111,
    /// Undefined mode
    Undefined = 0b11011,
    /// System mode
    System = 0b11111,
}

impl Mode {
    /// Convert from raw bits
    pub fn from_bits(bits: u8) -> Option<Self> {
        match bits & 0x1F {
            0b10000 => Some(Mode::User),
            0b10001 => Some(Mode::Fiq),
            0b10010 => Some(Mode::Irq),
            0b10011 => Some(Mode::Supervisor),
            0b10111 => Some(Mode::Abort),
            0b11011 => Some(Mode::Undefined),
            0b11111 => Some(Mode::System),
            _ => None,
        }
    }
}

/// CPSR register
#[derive(Clone, Copy, Debug)]
pub struct Cpsr {
    bits: u32,
}

impl Cpsr {
    /// Create from raw bits
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Get processor mode
    #[inline]
    pub fn mode(&self) -> Option<Mode> {
        Mode::from_bits((self.bits & 0x1F) as u8)
    }

    /// Check if IRQ disabled
    #[inline]
    pub fn irq_disabled(&self) -> bool {
        (self.bits & (1 << 7)) != 0
    }

    /// Check if FIQ disabled
    #[inline]
    pub fn fiq_disabled(&self) -> bool {
        (self.bits & (1 << 6)) != 0
    }
}

/// Read CPSR
#[inline]
pub fn read() -> Cpsr {
    let bits: u32;
    unsafe {
        asm!("mrs {}, cpsr", out(reg) bits, options(nomem, nostack, preserves_flags));
    }
    Cpsr::from_bits(bits)
}
