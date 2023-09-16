# 6502 emulator

Written in Rust.

## To do

### Backend

- [x] Implement all addressing modes
- [ ] Test all addressing modes
- [ ] Implement all instructions (DOING)
- [ ] Test all instructions

### Frontend

- [ ] Create simple CLI for basic functionality
- [ ] Read from ROM files
- [ ] TUI (DOING)
    - [x] Registers
    - [x] Flags
    - [ ] Memory
        - [x] View memory
        - [ ] Scroll memory
        - [ ] Edit memory
    - [ ] Display (for NES PPU)
    - [ ] Menus (to emulation mode)

## Specific system emulation

### Nintendo Entertainment System / Famicom

See [NESdev Wiki](https://www.nesdev.org/wiki/Nesdev_Wiki) for more.

#### CPU memory map

- 0x0000-0x07FF: 2KB internal RAM (as used in Bus.rs)
- 0x0800-0x1FFF: mirrors of internal RAM
- 0x2000-0x2007: PPU registers
- 0x2008-0x3FFF: mirrors of PPU registers
- 0x4000-0x4017: APU and I/O registers
- 0x4018-0x401F: normally disabled (APU and I/O functionality)
- 0x4020-0xFFFF: cartridge space (PRG ROM/RAM and mapper registers)

##### CPU-PPU registers

8 registers from 0x2000 to 0x2007, mirrored beyond that.

- 0x2000: PPUCTRL
- 0x2001: PPUMASK
- 0x2002: PPUSTATUS
- 0x2003: OAMADDR
- 0x2004: OAMDATA
- 0x2005: PPUSCROLL
- 0x2006: PPUADDR
- 0x2007: PPUDATA
- 0x4014: OAMDMA

#### PPU memory map

- 0x0000-0x1FFF: pattern memory ("CHR ROM")
- 0x2000-0x3EFF: nametable memory ("VRAM")
- 0x3F00-0x3FFF: palette memory
