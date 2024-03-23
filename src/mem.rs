use std::ops::Add;

const MEM_SIZE: u16 = 0x1000;

#[derive(Copy, Clone)]
pub struct Addr<T, const MAX: usize>(pub T)
where
    T: Copy + Clone + Add + Eq + PartialOrd + Into<usize>;

impl<const MAX: usize> From<u16> for Addr<u16, MAX> {
    fn from(value: u16) -> Self {
        Addr(value & 0x0FFF)
    }
}

impl<const MAX: usize> From<usize> for Addr<u16, MAX> {
    fn from(value: usize) -> Self {
        Addr::from((value & MAX) as u16)
    }
}

impl<const MAX: usize> Into<u16> for Addr<u16, MAX> {
    fn into(self) -> u16 {
        self.0
    }
}

impl<const MAX: usize> Into<usize> for Addr<u16, MAX> {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl<T, const MAX: usize> Add for Addr<T, MAX>
where
    T: Copy + Clone + Add<Output = T> + Eq + PartialOrd + Into<usize>,
{
    type Output = Addr<T, MAX>;
    fn add(self, rhs: Addr<T, MAX>) -> Addr<T, MAX> {
        Addr(self.0 + Addr::from(rhs).0)
    }
}

impl<T, const MAX: usize> Addr<T, MAX>
where
    T: Copy + Clone + Add<Output = T> + Eq + PartialOrd + Into<usize>,
{
    pub fn is_overflow(&self) -> bool {
        self.0.into() >= MAX
    }
}

pub struct Memory<const MAX: usize> {
    bytes: [u8; MAX],
}

impl<const MAX: usize> Memory<MAX> {
    pub fn init() -> Self {
        Self { bytes: [0u8; MAX] }
    }

    pub fn read(&self, addr: &Addr<u16, MAX>) -> u8 {
        self.bytes[<Addr<u16, MAX> as Into<usize>>::into(*addr)]
    }

    pub fn write(&mut self, addr: &Addr<u16, MAX>, byte: u8) {
        self.bytes[<Addr<u16, MAX> as Into<usize>>::into(*addr)] = byte;
    }

    pub fn read_block(&self, start_addr: Addr<u16, MAX>, len: usize) -> &[u8] {
        let start: usize = start_addr.into();
        let stop: usize = (start + len).min(MAX);
        
        &self.bytes[start..stop]
    }

    pub fn write_block(&mut self, start_addr: &Addr<u16, MAX>, data: &[u8]) {
        for i in 0..data.len().min(MAX) {
            self.write(&(*start_addr + Addr::from(i)), data[i]);
        }
    }

    pub fn get_display_block(&self) -> [u8; 256] {
        let mut result = [0u8; 256];
        for (i, &byte) in self.read_block(Addr::<u16, MAX>::from(0xF00usize), 0x100).iter().enumerate() {
            result[i] = byte;
        }

        result
    }
}
