struct CPU {
    // byte: u8, word: u16
    a: u8,  // accumulator
    x: u8,  // X register
    y: u8,  // Y register
    sp: u8,   // stack pointer
    pc: u16,   // program counter
    sr: u8,   // status register
    fetched: u8,
    addr_abs: u16,
    addr_rel: u16,
    opcode: u8,
    cycles: u8,
    bus: Option<Bus>,
}

impl CPU {
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

    fn empty() -> Self {
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
        };
    }

    fn connect_bus(&mut self, bus: Bus) {
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

    // Opcodes

    // Clock
    fn clock(&mut self) {
        if self.cycles == 0 {
            self.opcode = self.read(&self.bus.as_ref().unwrap(), self.pc);
            self.pc += 1;
        }
    }

    // Reset
    fn reset(&mut self) {
        self.pc = 0xFFFC;
    }

    // Interrupt request (irq)
    fn irq() {

    }

    // Not maskeable interrupt (nmi)
    fn nmi() {

    }

    // Fetch data
    fn fetch() -> u8 {
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