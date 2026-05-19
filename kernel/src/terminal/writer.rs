use limine::framebuffer::Framebuffer;

pub struct TerminalWriter {
    pub framebuffer: &'static Framebuffer,
    pub color: u32,
}

impl TerminalWriter {
    pub fn new(framebuffer: &'static Framebuffer, color: u32) -> Self {
        Self {
            framebuffer,
            color,
        }
    }

    pub fn write_char(&self, x: usize, y: usize, ch: u8) {
        let scale = 2;
        let base_x = x * 8 * scale;
        let base_y = y * 8 * scale;

        for row in 0..8 {
            for col in 0..8 {
                let pixel_on = (ch + row as u8 + col as u8) % 2 == 0;

                if pixel_on {
                    for dy in 0..scale {
                        for dx in 0..scale {
                            let px = base_x + col * scale + dx;
                            let py = base_y + row * scale + dy;

                            self.draw_pixel(px, py);
                        }
                    }
                }
            }
        }
    }

    fn draw_pixel(&self, x: usize, y: usize) {
        let pitch = self.framebuffer.pitch() as usize;
        let addr = self.framebuffer.addr();

        let offset = y * pitch + x * 4;

        unsafe {
            let pixel = addr.add(offset).cast::<u32>();
            pixel.write_volatile(self.color);
        }
    }

    pub fn clear(&self, color: u32) {
        let width = self.framebuffer.width() as usize;
        let height = self.framebuffer.height() as usize;
        let pitch = self.framebuffer.pitch() as usize;

        let addr = self.framebuffer.addr();

        for y in 0..height {
            for x in 0..width {
                let offset = y * pitch + x * 4;

                unsafe {
                    let pixel = addr.add(offset).cast::<u32>();
                    pixel.write_volatile(color);
                }
            }
        }
    }
}
