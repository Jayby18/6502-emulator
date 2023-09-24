use ratatui::{
    widgets::*,
    layout::{Layout, Constraint, Direction},
    style::{Color, Style},
};
use crossterm::{
    event::KeyCode,
    execute,
};

mod core;
mod io;
mod nes;

use core::{
    cpu::{CPU, Flags},
    bus::Bus,
};

enum Event<I> {
    Input(I),
    Tick,
}

#[allow(unused_variables)]
fn main() -> Result<(), std::io::Error> {
    // Init bus and CPU
    let bus: Bus = Bus::new();
    let mut cpu: CPU = CPU::new(bus);

    // Get CLI arguments
    let args: Vec<String> = std::env::args().collect();

    nes::load_nes_rom(&mut cpu, &dirs::home_dir().unwrap().join(args[1].clone()))?;
    cpu.reset();

    // cpu.write(0x00F1, 0x27);
    // let program = io::load_bytes(&dirs::home_dir().unwrap().join("stack.txt"))?;
    // cpu.load_program(program);
    // cpu.reset();

    cpu.reset();

    // Set up terminal
    let (mut terminal, rx) = stdr::setup_terminal!();

    // Render loop
    loop {
        // Get values from CPU
        let cpu_state = cpu.get_state();
        let mem = cpu.get_memory();

        // Draw terminal
        terminal.draw(|f| {
            // Set size
            let size = f.size();
            #[allow(clippy::needless_late_init)]
            let display_width;
            if (size.width / 2) % 2 == 0 {
                display_width = size.width / 2;
            } else {
                display_width = (size.width / 2) - 1;
            }
            let display_height = display_width / 256 * 240;

            // Divide screen into two halves, horizontally
            let halves = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(50),
                    Constraint::Length(display_width),
                ])
                .split(size);

            // Left half
            let left_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(4),
                    Constraint::Length(3),
                    Constraint::Min(3),
                ])
                .split(halves[0]);

            // Right half
            let right_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(20),
                    Constraint::Min(3),
                    Constraint::Length(6),
                ])
                .split(halves[1]);

            // println!("A register: {}", cpu_state[0]);

            // Register table
            let registers = Table::new(vec![
                Row::new(vec!["A", "X", "Y", "SP", "PC", "SR", "OP"]),
                Row::new(cpu_state.iter().cloned().map(|value| format!("0x{:02X}", value).to_string()).collect::<Vec<_>>())
            ])
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Registers")
            )
            .widths(&[
                Constraint::Percentage(13),
                Constraint::Percentage(13),
                Constraint::Percentage(13),
                Constraint::Percentage(13),
                Constraint::Percentage(13),
                Constraint::Percentage(13),
                Constraint::Percentage(13)
            ])
            .column_spacing(1);
            f.render_widget(registers, left_layout[0]);

            // Status flags
            let flags = Table::new(vec![
                Row::new(["C", "Z", "I", "D", "B", "U", "V", "N"]
                    .iter()
                    .map(|&flag| {
                        if cpu_state[5] & Flags::byte_from_str(flag) as u16 == 1 {
                            Cell::from(flag).style(Style::default().bg(Color::White).fg(Color::Black))
                        } else {
                            Cell::from(flag)
                        }
                    })
                ),
                // Row::new(format!("{:08b}", cpu_state[5]).chars().map(|c| c.to_string()).collect::<Vec<_>>())
            ])
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Flags")
            )
            .widths(&[
                Constraint::Percentage(12),
                Constraint::Percentage(12),
                Constraint::Percentage(12),
                Constraint::Percentage(12),
                Constraint::Percentage(12),
                Constraint::Percentage(12),
                Constraint::Percentage(12),
                Constraint::Percentage(12)
            ])
            .column_spacing(1);
            f.render_widget(flags, left_layout[1]);

            // Memory (from program start)
            let program: Vec<u8> = mem.iter().cloned().skip(0x0600).collect::<Vec<_>>();
            let indices: Vec<u16> = (0..program.len() as u16).collect();

            let program_list = Table::new(
                indices
                    .iter()
                    .map(|i| {
                        if indices[*i as usize] + 0x0600 == cpu_state[4] {
                            Row::new(vec![format!("0x{:04X}", 0x0600 + i), format!("0x{:02X}",program[*i as usize]).to_string()])
                                .style(Style::default().bg(Color::White).fg(Color::Black))
                        } else if program[*i as usize] == 0x00 {
                            Row::new(vec![format!("0x{:04X}", 0x0600 + i), "----".to_string()])
                        } else {
                            Row::new(vec![format!("0x{:04X}", 0x0600 + i), format!("0x{:02X}",program[*i as usize]).to_string()])
                        }
                    })
                )
                .header(
                    Row::new(vec!["Address", "Value"])
                )
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Memory (from program start)")
                )
                .widths(&[
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ])
                .column_spacing(1);
            f.render_widget(program_list, left_layout[2]);

            let stack: Vec<u8> = mem.iter().cloned().skip(0x0100).take(0xFF).collect::<Vec<_>>();
            let indices: Vec<u8> = (0..stack.len() as u8).rev().collect();

            let stack_list = Table::new(
                indices
                    .iter()
                    .map(|i| {
                        if (indices[*i as usize]) as u16 == cpu_state[3] {
                            Row::new(vec![format!("0x{:04X}", 0x0100 + *i as u16), format!("0x{0:2X}", stack[*i as usize]).to_string()])
                                .style(Style::default().bg(Color::White).fg(Color::Black))
                        } else if stack[*i as usize] == 0x00 {
                            Row::new(vec![format!("0x{0:4X}", 0x0100 + *i as u16), "----".to_string()])
                        } else {
                            Row::new(vec![format!("0x{0:4X}", 0x0100 + *i as u16), format!("0x{0:2X}", stack[*i as usize]).to_string()])
                        }
                    })
                )
                .header(
                    Row::new(vec!["Address", "Value"])
                    )
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Stack")
                    )
                .widths(&[
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                ])
                .column_spacing(1);
            f.render_widget(stack_list, right_layout[0]);

            // Help
            let help = Paragraph::new("<space>: advance to next cycle\n<enter>: start clock\nr: reset CPU\nq: quit application")
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Help")
                );
            f.render_widget(help, right_layout[2]);
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
                KeyCode::Char('r') => {
                    cpu.reset();
                },
                KeyCode::Enter => {
                    cpu.clock();
                },
                _ => {
                    
                },
            },
            Event::Tick => {
                
            },
        }
    }

    // Restore terminal
    stdr::restore_terminal!(terminal);

    Ok(())
}
