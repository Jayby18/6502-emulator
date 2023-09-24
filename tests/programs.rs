// use emulatorr::{
//     cpu::{
//         CPU,
//         AddressingMode,
//     },
//     bus::Bus,
// };

// Sum 3 and 5, write result to 0x0202
// A2 03 8E 00 02 A2 05 8E 01 02 A9 00 6D 00 02 6D 01 02 8D 02 02 00
// #[test]
// fn add_numbers() {
//     let mut cpu: CPU = CPU::new(Bus::new());
//     cpu.quick_start(vec![0xA2, 0x03, 0x8E, 0x00, 0x02, 0xA2, 0x05, 0x8E, 0x01, 0x02, 0xA9, 0x00, 0x6D, 0x00, 0x02, 0x02, 0x6D, 0x01, 0x02, 0x8D, 0x02, 0x02, 0x00]);
//     assert_eq!(cpu.read(0x0202), 0x08);
// }

// TODO: ChatGPT's idea for a program
// ```
// A9 01    // LDA #$01 - Load Accumulator with immediate value 0x01
// 85 00    // STA $00 - Store Accumulator to zero page address 0x00
// A2 05    // LDX #$05 - Load X register with immediate value 0x05
// 86 01    // STX $01 - Store X register to zero page address 0x01
// A0 0A    // LDY #$0A - Load Y register with immediate value 0x0A
// 84 02    // STY $02 - Store Y register to zero page address 0x02
// E6 00    // INC $00 - Increment value at zero page address 0x00
// D0 02    // BNE $02 - Branch if Not Equal (i.e., if Zero Flag is clear) to relative address $02
// C6 01    // DEC $01 - Decrement value at zero page address 0x01
// F0 02    // BEQ $02 - Branch if Equal (i.e., if Zero Flag is set) to relative address $02
// A5 00    // LDA $00 - Load Accumulator with value at zero page address 0x00
// 48       // PHA - Push Accumulator onto stack
// A5 01    // LDA $01 - Load Accumulator with value at zero page address 0x01
// 48       // PHA - Push Accumulator onto stack
// A5 02    // LDA $02 - Load Accumulator with value at zero page address 0x02
// 48       // PHA - Push Accumulator onto stack
// 68       // PLA - Pull (pop) Accumulator from stack
// 85 02    // STA $02 - Store Accumulator to zero page address 0x02
// 68       // PLA - Pull (pop) Accumulator from stack
// 85 01    // STA $01 - Store Accumulator to zero page address 0x01
// 68       // PLA - Pull (pop) Accumulator from stack
// 85 00    // STA $00 - Store Accumulator to zero page address 0x00
// ```
// This program loads the Accumulator, X register, and Y register with values, stores them to memory, increments and decrements memory values, branches based on the Zero Flag, and uses the stack to swap memory values.