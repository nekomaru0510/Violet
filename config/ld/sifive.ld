/* QEMU sifive_u */

MEMORY
{
  /*ROM (rx)  : ORIGIN = 0x20400000, LENGTH = 0x30000*/
  RAM (rwx) : ORIGIN = 0x80000000, LENGTH = 2M
}

INCLUDE config/ld/base.ld
