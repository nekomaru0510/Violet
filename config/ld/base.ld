OUTPUT_ARCH("riscv")

ENTRY(_start);
EXTERN(abort);

SECTIONS
{
    __HEAP_SIZE = 0x20000;
    __STACK_SIFT = 14; /* 0x4000 per core */
    __STACK_SIZE = (1 << __STACK_SIFT);

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
    } > RAM

    .heap :
    {
        *(.heap .heap.*);
        . = ALIGN(0x1000);
        __HEAP_BASE = .;
        . += __HEAP_SIZE;
        __HEAP_END = .;
    } > RAM

    .stack :
    {
        *(.stack .stack.*)
        . = ALIGN(0x1000);
        __KERNEL_SP_TOP = .;
        . += __STACK_SIZE*4;
        __KERNEL_TRAP_SP_BOTTOM = .;

        . = ALIGN(0x1000);
        . += __STACK_SIZE*4;
        __KERNEL_SP_BOTTOM = .;
    } > RAM

    /DISCARD/ :
    {
      *(.eh_frame);
    }

}
