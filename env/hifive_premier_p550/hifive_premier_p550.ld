/* QEMU Virt */

MEMORY
{
  /*ROM (rx)  : ORIGIN = 0x20400000, LENGTH = 0x30000*/
  RAM (rwx) : ORIGIN = 0x380200000, LENGTH = 256M
}

INCLUDE ../../config/ld/base.ld
