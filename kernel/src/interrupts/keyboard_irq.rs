use x86_64::instructions::port::Port;

use crate::input::keyboard;

pub extern "x86-interrupt" fn keyboard_interrupt_handler(
    _stack_frame: x86_64::structures::idt::InterruptStackFrame
) {
    unsafe {
        let mut port = Port::new(0x60);
        let scancode: u8 = port.read();

        if let Some(key) = translate_scancode(scancode) {
            keyboard::push_key(key);
        }

        let mut pic = Port::new(0x20);
        pic.write(0x20);
    }
}

fn translate_scancode(scancode: u8) -> Option<u8> {
    match scancode {
        0x1E => Some(b'a'),
        0x30 => Some(b'b'),
        0x2E => Some(b'c'),
        0x20 => Some(b'd'),
        0x12 => Some(b'e'),
        0x21 => Some(b'f'),
        0x22 => Some(b'g'),
        0x23 => Some(b'h'),
        0x17 => Some(b'i'),
        0x24 => Some(b'j'),
        0x25 => Some(b'k'),
        0x26 => Some(b'l'),
        0x32 => Some(b'm'),
        0x31 => Some(b'n'),
        0x18 => Some(b'o'),
        0x19 => Some(b'p'),
        0x10 => Some(b'q'),
        0x13 => Some(b'r'),
        0x1F => Some(b's'),
        0x14 => Some(b't'),
        0x16 => Some(b'u'),
        0x2F => Some(b'v'),
        0x11 => Some(b'w'),
        0x2D => Some(b'x'),
        0x15 => Some(b'y'),
        0x2C => Some(b'z'),
        0x1C => Some(b'\n'),
        0x39 => Some(b' '),
        _ => None,
    }
}
