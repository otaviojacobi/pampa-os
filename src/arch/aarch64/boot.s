.section ".text._start"
.global _start
_start:
    MRS x1, MPIDR_EL1
    AND x1, x1, #0xFF
    CBNZ x1, _loop_other_core
    LDR x1, =__stack_top
    MOV sp, x1
    B start_kernel
_loop_other_core:
    WFE
    B _loop_other_core
