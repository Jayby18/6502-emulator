// std::env::set_var::("RUST_BACKTRACE", "1");

pub mod cpu;
pub mod bus;
pub mod opcodes;

use cpu::CPU;
#[allow(unused_imports)]
use cpu::Flags;

use bus::Bus;

fn main() {
    println!("Hello world");
    let bus: Bus = Bus::new();
    let _cpu: CPU = CPU::new(bus);
    // TODO: what kind of reference to bus to pass into cpu?
}

#[cfg(test)]
mod test {
    use super::*;

    // Functions to init, load and run CPU
    fn init_cpu() -> CPU {
        let bus: Bus = Bus::new();
        let mut cpu: CPU = CPU::new(bus);

        cpu.write(0xFFFD, 0x60);
        cpu.write(0xFFFC, 0x00);

        cpu
    }

    fn load_program(cpu: &mut CPU, program: Vec<u8>) {
        for i in 0..program.len() {
            cpu.write(0x0600 + (i as u16), program[i]);
        }
    }

    fn start_cpu(cpu: &mut CPU) {
        cpu.reset();
        cpu.clock();
    }

    fn run_program(program: Vec<u8>) -> CPU {
        let mut cpu = init_cpu();
        load_program(&mut cpu, program);
        start_cpu(&mut cpu);
        cpu
    }

    #[test]
    fn test_0xa9_flags() {
        let mut cpu: CPU = run_program(vec![0xA9, 0x05, 0x00]);

        assert_eq!(cpu.get_a_reg(), 0x05);
        assert!(!cpu.get_flag(Flags::Z));
        assert!(!cpu.get_flag(Flags::N));
    }

    #[test]
    fn test_0xa9_zero_flag() {
        let mut cpu: CPU = run_program(vec![0xA9, 0x00, 0x00]);

        println!("Zero: {0:2X}", Flags::Z as u8);
        println!("Zero flag: {}", cpu.get_flag(Flags::Z));
        assert!(cpu.get_flag(Flags::Z));
    }

    #[test]
    fn test_lda_from_memory() {
        let program = vec![0xA5, 0x10, 0x00];
    
        let cpu = run_program(program);

        assert_eq!(cpu.get_a_reg(), 0x55);
    }

    #[test]
    fn test_adc() {
        let mut cpu: CPU = init_cpu();

        {   // no carry, no overflow
            println!("\nNo carry, no overflow");
            cpu.set_a_reg(0x0A);
            cpu.add_to_a(0x10);
            println!("{}", cpu.get_a_reg());
            assert_eq!(cpu.get_a_reg(), 0x1A);
            assert!(!cpu.get_flag(Flags::C));
            assert!(!cpu.get_flag(Flags::V));
        }

        {   // no carry -> carry, no overflow
            println!("\nNo carry -> carry, no overflow");
            cpu.set_a_reg(0xFF);
            cpu.add_to_a(0x01);
            assert_eq!(cpu.get_a_reg(), 0);
            assert!(cpu.get_flag(Flags::C));
            assert!(!cpu.get_flag(Flags::V));
        }

        {   // carry -> no carry, no overflow
            println!("\nCarry -> no carry, no overflow");
            cpu.set_flag(Flags::C, true);
            cpu.set_a_reg(0x0A);
            cpu.add_to_a(0x10);
            assert_eq!(cpu.get_a_reg(), 0x1A + 0x01);
            assert!(!cpu.get_flag(Flags::C));
            assert!(!cpu.get_flag(Flags::V));
        }

        {   // no carry, no overflow -> overflow
            println!("\nNo carry, no overflow -> overflow");
            cpu.set_flag(Flags::V, true);
            cpu.set_a_reg(0x0A);
            cpu.add_to_a(0x10);
            assert_eq!(cpu.get_a_reg(), 0x1A);
        }
    }
}
