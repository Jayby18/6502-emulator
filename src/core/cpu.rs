use crate::core::bus::Bus;

#[allow(dead_code)]
const NMI_VECTOR: u16 = 0xFFFA;
const RESET_VECTOR: u16 = 0xFFFC;
const IRQ_VECTOR: u16 = 0xFFFE;

#[allow(clippy::upper_case_acronyms)]
pub struct CPU {
    a: u8,          // accumulator
    x: u8,          // X register
    y: u8,          // Y register
    sp: u8,         // stack pointer
    pc: u16,        // program counter
    sr: u8,         // status register
    opcode: u8,     // current opcode
    bus: Bus,       // memory bus
}

/// Implement CPU's core functionality
impl CPU {
    pub fn new(bus: Bus) -> Self {
        CPU {
            a: 0x00,
            x: 0x00,
            y: 0x00,
            sp: 0xFF,
            pc: 0x0000,
            sr: 0x00,
            opcode: 0x00,
            bus,
        }
    }

    /// Write `u8` value to `u16` address
    pub fn write(&mut self, addr: u16, data: u8) {
        self.bus.write(addr, data);
    }

    /// Read `u8` value from `u16` address
    pub fn read(&mut self, addr: u16) -> u8 {
        self.bus.read(addr)
    }

    /// Write `u16` data from `u16` address (little endian)
    pub fn write_u16(&mut self, addr: u16, data: u16) {
        self.bus.write_u16(addr, data);
    }

    /// Read `u16` data from `u16` address (little endian)
    pub fn read_u16(&mut self, addr: u16) -> u16 {
        self.bus.read_u16(addr)
    }

    /// Push `u8` value to stack
    pub fn push(&mut self, data: u8) {
        self.sp -= 0x01;
        self.write(0x0100 + (self.sp as u16), data);
    }

    /// Push `u16` value to stack
    pub fn push_u16(&mut self, data: u16) {
        let lo = data as u8;
        let hi = (data >> 8) as u8;
        self.push(lo);
        self.push(hi);
    }

    /// Pop `u8` value off stack
    pub fn pop(&mut self) -> u8 {
        let data = self.read(0x0100 + (self.sp as u16));
        self.sp += 1;
        data
    }

    /// Pop `u16` value off stack (little endian)
    pub fn pop_u16(&mut self) -> u16 {
        let lo = self.pop() as u16;
        let hi = self.pop() as u16;
        (hi << 8) | lo
    }

    // Flags
    /// Return whether status register has specific `Flag`
    pub fn get_flag(&mut self, f: Flags) -> bool {
        // Return value of status register corresponding to flag f
        self.sr & (f as u8) != 0
    }

    /// Set `Flag` according to boolean
    pub fn set_flag(&mut self, f: Flags, v: bool) {
        if v {
            // set flags using bitwise OR
            // if current flag is 0110 and you pass 0001, it becomes 0111
            self.sr |= f as u8;
        } else {
            // set flags using XOR
            self.sr &= !(f as u8);
        }
    }

    /// Set zero (`Z`) and negative (`N`) flags according to `u8` value
    pub fn set_zero_negative_flags(&mut self, value: u8) {
        self.set_flag(Flags::Z, value == 0x00);
        self.set_flag(Flags::N, (value & 0x80) == 0x80);
    }
    
    /// Reset CPU
    pub fn reset(&mut self) {
        self.pc = RESET_VECTOR;

        // Reset all registers (except program counter)
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0xFF;
        self.sr = 0;
        self.opcode = 0;
    }

