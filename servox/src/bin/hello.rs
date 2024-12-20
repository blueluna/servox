// SPDX-License-Identifier: MIT
//! Hello UART example for SERV
#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;
use servox::GPIO_REG;

const UART_STOP_BIT: u32 = 0x00000100;

const DELAY: u32 = 0x00080000;

const TEXT: &str = "Hello, I'm Servant!\r\n";

#[entry]
fn main() -> ! {
    unsafe {
        core::ptr::write(GPIO_REG, 1);
    }
    riscv::asm::nop();
    riscv::asm::nop();

    loop {
        for byte in TEXT.as_bytes() {
            let mut output = (u32::from(*byte) | UART_STOP_BIT) << 1;
            // Timing is critical in the following loop
            while output > 0 {
                unsafe { core::ptr::write(GPIO_REG, output) };
                output >>= 1;
                riscv::asm::nop();
                riscv::asm::nop();
            }
        }

        riscv::asm::delay(DELAY);
    }
}
