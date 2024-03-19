use std::ops::Add;

const MEM_SIZE: u16 = 0x1000;

#[derive(Copy, Clone)]
pub struct Addr<T, const MAX: usize>(pub T)
where T: Copy + Clone + Add + Eq + PartialOrd + Into<usize>;

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
where T: Copy + Clone + Add<Output = T> + Eq + PartialOrd + Into<usize> {
    type Output = Addr<T, MAX>;
    fn add(self, rhs: Addr<T, MAX>) -> Addr<T, MAX> {
        Addr(self.0 + Addr::from(rhs).0)
    }
}

impl<T, const MAX: usize> Addr<T, MAX> 
where T: Copy + Clone + Add<Output = T> + Eq + PartialOrd + Into<usize> {
    pub fn is_overflow(&self) -> bool {
        self.0.into() >= MAX
    }
}

pub struct Memory<const MAX: usize> {
    bytes: [u8; MAX]
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

    pub fn read_block<const Len: usize>(&self, start_addr: &Addr<u16, MAX>) -> [u8; Len] {
        let mut output: [u8; Len] = [0u8; Len];
        for i in 0..Len {
            let bounds_checked: usize = Addr::<u16, MAX>::from(i).into();
            output[bounds_checked] = self.read(&(*start_addr + Addr::<_, MAX>::from(i as u16)));
        }

        output
    }

    pub fn write_block<const Len: usize>(&mut self, start_addr: &Addr<u16, MAX>, data: [u8; Len]) {
        for i in 0..Len {
            self.write(&(*start_addr + Addr::from(i)), data[i]);
        }
    }

    pub fn get_display_block(&self) -> [u8; 256] {
        return self.read_block(start_block, 0xFF).
    }
}
