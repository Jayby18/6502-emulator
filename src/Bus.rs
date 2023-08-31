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
        if addr >= 0x0000 && addr <= 0xFFFF {
            self.ram[addr as usize] = data;
        } else {
            panic!("Memory access out of bounds. RAM can only access between 0x0000 and 0xFFFF.");
        }
    }

    // Read from RAM at addr
    pub fn read(&mut self, addr: u16, read_only: bool) -> u8 {
        if addr >= 0x0000 && addr <= 0xFFFF {
            return self.ram[addr as usize];
        } else {
            panic!("Memory access out of bounds. RAM can only access between 0x0000 and 0xFFFF.");
        }
    }
}