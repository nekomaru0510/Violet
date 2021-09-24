OUTPUT_ARCH("riscv")

ENTRY(_start);
EXTERN(abort);

SECTIONS
{
    /* Memory : 0x8000_000 ~ 0x8010_0000 (256KiB)*/
    __START_BASE = 0x80000000;
    __RODATA_BASE = __START_BASE + 0x00020000;
    __DATA_BASE = __RODATA_BASE + 0x00008000;
    __BSS_BASE = __DATA_BASE + 0x00008000;
    __HEAP_BASE = __BSS_BASE + 0x00010000;
    __STACK_SIFT = 14; /* 0x4000 per core */
    __KERNEL_SP_BASE = __START_BASE + 0x00100000;

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
        SAMPLE_CONTAINER = .;
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