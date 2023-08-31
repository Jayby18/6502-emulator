use crate::bus::Bus;

pub struct CPU {
    // byte: u8, word: u16
    a: u8,          // accumulator
    x: u8,          // X register
    y: u8,          // Y register
    sp: u8,         // stack pointer
    pc: u16,        // program counter
    sr: u8,         // status register
    fetched: u8,    // data that has been fetched by fetch()
    addr_abs: u16,  // memory address to read from (absolute)
    addr_rel: u16,  // memory address to read from (relative)
    opcode: u8,     // current opcode
    cycles: u8,     // cycles left to run
    bus: Bus,      // memory bus
}

impl CPU {
    pub fn new(bus: Bus) -> Self {
        let cpu = CPU {
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
            bus,
        };

        return cpu;
    }

    // Write & read bus
    pub fn write(&mut self, addr: u16, data: u8) {
        self.bus.write(addr, data);
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        return self.bus.read(addr);
    }

    pub fn write_u16(&mut self, addr: u16, data: u16) {
        self.bus.write_u16(addr, data);
    }

    pub fn read_u16(&mut self, addr: u16) -> u16 {
        return self.bus.read_u16(addr);
    }

    // Flags
    pub fn get_flag(&mut self, f: Flags) -> bool {
        // Return value of status register corresponding to flag f
        if self.sr & (f as u8) != 0 {
            return true;
        } else {
            return false;
        }
    }

    pub fn set_flag(&mut self, f: Flags, v: bool) {
        // TODO: what is v again?
        if v {
            // set flags using bitwise OR
            // if current flag is 0110 and you pass 0001, it becomes 0111
            self.sr |= (f as u8);
        } else {
            // set flags using ...
            self.sr &= !(f as u8);
        }
    }
    
    pub fn get_a_reg(&self) -> u8 {
        return self.a;
    }

    pub fn get_status(&self) -> u8 {
        return self.sr;
    }
    // Reset
    pub fn reset(&mut self) {
        println!("Resetting. (PC: {:02X})", self.pc);
        self.pc = 0xFFFC;

        // Reset all registers (except program counter)
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0;
        self.sr = 0;
        self.fetched = 0;
        self.addr_abs = 0;
        self.addr_rel = 0;
        self.opcode = 0;
        self.cycles = 0;
    }

    // Clock
    pub fn clock(&mut self) {
        if self.pc == 0xFFFC {
            let program_start: u16 = self.read_u16(self.pc);
            self.pc = program_start;
        }

        loop {
            let opcode = self.read(self.pc);
            println!("\nNew clock cycle.");
            println!("PC: {}", self.pc);
            println!("OP: {}", opcode);
            self.pc += 1;

            // TODO: finish opcode matrix (https://i.redd.it/m23p0jhvfwx81.jpg, ignore greyed boxes)
            match opcode {
                0x00 => self.BRK(AddressingMode::IMP), 0x01 => self.ORA(AddressingMode::ZPX), 0x05 => self.ORA(AddressingMode::ZP0), 0x06 => self.ASL(AddressingMode::ZP0), 0x08 => self.PHP(AddressingMode::IMP), 0x09 => self.ORA(AddressingMode::IMM), 0x0A => self.ASL(AddressingMode::ACC), 0x0D => self.ORA(AddressingMode::ABS), 0x0E => self.ASL(AddressingMode::ABS),
                0x10 => self.BPL(AddressingMode::REL), 0x11 => self.ORA(AddressingMode::ZPY), 0x15 => self.ORA(AddressingMode::ZPX), 0x16 => self.ASL(AddressingMode::ZPX), 0x18 => self.CLC(AddressingMode::IMP), 0x1D => self.ORA(AddressingMode::ABX), 0x1E => self.ASL(AddressingMode::ABX),
                0x20 => self.JSR(AddressingMode::ABS), 0x21 => self.AND(AddressingMode::ZPX), 0x24 => self.BIT(AddressingMode::ZP0), 0x25 => self.AND(AddressingMode::ZP0), 0x26 => self.ROL(AddressingMode::ZP0), 0x28 => self.PLP(AddressingMode::IMP), 0x29 => self.AND(AddressingMode::IMM), 0x2A => self.ROL(AddressingMode::ACC), 0x2C => self.BIT(AddressingMode::ABS), 0x2D => self.AND(AddressingMode::ABS), 0x2E => self.ROL(AddressingMode::ABS),
                0x30 => self.BMI(AddressingMode::REL),
                0x40 => self.RTI(AddressingMode::IMP),
                0x50 => self.BVC(AddressingMode::REL),
                0x60 => self.RTS(AddressingMode::IMP),
                0x70 => self.BVS(AddressingMode::REL),
                0x80 => self.NOP(AddressingMode::IMM),
                0x90 => self.BCC(AddressingMode::REL),
                0xA0 => self.LDY(AddressingMode::IMM), 0xA1 => self.LDA(AddressingMode::ZPX), 0xA2 => self.LDX(AddressingMode::IMM), 0xA4 => self.LDY(AddressingMode::ZP0), 0xA5 => self.LDA(AddressingMode::ZP0), 0xA6 => self.LDX(AddressingMode::ZP0), 0xA8 => self.TAY(AddressingMode::IMP), 0xA9 => self.LDA(AddressingMode::IMM), 0xAA => self.TAX(AddressingMode::IMP), 0xAC => self.LDY(AddressingMode::ABS), 0xAD => self.LDA(AddressingMode::ABS), 0xAE => self.LDX(AddressingMode::ABS),
                _ => self.XXX(AddressingMode::IMP),
            }
        }
    }

