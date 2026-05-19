#![no_std]
#![no_main]

mod panic;
mod terminal;
mod input;

use limine::request::FramebufferRequest;
use terminal::writer::TerminalWriter;
use input::keyboard;

#[used]
#[link_section = ".requests"]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[link_section = ".requests"]
static END_MARKER: [u64; 2] = [0, 0];

fn handle_command(cmd: &str, term: &mut TerminalWriter) {
    match cmd {
        "help" => term.write_str("help exit clear\n"),
        "clear" => term.clear(0x202020),
        "exit" => term.write_str("no exit in kernel\n"),
        _ => {
            term.write_str("unknown command\n");
        }
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let fb = FRAMEBUFFER_REQUEST.get_response().unwrap()
        .framebuffers().next().unwrap();

    let mut term = TerminalWriter::new(fb, 0xFFFFFF);

    term.clear(0x202020);

    let mut buffer = [0u8; 128];
    let mut idx = 0;

    loop {
        if let Some(key) = keyboard::pop_key() {
            if key == b'\n' {
                let cmd = core::str::from_utf8(&buffer[..idx]).unwrap_or("");
                handle_command(cmd, &mut term);
                idx = 0;
                continue;
            }

            if idx < buffer.len() {
                buffer[idx] = key;
                idx += 1;
                term.write_char(key);
            }
        }
    }
}
