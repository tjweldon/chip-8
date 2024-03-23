use crate::chip::MEMORY_SIZE;
use crate::mem::Addr;

type MemAddr = Addr<u16, MEMORY_SIZE>;
type RegIdx = Addr<u8, 16>;

pub enum Opcode {
    /// Unused
    SYS(MemAddr),
    
    /// Clear Screen
    CLS,
    /// Return
    RET,
    /// Jump
    JP(MemAddr),
    /// Jump Relative to Register V0
    JPV0(MemAddr),
    /// Call Subroutine
    CALL(MemAddr),

    /// Skip if Equal, Immediate
    ISE(RegIdx, u8),
    /// Skip if Not Equal, Immediate
    ISNE(RegIdx, u8),

    /// Skip if Equal, Register
    RSE(RegIdx, RegIdx),
    /// Skip if Not Equal, Register
    RSNE(RegIdx, RegIdx),

    /// Load, Immediate
    ILD(RegIdx, u8),
    /// Load, Register
    RLD(RegIdx, RegIdx),
    /// Load Register I, Reference
    LDI(MemAddr),

    /// Accumulate Register, Immediate
    IADD(RegIdx, u8),
    /// Accumulate Register, Register
    RADD(RegIdx, RegIdx),
    /// Todo
    ADDI(RegIdx),

    /// OR Into Register, Register
    ROR(RegIdx, RegIdx),

    /// AND Into Register, Register
    RAND(RegIdx, RegIdx),
    /// XOR Into Register, Register
    RXOR(RegIdx, RegIdx),
    /// Subtract from register, Register
    RSUB(RegIdx, RegIdx),
    /// As Above, opposite sign
    RSUBN(RegIdx, RegIdx),
    
    /// Shift Register Right
    SHR(RegIdx),
    /// Shift Register Left
    SHL(RegIdx),

    /// Generate a random number, & with byte
    RND(RegIdx, u8),
    /// Draw Sprite
    DRW(RegIdx, RegIdx, u8),

    /// Skip the next instruction if key pressed
    SKP(RegIdx),
    /// Skip the next instruction if key not pressed
    SKNP(RegIdx),

    /// timer instructions 
    LDDT(RegIdx),
    SETDT(RegIdx),
    SETST(RegIdx),
    LDK(RegIdx),
    LDFI(RegIdx),
    LDBCD(RegIdx),
    
    /// Core Dump
    DUMP(RegIdx),
    /// Core Load
    LOAD(RegIdx),
}

impl Opcode {
    fn mem_addr(mut n2: u8, mut n1: u8, mut n0: u8) -> MemAddr {
        n2 &= 0x0F;
        n1 &= 0x0F;
        n0 &= 0x0F;
        MemAddr::from((n2 as u16) << 8 | (n1 as u16) << 4 | n0 as u16)
    }

    fn byte(mut k1: u8, mut k0: u8) -> u8 {
        k1 &= 0x0F;
        k0 &= 0x0F;

        (k1 << 4) | k0
    }

    fn reg(mut x: u8) -> RegIdx {
        x &= 0x0F;

        RegIdx::from(x)
    }

    fn parse(raw: [u8; 2]) -> Option<Opcode> {
        let nibbles: [u8; 4] = [raw[0] >> 4, raw[0] & 0x0F, raw[1] >> 4, raw[1] & 0x0F];
        use Opcode::*;
        match nibbles {
            [0x0, 0x0, 0xE, 0x0] => Some(CLS),
            [0x0, 0x0, 0xE, 0xE] => Some(RET),
            [0x0, n2, n1, n0] => Some(SYS(Self::mem_addr(n2, n1, n0))),
            [0x1, n2, n1, n0] => Some(JP(Self::mem_addr(n2, n1, n0))),
            [0x2, n2, n1, n0] => Some(CALL(Self::mem_addr(n2, n1, n0))),
            [0x3, x, k1, k0] => Some(ISE(Self::reg(x), Self::byte(k1, k0))),
            [0x4, x, k1, k0] => Some(ISNE(Self::reg(x), Self::byte(k1, k0))),
            [0x5, x, y, 0] => Some(RSE(Self::reg(x), Self::reg(y))),
            [0x6, x, k1, k0] => Some(ILD(Self::reg(x), Self::byte(k1, k0))),
            [0x7, x, k1, k0] => Some(IADD(Self::reg(x), Self::byte(k1, k0))),
            [0x8, x, y, 0] => Some(RLD(Self::reg(x), Self::reg(y))),
            [0x8, x, y, 1] => Some(ROR(Self::reg(x), Self::reg(y))),
            [0x8, x, y, 2] => Some(RAND(Self::reg(x), Self::reg(y))),
            [0x8, x, y, 3] => Some(RXOR(Self::reg(x), Self::reg(y))),
            [0x8, x, y, 4] => Some(RADD(Self::reg(x), Self::reg(y))),
            [0x8, x, y, 5] => Some(RSUB(Self::reg(x), Self::reg(y))),
            [0x8, x, _, 6] => Some(SHR(Self::reg(x))),
            [0x8, x, y, 7] => Some(RSUBN(Self::reg(x), Self::reg(y))),
            [0x8, x, _, 0xE] => Some(SHL(Self::reg(x))),
            [0x9, x, y, 0] => Some(RSNE(Self::reg(x), Self::reg(y))),
            [0xA, n2, n1, n0] => Some(LDI(Self::mem_addr(n2, n1, n0))),
            [0xB, n2, n1, n0] => Some(JPV0(Self::mem_addr(n2, n1, n0))),
            [0xC, x, k1, k0] => Some(RND(Self::reg(x), Self::byte(k1, k0))),
            [0xD, x, y, n] => Some(DRW(Self::reg(x), Self::reg(y), n)),
            [0xE, x, 0x9, 0xE] => Some(SKP(Self::reg(x))),
            [0xE, x, 0xA, 0x1] => Some(SKNP(Self::reg(x))),
            [0xF, x, 0x0, 0x7] => Some(LDDT(Self::reg(x))),
            [0xF, x, 0x0, 0xA] => Some(LDK(Self::reg(x))),
            [0xF, x, 0x1, 0x5] => Some(SETDT(Self::reg(x))),
            [0xF, x, 0x1, 0x8] => Some(SETST(Self::reg(x))),
            [0xF, x, 0x1, 0xE] => Some(ADDI(Self::reg(x))),
            [0xF, x, 0x2, 0x9] => Some(LDFI(Self::reg(x))),
            [0xF, x, 0x3, 0x3] => Some(LDBCD(Self::reg(x))),
            [0xF, x, 0x5, 0x5] => Some(DUMP(Self::reg(x))),
            [0xF, x, 0x6, 0x5] => Some(LOAD(Self::reg(x))),

            _ => None,
        }
    }
}