    // Interrupt request (irq)
    fn irq(&mut self) {

    }

    // Not maskeable interrupt (nmi)
    fn nmi(&mut self) {

    }

    // TODO: Fetch data
    fn fetch(&mut self) -> u8 {
        0
    }
}

// Addressing modes
enum AddressingMode {
    IMM,
    IMP,
    ZP0,
    ZPX,
    ZPY,
    ABS,
    ABX,
    ABY,
    IND,
    IDX,
    IDY,
    REL,
    ACC,
}

impl CPU {
    fn get_address(&mut self, mode: AddressingMode) -> u16 {
        match mode {
            AddressingMode::IMM => {
                let addr = self.pc;
                self.pc += 1;
                return addr;
            },
            AddressingMode::IMP => {
                panic!("not supported");
            },
            AddressingMode::ZP0 => {
                let addr = self.read(self.pc) as u16;
                self.pc += 1;
                return addr;
            },
            AddressingMode::ZPX => {
                let addr = self.read(self.pc).wrapping_add(self.x) as u16;
                self.pc += 1;
                return addr;
            },
            AddressingMode::ZPY => {
                let addr = self.read(self.pc).wrapping_add(self.y) as u16;
                self.pc += 1;
                return addr;
            },
            AddressingMode::ABS => {
                let addr = self.read_u16(self.pc);
                self.pc += 2;
                return addr;
            },
            AddressingMode::ABX => {
                let addr = self.read_u16(self.pc).wrapping_add(self.x as u16);
                self.pc += 2;
                return addr;
            },
            AddressingMode::ABY => {
                let addr = self.read_u16(self.pc).wrapping_add(self.y as u16);
                self.pc += 2;
                return addr;
            },
            AddressingMode::IND => todo!(),
            AddressingMode::IDX => {
                let base = self.read(self.pc);

                let ptr: u8 = (base as u8).wrapping_add(self.x);
                let lo = self.read(ptr as u16);
                let hi = self.read(ptr.wrapping_add(1) as u16);

                self.pc += 2;
                return (hi as u16) << 8 | (lo as u16);
            },
            AddressingMode::IDY => {
                let base = self.read(self.pc);

                let ptr: u8 = (base as u8).wrapping_add(self.y);
                let lo = self.read(ptr as u16);
                let hi = self.read(ptr.wrapping_add(1) as u16);

                self.pc += 2;
                return (hi as u16) << 8 | (lo as u16);
            },
            AddressingMode::REL => todo!(),
            AddressingMode::ACC => todo!(),
        }
    }
}

