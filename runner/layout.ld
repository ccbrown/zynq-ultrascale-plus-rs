MEMORY
{
   psu_ddr_0_MEM_0 : ORIGIN = 0x0, LENGTH = 0x7FF00000
}

ENTRY(_vector_table)

SECTIONS
{
.text : {
   . = ALIGN(2048);
   *(.text)
} > psu_ddr_0_MEM_0

.data : {
   . = ALIGN(64);
   *(.data)
} > psu_ddr_0_MEM_0

.bss (NOLOAD) : {
   . = ALIGN(64);
   __bss_start__ = .;
   *(.bss COMMON)
   __bss_end__ = .;
} > psu_ddr_0_MEM_0

.heap (NOLOAD) : {
    . = ALIGN(64);
    __heap_start__ = .;
    . += 0x80000;
    __heap_end__ = .;
} > psu_ddr_0_MEM_0

.stack (NOLOAD) : {
    . = ALIGN(64);
    . += 0x4000;
    __stack_top__ = .;
} > psu_ddr_0_MEM_0
}
