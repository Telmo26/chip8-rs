use chip8_rs::Chip8;

fn main() {
    let mut chip8 = Chip8::new();
    chip8.load_cartridge("test-roms/5-quirks.ch8");
    loop {
        chip8.cycle();
    };
}