// Instructions
impl CPU {
    fn ADC(&mut self, mode: AddressingMode) {}
    fn AND(&mut self, mode: AddressingMode) {}
    fn ASL(&mut self, mode: AddressingMode) {}
    fn BBR(&mut self, mode: AddressingMode) {}
    fn BBS(&mut self, mode: AddressingMode) {}
    fn BCC(&mut self, mode: AddressingMode) {}
    fn BCS(&mut self, mode: AddressingMode) {}
    fn BEQ(&mut self, mode: AddressingMode) {}
    fn BIT(&mut self, mode: AddressingMode) {}
    fn BMI(&mut self, mode: AddressingMode) {}
    fn BNE(&mut self, mode: AddressingMode) {}
    fn BPL(&mut self, mode: AddressingMode) {}
    fn BRA(&mut self, mode: AddressingMode) {}
    fn BRK(&mut self, mode: AddressingMode) {
        return
    }
    fn BVC(&mut self, mode: AddressingMode) {}
    fn BVS(&mut self, mode: AddressingMode) {}
    fn CLC(&mut self, mode: AddressingMode) {}
    fn CLD(&mut self, mode: AddressingMode) {}
    fn CLI(&mut self, mode: AddressingMode) {}
    fn CLV(&mut self, mode: AddressingMode) {}
    fn CMP(&mut self, mode: AddressingMode) {}
    fn CPX(&mut self, mode: AddressingMode) {}
    fn CPY(&mut self, mode: AddressingMode) {}
    fn DEC(&mut self, mode: AddressingMode) {}
    // TODO: set zero flag when result is 0, negative flag when negative
    fn DEX(&mut self, mode: AddressingMode) {
        self.x -= 1;
        if self.x == 0 {
            self.set_flag(Flags::Z, true);
            // TODO: v true or false?
        }
    }
    fn DEY(&mut self, mode: AddressingMode) {
        self.y -= 1;
    }
    fn EOR(&mut self, mode: AddressingMode) {}

    // Increment value at memory location
    fn INC(&mut self, mode: AddressingMode) {}

    // Increment the X register
    fn INX(&mut self, mode: AddressingMode) {
        self.x += 1;
        self.set_flag(Flags::Z, self.x == 0x00);
        self.set_flag(Flags::N, (self.x & 0x80) ==  0x80);
    }

    // Increment the Y register
    fn INY(&mut self, mode: AddressingMode) {
        self.y += 1;
        self.set_flag(Flags::Z, self.y == 0x80);
        self.set_flag(Flags::N, (self.y & 0x80) == 0x80);
    }

    // Jump to location
    fn JMP(&mut self, mode: AddressingMode) {
        self.pc = self.addr_abs;
    }

    // Jump to subroutine (pushing pc to stack before jump so the program can return)
    fn JSR(&mut self, mode: AddressingMode) {}

    // Load the accumulator
    fn LDA(&mut self, mode: AddressingMode) {
        let addr = self.get_address(mode);
        self.a = self.read(addr);
        self.set_flag(Flags::Z, self.a == 0x00);
        self.set_flag(Flags::N, (self.a & 0x80) == 0x80);
    }

    // Load the X register
    fn LDX(&mut self, mode: AddressingMode) {
        let addr = self.get_address(mode);
        self.x = self.read(addr);
        self.set_flag(Flags::Z, self.x == 0x00);
        self.set_flag(Flags::N, (self.x & 0x80) == 0x80);
    }

    // Load the Y register
    fn LDY(&mut self, mode: AddressingMode) {
        let addr = self.get_address(mode);
        self.y = self.read(addr);
        self.set_flag(Flags::Z, self.y == 0x00);
        self.set_flag(Flags::N, (self.y & 0x80) == 0x80);
    }
    fn LSR(&mut self, mode: AddressingMode) {}
    fn NOP(&mut self, mode: AddressingMode) {}

