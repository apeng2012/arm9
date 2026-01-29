# `arm9-rt`

> Startup code and minimal runtime for ARM9 microprocessors

This crate is forked from [cortex-m-rt](https://github.com/rust-embedded/cortex-m) and adapted for ARM9 processors, specifically targeting the Allwinner F1C100S SoC.

## Features

- ARM9 exception vector table (Reset, Undefined, SWI, Prefetch Abort, Data Abort, IRQ, FIQ)
- Stack setup for all processor modes (USR/SYS, FIQ, IRQ, SVC, ABT, UND)
- BSS zeroing and DATA initialization
- F1C100S boot header support (eGON.BT0 format)
- `#[entry]` and `#[exception]` proc macros

## F1C100S Boot Flow

When booting from SPI Flash or SD Card:
1. BROM loads boot0 image to SRAM at 0x00000000
2. Boot image structure:
   - 0x00-0x1F: eGON.BT0 header (added by mkboot.py)
   - 0x20-0x2F: BROM writes boot device info (reserved)
   - 0x30+: User code entry point
3. BROM validates header and jumps to 0x30

## Target

- Primary target: Allwinner F1C100S (ARM926EJ-S, ARMv5TEJ)
- Target triple: `armv5te-none-eabi`

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.61.0 and up.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
