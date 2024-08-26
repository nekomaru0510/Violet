/* QEMU Virt */

MEMORY
{
  /*ROM (rx)  : ORIGIN = 0x20400000, LENGTH = 0x30000*/
  RAM (rwx) : ORIGIN = 0x80100000, LENGTH = 1M
}

INCLUDE ../../config/ld/base.ld