    // Bitwise OR
    fn ORA(&mut self, mode: AddressingMode) {
        let addr = self.get_address(mode);
        self.a |= self.read(addr);
        self.set_flag(Flags::Z, self.a == 0x00);
        self.set_flag(Flags::N, (self.a & 0x80) == 0x80);
    }

    // Push accumulator to stack
    fn PHA(&mut self, mode: AddressingMode) -> u8 {
        self.write(0x0100 + (self.sp as u16), self.a);
        self.sp -= 1;
        return 0;
    }

    // Push status register to stack
    fn PHP(&mut self, mode: AddressingMode) -> u8 {
        self.write(0x0100 + (self.sp as u16), self.sr | (Flags::B as u8) | (Flags::U as u8));
        self.set_flag(Flags::B, false);
        self.set_flag(Flags::U, false);
        self.sp -= 1;
        return 0;
    }
    fn PHX(&mut self, mode: AddressingMode) {}
    fn PHY(&mut self, mode: AddressingMode) {}
    fn PLA(&mut self, mode: AddressingMode) {}
    fn PLP(&mut self, mode: AddressingMode) {}
    fn PLX(&mut self, mode: AddressingMode) {}
    fn PLY(&mut self, mode: AddressingMode) {}
    fn RMB(&mut self, mode: AddressingMode) {}
    fn ROL(&mut self, mode: AddressingMode) {}
    fn ROR(&mut self, mode: AddressingMode) {}
    fn RTI(&mut self, mode: AddressingMode) {}
    fn RTS(&mut self, mode: AddressingMode) {}
    fn SBC(&mut self, mode: AddressingMode) {}
    fn SEC(&mut self, mode: AddressingMode) {}
    fn SED(&mut self, mode: AddressingMode) {}
    fn SEI(&mut self, mode: AddressingMode) {}
    fn SMB(&mut self, mode: AddressingMode) {}

    // Store accumulator at address
    fn STA(&mut self, mode: AddressingMode) {
        let addr = self.get_address(mode);
        self.write(self.addr_abs, self.a);
    }
    fn STP(&mut self, mode: AddressingMode) {}
    fn STX(&mut self, mode: AddressingMode) {}
    fn STY(&mut self, mode: AddressingMode) {}
    fn STZ(&mut self, mode: AddressingMode) {}

    // TODO: set Z and N flags in transfer functions
    // Transfer accumulator to X register
    fn TAX(&mut self, mode: AddressingMode) {
        self.x = self.a;
    }

    // Transfer accumulator to Y register
    fn TAY(&mut self, mode: AddressingMode) {
        self.y = self.a;
    }

    fn TRB(&mut self, mode: AddressingMode) {}

    // Transfer stack pointer to X register
    fn TSX(&mut self, mode: AddressingMode) {
        self.x = self.sp;
    }

    fn TSB(&mut self, mode: AddressingMode) {}

    // Transfer X register to accumulator
    fn TXA(&mut self, mode: AddressingMode) {
        self.a = self.x;
    }

    // Transfer X register to stack pointer
    fn TXS(&mut self, mode: AddressingMode) {
        self.sp = self.x;
    }

    // Transfer Y register to accumulator
    fn TYA(&mut self, mode: AddressingMode) {
        self.a = self.y;
    }
    fn WAI(&mut self, mode: AddressingMode) {}

    // When an illegal opcode is passed, XXX() is run
    fn XXX(&mut self, mode: AddressingMode) {
        panic!("Illegal operation!");
    }
}

// Flags
pub enum Flags {
    C = 0b0000_0001,    // carry
    Z = 0b0000_0010,    // zero
    I = 0b0000_0100,    // disable interrupt
    D = 0b0000_1000,    // decimal  (unused for now)
    B = 0b0001_0000,    // break
    U = 0b0010_0000,    // unused
    V = 0b0100_0000,    // overflow
    N = 0b1000_0000,    // negative
}
