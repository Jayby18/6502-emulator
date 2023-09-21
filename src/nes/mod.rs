use crate::core::{
    cpu::CPU,
    bus::Bus,
};

fn init() {
    let mut bus: Bus = Bus::new();
    let mut cpu: CPU = CPU::new(bus);
}