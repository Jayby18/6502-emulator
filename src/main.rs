struct CPU {
    // byte: u8, word: u16
    A: u8,  // accumulator
    X: u8,  // X register
    Y: u8,  // Y register
    SP: u8,   // stack pointer
    PC: u16,   // program counter
    SR: u16,   // status register
}

impl CPU {
    fn new(A: u8, X: u8, Y: u8, SP: u8, PC: u16, SR: u16) -> Self {
        return CPU {
            A,
            X,
            Y,
            SP,
            PC,
            SR,
        };
    }

    fn reset(&mut self) {
        self.PC = 0xFFFC;
    }
}

struct Bus {
    cpu: CPU,
    ram: [u8; 64 * 1024],
}

impl Bus {
    fn new(cpu: CPU) -> Self {
        return Bus {
            cpu,
            ram: [0; 64 * 1024],
        };
    }

    // Write data to addr in RAM
    fn write(&mut self, addr: u16, data: u8) {
        if addr >= 0x0000 && addr <= 0xFFFF {
            self.ram[addr as usize] = data;
        } else {
            panic!("Memory access out of bounds. RAM can only access between 0x0000 and 0xFFFF.");
        }
    }

    // Read from RAM at addr
    fn read(&self, addr: u16, read_only: bool) -> u8 {
        if addr >= 0x0000 && addr <= 0xFFFF {
            return self.ram[addr as usize];
        } else {
            panic!("Memory access out of bounds. RAM can only access between 0x0000 and 0xFFFF.");
        }
    }
}

fn main() {
    println!("Hello world");
    let mut cpu: CPU = CPU::new(0x00, 0x00, 0x00, 0x00, 0x0000, 0x0000);
    cpu.reset();
    let mut bus: Bus = Bus::new(cpu);
}
