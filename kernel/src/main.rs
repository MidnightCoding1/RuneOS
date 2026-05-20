#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod panic;
mod terminal;
mod input;
mod interrupts;

use limine::request::FramebufferRequest;

use terminal::writer::TerminalWriter;

use input::keyboard;

use interrupts::idt;
use interrupts::pic;

use x86_64::instructions::interrupts as cpu_interrupts;

#[used]
#[unsafe(link_section = ".requests")]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
static END_MARKER: [u64; 2] = [0, 0];

fn handle_command(cmd: &str, term: &mut TerminalWriter) {
    match cmd {
        "help" => {
            term.write_str("help clear\n");
        }

        "clear" => {
            term.clear(0x202020);
        }

        _ => {
            term.write_str("unknown\n");
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    idt::init_idt();

    pic::init();

    cpu_interrupts::enable();

    let response = FRAMEBUFFER_REQUEST
        .response()
        .unwrap();

    let fb = response
        .framebuffers()[0];

    let mut term = TerminalWriter::new(fb, 0xFFFFFF);

    term.clear(0x202020);

    term.write_str("RuneOS\n");

    let mut buffer = [0u8; 128];

    let mut idx = 0;

    loop {
        if let Some(key) = keyboard::pop_key() {
            if key == b'\n' {
                term.write_char(b'\n');

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
