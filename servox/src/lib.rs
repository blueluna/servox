//! Utility library for Servant
#![no_std]

/// Servant GPIO base address
pub const GPIO_BASE: usize = 0x40000000;
/// Servant Timer base address
pub const TIMER_BASE: usize = 0x80000000;

/// GPIO register pointer
pub const GPIO_REG: *mut u32 = GPIO_BASE as *mut u32;

/// GPIO pin value set function
pub fn gpio_set(pin0: bool) {
    let value = if pin0 { 1 } else { 0 };
    unsafe { core::ptr::write(GPIO_REG, value) };
}
