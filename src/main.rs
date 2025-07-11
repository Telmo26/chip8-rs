use std::{thread, time::{self, Duration}};

use chip8_rs::Chip8;

fn main() {
    let mut chip8 = Chip8::new();
    chip8.load_cartridge("IBM Logo.ch8");
    loop {
        let time = time::Instant::now();
        chip8.cycle();

        while time.elapsed() < Duration::from_micros(16600) {
            thread::sleep(Duration::from_millis(1));
        }
    };
}
