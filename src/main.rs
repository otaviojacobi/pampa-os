#![no_std]
#![no_main]
use core::arch::global_asm;
use core::fmt::Write;
use core::panic::PanicInfo;

use crate::drivers::uart::Uart;

global_asm!(include_str!(env!("PAMPA_BOOT_ASM")));

mod arch;
mod drivers;
mod mm;

#[unsafe(no_mangle)]
pub extern "C" fn start_kernel() -> ! {
    let mut uart = Uart;
    let _ = write!(uart, "hello, otavio\n   ");
    unsafe {
        core::arch::asm!("svc #0");
    }
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn handle_cpu_exception() -> ! {
    let esr: u64;
    let far: u64;
    let elr: u64;

    unsafe {
        core::arch::asm!("MRS {}, ESR_EL1", out(reg) esr);
        core::arch::asm!("MRS {}, FAR_EL1", out(reg) far);
        core::arch::asm!("MRS {}, ELR_EL1", out(reg) elr);
    }

    panic!("out esr={esr:#x}, far={far:#x}, elr={elr:#x}");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut uart = Uart;
    let _ = write!(uart, "{info}");
    loop {}
}