    /// Start clock loop
    pub fn clock(&mut self) {
        // CPU starts from the 16-bit reset vector at `0xFFFC` (`RESET_VECTOR`)
        if self.pc == RESET_VECTOR {
            let program_start: u16 = self.read_u16(self.pc);
            self.pc = program_start;
        }

        loop {
            self.opcode = self.read(self.pc);
            self.pc += 1;

            if self.opcode == 0x00 { break }

            match self.opcode {
                0x00 => self.BRK(AddressingMode::IMP), 0x01 => self.ORA(AddressingMode::ZPX), 0x05 => self.ORA(AddressingMode::ZP0), 0x06 => self.ASL(AddressingMode::ZP0), 0x08 => self.PHP(AddressingMode::IMP), 0x09 => self.ORA(AddressingMode::IMM), 0x0A => self.ASL(AddressingMode::ACC), 0x0D => self.ORA(AddressingMode::ABS), 0x0E => self.ASL(AddressingMode::ABS),
                0x10 => self.BPL(AddressingMode::REL), 0x11 => self.ORA(AddressingMode::ZPY), 0x15 => self.ORA(AddressingMode::ZPX), 0x16 => self.ASL(AddressingMode::ZPX), 0x18 => self.CLC(AddressingMode::IMP), 0x1D => self.ORA(AddressingMode::ABX), 0x1E => self.ASL(AddressingMode::ABX),
                0x20 => self.JSR(AddressingMode::ABS), 0x21 => self.AND(AddressingMode::ZPX), 0x24 => self.BIT(AddressingMode::ZP0), 0x25 => self.AND(AddressingMode::ZP0), 0x26 => self.ROL(AddressingMode::ZP0), 0x28 => self.PLP(AddressingMode::IMP), 0x29 => self.AND(AddressingMode::IMM), 0x2A => self.ROL(AddressingMode::ACC), 0x2C => self.BIT(AddressingMode::ABS), 0x2D => self.AND(AddressingMode::ABS), 0x2E => self.ROL(AddressingMode::ABS),
                0x30 => self.BMI(AddressingMode::REL), 0x31 => self.AND(AddressingMode::ZPX), 0x35 => self.AND(AddressingMode::ZPX), 0x36 => self.ROL(AddressingMode::ZPX), 0x38 => self.SEC(AddressingMode::IMP), 0x39 => self.AND(AddressingMode::ABY), 0x3D => self.AND(AddressingMode::ABX), 0x3E => self.ROL(AddressingMode::ABX),
                0x40 => self.RTI(AddressingMode::IMP), 0x41 => self.EOR(AddressingMode::ZPX), 0x45 => self.EOR(AddressingMode::ZP0), 0x46 => self.LSR(AddressingMode::ZP0), 0x48 => self.PHA(AddressingMode::IMP), 0x49 => self.EOR(AddressingMode::IMM), 0x4A => self.LSR(AddressingMode::ACC), 0x4C => self.JMP(AddressingMode::ABS), 0x4D => self.EOR(AddressingMode::ABS), 0x4E => self.LSR(AddressingMode::ABS),
                0x50 => self.BVC(AddressingMode::REL), 0x51 => self.EOR(AddressingMode::ZPY), 0x55 => self.EOR(AddressingMode::ZPY), 0x56 => self.LSR(AddressingMode::ZPX), 0x58 => self.CLI(AddressingMode::IMP), 0x59 => self.EOR(AddressingMode::ABY), 0x5D => self.EOR(AddressingMode::ABX), 0x5E => self.LSR(AddressingMode::ABX),
                0x60 => self.RTS(AddressingMode::IMP), 0x61 => self.ADC(AddressingMode::ZPX), 0x65 => self.ADC(AddressingMode::ZP0), 0x66 => self.ROR(AddressingMode::ZP0), 0x68 => self.PLA(AddressingMode::IMP), 0x69 => self.ADC(AddressingMode::IMM), 0x6A => self.ROR(AddressingMode::ACC), 0x6C => self.JMP(AddressingMode::IND), 0x6D => self.ADC(AddressingMode::ABS), 0x6E => self.ROR(AddressingMode::ABS),
                0x70 => self.BVS(AddressingMode::REL), 0x71 => self.ADC(AddressingMode::ZPY), 0x75 => self.ADC(AddressingMode::ZPX), 0x78 => self.SEI(AddressingMode::IMP), 0x79 => self.ADC(AddressingMode::ABY), 0x7D => self.ADC(AddressingMode::ABX), 0x7E => self.ROR(AddressingMode::ABX),
                0x80 => self.NOP(AddressingMode::IMM), 0x81 => self.STA(AddressingMode::ZPX), 0x84 => self.STY(AddressingMode::ZP0), 0x85 => self.STA(AddressingMode::ZP0), 0x86 => self.STX(AddressingMode::ZP0), 0x88 => self.DEY(AddressingMode::IMP), 0x8A => self.TXA(AddressingMode::IMP), 0x8C => self.STY(AddressingMode::ABS), 0x8D => self.STA(AddressingMode::ABS), 0x8E => self.STX(AddressingMode::ABS),
                0x90 => self.BCC(AddressingMode::REL), 0x91 => self.STA(AddressingMode::IDY), 0x94 => self.STY(AddressingMode::ZPX), 0x95 => self.STA(AddressingMode::ZPX), 0x96 => self.STX(AddressingMode::ZPY), 0x98 => self.TYA(AddressingMode::IMP), 0x99 => self.STA(AddressingMode::ABY), 0x9A => self.TXS(AddressingMode::IMP), 0x9D => self.STA(AddressingMode::ABX),
                0xA0 => self.LDY(AddressingMode::IMM), 0xA1 => self.LDA(AddressingMode::ZPX), 0xA2 => self.LDX(AddressingMode::IMM), 0xA4 => self.LDY(AddressingMode::ZP0), 0xA5 => self.LDA(AddressingMode::ZP0), 0xA6 => self.LDX(AddressingMode::ZP0), 0xA8 => self.TAY(AddressingMode::IMP), 0xA9 => self.LDA(AddressingMode::IMM), 0xAA => self.TAX(AddressingMode::IMP), 0xAC => self.LDY(AddressingMode::ABS), 0xAD => self.LDA(AddressingMode::ABS), 0xAE => self.LDX(AddressingMode::ABS),
                0xB0 => self.BCS(AddressingMode::REL), 0xB1 => self.LDA(AddressingMode::ZPY), 0xB4 => self.LDY(AddressingMode::ZPX), 0xB5 => self.LDA(AddressingMode::ZPX), 0xB6 => self.LDX(AddressingMode::ZPY), 0xB8 => self.CLV(AddressingMode::IMP), 0xB9 => self.LDA(AddressingMode::ABY), 0xBA => self.TSX(AddressingMode::IMP), 0xBC => self.LDY(AddressingMode::ABX), 0xBD => self.LDA(AddressingMode::ABX), 0xBE => self.LDX(AddressingMode::ABY),
                0xC0 => self.CPY(AddressingMode::IMM), 0xC1 => self.CMP(AddressingMode::ZPX), 0xC4 => self.CPY(AddressingMode::ZP0), 0xC5 => self.CMP(AddressingMode::ZP0), 0xC6 => self.DEC(AddressingMode::ZP0), 0xC8 => self.CLV(AddressingMode::IMP), 0xC9 => self.CMP(AddressingMode::IMM), 0xCA => self.DEX(AddressingMode::IMP), 0xCC => self.CPY(AddressingMode::ABS), 0xCD => self.CMP(AddressingMode::ABS), 0xCE => self.DEC(AddressingMode::ABS),
                0xD0 => self.BNE(AddressingMode::REL), 0xD1 => self.CMP(AddressingMode::ZPY), 0xD5 => self.CMP(AddressingMode::ZPX), 0xD6 => self.DEC(AddressingMode::ZPX), 0xD8 => self.CLD(AddressingMode::IMP), 0xD9 => self.CMP(AddressingMode::ABY), 0xDD => self.CMP(AddressingMode::ABX), 0xDE => self.DEC(AddressingMode::ABX),
                0xE0 => self.CPX(AddressingMode::IMM), 0xE1 => self.SBC(AddressingMode::ZPX), 0xE4 => self.CPX(AddressingMode::ZP0), 0xE5 => self.SBC(AddressingMode::ZP0), 0xE6 => self.INC(AddressingMode::ZP0), 0xE8 => self.INX(AddressingMode::IMP), 0xE9 => self.SBC(AddressingMode::IMM), 0xEA => self.NOP(AddressingMode::IMP), 0xEC => self.CPX(AddressingMode::ABS), 0xED => self.SBC(AddressingMode::ABS), 0xEE => self.INC(AddressingMode::ABS),
                0xF0 => self.BEQ(AddressingMode::REL), 0xF1 => self.SBC(AddressingMode::ZPY), 0xF5 => self.SBC(AddressingMode::ZPX), 0xF6 => self.INC(AddressingMode::ZPX), 0xF8 => self.SED(AddressingMode::IMP), 0xF9 => self.SBC(AddressingMode::ABY), 0xFD => self.SBC(AddressingMode::ABX), 0xFE => self.INC(AddressingMode::ABX),
                _ => self.XXX(AddressingMode::IMP),
            }
        }
    }

