#![no_std]
#![no_main]

mod panic;

use core::panic::PanicInfo;
use limine::request::FramebufferRequest;

#[used]
#[link_section = ".requests"]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[link_section = ".requests"]
static END_MARKER: [u64; 2] = [0, 0];

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
