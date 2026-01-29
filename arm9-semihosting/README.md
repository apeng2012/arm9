# `arm9-semihosting`

> Semihosting for ARM9 processors

This crate is forked from [cortex-m-semihosting](https://github.com/rust-embedded/cortex-m) and adapted for ARM9 processors.

## Features

- ARM Angel semihosting interface (SVC #0x123456)
- Host I/O operations (stdout, stderr, file operations)
- Debug exit for QEMU testing

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
