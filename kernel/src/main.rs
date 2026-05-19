#![no_std]
#![no_main]

mod panic;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
