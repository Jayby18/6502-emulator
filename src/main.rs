// std::env::set_var::("RUST_BACKTRACE", "1");

use std::{
    io,
    thread,
    time::{Duration, Instant},
    sync::mpsc,
};
use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders, BorderType, StatefulWidget, Paragraph, Table, Row, List},
    layout::{Alignment, Layout, Constraint, Direction},
    style::{Color, Modifier, Style},
    Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

pub mod cpu;
pub mod bus;
pub mod opcodes;

#[allow(unused_imports)]
use cpu::CPU;
#[allow(unused_imports)]
use cpu::Flags;
#[allow(unused_imports)]
use bus::Bus;

enum Event<I> {
    Input(I),
    Tick,
}

fn main() -> Result<(), io::Error> {
    let bus: Bus = Bus::new();
    let mut cpu: CPU = CPU::new(bus);
    cpu.write(0x00F1, 0x27);
    cpu.load_program(vec![0xA9, 0x03, 0xA2, 0x10, 0x75, 0xE1, 0x00]);
    cpu.reset();
    // cpu.quick_start(vec![0xA9, 0x03, 0xA2, 0x10, 0x75, 0xE1, 0x00]);

    // Set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Clear terminal
    // terminal.clear()?;

    // User event handler
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move | | {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(| | Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    // Render loop
    loop {
        // Get values from CPU
        let cpu_state = cpu.get_state();

        // Draw terminal
        terminal.draw(|f| {
            // Set size
            let size = f.size();

            // Divide screen into two halves, horizontally
            let halves = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ])
                .split(size);

            // Left half
            let left_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(4),
                    Constraint::Length(4),
                    Constraint::Min(3),
                ])
                .split(halves[0]);

            // Right half
            let right_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ])
                .split(halves[1]);

            // println!("A register: {}", cpu_state[0]);

            let registers = Table::new(vec![
                Row::new(cpu_state.iter().map(|&value| value.to_string()).collect::<Vec<_>>())
            ])
            .header(
                Row::new(vec!["A", "X", "Y", "SP", "PC", "SR", "OP"])
            )
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("CPU state")
                    .border_type(BorderType::Plain)
            );

            f.render_widget(registers, left_layout[0]);

            let test = Paragraph::new(cpu.get_pc().to_string())
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Flags")
                        .border_type(BorderType::Plain)
                );

            f.render_widget(test, left_layout[1]);
        })?;

        // Handle user event
        match rx.recv().unwrap() {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    break;
                },
                KeyCode::Char(' ') => {
                    cpu.advance();
                },
                _ => {
                    
                },
            },
            Event::Tick => {
                
            },
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
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
    fn test_add_to_a() {
        let bus: Bus = Bus::new();
        let mut cpu: CPU = CPU::new(bus);

        {   // no carry, no overflow
            // println!("\nNo carry, no overflow");
            cpu.set_a_reg(0x0A);
            cpu.add_to_a(0x10);
            // println!("{}", cpu.get_a_reg());
            assert_eq!(cpu.get_a_reg(), 0x1A);
            assert!(!cpu.get_flag(Flags::C));
            assert!(!cpu.get_flag(Flags::V));
        }

        {   // no carry -> carry, no overflow
            // println!("\nNo carry -> carry, no overflow");
            cpu.set_a_reg(0xFF);
            cpu.add_to_a(0x01);
            assert_eq!(cpu.get_a_reg(), 0);
            assert!(cpu.get_flag(Flags::C));
            assert!(!cpu.get_flag(Flags::V));
        }

        {   // carry -> no carry, no overflow
            // println!("\nCarry -> no carry, no overflow");
            cpu.set_flag(Flags::C, true);
            cpu.set_a_reg(0x0A);
            cpu.add_to_a(0x10);
            assert_eq!(cpu.get_a_reg(), 0x1A + 0x01);
            assert!(!cpu.get_flag(Flags::C));
            assert!(!cpu.get_flag(Flags::V));
        }

        {   // no carry, no overflow -> overflow
            // println!("\nNo carry, no overflow -> overflow");
            cpu.set_a_reg(0x7F);
            cpu.add_to_a(0x04);
            assert_eq!(cpu.get_a_reg(), 0x83);
            assert!(cpu.get_flag(Flags::V));
        }
    }

    #[test]
    fn test_adc_imm() {
        let bus: Bus = Bus::new();
        let mut cpu: CPU = CPU::new(bus);

        // No carry -> no carry
        {
            cpu.quick_start(vec![0xA9, 0x10, 0x69, 0x02, 0x00]);
            assert_eq!(cpu.get_a_reg(), 0x12);
            assert!(!cpu.get_flag(Flags::C));
            assert!(!cpu.get_flag(Flags::V));
        }

        // Carry -> no carry
        {
            // TODO: how to verify carry?
            cpu.set_flag(Flags::C, true);
            cpu.quick_start(vec![0xA9, 0x10, 0x69, 0x02, 0x00]);
            assert_eq!(cpu.get_a_reg(), 0x12);
            assert!(!cpu.get_flag(Flags::C));
            assert!(!cpu.get_flag(Flags::V));
        }

        // No carry -> carry
        {
            cpu.quick_start(vec![0xA9, 0xFE, 0x69, 0x03, 0x00]);
            assert_eq!(cpu.get_a_reg(), 0x01);
            assert!(cpu.get_flag(Flags::C));
            assert!(!cpu.get_flag(Flags::V));
        }

        // Overflow -> no overflow
        {
            cpu.quick_start(vec![]);
        }

        // No overflow -> overflow
    }

    #[test]
    fn test_and_imm() {
        let bus: Bus = Bus::new();
        let mut cpu: CPU = CPU::new(bus);
        // LDA(IMM) with 0x6b, AND(IMM) with 0x2c
        cpu.quick_start(vec![0xA9, 0x6b, 0x29, 0x2c, 0x00]);

        assert_eq!(cpu.get_a_reg(), 0x28);
    }

    // write number to memory, lda immediate, ldx immediate, then adc with zpx
    #[test]
    fn test_adc_zpx() {
        let bus: Bus = Bus::new();
        let mut cpu: CPU = CPU::new(bus);
        cpu.write(0x00F1, 0x27);
        cpu.quick_start(vec![0xA9, 0x03, 0xA2, 0x10, 0x75, 0xE1, 0x00]);

        assert_eq!(cpu.get_a_reg(), 0x2A);
    }

    #[test]
    fn test_asl_acc() {
        let bus: Bus = Bus::new();
        let mut cpu: CPU = CPU::new(bus);

        {
            cpu.quick_start(vec![0xA9, 0b0010_1000, 0x0A, 0x00]);
            assert_eq!(cpu.get_a_reg(), 0b0101_0000);
            assert!(!cpu.get_flag(Flags::C));
            assert!(!cpu.get_flag(Flags::N));
            assert!(!cpu.get_flag(Flags::Z));
        }

        {
            cpu.quick_start(vec![0xA9, 0b1010_0000, 0x0A, 0x00]);
            assert_eq!(cpu.get_a_reg(), 0b0100_0000);
            assert!(cpu.get_flag(Flags::C));
            // println!("A: {}", cpu.get_a_reg());
        }

        {
            cpu.quick_start(vec![0xA9, 0b1000_0000, 0x0A, 0x00]);
            assert_eq!(cpu.get_a_reg(), 0x00);
            assert!(cpu.get_flag(Flags::C));
            assert!(cpu.get_flag(Flags::Z));
            assert!(!cpu.get_flag(Flags::N));
        }

        {
            cpu.quick_start(vec![0xA9, 0b0100_0000, 0x0A, 0x00]);
            assert_eq!(cpu.get_a_reg(), 0b1000_0000);
            assert!(cpu.get_flag(Flags::N));
            assert!(!cpu.get_flag(Flags::Z));
            assert!(!cpu.get_flag(Flags::C));
        }
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

    // TODO: test ASL with different mode

    // TODO: test all addressing modes (should be relatively simple, though, might not be necessary)
    // #[test]
    // fn test_addressing_modes() {
    //     let bus: Bus = Bus::new();
    //     let mut cpu: CPU = CPU::new(bus);
    //     use cpu::AddressingMode;
    //     assert_eq!(cpu.get_address(AddressingMode::IMM), cpu.get_pc() - 1);
    // }

    // TODO: test all (?) instructions

    // TODO: test all opcodes (or is it too much?)
}
