#![no_std]
#![no_main]
use core::arch::global_asm;
use core::panic::PanicInfo;
use core::ptr::write_volatile;

const UART_DR: *mut u32 = 0xFE20_1000 as *mut u32;

global_asm! {
    r#"
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
    "#
}

#[unsafe(no_mangle)]
pub extern "C" fn start_kernel() -> ! {
    unsafe {
        write_volatile(UART_DR, b'o' as u32);
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
