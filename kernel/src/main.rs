#![no_std]
#![no_main]

mod framebuffer;
mod panic;

use framebuffer::writer;
use limine::request::FramebufferRequest;

#[used]
#[link_section = ".requests"]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[link_section = ".requests"]
static END_MARKER: [u64; 2] = [0, 0];

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let framebuffer_response = FRAMEBUFFER_REQUEST
        .get_response()
        .unwrap();

    let framebuffer = framebuffer_response
        .framebuffers()
        .next()
        .unwrap();

    writer::clear(framebuffer, 0x202020);

    loop {}
}
