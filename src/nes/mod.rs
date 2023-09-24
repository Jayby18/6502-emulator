use std::path::PathBuf;
use crate::{core::cpu::CPU, io};

const HEADER_SIZE: usize = 16;

#[allow(non_camel_case_types)]
enum ROMFormat {
    NES2,
    iNES,
    INVALID,
}

pub fn load_nes_rom(cpu: &mut CPU, path: &PathBuf) -> Result<(), std::io::Error> {
    // Load ROM from file
    // TODO: get file as CLI argument
    let rom: Vec<u8> = io::load_rom(path)?;
    let header: &[u8] = &rom[0..HEADER_SIZE];
    let mut format: ROMFormat = ROMFormat::INVALID;

    // Check ROM format
    if header[0..4] == [0x4E, 0x45, 0x53, 0x1A] {
        if header[7] & 0x0C == 0x08 {
            println!("NES2.0");
            format = ROMFormat::NES2;
        } else {
            println!("iNES");
            format = ROMFormat::iNES;
        }
    }

    match format {
        ROMFormat::NES2 => {},
        ROMFormat::iNES => {
            // Check if trainer area is present
            let mut trainer_size: usize = 0;
            if header[6] & 0b10 == 0b10 {
                trainer_size = 512;
            }

            // Get PRG and CHR rom sizes
            let prg_rom_size: usize = (header[4] as usize + (header[9] as usize & 0b1111 << 7)) * 16384;
            let chr_rom_size: usize = (header[5] as usize + (header[9] as usize & 0b1111_0000 << 7)) * 8192;

            let trainer: Vec<_> = rom.iter()
                .skip(HEADER_SIZE)
                .take(trainer_size)
                .collect();

            for i in 0..trainer_size {
                cpu.write(0x7000 + i as u16, *trainer[i]);
            }

            let prg_rom: Vec<_> = rom.iter()
                .skip(HEADER_SIZE + trainer_size)
                .take(prg_rom_size)
                .collect();

            for i in 0..prg_rom_size {
                cpu.write(0x8000 + i as u16, *prg_rom[i]);
            }

            let chr_rom: Vec<_> = rom.iter()
                .skip(HEADER_SIZE + trainer_size + chr_rom_size)
                .take(chr_rom_size)
                .collect();

            for i in 0..chr_rom_size {
                cpu.write(i as u16, *chr_rom[i]);
            }
        },
        _ => {}
    }

    Ok(())
}
