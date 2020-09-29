
MEMORY
{
  /*ROM (rx)  : ORIGIN = 0x20400000, LENGTH = 0x30000*/
  RAM (rwx) : ORIGIN = 0x80000000, LENGTH = 256K
}

INCLUDE tools/config/ld/kernel.ld