    /// Advance CPU by 1 'step' (=/= 1 clock cycle)
    pub fn advance(&mut self) {
        // CPU starts from the 16-bit reset vector at 0xFFFC
        if self.pc == RESET_VECTOR {
            let program_start: u16 = self.read_u16(self.pc);
            self.pc = program_start;
        } else {
            self.opcode = self.read(self.pc);
            self.pc += 1;

            if self.opcode == 0x00 {
                return
            }

            match self.opcode {
                0x00 => self.BRK(AddressingMode::IMP), 0x01 => self.ORA(AddressingMode::ZPX), 0x05 => self.ORA(AddressingMode::ZP0), 0x06 => self.ASL(AddressingMode::ZP0), 0x08 => self.PHP(AddressingMode::IMP), 0x09 => self.ORA(AddressingMode::IMM), 0x0A => self.ASL(AddressingMode::ACC), 0x0D => self.ORA(AddressingMode::ABS), 0x0E => self.ASL(AddressingMode::ABS),
                0x10 => self.BPL(AddressingMode::REL), 0x11 => self.ORA(AddressingMode::ZPY), 0x15 => self.ORA(AddressingMode::ZPX), 0x16 => self.ASL(AddressingMode::ZPX), 0x18 => self.CLC(AddressingMode::IMP), 0x1D => self.ORA(AddressingMode::ABX), 0x1E => self.ASL(AddressingMode::ABX),
                0x20 => self.JSR(AddressingMode::ABS), 0x21 => self.AND(AddressingMode::ZPX), 0x24 => self.BIT(AddressingMode::ZP0), 0x25 => self.AND(AddressingMode::ZP0), 0x26 => self.ROL(AddressingMode::ZP0), 0x28 => self.PLP(AddressingMode::IMP), 0x29 => self.AND(AddressingMode::IMM), 0x2A => self.ROL(AddressingMode::ACC), 0x2C => self.BIT(AddressingMode::ABS), 0x2D => self.AND(AddressingMode::ABS), 0x2E => self.ROL(AddressingMode::ABS),
                0x30 => self.BMI(AddressingMode::REL), 0x31 => self.AND(AddressingMode::ZPX), 0x35 => self.AND(AddressingMode::ZPX), 0x36 => self.ROL(AddressingMode::ZPX), 0x38 => self.SEC(AddressingMode::IMP), 0x39 => self.AND(AddressingMode::ABY), 0x3D => self.AND(AddressingMode::ABX), 0x3E => self.ROL(AddressingMode::ABX),
                0x40 => self.RTI(AddressingMode::IMP), 0x41 => self.EOR(AddressingMode::ZPX), 0x45 => self.EOR(AddressingMode::ZP0), 0x46 => self.LSR(AddressingMode::ZP0), 0x48 => self.PHA(AddressingMode::IMP), 0x49 => self.EOR(AddressingMode::IMM), 0x4A => self.LSR(AddressingMode::ACC), 0x4C => self.JMP(AddressingMode::ABS), 0x4D => self.EOR(AddressingMode::ABS), 0x4E => self.LSR(AddressingMode::ABS),
                0x50 => self.BVC(AddressingMode::REL), 0x51 => self.EOR(AddressingMode::ZPY), 0x55 => self.EOR(AddressingMode::ZPY), 0x56 => self.LSR(AddressingMode::ZPX), 0x58 => self.CLI(AddressingMode::IMP), 0x59 => self.EOR(AddressingMode::ABY), 0x5D => self.EOR(AddressingMode::ABX), 0x5E => self.LSR(AddressingMode::ABX),
                0x60 => self.RTS(AddressingMode::IMP), 0x61 => self.ADC(AddressingMode::ZPX), 0x65 => self.ADC(AddressingMode::ZP0), 0x66 => self.ROR(AddressingMode::ZP0), 0x68 => self.PLA(AddressingMode::IMP), 0x69 => self.ADC(AddressingMode::IMM), 0x6A => self.ROR(AddressingMode::ACC), 0x6C => self.JMP(AddressingMode::IND), 0x6D => self.ADC(AddressingMode::ABS), 0x6E => self.ROR(AddressingMode::ABS),
                0x70 => self.BVS(AddressingMode::REL), 0x71 => self.ADC(AddressingMode::ZPY), 0x75 => self.ADC(AddressingMode::ZPX), 0x78 => self.SEI(AddressingMode::IMP), 0x79 => self.ADC(AddressingMode::ABY), 0x7D => self.ADC(AddressingMode::ABX), 0x7E => self.ROR(AddressingMode::ABX),
                0x80 => self.NOP(AddressingMode::IMM), 0x81 => self.STA(AddressingMode::ZPX), 0x84 => self.STY(AddressingMode::ZP0), 0x85 => self.STA(AddressingMode::ZP0), 0x86 => self.STX(AddressingMode::ZP0), 0x88 => self.DEY(AddressingMode::IMP), 0x8A => self.TXA(AddressingMode::IMP), 0x8C => self.STY(AddressingMode::ABS), 0x8D => self.STA(AddressingMode::ABS), 0x8E => self.STX(AddressingMode::ABS),
                0x90 => self.BCC(AddressingMode::REL), 0x91 => self.STA(AddressingMode::IDY), 0x94 => self.STY(AddressingMode::ZPX), 0x95 => self.STA(AddressingMode::ZPX), 0x96 => self.STX(AddressingMode::ZPY), 0x98 => self.TYA(AddressingMode::IMP), 0x99 => self.STA(AddressingMode::ABY), 0x9A => self.TXS(AddressingMode::IMP), 0x9D => self.STA(AddressingMode::ABX),
                0xA0 => self.LDY(AddressingMode::IMM), 0xA1 => self.LDA(AddressingMode::ZPX), 0xA2 => self.LDX(AddressingMode::IMM), 0xA4 => self.LDY(AddressingMode::ZP0), 0xA5 => self.LDA(AddressingMode::ZP0), 0xA6 => self.LDX(AddressingMode::ZP0), 0xA8 => self.TAY(AddressingMode::IMP), 0xA9 => self.LDA(AddressingMode::IMM), 0xAA => self.TAX(AddressingMode::IMP), 0xAC => self.LDY(AddressingMode::ABS), 0xAD => self.LDA(AddressingMode::ABS), 0xAE => self.LDX(AddressingMode::ABS),
                0xB0 => self.BCS(AddressingMode::REL), 0xB1 => self.LDA(AddressingMode::ZPY), 0xB4 => self.LDY(AddressingMode::ZPX), 0xB5 => self.LDA(AddressingMode::ZPX), 0xB6 => self.LDX(AddressingMode::ZPY), 0xB8 => self.CLV(AddressingMode::IMP), 0xB9 => self.LDA(AddressingMode::ABY), 0xBA => self.TSX(AddressingMode::IMP), 0xBC => self.LDY(AddressingMode::ABX), 0xBD => self.LDA(AddressingMode::ABX), 0xBE => self.LDX(AddressingMode::ABY),
                0xC0 => self.CPY(AddressingMode::IMM), 0xC1 => self.CMP(AddressingMode::ZPX), 0xC4 => self.CPY(AddressingMode::ZP0), 0xC5 => self.CMP(AddressingMode::ZP0), 0xC6 => self.DEC(AddressingMode::ZP0), 0xC8 => self.CLV(AddressingMode::IMP), 0xC9 => self.CMP(AddressingMode::IMM), 0xCA => self.DEX(AddressingMode::IMP), 0xCC => self.CPY(AddressingMode::ABS), 0xCD => self.CMP(AddressingMode::ABS), 0xCE => self.DEC(AddressingMode::ABS),
                0xD0 => self.BNE(AddressingMode::REL), 0xD1 => self.CMP(AddressingMode::ZPY), 0xD5 => self.CMP(AddressingMode::ZPX), 0xD6 => self.DEC(AddressingMode::ZPX), 0xD8 => self.CLD(AddressingMode::IMP), 0xD9 => self.CMP(AddressingMode::ABY), 0xDD => self.CMP(AddressingMode::ABX), 0xDE => self.DEC(AddressingMode::ABX),
                0xE0 => self.CPX(AddressingMode::IMM), 0xE1 => self.SBC(AddressingMode::ZPX), 0xE4 => self.CPX(AddressingMode::ZP0), 0xE5 => self.SBC(AddressingMode::ZP0), 0xE6 => self.INC(AddressingMode::ZP0), 0xE8 => self.INX(AddressingMode::IMP), 0xE9 => self.SBC(AddressingMode::IMM), 0xEA => self.NOP(AddressingMode::IMP), 0xEC => self.CPX(AddressingMode::ABS), 0xED => self.SBC(AddressingMode::ABS), 0xEE => self.INC(AddressingMode::ABS),
                0xF0 => self.BEQ(AddressingMode::REL), 0xF1 => self.SBC(AddressingMode::ZPY), 0xF5 => self.SBC(AddressingMode::ZPX), 0xF6 => self.INC(AddressingMode::ZPX), 0xF8 => self.SED(AddressingMode::IMP), 0xF9 => self.SBC(AddressingMode::ABY), 0xFD => self.SBC(AddressingMode::ABX), 0xFE => self.INC(AddressingMode::ABX),
                _ => self.XXX(AddressingMode::IMP),
            }
        }
    }

