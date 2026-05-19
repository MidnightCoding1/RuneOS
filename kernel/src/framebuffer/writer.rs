use limine::framebuffer::Framebuffer;

pub fn clear(framebuffer: &Framebuffer, color: u32) {
    let width = framebuffer.width() as usize;
    let height = framebuffer.height() as usize;
    let pitch = framebuffer.pitch() as usize;

    let buffer = framebuffer.addr();

    for y in 0..height {
        for x in 0..width {
            let pixel_offset = y * pitch + x * 4;

            unsafe {
                let pixel_ptr = buffer.add(pixel_offset).cast::<u32>();

                pixel_ptr.write_volatile(color);
            }
        }
    }
}
