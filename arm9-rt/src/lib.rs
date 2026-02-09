//! Startup code and minimal runtime for ARM9 microprocessors
//!
//! This crate contains all the required parts to build a `no_std` application (binary crate) that
//! targets an ARM9 microprocessor (ARMv4T/ARMv5TE architecture).
//!
//! # Features
//!
//! This crate takes care of:
//!
//! - The memory layout of the program, including the exception vector table
//! - Initializing `static` variables before the program entry point
//! - Setting up the stack pointers for different processor modes
//!
//! This crate provides the following attributes:
//!
//! - [`#[entry]`][attr-entry] to declare the entry point of the program
//! - [`#[exception]`][attr-exception] to override an exception handler
//!
//! # ARM9 Exception Model
//!
//! ARM9 uses a different exception model than Cortex-M:
//!
//! | Vector Offset | Exception          |
//! |---------------|-------------------|
//! | 0x00          | Reset             |
//! | 0x04          | Undefined         |
//! | 0x08          | SWI (Software Interrupt) |
//! | 0x0C          | Prefetch Abort    |
//! | 0x10          | Data Abort        |
//! | 0x14          | Reserved          |
//! | 0x18          | IRQ               |
//! | 0x1C          | FIQ               |
//!
//! # Requirements
//!
//! ## `memory.x`
//!
//! This crate expects the user to provide the memory layout via a linker script named `memory.x`.
//!
//! ```text
//! /* Example memory.x for a typical ARM9 device */
//! MEMORY
//! {
//!   FLASH : ORIGIN = 0x00000000, LENGTH = 512K
//!   RAM   : ORIGIN = 0x20000000, LENGTH = 64K
//! }
//! ```
//!
//! # Example
//!
//! ```no_run
//! #![no_main]
//! #![no_std]
//!
//! use panic_halt as _;
//! use cortex_m_rt::entry;
//!
//! #[entry]
//! fn main() -> ! {
//!     loop {}
//! }
//! ```
//!
//! [attr-entry]: attr.entry.html
//! [attr-exception]: attr.exception.html

#![deny(missing_docs)]
#![no_std]

extern crate arm9_rt_macros as macros;

use core::arch::global_asm;
use core::fmt;

// Re-export the entry macro
pub use macros::entry;
pub use macros::exception;

// ARM9 exception vector table and startup code
// ARM9 开发使用 ARM 模式（32位指令）
//
// F1C100S 启动流程说明:
// 从 SPI Flash / SD Card 启动时:
// 1. BROM 读取 boot0 镜像到 SRAM 0x00000000
// 2. boot0 镜像结构:
//    - 0x00-0x1F: eGON.BT0 header (由 mkboot.py 添加)
//    - 0x20-0x2F: BROM 会写入 boot device info (不能放代码!)
//    - 0x30+: 用户代码 (本文件编译后的内容)
// 3. BROM 验证 header 后跳转到 0x30 执行
global_asm!(
    r#"
    /*
     * F1C100S 入口点 - 必须位于 0x30
     * BROM 验证 eGON.BT0 header 后跳转到此处
     */
    .section .entry, "ax"
    .global _start
    .type _start, %function
    .arm
_start:
    @ 跳转到真正的 Reset 处理代码
    b Reset

    /*
     * ARM9 异常向量表
     * 注意: 向量表不在 0x00，需要在初始化时重定位或配置 CP15
     */
    .section .vector_table, "ax"
    .global __vector_table
    .arm
    .align 5

__vector_table:
    @ ARM9 向量表使用跳转指令
    ldr pc, _reset_addr
    ldr pc, _undef_addr
    ldr pc, _swi_addr
    ldr pc, _pabt_addr
    ldr pc, _dabt_addr
    nop
    ldr pc, _irq_addr
    ldr pc, _fiq_addr

_reset_addr:    .word Reset
_undef_addr:    .word Undefined
_swi_addr:      .word SWI
_pabt_addr:     .word PrefetchAbort
_dabt_addr:     .word DataAbort
_reserved:      .word 0
_irq_addr:      .word IRQ
_fiq_addr:      .word FIQ

    .section .Reset, "ax"
    .global Reset
    .type Reset, %function
    .arm
Reset:
    @ 禁用中断
    mrs r0, cpsr
    orr r0, r0, #0xC0
    msr cpsr_c, r0

    @ 确保 CP15 V 位 = 0，异常向量表在 0x00000000
    mrc p15, 0, r0, c1, c0, 0
    bic r0, r0, #0x2000
    mcr p15, 0, r0, c1, c0, 0

    @ 设置各模式的栈指针
    msr cpsr_c, #0xD1
    ldr sp, =_fiq_stack_start

    msr cpsr_c, #0xD2
    ldr sp, =_irq_stack_start

    msr cpsr_c, #0xD7
    ldr sp, =_abt_stack_start

    msr cpsr_c, #0xDB
    ldr sp, =_und_stack_start

    msr cpsr_c, #0xD3
    ldr sp, =_svc_stack_start

    msr cpsr_c, #0xDF
    ldr sp, =_stack_start

    bl __pre_init

    @ 初始化 .bss
    ldr r0, =__sbss
    ldr r1, =__ebss
    mov r2, #0
1:
    cmp r0, r1
    strlo r2, [r0], #4
    blo 1b

    @ 初始化 .data
    ldr r0, =__sdata
    ldr r1, =__edata
    ldr r2, =__sidata
2:
    cmp r0, r1
    ldrlo r3, [r2], #4
    strlo r3, [r0], #4
    blo 2b

    bl main

3:
    b 3b

    .size Reset, . - Reset
"#
);

// Default exception handlers
global_asm!(
    r#"
    .section .text.DefaultHandler, "ax"
    .global DefaultHandler_
    .type DefaultHandler_, %function
    .arm
DefaultHandler_:
    b DefaultHandler_
    .size DefaultHandler_, . - DefaultHandler_

    .section .text.DefaultPreInit, "ax"
    .global DefaultPreInit
    .type DefaultPreInit, %function
    .arm
DefaultPreInit:
    mov pc, lr
    .size DefaultPreInit, . - DefaultPreInit
"#
);

/// Registers saved during an exception (ARM9 style)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExceptionFrame {
    /// General purpose register r0
    pub r0: u32,
    /// General purpose register r1
    pub r1: u32,
    /// General purpose register r2
    pub r2: u32,
    /// General purpose register r3
    pub r3: u32,
    /// General purpose register r12
    pub r12: u32,
    /// Link register
    pub lr: u32,
    /// Program counter
    pub pc: u32,
    /// Program status register
    pub cpsr: u32,
}

impl fmt::Debug for ExceptionFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct Hex(u32);
        impl fmt::Debug for Hex {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "0x{:08x}", self.0)
            }
        }
        f.debug_struct("ExceptionFrame")
            .field("r0", &Hex(self.r0))
            .field("r1", &Hex(self.r1))
            .field("r2", &Hex(self.r2))
            .field("r3", &Hex(self.r3))
            .field("r12", &Hex(self.r12))
            .field("lr", &Hex(self.lr))
            .field("pc", &Hex(self.pc))
            .field("cpsr", &Hex(self.cpsr))
            .finish()
    }
}

/// Returns a pointer to the start of the heap
#[inline]
pub fn heap_start() -> *mut u32 {
    extern "C" {
        static mut __sheap: u32;
    }
    #[allow(unused_unsafe)]
    unsafe {
        core::ptr::addr_of_mut!(__sheap)
    }
}

#[export_name = "error: cortex-m-rt appears more than once in the dependency graph"]
#[doc(hidden)]
pub static __ONCE__: () = ();
