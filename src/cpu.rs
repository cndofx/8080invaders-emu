use crate::instructions::Instruction;

const MEMORY_SIZE: usize = 0x4000;

pub struct Cpu {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: usize,
    pub pc: usize,
    memory: Box<[u8]>,
    flags: Flags,
}

#[derive(Clone, Copy)]
struct Flags {
    z: bool, // zero
    s: bool, // sign
    p: bool, // parity
    cy: bool, // carry
             // ac: bool, // aux carry
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
            memory: vec![0; MEMORY_SIZE].into_boxed_slice(),
            flags: Flags {
                z: false,
                s: false,
                p: false,
                cy: false,
                // ac: false,
            },
        }
    }

    /// load data into memory at the specified address
    pub fn load(&mut self, data: &[u8], address: usize) {
        self.memory[address..address + data.len()].copy_from_slice(data);
    }

    /// fetch instruction at the current program counter
    pub fn fetch(&self) -> (Instruction, usize) {
        Instruction::disassemble(&self.memory, self.pc)
    }


    // todo: find a better way to handling incrementing PC when executing instructions
    // jumps and calls are probably an issue
    /// execute given instruction and increments the program counter
    // pub fn execute(&mut self, instruction: Instruction, instr_len: usize) {
    pub fn execute(&mut self, instruction: Instruction) {
        use Instruction::*;
        match instruction {
            NOP => {
                self.pc += 1;
            }
            ADD_M => {
                let addr = (((self.h as u16) << 8) | (self.h as u16)) as usize;
                let res = (self.a as u16) + (self.memory[addr] as u16);
                self.flags = Flags::get(res);
                self.a = (res & 0xFF) as u8;
                self.pc += 1;
            }
            ADI_D8(d8) => {
                let res = (self.a as u16) + (d8 as u16);
                self.flags = Flags::get(res);
                self.a = (res & 0xFF) as u8;
                self.pc += 2;
            }
            JMP_ADR(adr) => {
                self.pc = adr as usize;
            }
            LXI_SP_D16(d16) => {
                self.sp = d16 as usize;
                self.pc += 3;
            }
            MVI_B_D8(d8) => {
                self.b = d8;
                self.pc += 2;
            }
            _ => panic!("unimplemented instruction: {:X?}", instruction),
        }
    }

    pub fn print_state(&self) {
        println!("CPU State:");
        println!("PC = {:#06X}, SP = {:#06X}", self.pc, self.sp);
        println!("  Registers:");
        println!("    a = {:#04X}, b = {:#04X}, c = {:#04X}, d = {:#04X}", self.a, self.b, self.c, self.d);
        println!("    e = {:#04X}, h = {:#04X}, l = {:#04X}", self.e , self.h, self.l);
        println!("  Flags:");
        println!("    z = {}, s = {}", self.flags.z, self.flags.s);
        println!("    p = {}, cy = {}\n", self.flags.p, self.flags.cy);
    }

    fn _push_stack(&mut self, value: u16) {
        let bytes = value.to_le_bytes();
        self.memory[self.sp - 2..self.sp].copy_from_slice(&bytes);
        self.sp -= 2;
    }

    fn _pop_stack(&mut self) -> u16 {
        let value = ((self.memory[self.sp + 1] as u16) << 8) | (self.memory[self.sp] as u16);
        self.sp += 2;
        value
    }
}

impl Flags {
    fn get(result: u16) -> Self {
        Flags {
            z: (result & 0xFF) == 0,
            s: (result & 0x80) != 0,
            p: parity(result as u8),
            cy: result > 0xFF,
        }
    }
}

fn parity(n: u8) -> bool {
    n.count_ones() % 2 == 0
}

// fn parity_u16(n: u16) -> bool {
//     n.count_ones() % 2 == 0
// }

#[cfg(test)]
mod tests {
    use super::parity;

    #[test]
    fn test_parity() {
        let odd = 0b00000001;
        assert_eq!(parity(odd), false);
        let odd = 0b00010000;
        assert_eq!(parity(odd), false);
        let odd = 0b01010111;
        assert_eq!(parity(odd), false);

        // let odd = 0b0000000000000001;
        // assert_eq!(parity_u16(odd), false);
        // let odd = 0b0100000000000000;
        // assert_eq!(parity_u16(odd), false);
        // let odd = 0b0000100010000001;
        // assert_eq!(parity_u16(odd), false);

        let even = 0b00000011;
        assert_eq!(parity(even), true);
        let even = 0b11000000;
        assert_eq!(parity(even), true);
        let even = 0b11001111;
        assert_eq!(parity(even), true);

        // let even = 0b0000000000000011;
        // assert_eq!(parity_u16(even), true);
        // let even = 0b0100001000000110;
        // assert_eq!(parity_u16(even), true);
        // let even = 0b0000100110101001;
        // assert_eq!(parity_u16(even), true);
    }
}
