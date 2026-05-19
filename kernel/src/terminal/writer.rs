use limine::framebuffer::Framebuffer;

pub struct TerminalWriter {
    framebuffer: &'static Framebuffer,
    color: u32,
    x: usize,
    y: usize,
}

impl TerminalWriter {
    pub fn new(framebuffer: &'static Framebuffer, color: u32) -> Self {
        Self {
            framebuffer,
            color,
            x: 0,
            y: 0,
        }
    }

    pub fn write_char(&mut self, ch: u8) {
        if ch == b'\n' {
            self.x = 0;
            self.y += 1;
            return;
        }

        self.draw_char(self.x, self.y, ch);

        self.x += 1;

        if self.x > 80 {
            self.x = 0;
            self.y += 1;
        }
    }

    pub fn write_str(&mut self, s: &str) {
        for b in s.bytes() {
            self.write_char(b);
        }
    }

    fn draw_char(&self, x: usize, y: usize, _ch: u8) {
        let scale = 2;

        let base_x = x * 8 * scale;
        let base_y = y * 8 * scale;

        for dy in 0..8 * scale {
            for dx in 0..8 * scale {
                self.draw_pixel(base_x + dx, base_y + dy);
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
