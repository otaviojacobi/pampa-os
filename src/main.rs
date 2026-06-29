#![no_std]
#![no_main]
use core::arch::global_asm;
use core::panic::PanicInfo;
use core::ptr::write_volatile;

const UART_DR: *mut u32 = 0xFE20_1000 as *mut u32;

global_asm!(include_str!(env!("PAMPA_BOOT_ASM")));

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
