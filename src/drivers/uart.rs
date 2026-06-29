use core::fmt::Write;
use core::ptr::write_volatile;

pub struct Uart;

pub const UART_DR: *mut u32 = 0xFE20_1000 as *mut u32;

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            unsafe {
                write_volatile(UART_DR, b as u32);
            }
        }
        Ok(())
    }
}
