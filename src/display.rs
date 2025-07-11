use minifb::{Scale, Window, WindowOptions};

use super::utility::byte_to_bools;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Display {
    window: minifb::Window,
    buffer: [bool; 64 * 32],
}

impl Display {
    pub fn new() -> Display {
        let mut window = Window::new("chip8-rs - A CHIP-8 Emulator in Rust",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions {
                                    scale: Scale::X16,
                                    ..WindowOptions::default()
                                 }).unwrap();

        window.set_target_fps(60);

        Display { window, buffer: [false; 64 * 32] }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(false);
    }

    pub fn fill(&mut self, x: u16, y: u16, bytes: u8) {
        let byte_array = byte_to_bools(bytes);
        
        for i in 0..8 {
            let px = (x + i) % WIDTH as u16;
            let py = y % HEIGHT as u16;
            let idx = (py * WIDTH as u16 + px) as usize;
            self.buffer[idx] = byte_array[i as usize];
        }
    }

    pub fn update(&mut self) {
        let display_buffer: Vec<u32> = self.buffer.iter()
            .map(|p| if *p {0xFFFFFF} else  {0x000000})
            .collect();

        self.window.update_with_buffer(&display_buffer, WIDTH, HEIGHT).expect(
            "Unable to update the display"
        );
    }
}