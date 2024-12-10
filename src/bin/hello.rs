#![no_std]
#![no_main]

extern crate panic_halt;

use riscv;
use riscv_rt::entry;

const GPIO_BASE: usize = 0x40000000;

const UART_STOP_BIT: u32 = 0x00000100;

const DELAY: u32 = 0x00080000;

const TEXT: &str = "Hello, I'm Servant!\r\n";

#[entry]
fn main() -> ! {
    let gpio_ptr = GPIO_BASE as *mut u32;
    unsafe {
        core::ptr::write(gpio_ptr, 1);
    }
    riscv::asm::nop();
    riscv::asm::nop();

    loop {
        for byte in TEXT.as_bytes() {
            let mut output = (u32::from(*byte) | UART_STOP_BIT) << 1;
            // Timing is critical in the following loop
            while output > 0 {
                unsafe { core::ptr::write(gpio_ptr, output) };
                output >>= 1;
                riscv::asm::nop();
                riscv::asm::nop();
            }
        }

        riscv::asm::delay(DELAY);
    }
}
