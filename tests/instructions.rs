use mos_6502_emulator::{
    cpu::{
        CPU,
        Flags,
    },
    bus::Bus,
};

// TODO: Test all instructions
#[test]
fn lda_imm_flags() {
    let bus: Bus = Bus::new();
    let mut cpu: CPU = CPU::new(bus);
    cpu.quick_start(vec![0xA9, 0x05, 0x00]);

    assert_eq!(cpu.get_a(), 0x05);
    assert!(!cpu.get_flag(Flags::Z));
    assert!(!cpu.get_flag(Flags::N));
}

#[test]
fn lda_imm_zero_flag() {
    let bus: Bus = Bus::new();
    let mut cpu: CPU = CPU::new(bus);
    cpu.quick_start(vec![0xA9, 0x00, 0x00]);

    assert!(cpu.get_flag(Flags::Z));
}

#[test]
fn lda_zp0() {
    let bus: Bus = Bus::new();
    let mut cpu: CPU = CPU::new(bus);
    // write 0x55 to address 0x10
    cpu.write(0x10, 0x55);
    // LDA from address 0x10
    cpu.quick_start(vec![0xA5, 0x10, 0x00]);

    assert_eq!(cpu.get_a(), 0x55);
}

#[test]
fn adc_imm() {
    let bus: Bus = Bus::new();
    let mut cpu: CPU = CPU::new(bus);

    // No carry -> no carry
    {
        cpu.quick_start(vec![0xA9, 0x10, 0x69, 0x02, 0x00]);
        assert_eq!(cpu.get_a(), 0x12);
        assert!(!cpu.get_flag(Flags::C));
        assert!(!cpu.get_flag(Flags::V));
    }

    // Carry -> no carry
    {
        // TODO: how to verify carry?
        cpu.set_flag(Flags::C, true);
        cpu.quick_start(vec![0xA9, 0x10, 0x69, 0x02, 0x00]);
        assert_eq!(cpu.get_a(), 0x12);
        assert!(!cpu.get_flag(Flags::C));
        assert!(!cpu.get_flag(Flags::V));
    }

    // No carry -> carry
    {
        // LDA 0xFE, ADC 0x03. Should wrap around to 0x01.
        cpu.quick_start(vec![0xA9, 0xFE, 0x69, 0x03, 0x00]);
        assert_eq!(cpu.get_a(), 0x01);
        assert!(cpu.get_flag(Flags::C));
        assert!(!cpu.get_flag(Flags::V));
    }

    // No carry -> carry
    {
        // LDA 0xFE, ADC 0x12. Should wrap around to 0x10.
        cpu.quick_start(vec![0xA9, 0xFE, 0x69, 0x12, 0x00]);
        assert_eq!(cpu.get_a(), 0x10);
        assert!(cpu.get_flag(Flags::C));
        assert!(!cpu.get_flag(Flags::V));
    }

    // TODO: Overflow -> no overflow
    {
        cpu.quick_start(vec![]);
    }

    // TODO: No overflow -> overflow
}

#[test]
fn and_imm() {
    let bus: Bus = Bus::new();
    let mut cpu: CPU = CPU::new(bus);
    // LDA(IMM) with 0x6b, AND(IMM) with 0x2c
    cpu.quick_start(vec![0xA9, 0x6b, 0x29, 0x2c, 0x00]);

    assert_eq!(cpu.get_a(), 0x28);
}

#[test]
fn and_imm_zero() {
    let mut cpu: CPU = CPU::new(Bus::new());
    // LDA 0x6B, AND 0x14, BRK
    cpu.quick_start(vec![0xA9, 0x6B, 0x29, 0x14, 0x00]);
    assert!(cpu.get_flag(Flags::Z));
    assert_eq!(cpu.get_a(), 0x00);
}

// write number to memory, lda immediate, ldx immediate, then adc with zpx
#[test]
fn adc_zpx() {
    let bus: Bus = Bus::new();
    let mut cpu: CPU = CPU::new(bus);
    cpu.write(0x00F1, 0x27);
    cpu.quick_start(vec![0xA9, 0x03, 0xA2, 0x10, 0x75, 0xE1, 0x00]);

    assert_eq!(cpu.get_a(), 0x2A);
}

#[test]
fn asl_acc() {
    let bus: Bus = Bus::new();
    let mut cpu: CPU = CPU::new(bus);

    {
        cpu.quick_start(vec![0xA9, 0b0010_1000, 0x0A, 0x00]);
        assert_eq!(cpu.get_a(), 0b0101_0000);
        assert!(!cpu.get_flag(Flags::C));
        assert!(!cpu.get_flag(Flags::N));
        assert!(!cpu.get_flag(Flags::Z));
    }

    {
        cpu.quick_start(vec![0xA9, 0b1010_0000, 0x0A, 0x00]);
        assert_eq!(cpu.get_a(), 0b0100_0000);
        assert!(cpu.get_flag(Flags::C));
        // println!("A: {}", cpu.get_a());
    }

    {
        cpu.quick_start(vec![0xA9, 0b1000_0000, 0x0A, 0x00]);
        assert_eq!(cpu.get_a(), 0x00);
        assert!(cpu.get_flag(Flags::C));
        assert!(cpu.get_flag(Flags::Z));
        assert!(!cpu.get_flag(Flags::N));
    }

    {
        cpu.quick_start(vec![0xA9, 0b0100_0000, 0x0A, 0x00]);
        assert_eq!(cpu.get_a(), 0b1000_0000);
        assert!(cpu.get_flag(Flags::N));
        assert!(!cpu.get_flag(Flags::Z));
        assert!(!cpu.get_flag(Flags::C));
    }
}

