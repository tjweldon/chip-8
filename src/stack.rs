use std::ops::{Add, Sub}};

use crate::mem::Addr;
use anyhow::{Result, anyhow};

impl<const MAX: usize> From<usize> for Addr<u8, MAX> {
    fn from(value: usize) -> Self {
        Addr((value & 0xFFusize) as u8)
    }
}

impl<const MAX: usize> Into<u8> for Addr<u8, MAX> {
    fn into(self) -> u8 {
        self.0
    }
}

impl<const MAX: usize> Into<usize> for Addr<u8, MAX> {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl<const MAX: usize> Sub for Addr<u8, MAX> {
    type Output = Addr<u8, MAX>;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.checked_sub(rhs.0).unwrap_or(0))
    }
}

#[derive(Copy, Clone)]
#[repr(u32)]
enum ProgramCounter<const M_MAX: usize> {
    Empty,
    Pc(Addr<u16, M_MAX>)
}

pub struct Stack<const M_MAX: usize, const S_MAX: usize> {
    program_counters: [ProgramCounter<M_MAX>; S_MAX],
    ptr: Addr<u8, S_MAX>,
}

impl<const M_MAX: usize, const S_MAX: usize> Stack<M_MAX, S_MAX> {
    pub fn init() -> Self {
        Self {
            program_counters: [ProgramCounter::<M_MAX>::Empty; S_MAX],
            ptr: Addr::<u8, S_MAX>(0)
        }
    }

    pub fn push(&mut self, pc: Addr<u16, M_MAX>) -> Result<()> {
        if self.ptr.is_overflow() {
            Err(anyhow!("Stack overflow!"))
        } else {
            let idx: usize = Addr::<u8, S_MAX>::into(self.ptr);
            self.program_counters[idx] = ProgramCounter::Pc(pc);
            self.ptr = self.ptr + Addr(1);
            Ok(())
        }
    }

    pub fn pop(&mut self) -> Result<Addr<u16, M_MAX>> {
        let idx: Addr<u16, M_MAX> =  Addr::<u16, M_MAX>::from(1usize);
        let top_pc = self.program_counters[idx.into()];
        self.program_counters[idx.into()] = ProgramCounter::Empty;
        top_pc
    }
}
