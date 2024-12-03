#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;
use riscv;

const GPIO_BASE: usize = 0x40000000;

const DELAY: u32 = 0x100000;

#[entry]
fn main() -> ! {
    let gpio_ptr = GPIO_BASE as *mut u32;
    let mut gpio_value = 1;

    loop {
        gpio_value = !gpio_value;
        unsafe { core::ptr::write(gpio_ptr, gpio_value) };
        riscv::asm::delay(DELAY);
    }
}
