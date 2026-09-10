OUTPUT_ARCH("riscv")
ENTRY(_start);

MEMORY {
  RAM (rwx) : ORIGIN = 0x80000000, LENGTH = 256M
}

/* Define sections */
SECTIONS {
    __STACK_SHIFT = 14; /* 0x4000 per core */
    __STACK_SIZE = (1 << __STACK_SHIFT);

    . = ORIGIN(RAM);
    .init : { *(.init) } > RAM 
    .text : { *(.text .text.*) } > RAM 
    .rodata : { *(.rodata) } > RAM 
    .data : { *(.data) } > RAM 
    .bss (NOLOAD): { 
        __BSS_START = .;
        *(.bss .bss.*)
        __BSS_END = .;
    } > RAM
    .stack (NOLOAD):
    {
        *(.stack .stack.*)
        . = ALIGN(0x1000);
        __KERNEL_SP_TOP = .;
        . += __STACK_SIZE*4;
        __KERNEL_SP_BOTTOM = .;
    } > RAM
    . = ALIGN(4);
}
