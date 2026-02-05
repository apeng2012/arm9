//! Low level access to ARM9 processors
//!
//! This crate provides:
//!
//! - Access to ARM9 specific instructions
//! - Interrupt manipulation mechanisms
//! - CPSR register access
//!
//! # ARM9 vs Cortex-M
//!
//! ARM9 (ARMv4T/ARMv5TE) is significantly different from Cortex-M:
//!
//! - No NVIC (interrupt controller is external/chip-specific)
//! - No SysTick timer
//! - Different exception model (7 exceptions)
//! - Different processor modes (User/FIQ/IRQ/SVC/ABT/UND/SYS)
//! - CPSR instead of xPSR

#![deny(missing_docs)]
#![no_std]
#![allow(clippy::missing_inline_in_public_items)]

pub mod asm;
pub mod interrupt;
pub mod register;

#[cfg(feature = "critical-section-single-core")]
mod critical_section_impl;

#[cfg(feature = "critical-section-single-core")]
pub mod atomic;
