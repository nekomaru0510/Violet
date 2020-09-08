OUTPUT_ARCH("riscv")

ENTRY(_start);
EXTERN(abort);

SECTIONS
{
    /* Memory : 0x8000_000 ~ 0x8008_0000 (256KiB)*/
    __START_BASE = 0x80000000;
    __RODATA_BASE = 0x80020000;
    __DATA_BASE = 0x80028000;
    __BSS_BASE = 0x80030000;
    __HEAP_BASE = 0x80040000;
    __STACK_SIFT = 14; /* 0x4000 per core */
    __KERNEL_SP_BASE = 0x80080000;

    . = __START_BASE;

    .reset : 
    {
        KEEP(*(.reset.boot));
    }

    /* text: Program code section */
    .text :
    {
        *(.text. text.*);
    }

    /* rodata: Read-only data */
    . = __RODATA_BASE;
    .rodata :
    {
        *(.rdata .rodata. rodata.*);
    }

    /* data: Writable data */
    . = __DATA_BASE;
    .data :
    {
        *(.data .data.*);
        table = .;
    }

    . = __BSS_BASE;
    _bss_start = .;
    .bss :
    {
        *(.bss .bss.*);
    }

    . = __HEAP_BASE;
    .heap :
    {
        *(.heap .heap.*);
    }

    . = __KERNEL_SP_BASE;
    .stack :
    {
        *(.stack .stack.*)
    }

}