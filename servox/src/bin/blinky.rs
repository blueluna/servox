// SPDX-License-Identifier: MIT
//! Blinky example for SERV
#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;

const DELAY: u32 = 0x100000;

#[entry]
fn main() -> ! {
    let mut gpio_value = true;

    loop {
        gpio_value = !gpio_value;
        servox::gpio_set(gpio_value);
        riscv::asm::delay(DELAY);
    }
}
