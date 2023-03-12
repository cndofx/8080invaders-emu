use std::io::Read;

// http://www.emulator101.com/reference/8080-by-opcode.html

fn main() {
    let mut rom = std::fs::File::open("rom/invaders.h").unwrap();
    let mut data: Vec<u8> = Vec::new();
    rom.read_to_end(&mut data).unwrap();

    let mut pc = 0;
    while pc < data.len() {
        pc += disassemble(&data, pc);
    }
}

/// returns the size of the disassembled instruction
fn disassemble(data: &[u8], pc: usize) -> usize {
    print!("{:#06X} - ", pc);
    let d8 = data[pc + 1];
    let d16 = ((data[pc + 2] as u16) << 8) | data[pc + 1] as u16;
    match data[pc] {
        0x00 => {
            println!("NOP");
            1
        }
        0x01 => {
            println!("LXI {:#06X}", d16);
            3
        }
        0x21 => {
            println!("LXI H,{:#06X}", d16);
            3
        }
        0x32 => {
            println!("STA {:#06X}", d16);
            3
        }
        0x35 => {
            println!("DCR M");
            1
        }
        0x3E => {
            println!("MVI A,{:#04X}", d8);
            2
        }
        0x72 => {
            println!("MOV M,D");
            1
        }
        0xC3 => {
            println!("JMP {:#06X}", d16);
            3
        }
        0xC5 => {
            println!("PUSH B");
            1
        }
        0xCD => {
            println!("CALL {:#06X}", d16);
            3
        }
        0xD5 => {
            println!("PUSH D");
            1
        }
        0xE5 => {
            println!("PUSH H");
            1
        }
        0xF5 => {
            println!("PUSH PSW");
            1
        }
        _ => panic!("unimplemented instruction: {:#04X}", data[pc]),
    }
}
