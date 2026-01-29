# `arm9`

> Low level access to ARM9 processors (ARMv4T/ARMv5TE)

This crate is forked from [cortex-m](https://github.com/rust-embedded/cortex-m) and adapted for ARM9 processors, specifically targeting the Allwinner F1C100S SoC.

## Features

- ARM9 specific assembly instructions (WFI, DSB, DMB, ISB via CP15)
- CPSR register access and processor mode detection
- Interrupt enable/disable via CPSR I/F bits
- Critical section implementation for single-core ARM9

## Target

- Primary target: Allwinner F1C100S (ARM926EJ-S, ARMv5TEJ)
- Architecture: ARMv4T/ARMv5TE
- Target triple: `armv5te-none-eabi`

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.61 and up.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
