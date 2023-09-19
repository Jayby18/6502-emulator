#![allow(clippy::disallowed_names)]

use mos_6502_emulator::{
    cpu::{
        CPU,
        AddressingMode,
    },
    bus::Bus,
};

// TODO: Test all addressing modes (12, excl. IMP)

#[test]
fn imm() {
    let foo: u16 = 0x5931;
    let mut cpu: CPU = CPU::custom(0, 0, 0, 0, foo, 0, 0, Bus::new());
    assert_eq!(cpu.get_address(AddressingMode::IMM), foo);
}

#[test]
fn zp0() {
    let foo: u8 = 0x72;
    let bar: u8 = 0xF3;
    let mut cpu: CPU = CPU::custom(0, 0, 0, 0, foo as u16, 0, 0, Bus::new());
    cpu.write(foo as u16, bar);
    assert_eq!(cpu.get_address(AddressingMode::ZP0), bar as u16);
}

#[test]
fn zpx() {
    let foo: u8 = 0x72;
    let bar: u8 = 0x2C;
    let x: u8 = 0x1B;
    let mut cpu: CPU = CPU::custom(0, x, 0, 0, foo as u16, 0, 0, Bus::new());
    cpu.write(foo as u16, bar);
    assert_eq!(cpu.get_address(AddressingMode::ZPX), (bar + x) as u16);
}

#[test]
fn zpy() {
    let foo: u8 = 0x72;
    let bar: u8 = 0x2C;
    let y: u8 = 0x1B;
    let mut cpu: CPU = CPU::custom(0, 0, y, 0, foo as u16, 0, 0, Bus::new());
    cpu.write(foo as u16, bar);
    assert_eq!(cpu.get_address(AddressingMode::ZPY), (bar + y) as u16);
}

#[test]
fn abs() {
    let foo: u16 = 0x1234;
    let bar: u16 = 0x5621;
    let mut cpu: CPU = CPU::custom(0, 0, 0, 0, foo, 0, 0, Bus::new());
    cpu.write(foo, 0x21);
    cpu.write(foo + 1, 0x56);
    assert_eq!(cpu.get_address(AddressingMode::ABS), bar);
}

#[test]
fn abx() {
    let foo: u16 = 0x1234;
    let bar: u16 = 0x5621;
    let x: u8 = 0x2F;
    let mut cpu: CPU = CPU::custom(0, x, 0, 0, foo, 0, 0, Bus::new());
    cpu.write(foo, 0x21);
    cpu.write(foo + 1, 0x56);
    assert_eq!(cpu.get_address(AddressingMode::ABX), bar + x as u16);
}

#[test]
fn aby() {
    let foo: u16 = 0x1234;
    let bar: u16 = 0x5621;
    let y: u8 = 0x2F;
    let mut cpu: CPU = CPU::custom(0, 0, y, 0, foo, 0, 0, Bus::new());
    cpu.write(foo, 0x21);
    cpu.write(foo + 1, 0x56);
    assert_eq!(cpu.get_address(AddressingMode::ABY), bar + y as u16);
}

#[test]
fn ind() {
    let pc: u16 = 0x0301;
    let ptr: u16 = 0x4230;
    let addr: u16 = 0x04A9;
    let mut cpu: CPU = CPU::custom(0, 0, 0, 0, pc, 0, 0, Bus::new());
    cpu.write(pc, 0x30);
    cpu.write(pc + 1, 0x42);
    cpu.write(ptr, 0xA9);
    cpu.write(ptr + 1, 0x04);
    assert_eq!(cpu.get_address(AddressingMode::IND), addr);
}

#[test]
fn idx() {
    let pc: u16 = 0x0301;
    let x: u8 = 0x02;
    let base: u8 = 0x30;
    let ptr: u8 = base + x;
    let lo: u8 = 0x91;
    let hi: u8 = 0xEF;
    let addr: u16 = (hi as u16) << 8 | (lo as u16);
    let mut cpu: CPU = CPU::custom(0, x, 0, 0, pc, 0, 0, Bus::new());
    cpu.write(pc, base);
    cpu.write(ptr as u16, lo);
    cpu.write(ptr as u16 + 1, hi);
    assert_eq!(cpu.get_address(AddressingMode::IDX), { addr });
}


#[test]
fn idy() {
    let pc: u16 = 0x0301;
    let y: u8 = 0x02;
    let zp_addr: u8 = 0x30;
    let base: u8 = 0x91;
    let ptr: u8 = base + y;
    let addr: u8 = 0x00EF;
    let mut cpu: CPU = CPU::custom(0, 0, y, 0, pc, 0, 0, Bus::new());
    cpu.write(pc, zp_addr);
    cpu.write(zp_addr as u16, base);
    cpu.write(ptr as u16, addr);
    assert_eq!(cpu.get_address(AddressingMode::IDY), addr as u16);
}

#[test]
fn rel() {
    // TODO: how to test relative addressing?
}

#[test]
fn acc() {
    // TODO: how to test accumulator addressing?
}