    // TODO: Interrupt request (IRQ)
    // If interrupt disable flag clear, push PC and SR to stack and get next location from IRQ vector
    #[allow(unused)]
    fn irq(&mut self) {
        if !self.get_flag(Flags::I) {
            self.push_u16(self.pc);
            self.push(self.sr);
            self.pc = self.read_u16(IRQ_VECTOR)
        }
    }

    // TODO: Non-maskeable interrupt (NMI)
    // Push PC and SR to stack and get next location from NMI vector
    #[allow(unused)]
    fn nmi(&mut self) {
        todo!();
    }
}

/// Addressing modes
#[derive(PartialEq)]
#[allow(clippy::upper_case_acronyms, dead_code)]
pub enum AddressingMode {
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

/// Implement addressing modes
impl CPU {
    pub fn get_address(&mut self, mode: AddressingMode) -> u16 {
        match mode {
            AddressingMode::IMM => {
                let addr = self.pc;
                self.pc += 1;
                addr
            },
            AddressingMode::IMP => {
                panic!("not supported");
            },
            AddressingMode::ZP0 => {
                let addr = self.read(self.pc) as u16;
                self.pc += 1;
                addr
            },
            AddressingMode::ZPX => {
                let addr = self.read(self.pc).wrapping_add(self.x) as u16;
                self.pc += 1;
                addr
            },
            AddressingMode::ZPY => {
                let addr = self.read(self.pc).wrapping_add(self.y) as u16;
                self.pc += 1;
                addr
            },
            AddressingMode::ABS => {
                let addr = self.read_u16(self.pc);
                self.pc += 2;
                addr
            },
            AddressingMode::ABX => {
                let addr = self.read_u16(self.pc).wrapping_add(self.x as u16);
                self.pc += 2;
                addr
            },
            AddressingMode::ABY => {
                let addr = self.read_u16(self.pc).wrapping_add(self.y as u16);
                self.pc += 2;
                addr
            },
            // Indirect (IND) addressing: the program is supplied with a pointer.
            // The value that it reads there is the address that holds the operand.
            // 
            // E.g. PC is 0x0301. 0x0301 (+1) stores pointer 0x4230. Pointer 0x4230 (+1) stores address 0x04A9.
            // Returns address 0x04A9 because operand is there.
            AddressingMode::IND => {
                let addr = self.read_u16(self.pc);
                self.pc += 2;

                let ptr = self.read_u16(addr);
                self.pc += 2;
                ptr
            },
            // Indexed indirect addressing: the program is supplied with a zero-page pointer.
            // The X register is added to that pointer. This points to the address that holds the operand.
            // 
            // E.g. PC is 0x0301, X is 0x02. PC stores base 0x30. Pointer = 0x30 + 0x02 = 0x0032. This stores 0x91.
            // 0x0033 stores 0xEF. So this returns address 0xEF91 (operand is stored there).
            AddressingMode::IDX => {
                let base = self.read(self.pc);

                let ptr = base + self.x;
                let lo = self.read(ptr as u16);
                let hi = self.read((ptr + 1) as u16);

                self.pc += 2;
                (hi as u16) << 8 | (lo as u16)
            },
            // Indirect indexed addressing: the program is supplied with a zero-page address.
            // The value that it reads there + the Y register, is a pointer to the address that holds the operand.
            // 
            // E.g. PC is 0x0301, Y is 0x02. PC stores 0x30. Pointer 0x0030 stores 0x91. Address = 0x91 + 0x02 = 0x93.
            // Returns address 0x0093 because operand is there.
            AddressingMode::IDY => {
                let zp_addr = self.read(self.pc);
                let base = self.read(zp_addr as u16);

                let ptr = base + self.y;

                let lo = self.read(ptr as u16);

                self.pc += 2;
                lo as u16
            },
            AddressingMode::REL => todo!(),
            AddressingMode::ACC => todo!(),
        }
    }
}

/// Implement instructions
#[allow(non_snake_case)]
#[allow(unused)]
impl CPU {
    // Add with carry
    fn ADC(&mut self, mode: AddressingMode) {
        let addr: u16 = self.get_address(mode);
        let operand: u8 = self.read(addr);

        let result_u16: u16 = self.a as u16 + operand as u16 + (self.sr & 0x01) as u16;
        let result_u8: u8 = result_u16 as u8;

        // Set carry if unsigned overflow occurs
        self.set_flag(Flags::C, result_u16 > 0xFF);
        
        // Set V if signed overflow occurs
        self.set_flag(Flags::V, self.a & 0x80 == operand & 0x80 && self.a & 0x80 != result_u8 & 0x80);

        // Set accumulator equal to result
        self.a = result_u8;

        // set zero and negative flags
        self.set_zero_negative_flags(self.a);
    }

