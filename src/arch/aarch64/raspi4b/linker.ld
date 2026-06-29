ENTRY(_start)
SECTIONS
{
  . = 0x80000;
  __stack_top = .;
  .text : {
      KEEP(*(.text._start))
      *(.text .text.*)
  }
  .rodata : { *(.rodata .rodata.*) }
  .data : { *(.data .data.*) }
  .bss : {
    . = ALIGN(8);
    __bss_start = .;
    *(.bss .bss.*)
    . = ALIGN(8);
    __bss_end = .;
  }
  /DISCARD/ : { *(.eh_frame) *(.eh_frame_hdr) }
}
