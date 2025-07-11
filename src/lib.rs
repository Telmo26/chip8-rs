use std::{
    io::Read,
    fs::File,
};

mod display;
use display::Display;

mod utility;
use utility::reconstruct_byte;

pub struct Chip8 {
    pub memory: [u8; 4096],
    pc: u16,
    i: u16,
    stack: [u16; 16],
    sp: u8,
    delay_timer: u8,
    sound_timer: u8,
    v: [u8; 16],
    display: Display,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let display = Display::new();

        let mut chip8 = Chip8 {
            memory: [0; 4096],
            pc: 0x200,
            i: 0,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            v: [0; 16],
            display,
        };
        chip8.load_font();
        chip8
    }

    fn load_font(&mut self) {
        let font = [
            [0xF0, 0x90, 0x90, 0x90, 0xF0], // 0
            [0x20, 0x60, 0x20, 0x20, 0x70], // 1
            [0xF0, 0x10, 0xF0, 0x80, 0xF0], // 2
            [0xF0, 0x10, 0xF0, 0x10, 0xF0], // 3
            [0x90, 0x90, 0xF0, 0x10, 0x10], // 4
            [0xF0, 0x80, 0xF0, 0x10, 0xF0], // 5
            [0xF0, 0x80, 0xF0, 0x90, 0xF0], // 6
            [0xF0, 0x10, 0x20, 0x40, 0x40], // 7
            [0xF0, 0x90, 0xF0, 0x90, 0xF0], // 8
            [0xF0, 0x90, 0xF0, 0x10, 0xF0], // 9
            [0xF0, 0x90, 0xF0, 0x90, 0x90], // A
            [0xE0, 0x90, 0xE0, 0x90, 0xE0], // B
            [0xF0, 0x80, 0x80, 0x80, 0xF0], // C
            [0xE0, 0x90, 0x90, 0x90, 0xE0], // D
            [0xF0, 0x80, 0xF0, 0x80, 0xF0], // E
            [0xF0, 0x80, 0xF0, 0x80, 0x80]  // F
        ];
        
        for (i, character) in font.iter().enumerate() {
            let offset = 0x050 + i * 5;
            self.memory[offset..offset+5].copy_from_slice(character);
        }
    }

    pub fn load_cartridge(&mut self, path: &str) {
        let mut file = File::open(path).expect("Error while opening file");
        let mut buffer: Vec<u8> = Vec::new();
        
        file.read_to_end(&mut buffer).expect("Error reading file");

        for (i, byte) in buffer.iter().enumerate() {
            self.memory[0x200 + i] = *byte;
        };
    }

    pub fn cycle(&mut self) {
        let opcode = ((self.memory[self.pc as usize]) as u16) << 8 
                            | self.memory[(self.pc + 1) as usize] as u16;

        self.pc += 2;

        self.execute_opcode(opcode);

        self.delay_timer = self.delay_timer.saturating_sub(1);
        self.sound_timer = self.sound_timer.saturating_sub(1);
    }

    fn execute_opcode(&mut self, opcode: u16) {
        // Here we divide the opcode into its 4 nibbles
        let mut new_opcode = opcode;
        let mut nibbles: [u16; 4] = [0 ; 4];
        for i in (0..nibbles.len()).rev() {
            nibbles[i] = new_opcode % 16;
            new_opcode = new_opcode >> 4;
        }

        match nibbles[0] {
            0x0 => match nibbles[3] {
                // Clear display
                0x0 => self.display.clear(),
                _ => panic!("Opcode {opcode:#X} not yet implemented!"),
            },
            // Jump
            0x1 => {
                let address = reconstruct_byte(nibbles[1], nibbles[2], nibbles[3]);
                self.pc = address;
            },
            // Set register
            0x6 => self.v[nibbles[1] as usize] = (nibbles[2] << 4 | nibbles[3]) as u8, 
            // Add to register
            0x7 => self.v[nibbles[1] as usize] += (nibbles[2] << 4 | nibbles[3]) as u8,
            // Set I register
            0xA => self.i = reconstruct_byte(nibbles[1], nibbles[2], nibbles[3]),
            // Draw call
            0xD => self.draw(nibbles[1], nibbles[2], nibbles[3]),
            _ => panic!("Opcode {opcode:#X} not yet implemented!"),
        }
    }

    fn draw(&mut self, vx: u16, vy: u16, n: u16) {
        let (x, y) = (self.v[vx as usize] as u16, self.v[vy as usize] as u16);

        for i in 0..n {
            let bytes = self.memory[(self.i + i) as usize];
            self.display.fill(x, y + i, bytes);
        };
        self.display.update();
    }



}