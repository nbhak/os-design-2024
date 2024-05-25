#![no_std]
#![no_main]

use core::ptr;

#[no_mangle]

pub extern "C" fn kmain() -> ! {
    const GPIO_BASE: usize = 0x3F000000 + 0x200000;
    const GPIO_FSEL1: *mut u32 = (GPIO_BASE + 0x04) as *mut u32;
    const GPIO_SET0: *mut u32 = (GPIO_BASE + 0x1C) as *mut u32;
    const GPIO_CLR0: *mut u32 = (GPIO_BASE + 0x28) as *mut u32;

    // FIXME: STEP 1: Set GPIO Pin 16 as output.
    // FIXME: STEP 2: Continuously set and clear GPIO 16.
    loop {}
}

fn spin_sleep_us(us: u64) {
    for _ in 0..(us * 6) {
        unsafe {
            core::arch::asm!("nop");
        }
    }
}

fn spin_sleep_ms(ms: u64) {
    spin_sleep_us(ms * 1000);
}

use panic_halt as _;
