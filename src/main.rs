pub mod cpu;
pub mod bus;
pub mod opcodes;

use cpu::CPU;
use cpu::Flags;

use bus::Bus;

fn main() {
    println!("Hello world");
    let bus: Bus = Bus::new();
    let mut cpu: CPU = CPU::new(bus);
    // TODO: what kind of reference to bus to pass into cpu?
}

#[cfg(test)]
mod test {
    use super::*;

    fn run_program(program: Vec<u8>) -> CPU {
        let bus: Bus = Bus::new();
        let mut cpu: CPU = CPU::new(bus);
        let mut addr = 0x0600;
    
        // Write instructions to RAM
        for i in 0..program.len() {
            cpu.write(addr + (i as u16), program[i]);
        }
    
        // Point reset instruction to program at 0x0600
        // TODO: flip endianness
        cpu.write(0xFFFC, 0x06);
        cpu.write(0xFFFD, 0x00);
    
        // Reset and start clock
        cpu.reset();
        cpu.clock();

        println!("\n SR: {:08b}", cpu.get_status());
    
        return cpu;
    }

    #[test]
    fn test_0xa9_flags() {
        let mut cpu: CPU = run_program(vec![0xA9, 0x05, 0x00]);

        assert_eq!(cpu.get_a_reg(), 0x05);
        assert_eq!(cpu.get_flag(Flags::Z), false);
        assert_eq!(cpu.get_flag(Flags::N), false);
    }

    #[test]
    fn test_0xa9_zero_flag() {
        let mut cpu: CPU = run_program(vec![0xA9, 0x00, 0x00]);

        println!("Zero: {0:2X}", Flags::Z as u8);
        println!("Zero flag: {}", cpu.get_flag(Flags::Z));
        assert_eq!(cpu.get_flag(Flags::Z), true);
    }
}
