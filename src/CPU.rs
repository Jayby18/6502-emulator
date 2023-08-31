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
        return self.bus.read(addr, false);
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

    // Clock
    // fn clock(&mut self) {
    //     if self.cycles == 0 {
    //         self.opcode = self.read(self.pc);
    //         self.pc += 1;

    //         // TODO: Get Starting number of cycles
    //         self.cycles = self.lookup[self.opcode as usize].cycles;

    //         let additional_cycle1: u8 = (self.lookup[self.opcode as usize].addr_mode.unwrap())();
    //         let additional_cycle2: u8 = (self.lookup[self.opcode as usize].operate.unwrap())();

    //         self.cycles += additional_cycle1 & additional_cycle2;
    //     }

    //     self.cycles -= 1;
    // }

    // Reset
    pub fn reset(&mut self) {
        println!("Resetting. (PC: {0:2X})", self.pc);
        self.pc = 0xFFFC;
    }

    // Clock
    pub fn clock(&mut self) {
        if self.pc == 0xFFFC {
            let program_start: u16 = u16::from(self.read(self.pc)) << 8 + u16::from(self.read(self.pc + 1));
            self.pc = program_start;
        }

        loop {
            let opcode = self.read(self.pc);
            println!("\nNew clock cycle.");
            println!("PC: {0:2X}", self.pc);
            println!("OP: {0:2X}", opcode);
            self.pc += 1;

            match opcode {
                // 0x00 => self.BRK(),
                0x00 => { return; },
                0xA9 => self.LDA(),
                _ => self.XXX(),
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
impl CPU {
    fn IMP(&mut self) -> u8 {
        self.fetched = self.a;
        return 0;
    }
    fn ZP0(&mut self) -> u8 {
        self.addr_abs = self.read(self.pc).into();
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        return 0;
    }
    fn ABS(&mut self) -> u8 {
        let lo: u16 = self.read(self.pc).into();
        self.pc += 1;
        let hi: u16 = self.read(self.pc).into();
        self.pc += 1;

        self.addr_abs = (hi << 8) | lo;
        return 0;
    }
    fn ABX(&mut self) -> u8 {
        let lo: u16 = self.read(self.pc).into();
        self.pc += 1;
        let hi: u16 = self.read(self.pc).into();
        self.pc += 1;

        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += <u8 as Into<u16>>::into(self.x);

        if (self.addr_abs & 0xFF00) != (hi << 8) {
            return 1;
        } else {
            return 0;
        }
    }
    fn ABY(&mut self) -> u8 {
        let lo: u16 = self.read(self.pc).into();
        self.pc += 1;
        let hi: u16 = self.read(self.pc).into();
        self.pc += 1;

        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += <u8 as Into<u16>>::into(self.x);

        if (self.addr_abs & 0xFF00) != (hi << 8) {
            return 1;
        } else {
            return 0;
        }
    }
    fn IMM(&mut self) -> u8 {
        self.addr_abs = self.pc;    // pc++ in example, does this work?
        self.pc += 1;
        return 0;
    }
    fn ZPX(&mut self) -> u8 {
        self.addr_abs = (self.read(self.pc) + self.x).into();
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        return 0;
    }
    fn ZPY(&mut self) -> u8 {
        self.addr_abs = (self.read(self.pc) + self.y).into();
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        return 0;
    }
    fn REL() -> u8 { 0 }
    // fn IND(&mut self) -> u8 {
    //     let ptr_lo: u16 = self.read(self.pc).into();
    //     self.pc += 1;
    //     let ptr_hi: u16 = self.read(self.pc).into();
    //     self.pc += 1;

    //     let ptr: u16 = (ptr_hi << 8) | ptr_lo;
    
    //     self.addr_abs = ((self.read(ptr + 1) << 8) | self.read(ptr + 0)).into();

    //     return 0;
    // }
    fn IZX() -> u8 {
        0
    }
    fn IZY() -> u8 { 0 }
}

// Instructions
impl CPU {
    fn ADC(&mut self) {}
    fn AND(&mut self) {}
    fn ASL(&mut self) {}
    fn BBR(&mut self) {}
    fn BBS(&mut self) {}
    fn BCC(&mut self) {}
    fn BCS(&mut self) {}
    fn BEQ(&mut self) {}
    fn BIT(&mut self) {}
    fn BMI(&mut self) {}
    fn BNE(&mut self) {}
    fn BPL(&mut self) {}
    fn BRA(&mut self) {}
    fn BRK(&mut self) {
        return
    }
    fn BVC(&mut self) {}
    fn BVS(&mut self) {}
    fn CLC(&mut self) {}
    fn CLD(&mut self) {}
    fn CLI(&mut self) {}
    fn CLV(&mut self) {}
    fn CMP(&mut self) {}
    fn CPX(&mut self) {}
    fn CPY(&mut self) {}
    fn DEC(&mut self) {}
    // TODO: set zero flag when result is 0, negative flag when negative
    fn DEX(&mut self) {
        self.x -= 1;
        if self.x == 0 {
            self.set_flag(Flags::Z, true);
            // TODO: v true or false?
        }
    }
    fn DEY(&mut self) {
        self.y -= 1;
    }
    fn EOR(&mut self) {}

    // Increment value at memory location
    fn INC(&mut self) {}

    // Increment the X register
    fn INX(&mut self) {
        self.x += 1;
        self.set_flag(Flags::Z, self.x == 0x00);
        self.set_flag(Flags::N, (self.x & 0x80) ==  0x80);
    }

    // Increment the Y register
    fn INY(&mut self) {
        self.y += 1;
        self.set_flag(Flags::Z, self.y == 0x80);
        self.set_flag(Flags::N, (self.y & 0x80) == 0x80);
    }

    // Jump to location
    fn JMP(&mut self) {
        self.pc = self.addr_abs;
    }

    // Jump to subroutine (pushing pc to stack before jump so the program can return)
    fn JSR(&mut self) {}

    // Load the accumulator
    fn LDA(&mut self) {
        // self.fetch();
        // self.a = self.fetched;

        // Immediate addressing for now
        let operand = self.read(self.pc);
        self.pc += 1;

        println!("LDA operand: {0:2X}", operand);
        self.a = operand;

        // Set zero and negative flags, depending on operand
        self.set_flag(Flags::Z, self.a == 0x00);
        self.set_flag(Flags::N, (self.a & 0x80) == 0x80);
    }

    // Load the X register
    fn LDX(&mut self) {
        self.fetch();
        self.x = self.fetched;
        self.set_flag(Flags::Z, self.x == 0x00);
        self.set_flag(Flags::N, (self.x & 0x80) == 0x80);
    }

    // Load the Y register
    fn LDY(&mut self, value: u8) {
        self.fetch();
        self.y = self.fetched;
        self.set_flag(Flags::Z, self.y == 0x00);
        self.set_flag(Flags::N, (self.y & 0x80) == 0x80);
    }
    fn LSR() {}
    fn NOP() {}

    // Bitwise OR
    fn ORA(&mut self) {
        self.fetch();
        self.a |= self.fetched;
        self.set_flag(Flags::Z, self.a == 0x00);
        self.set_flag(Flags::N, (self.a & 0x80) == 0x80);
    }

    // Push accumulator to stack
    fn PHA(&mut self) -> u8 {
        self.write(0x0100 + (self.sp as u16), self.a);
        self.sp -= 1;
        return 0;
    }

    // Push status register to stack
    fn PHP(&mut self) -> u8 {
        self.write(0x0100 + (self.sp as u16), self.sr | (Flags::B as u8) | (Flags::U as u8));
        self.set_flag(Flags::B, false);
        self.set_flag(Flags::U, false);
        self.sp -= 1;
        return 0;
    }
    fn PHX(&mut self) {}
    fn PHY(&mut self) {}
    fn PLA(&mut self) {}
    fn PLP(&mut self) {}
    fn PLX(&mut self) {}
    fn PLY(&mut self) {}
    fn RMB(&mut self) {}
    fn ROL(&mut self) {}
    fn ROR(&mut self) {}
    fn RTI(&mut self) {}
    fn RTS(&mut self) {}
    fn SBC(&mut self) {}
    fn SEC(&mut self) {}
    fn SED(&mut self) {}
    fn SEI(&mut self) {}
    fn SMB(&mut self) {}

    // Store accumulator at address
    fn STA(&mut self) {
        self.write(self.addr_abs, self.a);
    }
    fn STP(&mut self) {}
    fn STX(&mut self) {}
    fn STY(&mut self) {}
    fn STZ(&mut self) {}

    // TODO: set Z and N flags in transfer functions
    // Transfer accumulator to X register
    fn TAX(&mut self) {
        self.x = self.a;
    }

    // Transfer accumulator to Y register
    fn TAY(&mut self) {
        self.y = self.a;
    }

    fn TRB(&mut self) {}

    // Transfer stack pointer to X register
    fn TSX(&mut self) {
        self.x = self.sp;
    }

    fn TSB() {}

    // Transfer X register to accumulator
    fn TXA(&mut self) {
        self.a = self.x;
    }

    // Transfer X register to stack pointer
    fn TXS(&mut self) {
        self.sp = self.x;
    }

    // Transfer Y register to accumulator
    fn TYA(&mut self) {
        self.a = self.y;
    }
    fn WAI(&mut self) {}

    // When an illegal opcode is passed, XXX() is run
    fn XXX(&mut self) {
        panic!("Illegal operation!");
    }
}

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
