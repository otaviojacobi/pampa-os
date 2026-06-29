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
  .bss : { *(.bss .bss.*) }
  /DISCARD/ : { *(.eh_frame) *(.eh_frame_hdr) }
}
