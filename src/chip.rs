use std::fmt::Debug;

use crate::{
    clock::Clock, keyboard::Keyboard, mem::{Addr, Memory}, opcodes::Opcode, stack::Stack
};
use rand::prelude::*;
use crate::screen::Screen;
use anyhow::Result;

pub const MEMORY_SIZE: usize = 4096;
pub const STACK_SIZE: usize = 16;

pub struct Registers {
    pub v: [u8; 16],
    pub i: Addr<u16, MEMORY_SIZE>,
}

impl Registers {
    pub fn init() -> Self {
        Self {
            v: [0u8; 16],
            i: Addr::<u16, MEMORY_SIZE>::from(0u16),
        }
    }
}

pub struct Chip {
    memory: Memory<MEMORY_SIZE>,
    stack: Stack<MEMORY_SIZE, STACK_SIZE>,
    registers: Registers,
    pc: Addr<u16, MEMORY_SIZE>,
    screen: Screen,
    keyboard: Keyboard,
    clock: Clock
}

impl Chip {
    pub fn init() -> Self {
        Self {
            memory: Memory::<MEMORY_SIZE>::init(),
            stack: Stack::<MEMORY_SIZE, STACK_SIZE>::init(),
            registers: Registers::init(),
            pc: Addr::<u16, MEMORY_SIZE>::from(0x200u16),
            screen: Screen::init(),
            keyboard: Keyboard::init(),
            clock: Clock::init(60.0)
        }
    }

    fn set_reg(&mut self, idx: Addr<u8, 16>, value: u8) {
        let reg: usize = idx.into();
        self.registers.v[reg] = value;
    }

    fn get_reg(&self, idx: Addr<u8, 16>) -> u8 {
        let reg: usize = idx.into();
        self.registers.v[reg]
    }

    fn consume_opcode(&mut self, opcode: Opcode) -> Result<()> {
        use Opcode::*;
        let mut rng = rand::thread_rng();
        let mut next_pc: Addr<u16, MEMORY_SIZE> = Addr(1) + self.pc;
        match opcode {
            CLS => {
                self.screen.clear();
            },
            RET => {
                self.pc = self.stack.pop()?;
            }
            JP(addr) => {
                next_pc = addr;
            },
            CALL(addr) => {
                self.stack.push(self.pc)?;
                next_pc = addr;
            }
            ISE(reg, x) => {
                if self.get_reg(reg) == x {
                    next_pc = next_pc + Addr(1);
                }
            }
            ISNE(reg, x) => {
                if self.get_reg(reg) != x {
                    next_pc = next_pc + Addr(1);
                }
            }
            RSE(reg_x, reg_y) => {
                if self.get_reg(reg_x) == self.get_reg(reg_y) {
                   next_pc = next_pc + Addr(1);
                }
            }
            ILD(reg, x) => {
                self.set_reg(reg, x);
            }
            IADD(reg, x) => {
                self.set_reg(reg, self.get_reg(reg).overflowing_add(x).0);
            }
            RLD(reg_dst, reg_src) => {
                self.set_reg(reg_dst, self.get_reg(reg_src));
            }
            ROR(reg_a, reg_b) => {
                self.set_reg(reg_a, self.get_reg(reg_a) | self.get_reg(reg_b));
            }
            RAND(reg_a, reg_b) => {
                self.set_reg(reg_a, self.get_reg(reg_a) & self.get_reg(reg_b));
            }
            RXOR(reg_a, reg_b) => {
                self.set_reg(reg_a, self.get_reg(reg_a) ^ self.get_reg(reg_b));
            }
            RADD(reg_a, reg_b) => {
                let (result, overflow) = self
                    .get_reg(reg_a)
                    .overflowing_add(self.get_reg(reg_b));
                self.set_reg(reg_a, result);
                self.set_reg(Addr(0xF), match overflow { true => 1, false => 0 });
            }
            RSUB(reg_a, reg_b) => {
                let (result, underflow) = self
                    .get_reg(reg_a)
                    .overflowing_sub(self.get_reg(reg_b));
                self.set_reg(reg_a, result);
                self.set_reg(Addr(0xF), match underflow { true => 0, false => 1 })
            }
            RSUBN(reg_a, reg_b) => {
                let (result, underflow) = self
                    .get_reg(reg_b)
                    .overflowing_sub(self.get_reg(reg_a));
                self.set_reg(reg_a, result);
                self.set_reg(Addr(0xF), match underflow { true => 0, false => 1 })
            }
            SHR(reg) => {
                let byte = self.get_reg(reg);
                self.set_reg(Addr(0xF), byte & 0b0000_0001);
                self.set_reg(reg, byte >> 1);
            }
            SHL(reg) => {
                let byte = self.get_reg(reg);
                self.set_reg(Addr(0xF), byte >> 7);
                self.set_reg(reg, byte << 1);
            }
            RND(reg, mask) => {
                let random: u8 = rng.gen();
                self.set_reg(reg, random & mask);
            }
            DRW(x, y, sprite_len) => {
                let sprite_start = self.registers.i.clone();
                let (x, y): (u8, u8) = (x.into(), y.into());
                let sprite_data =self.memory.read_block(sprite_start, sprite_len as usize); 
                let overlap: u8 = match self.screen.blit_sprite(sprite_data, (x, y)) {
                    true => 0x01,
                    false => 0x00,
                };
                self.set_reg(Addr(0xF), overlap);
            }
            SKP(key) => {
                if self.keyboard.key_is_pressed(key) {
                    next_pc = next_pc + Addr(1);
                }
            }
            SKNP(key) => {
                if !self.keyboard.key_is_pressed(key) {
                    next_pc = next_pc + Addr(1);
                }
            }
            _ => {}
        }

        self.pc = next_pc;
        Ok(())
    }
}

