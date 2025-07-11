use chip8_rs::Chip8;

fn main() {
    let mut chip8 = Chip8::new();
    chip8.load_cartridge("IBM Logo.ch8");
    loop {
        chip8.cycle();
    };
}
