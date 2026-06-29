.section ".text._start"
.global _start
_start:
    MRS x1, MPIDR_EL1
    AND x1, x1, #0xFF
    CBNZ x1, _loop_other_core
    MOV x1, #0x80000
    MOV sp, x1
    B start_kernel
_loop_other_core:
    WFE
    B _loop_other_core
