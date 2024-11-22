use crate::chip8::opcodes::Opcode;
use crate::chip8::{display, memory, utils::rand_byte};

use super::memory::{FONTSET, FONTSET_ADDR_INIT};

#[derive(Debug)]
pub struct Chip8 {
    pub reg: [u8; 16],
    pub ram: [u8; 4096],
    pub ireg: u16,
    pub pc: u16,
    pub stack: [u16; 16],
    pub sp: u8,
    pub dt: u8,
    pub st: u8,
    pub keys: [bool; 16],
    pub display: [bool; 64 * 32],
    pub opcode: u16,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            reg: [0; 16],
            ram: [0; 4096],
            ireg: 0,
            pc: memory::ADDR_INIT,
            stack: [0; 16],
            sp: 0,
            dt: 0,
            st: 0,
            keys: [false; 16],
            display: [false; 64 * 32],
            opcode: 0,
        }
    }

    fn push(&mut self, val: u16) {
        if self.sp == 16 {
            panic!("Cannot push full stack.");
        }

        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        if self.sp == 0 {
            panic!("Cannot pop from empty stack.");
        }

        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    pub fn load_rom(&mut self, filename: &str) {
        memory::load_rom(&mut self.ram, filename);
    }

    pub fn load_fontset(&mut self) {
        memory::load_fontset(&mut self.ram, &FONTSET);
    }

    pub fn cycle(&mut self) {
        self.opcode =
            (self.ram[self.pc as usize] as u16) << 8 | self.ram[self.pc as usize + 1] as u16;
        self.pc += 2;

        // println!("Executing opcode: {:#04x}", self.opcode);

        if let Some(decoded) = self.decode_opcode(self.opcode) {
            // println!("Executing opcode (decoded): {:#?}", decoded);
            self.execute(decoded);
        } else {
            panic!("Unknown opcode: {:#04x}", self.opcode);
        }

        if self.dt > 0 {
            self.dt -= 1;
        }
        if self.st > 0 {
            self.st -= 1;
        }
    }

    pub fn emulate(&mut self, title: &str, delay: u64) {
        display::emulate(title, delay, self);
    }

    fn decode_opcode(&self, opcode: u16) -> Option<Opcode> {
        match opcode & 0xF000 {
            0x0000 => match opcode {
                0x00E0 => Some(Opcode::OP00E0),
                0x00EE => Some(Opcode::OP00EE),
                _ => None,
            },
            0x1000 => Some(Opcode::OP1NNN),
            0x2000 => Some(Opcode::OP2NNN),
            0x3000 => Some(Opcode::OP3XKK),
            0x4000 => Some(Opcode::OP4XKK),
            0x5000 => Some(Opcode::OP5XY0),
            0x6000 => Some(Opcode::OP6XKK),
            0x7000 => Some(Opcode::OP7XKK),
            0x8000 => match opcode & 0x000F {
                0x0 => Some(Opcode::OP8XY0),
                0x1 => Some(Opcode::OP8XY1),
                0x2 => Some(Opcode::OP8XY2),
                0x3 => Some(Opcode::OP8XY3),
                0x4 => Some(Opcode::OP8XY4),
                0x5 => Some(Opcode::OP8XY5),
                0x6 => Some(Opcode::OP8XY6),
                0x7 => Some(Opcode::OP8XY7),
                0xE => Some(Opcode::OP8XYE),
                _ => None,
            },
            0x9000 => Some(Opcode::OP9XY0),
            0xA000 => Some(Opcode::OPANNN),
            0xB000 => Some(Opcode::OPBNNN),
            0xC000 => Some(Opcode::OPCXKK),
            0xD000 => Some(Opcode::OPDXYN),
            0xE000 => match opcode & 0x00FF {
                0x9E => Some(Opcode::OPEX9E),
                0xA1 => Some(Opcode::OPEXA1),
                _ => None,
            },
            0xF000 => match opcode & 0x00FF {
                0x07 => Some(Opcode::OPFX07),
                0x0A => Some(Opcode::OPFX0A),
                0x15 => Some(Opcode::OPFX15),
                0x18 => Some(Opcode::OPFX18),
                0x1E => Some(Opcode::OPFX1E),
                0x29 => Some(Opcode::OPFX29),
                0x33 => Some(Opcode::OPFX33),
                0x55 => Some(Opcode::OPFX55),
                0x65 => Some(Opcode::OPFX65),
                _ => None,
            },
            _ => None,
        }
    }

    // https://austinmorlan.com/posts/chip8_emulator/#the-instructions
    fn execute(&mut self, opcode: Opcode) {
        match opcode {
            Opcode::OP00E0 => self.display = [false; 64 * 32],
            Opcode::OP00EE => {
                // println!("Stack Pointer: {}, Stack: {:?}", self.sp, self.stack);
                let nnn = self.pop();

                self.pc = nnn;
            }
            Opcode::OP1NNN => {
                // println!("Stack Pointer: {}, Stack: {:?}", self.sp, self.stack);
                let nnn = self.opcode & 0xFFF;

                self.pc = nnn;
            }
            Opcode::OP2NNN => {
                let nnn = self.opcode & 0xFFF;

                self.push(self.pc);
                self.pc = nnn;
            }
            Opcode::OP3XKK => {
                let vx = (self.opcode & 0xF00) >> 8;
                /*
                Keep bits beetween the 8-11 which rappresent the register, just then push the obtained register by 8 bits

                opcode:  1010 0010 0011 0100  (0xA234)
                mask:    0000 1111 0000 0000  (0x0F00)
                -----------------------------
                result:  0000 0010 0000 0000  (0x0200) ==> 0x0200 >> 8 = 0x02 or 0000 0000 0000 0020
                */

                let kk = self.opcode & 0xFF;

                if self.reg[vx as usize] == kk as u8 {
                    self.pc += 2;
                }
            }
            Opcode::OP4XKK => {
                let vx = (self.opcode & 0xF00) >> 8;
                let kk = self.opcode & 0xFF;

                if self.reg[vx as usize] != kk as u8 {
                    self.pc += 2;
                }
            }
            Opcode::OP5XY0 => {
                let vx = (self.opcode & 0xF00) >> 8;
                let vy = (self.opcode & 0xF0) >> 4;

                if self.reg[vx as usize] == self.reg[vy as usize] {
                    self.pc += 2;
                }
            }
            Opcode::OP6XKK => {
                let vx = (self.opcode & 0xF00) >> 8;
                let kk = self.opcode & 0xFF;

                self.reg[vx as usize] = kk as u8;
            }
            Opcode::OP7XKK => {
                let vx = (self.opcode & 0xF00) >> 8;
                let kk = self.opcode & 0xFF;

                // println!("OP7XKK vx reg: {}", self.reg[vx as usize]); // 43
                // println!("OP7XKK kk reg: {}", kk); // 255
                self.reg[vx as usize] = self.reg[vx as usize].wrapping_add(kk as u8)
            }
            Opcode::OP8XY0 => {
                let vx = (self.opcode & 0xF00) >> 8;
                let vy = (self.opcode & 0xF0) >> 4;

                self.reg[vx as usize] = self.reg[vy as usize];
            }
            Opcode::OP8XY1 => {
                let vx = (self.opcode & 0xF00) >> 8;
                let vy = (self.opcode & 0xF0) >> 4;

                self.reg[vx as usize] |= self.reg[vy as usize];
            }
            Opcode::OP8XY2 => {
                let vx = (self.opcode & 0xF00) >> 8;
                let vy = (self.opcode & 0xF0) >> 4;

                self.reg[vx as usize] &= self.reg[vy as usize];
            }
            Opcode::OP8XY3 => {
                let vx = (self.opcode & 0xF00) >> 8;
                let vy = (self.opcode & 0xF0) >> 4;

                self.reg[vx as usize] ^= self.reg[vy as usize];
            }
            Opcode::OP8XY4 => {
                let vx = (self.opcode & 0xF00) >> 8;
                let vy = (self.opcode & 0xF0) >> 4;

                let (newvx, carry) = self.reg[vx as usize].overflowing_add(self.reg[vy as usize]);
                self.reg[0xF] = if carry { 1 } else { 0 };

                self.ram[vx as usize] = newvx;
            }
            Opcode::OP8XY5 => {
                let vx = (self.opcode & 0xF00) >> 8;
                let vy = (self.opcode & 0xF0) >> 4;

                let (newvx, borrow) = self.reg[vx as usize].overflowing_sub(self.reg[vy as usize]);
                self.reg[0xF] = if borrow { 0 } else { 1 };

                // println!("OP8XY5 vx: {}", self.reg[vx as usize]);
                // println!("OP8XY5 vy: {}", self.reg[vy as usize]);
                self.ram[vx as usize] = newvx;
            }
            Opcode::OP8XY6 => {
                let vx = (self.opcode & 0xF00) >> 8;

                self.reg[0xF] = self.reg[vx as usize] & 0x1;

                self.reg[vx as usize] >>= 1;
            }
            Opcode::OP8XY7 => {
                let vx = (self.opcode & 0xF00) >> 8;
                let vy = (self.opcode & 0xF0) >> 4;

                let (newvx, borrow) = self.reg[vy as usize].overflowing_sub(self.reg[vx as usize]);
                self.reg[0xF] = if borrow { 0 } else { 1 };

                self.ram[vx as usize] = newvx;
            }
            Opcode::OP8XYE => {
                let vx = (self.opcode & 0xF00) >> 8;

                self.reg[0xF] = (self.reg[vx as usize] & 0x80) >> 7;

                self.reg[vx as usize] <<= 1;
            }
            Opcode::OP9XY0 => {
                let vx = (self.opcode & 0xF00) >> 8;
                let vy = (self.opcode & 0xF0) >> 4;

                if self.reg[vx as usize] != self.reg[vy as usize] {
                    self.pc += 2;
                }
            }
            Opcode::OPANNN => {
                let nnn = self.opcode & 0xFFF;

                self.ireg = nnn;
            }
            Opcode::OPBNNN => {
                let nnn = self.opcode & 0xFFF;

                self.pc = nnn + self.reg[0] as u16;
            }
            Opcode::OPCXKK => {
                let vx = (self.opcode & 0xF00) >> 8;
                let kk = self.opcode & 0xFF;

                self.reg[vx as usize] = rand_byte() & kk as u8;
            }
            Opcode::OPDXYN => {
                let vx = self.reg[((self.opcode & 0x0F00) >> 8) as usize];
                let vy = self.reg[((self.opcode & 0x00F0) >> 4) as usize];
                let n = (self.opcode & 0x000F) as u8;

                self.reg[0xF] = 0;
                for bindex in 0..n {
                    let y = (vy.wrapping_add(bindex)) % 32;
                    let sprite_byte = self.ram[(self.ireg + bindex as u16) as usize];
                    for btindex in 0..8 {
                        let x = (vx.wrapping_add(btindex)) % 64;
                        let pindex = (y as usize) * 64 + (x as usize);
                        let spixel = (sprite_byte >> (7 - btindex)) & 1;

                        if spixel == 1 {
                            if self.display[pindex] == true {
                                self.reg[0xF] = 1;
                            }
                            self.display[pindex] ^= true;
                        }
                    }
                }
            }
            Opcode::OPEX9E => {
                let vx = (self.opcode & 0xF00) >> 8;

                let key = self.reg[vx as usize];
                if self.keys[key as usize] {
                    self.pc += 2;
                }
            }
            Opcode::OPEXA1 => {
                let vx = (self.opcode & 0xF00) >> 8;

                let key = self.reg[vx as usize];
                if !self.keys[key as usize] {
                    self.pc += 2;
                }
            }
            Opcode::OPFX07 => {
                let vx = (self.opcode & 0xF00) >> 8;

                self.reg[vx as usize] = self.dt;
            }
            Opcode::OPFX0A => {
                let vx = (self.opcode & 0xF00) >> 8;

                for i in 0..self.keys.len() {
                    if i != (self.keys.len() - 1) {
                        if self.keys[i] {
                            self.reg[vx as usize] = i as u8;
                            break;
                        }
                    } else {
                        self.pc -= 2;
                    }
                }
            }
            Opcode::OPFX15 => {
                let vx = (self.opcode & 0xF00) >> 8;

                self.dt = self.reg[vx as usize];
            }
            Opcode::OPFX18 => {
                let vx = (self.opcode & 0xF00) >> 8;

                self.st = self.reg[vx as usize];
            }
            Opcode::OPFX1E => {
                let vx = (self.opcode & 0xF00) >> 8;

                self.ireg += self.reg[vx as usize] as u16;
            }
            Opcode::OPFX29 => {
                let vx = (self.opcode & 0xF00) >> 8;

                self.ireg = (FONTSET_ADDR_INIT + (5 * self.reg[vx as usize])) as u16;
            }
            Opcode::OPFX33 => {
                let vx = (self.opcode & 0xF00) >> 8;
                let mut val = self.reg[vx as usize];

                self.ram[self.ireg as usize + 2] = val % 10;
                val /= 10;

                self.ram[self.ireg as usize + 1] = val % 10;
                val /= 10;

                self.ram[self.ireg as usize] = val % 10;
            }
            Opcode::OPFX55 => {
                let vx = (self.opcode & 0xF00) >> 8;

                let mut i = 0;
                while i <= vx as usize {
                    self.ram[self.ireg as usize + i] = self.reg[i];
                    i += 1;
                }
            }
            Opcode::OPFX65 => {
                let vx = (self.opcode & 0xF00) >> 8;

                let mut i = 0;
                while i <= vx as usize {
                    self.reg[i] = self.ram[self.ireg as usize + i];
                    i += 1;
                }
            }
        }
    }
}
