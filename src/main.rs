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
    // fn new(A: u8, X: u8, Y: u8, SP: u8, PC: u16, SR: u16) -> Self {
    //     return CPU {
    //         A,
    //         X,
    //         Y,
    //         SP,
    //         PC,
    //         SR,
    //     }
    // }

    fn reset(&self) {
        self.PC = 0xFFFC;
    }
}

struct Bus {
    cpu: CPU,
    ram: Vec<u8>,
}

impl Bus {
    fn new(cpu: CPU, ram_size: usize) -> Self {
        return Bus {
            cpu,
            ram: vec![0; ram_size],
        };
    }

    fn write(&self, addr: u16, data: u8) {
        if addr >= 0x0000 && addr <= 0xFFFF {
            self.ram[addr] = data;
        }        
    }

    fn read(addr: u16, read_only: bool) -> u8 {
        return 0;
    }
}

fn main() {
    println!("Hello world");
    let cpu: CPU;
    cpu.reset();
    let bus: Bus;
}
