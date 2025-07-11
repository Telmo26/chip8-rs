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

    pub fn fill(&mut self, x: usize, y: usize, bytes: u8) -> Result<bool, HeightError> {
        if !( y < HEIGHT ) {
            return Err(HeightError)
        }

        let byte_array = byte_to_bools(bytes);
        let mut collision =  false;
        
        for i in 0..8 {
            let px = if x + i < WIDTH { x + i } else { WIDTH - 1 } ;
            let idx = (y * WIDTH + px) as usize;

            if self.buffer[idx] && byte_array[i as usize] {
                collision = true;
            }
            self.buffer[idx] = self.buffer[idx] ^ byte_array[i as usize];
        };
        Ok(collision)
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

pub struct HeightError;