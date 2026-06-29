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
    B.HS _drop_el2_el1
    STR xzr, [x1], #8
    B _zero_bss
_drop_el2_el1:
    MRS x1, CurrentEL
    LSR x1, x1, #2
    CMP x1, #0b10
    B.NE start_kernel
_lower_el:
    // There is only one way to lower permissions from EL2 -> EL1
    // And that is calling ERET
    MOV x1, #(1 << 31)      // HCR_EL2.RW = 1  → EL1 runs AArch64
    MSR HCR_EL2, x1

    MOV x1, #0x3C5          // SPSR_EL2: mode EL1h (0x5) + DAIF masked (0x3C0)
    MSR SPSR_EL2, x1

    ADR x1, start_kernel    // ELR_EL2: the address ERET will jump to
    MSR ELR_EL2, x1

    LDR x1, =__stack_top    // SP_EL1: the stack EL1 will use
    MSR SP_EL1, x1

    ERET                    // PC ← ELR_EL2, PSTATE ← SPSR_EL2 → now at EL1, in start_kernel
_loop_other_core:
    WFE
    B _loop_other_core