    // Logical AND
    fn AND(&mut self, mode: AddressingMode) {
        let addr: u16 = self.get_address(mode);
        let value: u8 = self.read(addr);
        self.a &= value;
        self.set_zero_negative_flags(self.a);
    }

    // Arithmetic shift left
    fn ASL(&mut self, mode: AddressingMode) {
        println!("ASL");
        if mode == AddressingMode::ACC {
            // Move bit 7 to carry
            self.set_flag(Flags::C, self.a & 0x80 == 0x80);

            // Shift other bits to the left
            self.a <<= 1;

            self.set_zero_negative_flags(self.a);
        } else {
            let addr: u16 = self.get_address(mode);
            let value: u8 = self.read(addr);
            
            // Move bit 7 to carry
            self.set_flag(Flags::C, value & 0x80 == 0x80);

            // Shift other bits to the left, and set Z and N flags
            let result = value << 1;
            self.write(addr, result);
            self.set_zero_negative_flags(result);
        }
    }

    // Branch if carry clear
    fn BCC(&mut self, mode: AddressingMode) {
        if !self.get_flag(Flags::C) {
            let offset = self.read(self.pc);
            if offset >= 128 {
                // offset is negative
                self.pc -= ((!offset) + 1) as u16;
            } else {
                // offset is positive
                self.pc += offset as u16;
            }
        }
    }

    // Branch if carry set
    fn BCS(&mut self, mode: AddressingMode) {
        if self.get_flag(Flags::C) {
            let offset = self.read(self.pc);
            if offset >= 128 {
                // offset is negative
                self.pc -= ((!offset) + 1) as u16;
            } else {
                // offset is positive
                self.pc += offset as u16;
            }
        }
    }

