struct CPU {
    // byte: u8, word: u16
    A: u8,  // accumulator
    X: u8,  // X register
    Y: u8,  // Y register
    SP: u8,   // stack pointer
    PC: u16,   // program counter
    SR: u8,   // status register
}

impl CPU {
    fn new(A: u8, X: u8, Y: u8, SP: u8, PC: u16, SR: u8) -> Self {
        return CPU {
            A,
            X,
            Y,
            SP,
            PC,
            SR,
        };
    }

    fn connect_bus(n: &Bus) {
        // 
    }

    // Write & read bus

    fn write(bus: &mut Bus, addr: u16, data: u8) {
        bus.write(addr, data);
    }

    fn read(bus: &Bus, addr: u16) -> u8 {
        bus.read(addr, false)
    }

    // Flags

    fn get_flag(f: Flags) -> u8 {

    }

    fn set_flag(f: Flags, v: bool) {

    }

    // Addressing modes

    // Opcodes

    // Clock
    fn clock() {

    }

    // Reset
    fn reset(&mut self) {
        self.PC = 0xFFFC;
    }

    // Interrupt request (irq)

    // Not maskeable interrupt (nmi)

    // Fetch data
    fn fetch() -> u8 {
        0
    }

    let mut fetched: u8 = 0x00;
    let mut addr_abs: u16 = 0x000;
    let mut addr_rel: u16 = 0x00;
    let mut opcode: u8 = 0x00;
    let mut cycles: u8 = 0;
}

enum Flags {
    C = 0b0000_0001,    // carry
    Z = 0b0000_0010,    // zero
    I = 0b0000_0100,    // disable interrupt
    D = 0b0000_1000,    // decimal
    B = 0b0001_0000,    // break
    U = 0b0010_0000,    // unused
    V = 0b0100_0000,    // overflow
    N = 0b1000_0000,    // negative
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
    let mut cpu: CPU = CPU::new(0x00, 0x00, 0x00, 0x00, 0x0000, 0x00);
    cpu.reset();
    let mut bus: Bus = Bus::new(cpu);
}
