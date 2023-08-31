pub struct Bus {
    ram: [u8; 64 * 1024],
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            ram: [0; 64 * 1024],
        }
    }

    // Write data to addr in RAM
    pub fn write(&mut self, addr: u16, data: u8) {
        self.ram[addr as usize] = data;
    }

    // Read from RAM at addr
    pub fn read(&mut self, addr: u16) -> u8 {
        return self.ram[addr as usize];
    }

    // Write u16 data (little endian)
    pub fn write_u16(&mut self, addr: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.write(addr, lo);
        self.write(addr + 1, hi);
    }

    // Read u16 data (little endian)
    pub fn read_u16(&mut self, addr: u16) -> u16 {
        let lo = self.read(addr) as u16;
        let hi = self.read(addr + 1) as u16;
        return (hi << 8) | (lo as u16);
    }
}