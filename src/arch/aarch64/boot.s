.section ".text._start"
.global _start
_start:
    MRS x1, MPIDR_EL1
    AND x1, x1, #0xFF
    CBNZ x1, _loop_other_core
    LDR x1, =__stack_top
    MOV sp, x1
    LDR x1, =__bss_start
    LDR x2, =__bss_end
_zero_bss:
    CMP x1, x2
    B.HS start_kernel
    STR xzr, [x1], #8
    B _zero_bss
_loop_other_core:
    WFE
    B _loop_other_core
