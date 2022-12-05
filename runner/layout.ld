ENTRY(_Reset)
SECTIONS
{
    . = 0x40000000;
    .text : { *(.text) }
    .data : { *(.data) }
    .bss : { *(.bss COMMON) }
    .heap (NOLOAD) : {
        . = ALIGN(64);
        _heap_start = .;
        . += 0x4000;
        _heap_end = .;
    }
    .stack (NOLOAD) : {
        . = ALIGN(64);
        . += 0x4000;
        stack_top = .;
    }
}
