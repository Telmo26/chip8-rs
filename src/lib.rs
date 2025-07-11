use std::{
    fs::File, io::Read, 
};

use rand;

mod display;
use display::Display;

mod utility;
use utility::reconstruct_byte;

use crate::display::HeightError;

pub struct Chip8 {
    pub memory: [u8; 4096],
    pc: u16,
    i: u16,
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
    v: [u8; 16],
    pub display: Display,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let display = Display::new();

        let mut chip8 = Chip8 {
            memory: [0; 4096],
            pc: 0x200,
            i: 0,
            stack: Vec::with_capacity(16),
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
        let nibbles = Self::divide_opcode(&opcode);
        
        // println!("Opcode = {opcode:X}, nibbles = {nibbles:?}");

        match nibbles[0] {
            0x0 => match nibbles[3] {
                // Clear display
                0x0 => self.display.clear(),

                // Returning from a subroutine
                0xE => self.pc = self.stack.pop().unwrap(), // If this crashes, it means we tried
                                                            // to return from a subroutine without
                                                            // being in a subroutine, which indicates
                                                            // a bigger issue, and should crash.

                _ => panic!(), // This is 'loading machine code' that we have to ignore
            },

            // Jump
            0x1 => self.pc = reconstruct_byte(&nibbles[1..]),

            // Subroutine calling
            0x2 => {
                self.stack.push(self.pc);
                self.pc = reconstruct_byte(&nibbles[1..])
            },

            // 3XNN -> if (VX == NN) skip one code block
            0x3 => if self.v[nibbles[1]] == reconstruct_byte(&nibbles[2..]) as u8 { self.pc += 2 },

            // 4XNN -> if (VX != NN) skip one code block
            0x4 => if self.v[nibbles[1]] != reconstruct_byte(&nibbles[2..]) as u8 { self.pc += 2 },

            // 5XY0 -> if (VX == VY) skip one code block
            0x5 => if self.v[nibbles[1]] == self.v[nibbles[2]] { self.pc += 2 },

            // Set register
            0x6 => self.v[nibbles[1]] = reconstruct_byte(&nibbles[2..]) as u8, 

            // Add to register
            0x7 => self.v[nibbles[1]] = self.v[nibbles[1]].wrapping_add(reconstruct_byte(&nibbles[2..]) as u8),
            
            // Arithmetical operations : 8XYZ
            0x8 => match nibbles[3] {
                // Set VX = VY
                0x0 => self.v[nibbles[1]] = self.v[nibbles[2]],

                // VX = VX OR VY
                0x1 => self.v[nibbles[1]] = self.v[nibbles[1]] | self.v[nibbles[2]],

                // VX = VX AND VY
                0x2 => self.v[nibbles[1]] = self.v[nibbles[1]] & self.v[nibbles[2]],

                // VX = VX XOR VY
                0x3 => self.v[nibbles[1]] = self.v[nibbles[1]] ^ self.v[nibbles[2]],

                // VX = VX + VY with carry
                0x4 => {
                    let overflow: bool;
                    (self.v[nibbles[1]], overflow) = self.v[nibbles[1]].overflowing_add(self.v[nibbles[2]]);
                    self.v[0xF] = if overflow { 1 } else { 0 };
                },

                // VX = VX - VY with carry
                0x5 => {
                    let underflow: bool;
                    (self.v[nibbles[1]], underflow) = self.v[nibbles[1]].overflowing_sub(self.v[nibbles[2]]);
                    self.v[0xF] = if underflow { 0 } else { 1 };
                },

                // VX = VY, VX >> 1, VF = bit shifted
                0x6 => {
                    let bottom_bit = self.v[nibbles[1]] % 2;
                    self.v[nibbles[1]] =  self.v[nibbles[1]] >> 1;
                    self.v[0xF] = bottom_bit;
                },

                // VX = VY - VX with carry
                0x7 => {
                    let underflow: bool;
                    (self.v[nibbles[1]], underflow) = self.v[nibbles[2]].overflowing_sub(self.v[nibbles[1]]);
                    self.v[0xF] = if underflow { 0 } else { 1 };
                },

                // VX = VY, VX << 1, VF = bit shifted
                0xE => {
                    let top_bit = self.v[nibbles[1]] / (1_u8 << 7);
                    self.v[nibbles[1]] = self.v[nibbles[1]] << 1;
                    self.v[0xF] = top_bit;
                },
                _ => panic!("Opcode {opcode:#X} not recognised!"),
            }

            // 9XY0 -> if (VX != VY) skip one code block
            0x9 => if self.v[nibbles[1]] != self.v[nibbles[2]] { self.pc += 2 },

            // Set I register
            0xA => self.i = reconstruct_byte(&nibbles[1..]),

            // Jump with offset
            0xB => self.pc = (self.v[0x0] as u16).wrapping_add(reconstruct_byte(&nibbles[1..])),

            // Random
            0xC => {
                let number: u8 = rand::random_range(..=u8::MAX);
                self.v[nibbles[1]] = number & reconstruct_byte(&nibbles[2..]) as u8;
            }

            // Draw call
            0xD => self.draw(nibbles[1], nibbles[2], nibbles[3]),

            // Non-blocking key operations
            0xE => {
                match nibbles[2] {
                    // Skips next iteration if VX key is pressed
                    0x9 => if self.display.check_key(self.v[nibbles[1]]) { self.pc += 2 },

                    // Skips next iteration if VX key is not pressed
                    0xA => if !self.display.check_key(self.v[nibbles[1]]) { self.pc += 2 },
                    
                    _ => panic!("Opcode {opcode:#X} not recognised!")
                }
            },

            0xF => match reconstruct_byte(&nibbles[2..]) {
                // VX = delay
                0x07 => self.v[nibbles[1]] = self.delay_timer,

                // VX = key pressed (blocking)
                0x0A => {
                    println!("Checking for keys");
                    let vx = &mut self.v[nibbles[1]];
                    match self.display.get_key() {
                        Ok(code) => {
                            *vx = code;
                            println!("Key pressed : {code}");
                        },
                        Err(_) => self.pc -= 2,
                    }
                },

                // Set delay timer to VX
                0x15 => self.delay_timer = self.v[nibbles[1]],

                // Set sound time to VX
                0x18 => self.sound_timer = self.v[nibbles[1]],

                // Register I increment
                0x1E => self.i = self.i.wrapping_add((self.v[nibbles[1]]) as u16),

                // Font character
                0x29 => self.i = 0x050 + (self.v[nibbles[1]] * 5) as u16,

                // Binary-coded decimal conversion
                0x33 => {
                    let mut vx = self.v[nibbles[1]];
                    for i in 0..3 {
                        self.memory[self.i as usize + (2 - i)] = vx % 10;
                        vx /= 10;
                    }
                },

                // Registers dump
                0x55 => {
                    for i in 0..=nibbles[1] {
                        self.memory[self.i as usize + i] = self.v[i];
                    }
                },

                // Registers load
                0x65 => {
                    for i in 0..=nibbles[1] {
                        self.v[i] = self.memory[self.i as usize + i];
                    }
                },

                _ => panic!("Opcode {opcode:#X} not recognised!")
            }

            _ => panic!("Opcode {opcode:#X} not yet implemented!"),
        }
    }

    fn divide_opcode(opcode: &u16) -> [usize; 4] {
        // Here we divide the opcode into its 4 nibbles
        let mut new_opcode = *opcode;
        let mut nibbles: [usize; 4] = [0 ; 4];
        for i in (0..nibbles.len()).rev() {
            nibbles[i] = (new_opcode % 16) as usize;
            new_opcode = new_opcode >> 4;
        }
        nibbles
    }

    fn draw(&mut self, vx: usize, vy: usize, n: usize) {
        let (x, y) = (self.v[vx] as usize, self.v[vy] as usize);
        self.v[0xF] = 0;

        for i in 0..n {
            let bytes = self.memory[self.i as usize + i];

            match self.display.fill(x, y + i, bytes) {
                Ok(collison) => if collison { self.v[0xF] = 1 },
                Err(HeightError) => (),
            }
        };
        self.display.update();
    }
}