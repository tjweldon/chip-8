use crate::{
    mem::{Addr, Memory},
    stack::Stack,
};

pub const MEMORY_SIZE: usize = 4096;
pub const STACK_SIZE: usize = 16;

pub struct Registers {
    pub v: [u8; 16],
    pub i: Addr<u16, MEMORY_SIZE>,
}

pub struct Chip {
    memory: Memory<MEMORY_SIZE>,
    stack: Stack<MEMORY_SIZE, STACK_SIZE>,
    registers: Registers,
    pc: Addr<u16, MEMORY_SIZE>,
}
