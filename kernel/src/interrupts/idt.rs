use x86_64::structures::idt::InterruptDescriptorTable;

use crate::interrupts::handlers;
use crate::interrupts::keyboard_irq;

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub fn init_idt() {
    unsafe {
        IDT.breakpoint.set_handler_fn(handlers::breakpoint_handler);
        IDT.double_fault.set_handler_fn(handlers::double_fault_handler);
        IDT.page_fault.set_handler_fn(handlers::page_fault_handler);
        IDT.general_protection_fault.set_handler_fn(handlers::gpf_handler);

        IDT[32].set_handler_fn(timer_interrupt);
        IDT[33].set_handler_fn(keyboard_irq::keyboard_interrupt_handler);

        IDT.load();
    }
}

extern "x86-interrupt" fn timer_interrupt(_stack_frame: x86_64::structures::idt::InterruptStackFrame) {
}
