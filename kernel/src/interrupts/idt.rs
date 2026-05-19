use x86_64::structures::idt::InterruptDescriptorTable;

use crate::interrupts::handlers;

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub fn init_idt() {
    unsafe {
        IDT.breakpoint.set_handler_fn(handlers::breakpoint_handler);
        IDT.double_fault.set_handler_fn(handlers::double_fault_handler);
        IDT.page_fault.set_handler_fn(handlers::page_fault_handler);
        IDT.general_protection_fault.set_handler_fn(handlers::gpf_handler);

        IDT.load();
    }
}
