use minifb::{Key, KeyRepeat, Scale, Window, WindowOptions};

use super::utility::byte_to_bools;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

const CHIP8_KEYS_AZERTY: [Key; 16] = [
    Key::X, // 0
    Key::Key1, // 1
    Key::Key2, // 2
    Key::Key3, // 3
    Key::A, // 4
    Key::Z, // 5
    Key::E, // 6
    Key::Q, // 7
    Key::S, // 8
    Key::D, // 9
    Key::W, // 10 (A)
    Key::C, // 11 (B)
    Key::Key4, // 12 (C)
    Key::R, // 13 (D)
    Key::F, // 14 (E)
    Key::V, // 15 (F)
];

pub struct Display {
    pub window: minifb::Window,
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

    pub fn check_key(&mut self, key: u8) -> bool {
        self.window.update();
        self.window.is_key_down(CHIP8_KEYS_AZERTY[key as usize])
    }

    pub fn get_key(&self) -> Result<u8, NoKeysError> {
        let keys = self.window.get_keys_pressed(KeyRepeat::Yes);
        if keys.len() == 0 {
            println!("No keys");
            Err(NoKeysError)
        } else {
            println!("Keys : {keys:?}");
            let pressed_key = keys[0];
            for (code, key) in CHIP8_KEYS_AZERTY.iter().enumerate() {
                println!("Checking key {key:?}");
                if pressed_key == *key {
                    return Ok(code as u8)
                }
            }
            Err(NoKeysError)
        }
    }
}

pub struct HeightError;

pub struct NoKeysError;