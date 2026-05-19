use x86_64::instructions::port::Port;

const PIC1_COMMAND: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;

const PIC2_COMMAND: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

const ICW1_INIT: u8 = 0x10;
const ICW1_ICW4: u8 = 0x01;

const ICW4_8086: u8 = 0x01;

pub fn init_pic() {
    unsafe {
        let mut pic1_command = Port::new(PIC1_COMMAND);
        let mut pic1_data = Port::new(PIC1_DATA);

        let mut pic2_command = Port::new(PIC2_COMMAND);
        let mut pic2_data = Port::new(PIC2_DATA);

        pic1_command.write(ICW1_INIT | ICW1_ICW4);
        pic2_command.write(ICW1_INIT | ICW1_ICW4);

        pic1_data.write(0x20);
        pic2_data.write(0x28);

        pic1_data.write(4);
        pic2_data.write(2);

        pic1_data.write(ICW4_8086);
        pic2_data.write(ICW4_8086);

        pic1_data.write(0);
        pic2_data.write(0);
    }
}
