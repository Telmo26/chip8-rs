use std::{thread, time::Duration};

use chip8_rs::Chip8;

fn main() {
    let mut chip8 = Chip8::new();
    chip8.load_cartridge("test-roms/3-corax+.ch8");
    loop {
        chip8.cycle();
        thread::sleep(Duration::from_millis(500));
    };
}