#[test]
fn beq_rel_pos() {
    let mut cpu: CPU = CPU::new(Bus::new());
    // LDA 0xA9, AND 0xC0, BEQ -> LDA 0xFF, BRK if no zero flag (A would remain 0xA9)
    cpu.quick_start(vec![0xA9, 0x2A, 0x29, 0xC0, 0xF0, 0x03, 0x00, 0x00, 0xA9, 0xFF, 0x00]);
    assert!(!cpu.get_flag(Flags::Z));
    assert_eq!(cpu.get_a(), 0xFF);
}

#[test]
fn beq_rel_neg() {
    let mut cpu: CPU = CPU::new(Bus::new());
    cpu.write(0x05D5, 0xA9);
    cpu.write(0x05D6, 0xFF);
    // LDA 0xA9, AND 0xC0, BEQ -> LDA 0xFF, LDA 0xAF if no zero flag
    cpu.quick_start(vec![0xA9, 0x2A, 0x29, 0xC0, 0xF0, 0xD0, 0xA9, 0xAF]);
    assert_eq!(cpu.get_a(), 0xFF);
}

// TODO: test what happens if underflow occurs
// #[test]
// fn beq_rel_under() {

// }

#[test]
fn clc_imp() {
    let mut cpu: CPU = CPU::new(Bus::new());
    assert!(!cpu.get_flag(Flags::C));
    cpu.set_flag(Flags::C, true);
    assert!(cpu.get_flag(Flags::C));
    cpu.quick_start(vec![0x18, 0x00]);
    assert!(!cpu.get_flag(Flags::C));
}

#[test]
fn cld_imp() {
    let mut cpu: CPU = CPU::new(Bus::new());
    assert!(!cpu.get_flag(Flags::D));
    cpu.set_flag(Flags::D, true);
    assert!(cpu.get_flag(Flags::D));
    cpu.quick_start(vec![0xD8, 0x00]);
    assert!(!cpu.get_flag(Flags::D));
}

#[test]
fn cli_imp() {
    let mut cpu: CPU = CPU::new(Bus::new());
    assert!(!cpu.get_flag(Flags::I));
    cpu.set_flag(Flags::I, true);
    assert!(cpu.get_flag(Flags::I));
    cpu.quick_start(vec![0x58, 0x00]);
    assert!(!cpu.get_flag(Flags::I));
}

#[test]
fn clv_imp() {
    let mut cpu: CPU = CPU::new(Bus::new());
    assert!(!cpu.get_flag(Flags::V));
    cpu.set_flag(Flags::V, true);
    assert!(cpu.get_flag(Flags::V));
    cpu.quick_start(vec![0xB8, 0x00]);
    assert!(!cpu.get_flag(Flags::V));
}

// TODO: test ASL(ABS)
// #[test]
// fn test_asl_abs() {
//     let bus: Bus = Bus::new();
//     let mut cpu: CPU = CPU::new(bus);

//     // TODO: LDA(IMM): 0b0010_1000, STA(ABS) to addr: 0x3B01, ASL(ABS) to addr: 0x3B01
//     {
//         cpu.quick_start(vec![0xA9, 0b0010_1000, 0x8D, 0x01, 0x3B, 0x0E, 0x01, 0x3B, 0x00]);
//         assert_eq!(cpu.read(0x3B), 0b0101_0000);
//     }
// }

// TODO: test ASL with different mode(s)

#[test]
fn jmp_abs() {
    let bus: Bus = Bus::new();
    let mut cpu: CPU = CPU::new(bus);
    cpu.write(0x3000, 0xA9);
    cpu.write(0x3001, 0x04);
    cpu.write(0x3002, 0x00);

    // LDA 0x02, JMP to 0x3000. Then LDA 0x04 and BRK.
    cpu.quick_start(vec![0xA9, 0x02, 0x4C, 0x00, 0x30]);
    assert_eq!(cpu.get_a(), 0x04);
}

#[test]
fn jmp_ind() {
    let bus: Bus = Bus::new();
    let mut cpu: CPU = CPU::new(bus);
    cpu.write(0x1234, 0x30);
    cpu.write(0x1235, 0x24);
    cpu.write(0x2430, 0xA9);
    cpu.write(0x2431, 0x04);
    cpu.write(0x2432, 0x00);

    // LDA 0x02, JMP to pointer specified by 0x1234 (so to 0x2430). Then LDA 0x04 and BRK.
    cpu.quick_start(vec![0xA9, 0x02, 0x6C, 0x34, 0x12]);
    assert_eq!(cpu.get_a(), 0x04);
}

#[test]
fn sec_imp() {
    let mut cpu: CPU = CPU::new(Bus::new());
    assert!(!cpu.get_flag(Flags::C));
    cpu.quick_start(vec![0x38, 0x00]);
    assert!(cpu.get_flag(Flags::C));
}

#[test]
fn sed_imp() {
    let mut cpu: CPU = CPU::new(Bus::new());
    assert!(!cpu.get_flag(Flags::D));
    cpu.quick_start(vec![0xF8, 0x00]);
    assert!(cpu.get_flag(Flags::D));
}

#[test]
fn sei_imp() {
    let mut cpu: CPU = CPU::new(Bus::new());
    assert!(!cpu.get_flag(Flags::I));
    cpu.quick_start(vec![0x78, 0x00]);
    assert!(cpu.get_flag(Flags::I));
}

#[test]
fn sta_zp0() {
    let mut cpu: CPU = CPU::new(Bus::new());
    cpu.quick_start(vec![0xA9, 0xFF, 0x85, 0xAB]);
    assert_eq!(cpu.read(0xAB), 0xFF);
}