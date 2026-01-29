/* Sample memory.x file used for cortex-m-rt examples and tests only.
 * You must provide your own memory.x with values correct for your device,
 * don't just copy these.
 */

/*
 * F1C100S 内存布局:
 * SRAM: 0x00000000 - 0x00007FFF (32KB)
 * DDR:  需要初始化后才能使用
 *
 * Boot 镜像加载到 SRAM 0x00000000:
 * - 0x00-0x1F: eGON.BT0 header
 * - 0x20-0x2F: BROM boot device info
 * - 0x30+: 用户代码入口
 */

MEMORY
{
  /* FLASH and RAM are mandatory memory regions */
  /* Update examples/data_overflow.rs if you change these sizes. */
  FLASH : ORIGIN = 0x00000000, LENGTH = 256K
  RAM : ORIGIN = 0x20000000, LENGTH = 64K

  /* More memory regions can declared: for example this is a second RAM region */
  /* CCRAM : ORIGIN = 0x10000000, LENGTH = 8K */
}

/* The location of the stack can be overridden using the `_stack_start` symbol.
   By default it will be placed at the end of the RAM region */
/* _stack_start = ORIGIN(CCRAM) + LENGTH(CCRAM); */

/* The location of the .text section can be overridden using the `_stext` symbol.
   By default it will place after .vector_table */
/* _stext = ORIGIN(FLASH) + 0x40c; */
