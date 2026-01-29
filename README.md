# ARM9 Crates for F1C100S

This repository contains Rust crates for ARM9 processors, specifically targeting the Allwinner F1C100S SoC (ARM926EJ-S core).

Forked from [cortex-m](https://github.com/rust-embedded/cortex-m) and adapted for ARM9 architecture.

## Crates

* [`arm9`](arm9/): CPU peripheral access and intrinsics for ARM9
* [`arm9-rt`](arm9-rt/): Startup code and exception handling
* [`arm9-semihosting`](arm9-semihosting/): Semihosting support for debugging
* [`panic-semihosting`](panic-semihosting/): Panic handler using semihosting

## Target Device

- **SoC**: Allwinner F1C100S
- **Core**: ARM926EJ-S (ARMv5TEJ)
- **Target triple**: `armv5te-none-eabi`

## Key Differences from Cortex-M

- No NVIC (interrupt controller is external/chip-specific)
- No SysTick timer
- Different exception model (7 exceptions vs Cortex-M's vector table)
- Different processor modes (User/FIQ/IRQ/SVC/ABT/UND/SYS)
- CPSR instead of xPSR
- ARM Angel semihosting (SVC #0x123456) instead of BKPT

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
