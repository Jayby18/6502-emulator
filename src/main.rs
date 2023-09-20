// std::env::set_var::("RUST_BACKTRACE", "1");

use std::{
    thread,
    time::{Duration, Instant},
    sync::mpsc,
};
use ratatui::{
    backend::CrosstermBackend,
    widgets::*,
    layout::{Layout, Constraint, Direction},
    style::{Color, Style},
    Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use dirs;

mod cpu;
mod bus;

use cpu::CPU;
use cpu::Flags;
use bus::Bus;

mod io;

enum Event<I> {
    Input(I),
    Tick,
}

#[allow(unused_variables)]
fn main() -> Result<(), std::io::Error> {
    // Init bus and CPU
    let bus: Bus = Bus::new();
    let mut cpu: CPU = CPU::new(bus);
    cpu.write(0x00F1, 0x27);

    let program = io::load_bytes(&dirs::home_dir().unwrap().join("stack.txt"))?;
    cpu.load_program(program);
    
    cpu.reset();

    // Set up terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Clear terminal
    terminal.clear()?;

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

            if last_tick.elapsed() >= tick_rate && tx.send(Event::Tick).is_ok() {
                last_tick = Instant::now();
            }
        }
    });

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

            // TODO: proper implementation
            // For now, I just write to memory starting at address 0x3000.
            // In NES, there are 32 horizontal and 30 vertical tiles, each 8x8 pixels.
            // Each tile gets a memory address (starting left top), and the bits correspond to the pixels.
            // Here, I'll start with 8 by 8 virtual pixels.

            let v_pixels: Vec<u8> = vec![
                0, 0, 1, 1, 1, 1, 0, 0,
                0, 0, 0, 1, 1, 0, 0, 0,
                0, 0, 1, 0, 0, 1, 0, 0,
                0, 1, 0, 0, 0, 0, 1, 1,
                0, 1, 1, 1, 1, 1, 1, 1,
                0, 1, 1, 1, 1, 1, 1, 1,
                0, 0, 1, 1, 1, 1, 1, 0,
                0, 0, 0, 1, 1, 0, 0, 0,
            ];

            let v_pixel_array: Vec<Vec<u8>> = v_pixels.chunks(8).map(|chunk| chunk.to_vec()).collect();
            
            // TODO: Display (PPU)

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
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}