    // Branch if equal (zero flag set)
    fn BEQ(&mut self, mode: AddressingMode) {
        if self.get_flag(Flags::Z) {
            let offset = self.read(self.pc);
            if offset >= 128 {
                // offset is negative
                self.pc -= ((!offset) + 1) as u16;
            } else {
                // offset is positive
                self.pc += offset as u16;
            }
        }
    }

    // Bit test
    fn BIT(&mut self, mode: AddressingMode) {
        let addr: u16 = self.get_address(mode);
        let value: u8 = self.read(addr);

        let result = self.a & value;

        self.set_flag(Flags::Z, result == 0x00);
        self.set_flag(Flags::V, (value & 0x40) == 0x40);
        self.set_flag(Flags::N, (value & 0x80) == 0x80);
    }

    // Branch if minus (negative flag set)
    fn BMI(&mut self, mode: AddressingMode) {
        if self.get_flag(Flags::N) {
            let offset = self.read(self.pc);
            if offset >= 128 {
                // offset is negative
                self.pc -= ((!offset) + 1) as u16;
            } else {
                // offset is positive
                self.pc += offset as u16;
            }
        }
    }

    // Branch if not equal (zero flag clear)
    fn BNE(&mut self, mode: AddressingMode) {
        if !self.get_flag(Flags::Z) {
            let offset = self.read(self.pc);
            if offset >= 128 {
                // offset is negative
                self.pc -= ((!offset) + 1) as u16;
            } else {
                // offset is positive
                self.pc += offset as u16;
            }
        }
    }

    // Branch if positive (negative flag clear)
    fn BPL(&mut self, mode: AddressingMode) {
        if !self.get_flag(Flags::N) {
            let offset = self.read(self.pc);
            if offset >= 128 {
                // offset is negative
                self.pc -= ((!offset) + 1) as u16;
            } else {
                // offset is positive
                self.pc += offset as u16;
            }
        }
    }

    // Force interruption
    fn BRK(&mut self, mode: AddressingMode) {
        // Push PC to stack
        self.push_u16(self.pc);
        // Set break flag to 1
        self.set_flag(Flags::B, true);
        // Push SR to stack
        self.push(self.sr);
        // Load IRQ interrupt vector at 0xFFFE (+1) into PC
        self.pc = self.read_u16(IRQ_VECTOR);
        // Set disable interrupt flag so other interrupts don't happen
        self.set_flag(Flags::I, true);
    }

    // Branch if overflow clear
    fn BVC(&mut self, mode: AddressingMode) {
        if !self.get_flag(Flags::V) {
            let offset = self.read(self.pc);
            if offset >= 128 {
                // offset is negative
                self.pc -= ((!offset) + 1) as u16;
            } else {
                // offset is positive
                self.pc += offset as u16;
            }
        }
    }

    // Branch if overflow set
    fn BVS(&mut self, mode: AddressingMode) {
        if self.get_flag(Flags::V) {
            let offset = self.read(self.pc);
            if offset >= 128 {
                // offset is negative
                self.pc -= ((!offset) + 1) as u16;
            } else {
                // offset is positive
                self.pc += offset as u16;
            }
        }
    }

    // Clear the carry flag to zero
    fn CLC(&mut self, mode: AddressingMode) {
        self.set_flag(Flags::C, false);
    }

    // Clear decimal mode flag to zero
    fn CLD(&mut self, mode: AddressingMode) {
        self.set_flag(Flags::D, false);
    }

    // Clear interrupt disable flag to zero
    fn CLI(&mut self, mode: AddressingMode) {
        self.set_flag(Flags::I, false);
    }

    // Clear overflow flag to zero
    fn CLV(&mut self, mode: AddressingMode) {
        self.set_flag(Flags::V, false);
    }

    // Compare accumulator to memory value
    fn CMP(&mut self, mode: AddressingMode) {
        let addr = self.get_address(mode);
        let value = self.read(addr);
        let result = self.a - value;
        self.set_zero_negative_flags(result);
        self.set_flag(Flags::C, self.a >= value);
    }

    // Compare X register to memory value
    fn CPX(&mut self, mode: AddressingMode) {
        let addr = self.get_address(mode);
        let value = self.read(addr);
        let result = self.x - value;
        self.set_zero_negative_flags(result);
        self.set_flag(Flags::C, self.x >= value);
    }

    // Compare Y register to memory value
    fn CPY(&mut self, mode: AddressingMode) {
        let addr = self.get_address(mode);
        let value = self.read(addr);
        let result = self.y - value;
        self.set_zero_negative_flags(result);
        self.set_flag(Flags::C, self.y >= value);
    }

    // Decrement memory
    fn DEC(&mut self, mode: AddressingMode) {
        let addr: u16 = self.get_address(mode);
        let mut value: u8 = self.read(addr);
        value -= 1;
        self.write(addr, value);
        self.set_zero_negative_flags(value);
    }

    // Decrement X
    fn DEX(&mut self, mode: AddressingMode) {
        self.x -= 1;
        self.set_zero_negative_flags(self.x);
    }

    // Decrement Y
    fn DEY(&mut self, mode: AddressingMode) {
        self.y -= 1;
        self.set_zero_negative_flags(self.y);
    }

    // Exclusive OR
    fn EOR(&mut self, mode: AddressingMode) {
        let addr: u16 = self.get_address(mode);
        let value: u8 = self.read(addr);
        self.a ^= value;
        self.set_zero_negative_flags(self.a);
    }

    // Increment memory
    fn INC(&mut self, mode: AddressingMode) {
        let addr: u16 = self.get_address(mode);
        let mut value: u8 = self.read(addr);
        value += 1;
        self.write(addr, value);
        self.set_zero_negative_flags(value);
    }

