OUTPUT_ARCH("riscv")

ENTRY(_start);
EXTERN(abort);

SECTIONS
{
    __HEAP_SIZE = 0x10000;
    __STACK_SIFT = 14; /* 0x4000 per core */
    __STACK_SIZE = (1 << __STACK_SIFT) * 4;

    .reset : 
    {
        KEEP(*(.reset.boot));
    } > RAM

    /* text: Program code section */
    .text :
    {
        *(.text. text.*);
    } > RAM

    /* rodata: Read-only data */
    .rodata :
    {
        *(.rdata .rodata. rodata.*);
    } > RAM

    /* data: Writable data */
    .data :
    {
        *(.data .data.*);
    } > RAM

    /* init_calls: application initialize functions */
    .init_calls :
    {
        INIT_CALLS_HEAD = .;
        *(.init_calls .init_calls.*);
    } > RAM

    .bss :
    {
        *(.bss .bss.*);
    }

    __HEAP_BASE = .;
    .heap :
    {
        *(.heap .heap.*);
    }

    __KERNEL_SP_BASE = . + __STACK_SIZE + __HEAP_SIZE;
    .stack :
    {
        *(.stack .stack.*)
    }

    /DISCARD/ :
    {
      *(.eh_frame);
    }

}
