// mod CPU;
// mod Bus;
// mod opcodes;

struct CPU<'a> {
    // byte: u8, word: u16
    a: u8,  // accumulator
    x: u8,  // X register
    y: u8,  // Y register
    sp: u8,   // stack pointer
    pc: u16,   // program counter
    sr: u8,   // status register
    fetched: u8,    // data that has been fetched by fetch()
    addr_abs: u16,  // memory address to read from (absolute)
    addr_rel: u16,  // memory address to read from (relative)
    opcode: u8,     // current opcode
    cycles: u8,     // cycles left to run
    bus: Option<&'a Bus<'a>>,
    lookup: Vec<&'a Instruction<'a>>,
}

impl<'a> CPU<'a> {
    // fn new(a: u8, x: u8, y: u8, sp: u8, pc: u16, sr: u8, fetched: u8, addr_abs: u16, addr_rel: u16, opcode: u8, cycles: u8) -> Self {
    //     return CPU {
    //         a,
    //         x,
    //         y,
    //         sp,
    //         pc,
    //         sr,
    //         fetched,
    //         addr_abs,
    //         addr_rel,
    //         opcode,
    //         cycles,
    //     };
    // }

    fn empty(&self) -> Self {
        return CPU {
            a: 0x00,
            x: 0x00,
            y: 0x00,
            sp: 0x00,
            pc: 0x0000,
            sr: 0x00,
            fetched: 0x00,
            addr_abs: 0x0000,
            addr_rel: 0x00,
            opcode: 0x00,
            cycles: 0,
            bus: None,
            lookup: vec![   // TODO: create entire lookup table
                &Instruction { name: "BRK", operate: Some(self.BRK), addr_mode: &IMM, cycles: 7 }
            ]
        };
    }

    fn connect_bus(&mut self, bus: &'a Bus<'a>) {
        self.bus = Some(bus);
    }

    // Write & read bus

    fn write(&self, bus: &mut Bus, addr: u16, data: u8) {
        bus.write(addr, data);
    }

    fn read(&self, bus: &Bus, addr: u16) -> u8 {
        bus.read(addr, false)
    }

    // Flags

    fn get_flag(&self, f: Flags) -> u8 {
        0
    }

    fn set_flag(&mut self, f: Flags, v: bool) {

    }

    // Addressing modes
    fn IMP() -> u8 {}
    fn ZP0() -> u8 {}
    fn ZPY() -> u8 {}
    fn ABS() -> u8 {}
    fn ABY() -> u8 {}
    fn IZX() -> u8 {}
    fn IMM() -> u8 {}
    fn ZPX() -> u8 {}
    fn REL() -> u8 {}
    fn ABX() -> u8 {}
    fn IND() -> u8 {}
    fn IZY() -> u8 {}

    // Opcodes
    fn ADC() -> u8 {}
    fn AND() -> u8 {}
    fn ASL() -> u8 {}
    fn BBR() -> u8 {}
    fn BBS() -> u8 {}
    fn BCC() -> u8 {}
    fn BCS() -> u8 {}
    fn BEQ() -> u8 {}
    fn BIT() -> u8 {}
    fn BMI() -> u8 {}
    fn BNE() -> u8 {}
    fn BPL() -> u8 {}
    fn BRA() -> u8 {}
    fn BVC() -> u8 {}
    fn BVS() -> u8 {}
    fn CLC() -> u8 {}
    fn CLD() -> u8 {}
    fn CLI() -> u8 {}
    fn CLV() -> u8 {}
    fn CMP() -> u8 {}
    fn CPX() -> u8 {}
    fn CPY() -> u8 {}
    fn DEC() -> u8 {}
    fn DEX() -> u8 {}
    fn EOR() -> u8 {}
    fn INC() -> u8 {}
    fn INX() -> u8 {}
    fn INY() -> u8 {}
    fn JMP() -> u8 {}
    fn JSR() -> u8 {}
    fn LDA() -> u8 {}
    fn LDX() -> u8 {}
    fn LDY() -> u8 {}
    fn LSR() -> u8 {}
    fn NOP() -> u8 {}
    fn ORA() -> u8 {}
    fn PHA() -> u8 {}
    fn PHP() -> u8 {}
    fn PHX() -> u8 {}
    fn PHY() -> u8 {}
    fn PLA() -> u8 {}
    fn PLP() -> u8 {}
    fn PLX() -> u8 {}
    fn PLY() -> u8 {}
    fn RMB() -> u8 {}
    fn ROL() -> u8 {}
    fn ROR() -> u8 {}
    fn RTI() -> u8 {}
    fn RTS() -> u8 {}
    fn SBC() -> u8 {}
    fn SEC() -> u8 {}
    fn SED() -> u8 {}
    fn SEI() -> u8 {}
    fn SMB() -> u8 {}
    fn STA() -> u8 {}
    fn STP() -> u8 {}
    fn STX() -> u8 {}
    fn STY() -> u8 {}
    fn STZ() -> u8 {}
    fn TAX() -> u8 {}
    fn TAY() -> u8 {}
    fn TRB() -> u8 {}
    fn TSB() -> u8 {}
    fn TXA() -> u8 {}
    fn TXS() -> u8 {}
    fn TYA() -> u8 {}
    fn WAI() -> u8 {}
    fn XXX() -> u8 {}

    // Clock
    fn clock(&mut self) {
        if self.cycles == 0 {
            self.opcode = self.read(&self.bus.as_ref().unwrap(), self.pc);
            self.pc += 1;

            // TODO: Get Starting number of cycles
            self.cycles = self.lookup[self.opcode as usize].cycles;

            let additional_cycle1: u8 = (self.lookup[self.opcode as usize].addr_mode.unwrap())();
            let additional_cycle2: u8 = (self.lookup[self.opcode as usize].operate.unwrap())();

            self.cycles += additional_cycle1 & additional_cycle2;
        }

        self.cycles -= 1;
    }

    // Reset
    fn reset(&mut self) {
        self.pc = 0xFFFC;
    }

    // Interrupt request (irq)
    fn irq(&mut self) {

    }

    // Not maskeable interrupt (nmi)
    fn nmi(&mut self) {

    }

    // Fetch data
    fn fetch(&mut self) -> u8 {
        0
    }
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

struct Bus<'a> {
    cpu: Option<&'a CPU<'a>>,
    ram: [u8; 64 * 1024],
}

impl<'a> Bus<'a> {
    fn new(cpu: &'a CPU<'a>) -> Self {
        Bus {
            cpu: Some(cpu),
            ram: [0; 64 * 1024],
        }
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

struct Instruction<'a> {
    name: &'a str,
    operate: Option<&'a fn()>,
    addr_mode: Option<&'a fn()>,   // 
    cycles: u8,
}

impl<'a> Instruction<'a> {
    fn new(name: &str) -> Self {
        Instruction {
            name,
            operate: None,
            addr_mode: None,
            cycles: 0,
        }
    }
}

fn main() {
    println!("Hello world");
    let mut cpu: CPU = CPU::empty();
    let mut bus: Bus = Bus::new(&cpu);
    cpu.connect_bus(&bus);
    cpu.reset();
}