    // Increment X
    fn INX(&mut self, mode: AddressingMode) {
        self.x += 1;
        self.set_zero_negative_flags(self.x);
    }

    // Increment Y
    fn INY(&mut self, mode: AddressingMode) {
        self.y += 1;
        self.set_zero_negative_flags(self.y);
    }

    // Jump
    fn JMP(&mut self, mode: AddressingMode) {
        let addr = self.get_address(mode);
        self.pc = addr;
    }

    // Jump to subroutine
    fn JSR(&mut self, mode: AddressingMode) {
        self.push_u16(self.pc - 1);
        let addr = self.get_address(mode);
        self.pc = addr;
    }

    // Load the accumulator
    fn LDA(&mut self, mode: AddressingMode) {
        let addr = self.get_address(mode);
        self.a = self.read(addr);
        self.set_zero_negative_flags(self.a);
    }

    // Load the X register
    fn LDX(&mut self, mode: AddressingMode) {
        let addr = self.get_address(mode);
        self.x = self.read(addr);
        self.set_zero_negative_flags(self.x);
    }

    // Load the Y register
    fn LDY(&mut self, mode: AddressingMode) {
        let addr = self.get_address(mode);
        self.y = self.read(addr);
        self.set_zero_negative_flags(self.y);
    }

    // Logical shift right
    fn LSR(&mut self, mode: AddressingMode) {
        println!("LSR");
        if mode == AddressingMode::ACC {
            // Move bit 0 to carry
            self.set_flag(Flags::C, self.a & 0x01 == 0x01);

            // Shift other bits to the right
            self.a >>= 1;

            self.set_zero_negative_flags(self.a);
        } else {
            let addr = self.get_address(mode);
            let value = self.read(addr);

            // Move bit 0 to carry
            self.set_flag(Flags::C, value & 0x01 == 0x01);

            // Shift other bits to the right, and set Z and N flags
            let result = value >> 1;
            self.write(addr, result);
            self.set_zero_negative_flags(result);
        }
    }

    // No operation
    fn NOP(&mut self, mode: AddressingMode) {}

    // Inclusive OR
    fn ORA(&mut self, mode: AddressingMode) {
        let addr: u16 = self.get_address(mode);
        let value: u8 = self.read(addr);
        self.a |= value;
        self.set_zero_negative_flags(self.a);
    }

    // Push accumulator to stack
    fn PHA(&mut self, mode: AddressingMode) {
        self.push(self.a);
    }

    // Push status register to stack
    fn PHP(&mut self, mode: AddressingMode) {
        self.push(self.sr);
    }

    // Pull accumulator (from stack)
    fn PLA(&mut self, mode: AddressingMode) {
        self.a = self.pop();
        self.set_zero_negative_flags(self.a);
    }

    // Pull processor status
    fn PLP(&mut self, mode: AddressingMode) {
        self.sr = self.pop();
    }

    // Rotate left
    // Shift left, while setting bit 0 equal to carry and setting new carry equal to old bit 7
    fn ROL(&mut self, mode: AddressingMode) {
        if mode == AddressingMode::ACC {
            // Store old bit 7
            let bit_seven: bool = self.a & 0x80 == 0x80;

            // Shift bits left
            let mut result = self.a << 1;

            // Set bit 0 equal to old carry
            result |= self.sr & 0x01;

            // Load result to accumulator
            self.a = result;

            // Set new carry to old bit 7
            self.set_flag(Flags::C, bit_seven);

            // Set other flags
            self.set_zero_negative_flags(self.a);
        } else {
            let addr = self.get_address(mode);
            let value = self.read(addr);

            // Store old bit 7
            let bit_seven: bool = value & 0x80 == 0x80;

            // Shift bits left
            let mut result = value << 1;

            // Set bit 0 equal to old carry
            result |= self.sr & 0x01;

            // Write result
            self.write(addr, result);

            // Set new carry to old bit 7
            self.set_flag(Flags::C, bit_seven);

            // Set other flags
            self.set_zero_negative_flags(result);
        }
    }

    // Rotate right
    fn ROR(&mut self, mode: AddressingMode) {
        if mode == AddressingMode::ACC {
            // Store old bit 0
            let bit_zero: bool = self.a & 0x01 == 0x01;

            // Shift bits to the right
            let mut result = self.a >> 1;

            // Set bit 7 equal to old carry
            result |= (self.sr & 0x01) << 7;

            // Load result to accumulator
            self.a = result;

            // Set new carry to old bit 0
            self.set_flag(Flags::C, bit_zero);

            // Set other flags
            self.set_zero_negative_flags(self.a);
        } else {
            let addr = self.get_address(mode);
            let value = self.read(addr);

            // Store old bit 0
            let bit_zero: bool = value & 0x01 == 0x01;

            // Shift bits to the right
            let mut result = value << 1;

            // Set bit 7 equal to old carry
            result |= (self.sr & 0x01) << 7;

            // Write result
            self.write(addr, result);

            // Set new carry to old bit 0
            self.set_flag(Flags::C, bit_zero);

            // Set other flags
            self.set_zero_negative_flags(result);
        }
    }

    // Return from interrupt
    fn RTI(&mut self, mode: AddressingMode) {
        self.sr = self.pop();
        self.pc = self.pop_u16();
    }

    // Return from subroutine
    fn RTS(&mut self, mode: AddressingMode) {
        self.pc = self.pop_u16();
    }

