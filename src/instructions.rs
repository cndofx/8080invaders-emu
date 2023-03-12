#[derive(Debug)]
pub enum Instruction {
    NOP,            // 0x00
    LXI_B_D16(u16), // 0x01
    STAX_B,         // 0x02
    INX_B,          // 0x03
    INR_B,          // 0x04
    DCR_B,          // 0x05
    MVI_B_D8(u8),   // 0x06
    RLC,            // 0x07

    DAD_B,        // 0x09
    LDAX_B,       // 0x0A
    DCX_B,        // 0x0B
    INR_C,        // 0x0C
    DCR_C,        // 0x0D
    MVI_C_D8(u8), // 0x0E
    RRC,          // 0x0F

    LXI_D_D16(u16), // 0x11
    STAX_D,         // 0x12
    INX_D,          // 0x13
    INR_D,          // 0x14
    DCR_D,          // 0x15
    MVI_D_D8(u8),   // 0x16
    RAL,            // 0x17

    DAD_D,        // 0x19
    LDAX_D,       // 0x1A
    DCX_D,        // 0x1B
    INR_E,        // 0x1C
    DCR_E,        // 0x1D
    MVI_E_D8(u8), // 0x1E
    RAR,          // 0x1F

    LXI_H_D16(u16), // 0x21
    SHLD_ADR(u16),  // 0x22
    INX_H,          // 0x23
    INR_H,          // 0x24
    DCR_H,          // 0x25
    MVI_H_D8(u8),   // 0x26
    DAA,            // 0x27

    DAD_H,         // 0x29
    LHLD_ADR(u16), // 0x2A
    DCX_H,         // 0x2B
    INR_L,         // 0x2C
    DCR_L,         // 0x2D
    MVI_L_D8(u8),  // 0x2E
    CMA,           // 0x2F

    LXI_SP_D16(u16), // 0x31
    STA_ADR(u16),    // 0x32
    INX_SP,          // 0x33
    INR_M,           // 0x34
    DCR_M,           // 0x35
    MVI_M_D8(u8),    // 0x36
    STC,             // 0x37

    DAD_SP,       // 0x39
    LDA_ADR(u16), // 0x3A
    DCX_SP,       // 0x3B
    INR_A,        // 0x3C
    DCR_A,        // 0x3D
    MVI_A_D8(u8), // 0x3E
    CMC,          // 0x3F

    MOV_B_B,
    MOV_B_C,
    MOV_B_D,
    MOV_B_E,
    MOV_B_H,
    MOV_B_L,
    MOV_B_M,
    MOV_B_A,

    MOV_C_B,
    MOV_C_C,
    MOV_C_D,
    MOV_C_E,
    MOV_C_H,
    MOV_C_L,
    MOV_C_M,
    MOV_C_A,

    MOV_D_B,
    MOV_D_C,
    MOV_D_D,
    MOV_D_E,
    MOV_D_H,
    MOV_D_L,
    MOV_D_M,
    MOV_D_A,

    MOV_E_B,
    MOV_E_C,
    MOV_E_D,
    MOV_E_E,
    MOV_E_H,
    MOV_E_L,
    MOV_E_M,
    MOV_E_A,

    MOV_H_B,
    MOV_H_C,
    MOV_H_D,
    MOV_H_E,
    MOV_H_H,
    MOV_H_L,
    MOV_H_M,
    MOV_H_A,

    MOV_L_B,
    MOV_L_C,
    MOV_L_D,
    MOV_L_E,
    MOV_L_H,
    MOV_L_L,
    MOV_L_M,
    MOV_L_A,

    MOV_M_B,
    MOV_M_C,
    MOV_M_D,
    MOV_M_E,
    MOV_M_H,
    MOV_M_L,

    HLT,

    // MOV_M_M,
    MOV_M_A,

