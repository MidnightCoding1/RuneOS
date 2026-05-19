#![no_std]
#![no_main]

mod panic;
mod terminal;

use limine::request::FramebufferRequest;
use terminal::writer::TerminalWriter;

#[used]
#[link_section = ".requests"]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[link_section = ".requests"]
static END_MARKER: [u64; 2] = [0, 0];

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let framebuffer_response = FRAMEBUFFER_REQUEST.get_response().unwrap();
    let framebuffer = framebuffer_response.framebuffers().next().unwrap();

    let mut terminal = TerminalWriter::new(framebuffer, 0xFFFFFF);

    terminal.clear(0x202020);

    terminal.write_str("RUNEOS");

    loop {}
}