    // Subtract with carry
    fn SBC(&mut self, mode: AddressingMode) {
        let addr = self.get_address(mode);
        let operand = self.read(addr);

        let result_i16: i16 = self.a as i16 - operand as i16 - (self.sr & 0x01) as i16;
        let result_u8: u8 = result_i16 as u8;

        // TODO: clear C if overflow in bit 7 (???)
        self.set_flag(Flags::C, result_i16 < 0xFF);

        // Set V if signed overflow occurs
        self.set_flag(Flags::V, self.a & 0x80 != operand & 0x80 && operand & 0x80 == result_u8 & 0x80);

        // self.a -= operand + (1 - (self.sr & 0x01));
        self.a = result_u8;

        // Set zero and negative flags
        self.set_zero_negative_flags(self.a);
    }

    // Set carry flag to 1
    fn SEC(&mut self, mode: AddressingMode) {
        self.set_flag(Flags::C, true);
    }

    // Set decimal flag to 1
    fn SED(&mut self, mode: AddressingMode) {
        self.set_flag(Flags::D, true);
    }

    // Set interrupt disable flag to 1
    fn SEI(&mut self, mode: AddressingMode) {
        self.set_flag(Flags::I, true);
    }

    // Store accumulator
    fn STA(&mut self, mode: AddressingMode) {
        let addr: u16 = self.get_address(mode);
        self.write(addr, self.a);
    }

    // Store the x register
    fn STX(&mut self, mode: AddressingMode) {
        let addr: u16 = self.get_address(mode);
        self.write(addr, self.x);
    }

    // Store the y register
    fn STY(&mut self, mode: AddressingMode) {
        let addr: u16 = self.get_address(mode);
        self.write(addr, self.y);
    }

    // Transfer accumulator to X register
    fn TAX(&mut self, mode: AddressingMode) {
        self.x = self.a;
        self.set_zero_negative_flags(self.x);
    }

    // Transfer accumulator to Y register
    fn TAY(&mut self, mode: AddressingMode) {
        self.y = self.a;
        self.set_zero_negative_flags(self.y);
    }

    // Transfer stack pointer to X register
    fn TSX(&mut self, mode: AddressingMode) {
        self.x = self.sp;
        self.set_zero_negative_flags(self.x);
    }

    // Transfer X register to accumulator
    fn TXA(&mut self, mode: AddressingMode) {
        self.a = self.x;
        self.set_zero_negative_flags(self.a);
    }

    // Transfer X register to stack pointer
    fn TXS(&mut self, mode: AddressingMode) {
        self.sp = self.x;
    }

    // Transfer Y register to accumulator
    fn TYA(&mut self, mode: AddressingMode) {
        self.a = self.y;
        self.set_zero_negative_flags(self.a);
    }

    // When an illegal opcode is passed, XXX() is run
    fn XXX(&mut self, mode: AddressingMode) {
        panic!("Illegal operation!");
    }
}

/// Flags (in order from least to most significant bit)
#[allow(unused)]
pub enum Flags {
    /// Carry
    C = 0b0000_0001,
    /// Zero
    Z = 0b0000_0010,
    /// Disable interrupt
    I = 0b0000_0100,
    /// Decimal mode (not implemented)
    D = 0b0000_1000,
    /// Break
    B = 0b0001_0000,
    /// Unused
    U = 0b0010_0000,
    /// Overflow
    V = 0b0100_0000,
    /// Negative
    N = 0b1000_0000,
}

impl Flags {
    /// Takes a letter as a `&str` and returns binary bitflag value, if flag with that letter exists
    pub fn byte_from_str(string: &str) -> u8 {
        match string {
            "C" => 0b0000_0001,
            "Z" => 0b0000_0010,
            "I" => 0b0000_0100,
            "D" => 0b0000_1000,
            "B" => 0b0001_0000,
            "U" => 0b0010_0000,
            "V" => 0b0100_0000,
            "N" => 0b1000_0000,
            _ => 0,
        }
    }
}

/// Debugging/testing functions
impl CPU {
    /// Write program defined as `Vec<u8>` to memory
    pub fn load_program(&mut self, program: Vec<u8>) {
        // Point to program start address
        self.write_u16(0xFFFC, 0x0600);

        // Write program
        #[allow(clippy::needless_range_loop)]
        for i in 0..program.len() {
            // println!("Writing {} to {}", program[i], i as u16);
            self.write(0x0600 + (i as u16), program[i]);
        }
    }

    /// Write program to memory, reset, and start clock
    #[allow(unused)]
    pub fn quick_start(&mut self, program: Vec<u8>) {
        self.load_program(program);
        self.reset();
        self.clock();
    }

    // Get registers
    pub fn get_pc(&self) -> u16 { self.pc }
    pub fn get_sp(&self) -> u8 { self.sp }
    pub fn get_sr(&self) -> u8 { self.sr }
    pub fn get_a(&self) -> u8 { self.a }
    pub fn get_x(&self) -> u8 { self.x }
    pub fn get_y(&self) -> u8 { self.y }
    pub fn get_opcode(&self) -> u8 { self.opcode }

    /// Return all registers as single `Vec<u16>`
    pub fn get_state(&self) -> Vec<u16> {
        vec![
            self.get_a() as u16,
            self.get_x() as u16,
            self.get_y() as u16,
            self.get_sp() as u16,
            self.get_pc(),
            self.get_sr() as u16,
            self.get_opcode() as u16,
        ]
    }

    /// Get entirety of memory
    pub fn get_memory(&self) -> [u8; 64 * 1024] {
        self.bus.get_ram()
    }

    /// Construct CPU with custom values
    #[allow(clippy::too_many_arguments, unused)]
    pub fn custom(a: u8, x: u8, y: u8, sp: u8, pc: u16, sr: u8, opcode: u8, bus: Bus,) -> Self {
        CPU {
            a,
            x,
            y,
            sp,
            pc,
            sr,
            opcode,
            bus,
        }
    }
}