    MOV_A_B,
    MOV_A_C,
    MOV_A_D,
    MOV_A_E,
    MOV_A_H,
    MOV_A_L,
    MOV_A_M,
    MOV_A_A,
}

impl Instruction {
    // todo: error handling
    /// returns an `Instruction` along with it's size
    fn disassemble(data: &[u8], pc: usize) -> (Self, usize) {
        let d8 = data[pc + 1];
        let d16 = ((data[pc + 2] as u16) << 8) | data[pc + 1] as u16;
        match data[pc] {
            0x00 => (Instruction::NOP, 1),
            0x01 => (Instruction::LXI_B_D16(d16), 3),
            0x02 => (Instruction::STAX_B, 1),
            0x03 => (Instruction::INX_B, 1),
            0x04 => (Instruction::INR_B, 1),
            0x05 => (Instruction::DCR_B, 1),
            0x06 => (Instruction::MVI_B_D8(d8), 2),
            0x07 => (Instruction::RLC, 1),

            0x09 => (Instruction::DAD_B, 1),
            0x0A => (Instruction::LDAX_B, 1),
            0x0B => (Instruction::DCX_B, 1),
            0x0C => (Instruction::INR_C, 1),
            0x0D => (Instruction::DCR_C, 1),
            0x0E => (Instruction::MVI_C_D8(d8), 2),
            0x0F => (Instruction::RRC, 1),

            0x11 => (Instruction::LXI_D_D16(d16), 3),
            0x12 => (Instruction::STAX_D, 1),
            0x13 => (Instruction::INX_D, 1),
            0x14 => (Instruction::INR_D, 1),
            0x15 => (Instruction::DCR_D, 1),
            0x16 => (Instruction::MVI_D_D8(d8), 2),
            0x17 => (Instruction::RAL, 1),

            0x19 => (Instruction::DAD_D, 1),
            0x1A => (Instruction::LDAX_D, 1),
            0x1B => (Instruction::DCX_D, 1),
            0x1C => (Instruction::INR_E, 1),
            0x1D => (Instruction::DCR_E, 1),
            0x1E => (Instruction::MVI_E_D8(d8), 2),
            0x1F => (Instruction::RAR, 1),

            0x21 => (Instruction::LXI_H_D16(d16), 3),
            0x22 => (Instruction::SHLD_ADR(d16), 3),
            0x23 => (Instruction::INX_H, 1),
            0x24 => (Instruction::INR_H, 1),
            0x25 => (Instruction::DCR_H, 1),
            0x26 => (Instruction::MVI_H_D8(d8), 2),
            0x27 => (Instruction::DAA, 1),

            0x29 => (Instruction::DAD_H, 1),
            0x2A => (Instruction::LHLD_ADR(d16), 3),
            0x2B => (Instruction::DCX_H, 1),
            0x2C => (Instruction::INR_L, 1),
            0x2D => (Instruction::DCR_L, 1),
            0x2E => (Instruction::MVI_L_D8(d8), 2),
            0x2F => (Instruction::CMA, 1),

            0x31 => (Instruction::LXI_SP_D16(d16), 3),
            0x32 => (Instruction::STA_ADR(d16), 3),
            0x33 => (Instruction::INX_SP, 1),
            0x34 => (Instruction::INR_M, 1),
            0x35 => (Instruction::DCR_M, 1),
            0x36 => (Instruction::MVI_M_D8(d8), 2),
            0x37 => (Instruction::STC, 1),

            0x39 => (Instruction::DAD_SP, 1),
            0x3A => (Instruction::LDA_ADR(d16), 3),
            0x3B => (Instruction::DCX_SP, 1),
            0x3C => (Instruction::INR_A, 1),
            0x3D => (Instruction::DCR_A, 1),
            0x3E => (Instruction::MVI_A_D8(d8), 2),
            0x3F => (Instruction::CMC, 1),

            0x40 => (Instruction::MOV_B_B, 1),
            0x41 => (Instruction::MOV_B_C, 1),
            0x42 => (Instruction::MOV_B_D, 1),
            0x43 => (Instruction::MOV_B_E, 1),
            0x44 => (Instruction::MOV_B_H, 1),
            0x45 => (Instruction::MOV_B_L, 1),
            0x46 => (Instruction::MOV_B_M, 1),
            0x47 => (Instruction::MOV_B_A, 1),

            0x48 => (Instruction::MOV_C_B, 1),
            0x49 => (Instruction::MOV_C_C, 1),
            0x4A => (Instruction::MOV_C_D, 1),
            0x4B => (Instruction::MOV_C_E, 1),
            0x4C => (Instruction::MOV_C_H, 1),
            0x4D => (Instruction::MOV_C_L, 1),
            0x4E => (Instruction::MOV_C_M, 1),
            0x4F => (Instruction::MOV_C_A, 1),

            0x50 => (Instruction::MOV_D_B, 1),
            0x51 => (Instruction::MOV_D_C, 1),
            0x52 => (Instruction::MOV_D_D, 1),
            0x53 => (Instruction::MOV_D_E, 1),
            0x54 => (Instruction::MOV_D_H, 1),
            0x55 => (Instruction::MOV_D_L, 1),
            0x56 => (Instruction::MOV_D_M, 1),
            0x57 => (Instruction::MOV_D_A, 1),
            
            0x58 => (Instruction::MOV_E_B, 1),
            0x59 => (Instruction::MOV_E_C, 1),
            0x5A => (Instruction::MOV_E_D, 1),
            0x5B => (Instruction::MOV_E_E, 1),
            0x5C => (Instruction::MOV_E_H, 1),
            0x5D => (Instruction::MOV_E_L, 1),
            0x5E => (Instruction::MOV_E_M, 1),
            0x5F => (Instruction::MOV_E_A, 1),

            0x60 => (Instruction::MOV_H_B, 1),
            0x61 => (Instruction::MOV_H_C, 1),
            0x62 => (Instruction::MOV_H_D, 1),
            0x63 => (Instruction::MOV_H_E, 1),
            0x64 => (Instruction::MOV_H_H, 1),
            0x65 => (Instruction::MOV_H_L, 1),
            0x66 => (Instruction::MOV_H_M, 1),
            0x67 => (Instruction::MOV_H_A, 1),

            0x68 => (Instruction::MOV_L_B, 1),
            0x69 => (Instruction::MOV_L_C, 1),
            0x6A => (Instruction::MOV_L_D, 1),
            0x6B => (Instruction::MOV_L_E, 1),
            0x6C => (Instruction::MOV_L_H, 1),
            0x6D => (Instruction::MOV_L_L, 1),
            0x6E => (Instruction::MOV_L_M, 1),
            0x6F => (Instruction::MOV_L_A, 1),

            0x70 => (Instruction::MOV_M_B, 1),
            0x71 => (Instruction::MOV_M_C, 1),
            0x72 => (Instruction::MOV_M_D, 1),
            0x73 => (Instruction::MOV_M_E, 1),
            0x74 => (Instruction::MOV_M_H, 1),
            0x75 => (Instruction::MOV_M_L, 1),

            0x76 => (Instruction::HLT, 1),

            // 0x76 => (Instruction::MOV_M_M, 1),
            0x77 => (Instruction::MOV_M_A, 1),

            0x78 => (Instruction::MOV_A_B, 1),
            0x79 => (Instruction::MOV_A_C, 1),
            0x7A => (Instruction::MOV_A_D, 1),
            0x7B => (Instruction::MOV_A_E, 1),
            0x7C => (Instruction::MOV_A_H, 1),
            0x7D => (Instruction::MOV_A_L, 1),
            0x7E => (Instruction::MOV_A_M, 1),
            0x7F => (Instruction::MOV_A_A, 1),

            _ => panic!("unknown instruction while disassembling: {:#04X}", data[pc]),
        }
    }
}
