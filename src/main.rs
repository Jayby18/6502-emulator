// std::env::set_var::("RUST_BACKTRACE", "1");

pub mod cpu;
pub mod bus;
pub mod opcodes;

#[allow(unused_imports)]
use cpu::CPU;
#[allow(unused_imports)]
use cpu::Flags;

#[allow(unused_imports)]
use bus::Bus;

fn main() {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lda_imm_flags() {
        let bus: Bus = Bus::new();
        let mut cpu: CPU = CPU::new(bus);
        cpu.quick_start(vec![0xA9, 0x05, 0x00]);

        assert_eq!(cpu.get_a_reg(), 0x05);
        assert!(!cpu.get_flag(Flags::Z));
        assert!(!cpu.get_flag(Flags::N));
    }

    #[test]
    fn test_lda_imm_zero_flag() {
        let bus: Bus = Bus::new();
        let mut cpu: CPU = CPU::new(bus);
        cpu.quick_start(vec![0xA9, 0x00, 0x00]);

        assert!(cpu.get_flag(Flags::Z));
    }

    #[test]
    fn test_lda_zp0() {
        let bus: Bus = Bus::new();
        let mut cpu: CPU = CPU::new(bus);
        // write 0x55 to address 0x10
        cpu.write(0x10, 0x55);
        // LDA from address 0x10
        cpu.quick_start(vec![0xA5, 0x10, 0x00]);

        assert_eq!(cpu.get_a_reg(), 0x55);
    }

    #[test]
    fn test_adc() {
        let bus: Bus = Bus::new();
        let mut cpu: CPU = CPU::new(bus);

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
            cpu.set_a_reg(0x7F);
            cpu.add_to_a(0x04);
            assert_eq!(cpu.get_a_reg(), 0x83);
            assert!(cpu.get_flag(Flags::V));
        }
    }
}
