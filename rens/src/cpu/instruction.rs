use super::{addressing::AddressingMode, Bus, CpuError};
use crate::{
    memory::Result,
    register::{P_FLAGS_B, P_FLAGS_C, P_FLAGS_U},
};

#[derive(Debug)]
pub(super) struct InstructionProcessor;

impl InstructionProcessor {
    pub fn process(&self, ins: u8, bus: &mut Bus) -> std::result::Result<u32, CpuError> {
        let instruction =
            InstructionInfo::from_code(ins).ok_or(CpuError::UnknownInstruction(ins))?;
        instruction.invoke(bus).map_err(|e| e.into())
    }
}

#[derive(Debug)]
enum InstructionType {
    Common,
    CrossingPage,
    Branch,
}

#[derive(Debug)]
enum Instruction {
    Jmp,
    Ldx,
    Stx,
    Jsr,
    Nop,
    Sec,
    Bcs,
    Clc,
    Bcc,
    Lda,
    Beq,
    Bne,
    Sta,
    Bit,
    Bvs,
    Bvc,
    Bpl,
    Rts,
    Sei,
    Asl,
    Sed,
    Php,
    Pla,
    And,
    Cmp,
    Cld,
    Pha,
    Plp,
    Bmi,
    Ora,
    Clv,
    Eor,
    Adc,
    Ldy,
    Cpy,
    Cpx,
    Sbc,
    Iny,
    Inx,
    Dey,
    Dex,
    Tay,
    Tax,
    Txa,
    Tya,
    Tsx,
    Txs,
    Rti,
    Lsr,
    Ror,
    Rol,
    Sty,
    Inc,
    Dec,
    Dop,
    Top,
    Lax,
    Aax,
    Dcp,
    Isc,
    Slo,
    Rla,
    Sre,
    Rra,
}
#[derive(Debug)]
struct InstructionInfo {
    code: u8,
    ins: Instruction,
    mode: AddressingMode,
    cycles: u32,
    ins_type: InstructionType,
}

impl InstructionInfo {
    /// 返回寻址模式和时钟周期
    fn from_code(ins: u8) -> Option<Self> {
        Some(match ins {
            //JMP
            0x4C => Self {
                code: ins,
                ins: Instruction::Jmp,
                mode: AddressingMode::Absolute,
                cycles: 3,
                ins_type: InstructionType::Common,
            },

            0x6C => Self {
                code: ins,
                ins: Instruction::Jmp,
                mode: AddressingMode::Indirect,
                cycles: 5,
                ins_type: InstructionType::Common,
            },

            // LDX
            0xA2 => Self {
                code: ins,
                ins: Instruction::Ldx,
                mode: AddressingMode::Immediate,
                cycles: 2,
                ins_type: InstructionType::Common,
            },
            0xAE => Self {
                code: ins,
                ins: Instruction::Ldx,
                mode: AddressingMode::Absolute,
                cycles: 4,
                ins_type: InstructionType::Common,
            },
            0xA6 => Self {
                code: ins,
                ins: Instruction::Ldx,
                mode: AddressingMode::ZeroPage,
                cycles: 3,
                ins_type: InstructionType::Common,
            },
            0xB6 => Self {
                code: ins,
                ins: Instruction::Ldx,
                mode: AddressingMode::ZeroPageY,
                cycles: 4,
                ins_type: InstructionType::Common,
            },
            0xBE => Self {
                code: ins,
                ins: Instruction::Ldx,
                mode: AddressingMode::AbsoluteY,
                cycles: 4,
                ins_type: InstructionType::CrossingPage,
            },

            // STX
            0x86 => Self {
                code: ins,
                ins: Instruction::Stx,
                mode: AddressingMode::ZeroPage,
                cycles: 3,
                ins_type: InstructionType::Common,
            },
            0x8E => Self {
                code: ins,
                ins: Instruction::Stx,
                mode: AddressingMode::Absolute,
                cycles: 4,
                ins_type: InstructionType::Common,
            },
            0x96 => Self {
                code: ins,
                ins: Instruction::Stx,
                mode: AddressingMode::ZeroPageY,
                cycles: 4,
                ins_type: InstructionType::Common,
            },

            // JSR
            0x20 => Self {
                code: ins,
                ins: Instruction::Jsr,
                mode: AddressingMode::Absolute,
                cycles: 6,
                ins_type: InstructionType::Common,
            },

            //NOP
            0x1A => Self {
                code: ins,
                ins: Instruction::Nop,
                mode: AddressingMode::Implicit,
                cycles: 2,
                ins_type: InstructionType::Common,
            },
            0x3A => Self {
                code: ins,
                ins: Instruction::Nop,
                mode: AddressingMode::Implicit,
                cycles: 2,
                ins_type: InstructionType::Common,
            },
            0x5A => Self {
                code: ins,
                ins: Instruction::Nop,
                mode: AddressingMode::Implicit,
                cycles: 2,
                ins_type: InstructionType::Common,
            },
            0x7A => Self {
                code: ins,
                ins: Instruction::Nop,
                mode: AddressingMode::Implicit,
                cycles: 2,
                ins_type: InstructionType::Common,
            },
            0xDA => Self {
                code: ins,
                ins: Instruction::Nop,
                mode: AddressingMode::Implicit,
                cycles: 2,
                ins_type: InstructionType::Common,
            },
            0xEA => Self {
                code: ins,
                ins: Instruction::Nop,
                mode: AddressingMode::Implicit,
                cycles: 2,
                ins_type: InstructionType::Common,
            },
            0xFA => Self {
                code: ins,
                ins: Instruction::Nop,
                mode: AddressingMode::Implicit,
                cycles: 2,
                ins_type: InstructionType::Common,
            },

            // SEC
            0x38 => Self {
                code: ins,
                ins: Instruction::Sec,
                mode: AddressingMode::Implicit,
                cycles: 2,
                ins_type: InstructionType::Common,
            },

            // BCS
            0xB0 => Self {
                code: ins,
                ins: Instruction::Bcs,
                mode: AddressingMode::Relative,
                cycles: 2,
                ins_type: InstructionType::Branch,
            },

            // CLC
            0x18 => Self {
                code: ins,
                ins: Instruction::Clc,
                mode: AddressingMode::Implicit,
                cycles: 2,
                ins_type: InstructionType::Common,
            },

            // BCC
            0x90 => Self {
                code: ins,
                ins: Instruction::Bcc,
                mode: AddressingMode::Relative,
                cycles: 2,
                ins_type: InstructionType::Branch,
            },

            // LDA
            0xA9 => Self {
                code: ins,
                ins: Instruction::Lda,
                mode: AddressingMode::Immediate,
                cycles: 2,
                ins_type: InstructionType::Common,
            },
            0xA5 => Self {
                code: ins,
                ins: Instruction::Lda,
                mode: AddressingMode::ZeroPage,
                cycles: 3,
                ins_type: InstructionType::Common,
            },
            0xB5 => Self {
                code: ins,
                ins: Instruction::Lda,
                mode: AddressingMode::ZeroPageX,
                cycles: 4,
                ins_type: InstructionType::Common,
            },
            0xAD => Self {
                code: ins,
                ins: Instruction::Lda,
                mode: AddressingMode::Absolute,
                cycles: 4,
                ins_type: InstructionType::Common,
            },
            0xBD => Self {
                code: ins,
                ins: Instruction::Lda,
                mode: AddressingMode::AbsoluteX,
                cycles: 4,
                ins_type: InstructionType::CrossingPage,
            },
            0xB9 => Self {
                code: ins,
                ins: Instruction::Lda,
                mode: AddressingMode::AbsoluteY,
                cycles: 4,
                ins_type: InstructionType::CrossingPage,
            },
            0xA1 => Self {
                code: ins,
                ins: Instruction::Lda,
                mode: AddressingMode::IndirectX,
                cycles: 6,
                ins_type: InstructionType::Common,
            },
            0xB1 => Self {
                code: ins,
                ins: Instruction::Lda,
                mode: AddressingMode::IndirectY,
                cycles: 5,
                ins_type: InstructionType::CrossingPage,
            },

            // BEQ
            0xF0 => Self {
                code: ins,
                ins: Instruction::Beq,
                mode: AddressingMode::Relative,
                cycles: 2,
                ins_type: InstructionType::Branch,
            },

            // BNE
            0xD0 => Self {
                code: ins,
                ins: Instruction::Bne,
                mode: AddressingMode::Relative,
                cycles: 2,
                ins_type: InstructionType::Branch,
            },

            // STA
            0x85 => Self {
                code: ins,
                ins: Instruction::Sta,
                mode: AddressingMode::ZeroPage,
                cycles: 3,
                ins_type: InstructionType::Common,
            },
            0x95 => Self {
                code: ins,
                ins: Instruction::Sta,
                mode: AddressingMode::ZeroPageX,
                cycles: 4,
                ins_type: InstructionType::Common,
            },
            0x8D => Self {
                code: ins,
                ins: Instruction::Sta,
                mode: AddressingMode::Absolute,
                cycles: 4,
                ins_type: InstructionType::Common,
            },
            0x9D => Self {
                code: ins,
                ins: Instruction::Sta,
                mode: AddressingMode::AbsoluteX,
                cycles: 5,
                ins_type: InstructionType::Common,
            },
            0x99 => Self {
                code: ins,
                ins: Instruction::Sta,
                mode: AddressingMode::AbsoluteY,
                cycles: 5,
                ins_type: InstructionType::Common,
            },
            0x81 => Self {
                code: ins,
                ins: Instruction::Sta,
                mode: AddressingMode::IndirectX,
                cycles: 6,
                ins_type: InstructionType::Common,
            },
            0x91 => Self {
                code: ins,
                ins: Instruction::Sta,
                mode: AddressingMode::IndirectY,
                cycles: 6,
                ins_type: InstructionType::Common,
            },

            // BIT
            0x24 => Self {
                code: ins,
                ins: Instruction::Bit,
                mode: AddressingMode::ZeroPage,
                cycles: 3,
                ins_type: InstructionType::Common,
            },
            0x2C => Self {
                code: ins,
                ins: Instruction::Bit,
                mode: AddressingMode::Absolute,
                cycles: 4,
                ins_type: InstructionType::Common,
            },

            // BVS
            0x70 => Self {
                code: ins,
                ins: Instruction::Bvs,
                mode: AddressingMode::Relative,
                cycles: 2,
                ins_type: InstructionType::Branch,
            },

            // BVC
            0x50 => Self {
                code: ins,
                ins: Instruction::Bvc,
                mode: AddressingMode::Relative,
                cycles: 2,
                ins_type: InstructionType::Branch,
            },

            // BPL
            0x10 => Self {
                code: ins,
                ins: Instruction::Bpl,
                mode: AddressingMode::Relative,
                cycles: 2,
                ins_type: InstructionType::Branch,
            },

            // RTS
            0x60 => Self {
                code: ins,
                ins: Instruction::Rts,
                mode: AddressingMode::Implicit,
                cycles: 6,
                ins_type: InstructionType::Common,
            },

            // SEI
            0x78 => Self {
                code: ins,
                ins: Instruction::Sei,
                mode: AddressingMode::Implicit,
                cycles: 2,
                ins_type: InstructionType::Common,
            },

            // ASL
            0x0A => Self {
                code: ins,
                ins: Instruction::Asl,
                mode: AddressingMode::Accumulator,
                cycles: 2,
                ins_type: InstructionType::Common,
            },

            0x06 => Self {
                code: ins,
                ins: Instruction::Asl,
                mode: AddressingMode::ZeroPage,
                cycles: 5,
                ins_type: InstructionType::Common,
            },

            0x16 => Self {
                code: ins,
                ins: Instruction::Asl,
                mode: AddressingMode::ZeroPageX,
                cycles: 6,
                ins_type: InstructionType::Common,
            },

            0x0E => Self {
                code: ins,
                ins: Instruction::Asl,
                mode: AddressingMode::Absolute,
                cycles: 6,
                ins_type: InstructionType::Common,
            },

            0x1E => Self {
                code: ins,
                ins: Instruction::Asl,
                mode: AddressingMode::AbsoluteX,
                cycles: 7,
                ins_type: InstructionType::Common,
            },

            // SED
            0xF8 => Self {
                code: ins,
                ins: Instruction::Sed,
                mode: AddressingMode::Implicit,
                cycles: 2,
                ins_type: InstructionType::Common,
            },

            // PHP
            0x08 => Self {
                code: ins,
                ins: Instruction::Php,
                mode: AddressingMode::Implicit,
                cycles: 3,
                ins_type: InstructionType::Common,
            },
            // PLA
            0x68 => Self {
                code: ins,
                ins: Instruction::Pla,
                mode: AddressingMode::Implicit,
                cycles: 4,
                ins_type: InstructionType::Common,
            },

            // AND
            0x29 => Self {
                code: ins,
                ins: Instruction::And,
                mode: AddressingMode::Immediate,
                cycles: 2,
                ins_type: InstructionType::Common,
            },
            0x25 => Self {
                code: ins,
                ins: Instruction::And,
                mode: AddressingMode::ZeroPage,
                cycles: 3,
                ins_type: InstructionType::Common,
            },
            0x35 => Self {
                code: ins,
                ins: Instruction::And,
                mode: AddressingMode::ZeroPageX,
                cycles: 4,
                ins_type: InstructionType::Common,
            },
            0x2D => Self {
                code: ins,
                ins: Instruction::And,
                mode: AddressingMode::Absolute,
                cycles: 4,
                ins_type: InstructionType::Common,
            },
            0x3D => Self {
                code: ins,
                ins: Instruction::And,
                mode: AddressingMode::AbsoluteX,
                cycles: 4,
                ins_type: InstructionType::CrossingPage,
            },
            0x39 => Self {
                code: ins,
                ins: Instruction::And,
                mode: AddressingMode::AbsoluteY,
                cycles: 4,
                ins_type: InstructionType::CrossingPage,
            },
            0x21 => Self {
                code: ins,
                ins: Instruction::And,
                mode: AddressingMode::IndirectX,
                cycles: 6,
                ins_type: InstructionType::Common,
            },
            0x31 => Self {
                code: ins,
                ins: Instruction::And,
                mode: AddressingMode::IndirectY,
                cycles: 5,
                ins_type: InstructionType::CrossingPage,
            },

            // CMP
            0xC9 => Self {
                code: ins,
                ins: Instruction::Cmp,
                mode: AddressingMode::Immediate,
                cycles: 2,
                ins_type: InstructionType::Common,
            },
            0xC5 => Self {
                code: ins,
                ins: Instruction::Cmp,
                mode: AddressingMode::ZeroPage,
                cycles: 3,
                ins_type: InstructionType::Common,
            },
            0xD5 => Self {
                code: ins,
                ins: Instruction::Cmp,
                mode: AddressingMode::ZeroPageX,
                cycles: 4,
                ins_type: InstructionType::Common,
            },
            0xCD => Self {
                code: ins,
                ins: Instruction::Cmp,
                mode: AddressingMode::Absolute,
                cycles: 4,
                ins_type: InstructionType::Common,
            },
            0xDD => Self {
                code: ins,
                ins: Instruction::Cmp,
                mode: AddressingMode::AbsoluteX,
                cycles: 4,
                ins_type: InstructionType::CrossingPage,
            },
            0xD9 => Self {
                code: ins,
                ins: Instruction::Cmp,
                mode: AddressingMode::AbsoluteY,
                cycles: 4,
                ins_type: InstructionType::CrossingPage,
            },
            0xC1 => Self {
                code: ins,
                ins: Instruction::Cmp,
                mode: AddressingMode::IndirectX,
                cycles: 6,
                ins_type: InstructionType::Common,
            },
            0xD1 => Self {
                code: ins,
                ins: Instruction::Cmp,
                mode: AddressingMode::IndirectY,
                cycles: 5,
                ins_type: InstructionType::CrossingPage,
            },
            // CLD
            0xD8 => Self {
                code: ins,
                ins: Instruction::Cld,
                mode: AddressingMode::Implicit,
                cycles: 2,
                ins_type: InstructionType::Common,
            },

            // PHA
            0x48 => Self {
                code: ins,
                ins: Instruction::Pha,
                mode: AddressingMode::Implicit,
                cycles: 3,
                ins_type: InstructionType::Common,
            },

            // PLP
            0x28 => Self {
                code: ins,
                ins: Instruction::Plp,
                mode: AddressingMode::Implicit,
                cycles: 4,
                ins_type: InstructionType::Common,
            },

            // BMI
            0x30 => Self {
                code: ins,
                ins: Instruction::Bmi,
                mode: AddressingMode::Relative,
                cycles: 2,
                ins_type: InstructionType::Branch,
            },

            // ORA
            0x09 => Self {
                code: ins,
                ins: Instruction::Ora,
                mode: AddressingMode::Immediate,
                cycles: 2,
                ins_type: InstructionType::Common,
            },
            0x05 => Self {
                code: ins,
                ins: Instruction::Ora,
                mode: AddressingMode::ZeroPage,
                cycles: 3,
                ins_type: InstructionType::Common,
            },
            0x15 => Self {
                code: ins,
                ins: Instruction::Ora,
                mode: AddressingMode::ZeroPageX,
                cycles: 4,
                ins_type: InstructionType::Common,
            },
            0x0D => Self {
                code: ins,
                ins: Instruction::Ora,
                mode: AddressingMode::Absolute,
                cycles: 4,
                ins_type: InstructionType::Common,
            },
            0x1D => Self {
                code: ins,
                ins: Instruction::Ora,
                mode: AddressingMode::AbsoluteX,
                cycles: 4,
                ins_type: InstructionType::CrossingPage,
            },
            0x19 => Self {
                code: ins,
                ins: Instruction::Ora,
                mode: AddressingMode::AbsoluteY,
                cycles: 4,
                ins_type: InstructionType::CrossingPage,
            },
            0x01 => Self {
                code: ins,
                ins: Instruction::Ora,
                mode: AddressingMode::IndirectX,
                cycles: 6,
                ins_type: InstructionType::Common,
            },
            0x11 => Self {
                code: ins,
                ins: Instruction::Ora,
                mode: AddressingMode::IndirectY,
                cycles: 5,
                ins_type: InstructionType::CrossingPage,
            },

            // CLV
            0xB8 => Self {
                code: ins,
                ins: Instruction::Clv,
                mode: AddressingMode::Implicit,
                cycles: 2,
                ins_type: InstructionType::Common,
            },

            // EOR
            0x49 => Self {
                code: ins,
                ins: Instruction::Eor,
                mode: AddressingMode::Immediate,
                cycles: 2,
                ins_type: InstructionType::Common,
            },
            0x45 => Self {
                code: ins,
                ins: Instruction::Eor,
                mode: AddressingMode::ZeroPage,
                cycles: 3,
                ins_type: InstructionType::Common,
            },
            0x55 => Self {
                code: ins,
                ins: Instruction::Eor,
                mode: AddressingMode::ZeroPageX,
                cycles: 4,
                ins_type: InstructionType::Common,
            },
            0x4D => Self {
                code: ins,
                ins: Instruction::Eor,
                mode: AddressingMode::Absolute,
                cycles: 4,
                ins_type: InstructionType::Common,
            },
            0x5D => Self {
                code: ins,
                ins: Instruction::Eor,
                mode: AddressingMode::AbsoluteX,
                cycles: 4,
                ins_type: InstructionType::CrossingPage,
            },
            0x59 => Self {
                code: ins,
                ins: Instruction::Eor,
                mode: AddressingMode::AbsoluteY,
                cycles: 4,
                ins_type: InstructionType::CrossingPage,
            },
            0x41 => Self {
                code: ins,
                ins: Instruction::Eor,
                mode: AddressingMode::IndirectX,
                cycles: 6,
                ins_type: InstructionType::Common,
            },
            0x51 => Self {
                code: ins,
                ins: Instruction::Eor,
                mode: AddressingMode::IndirectY,
                cycles: 5,
                ins_type: InstructionType::CrossingPage,
            },

            // ADC
            0x69 => Self {
                code: ins,
                ins: Instruction::Adc,
                mode: AddressingMode::Immediate,
                cycles: 2,
                ins_type: InstructionType::Common,
            },
            0x65 => Self {
                code: ins,
                ins: Instruction::Adc,
                mode: AddressingMode::ZeroPage,
                cycles: 3,
                ins_type: InstructionType::Common,
            },
            0x75 => Self {
                code: ins,
                ins: Instruction::Adc,
                mode: AddressingMode::ZeroPageX,
                cycles: 4,
                ins_type: InstructionType::Common,
            },
            0x6D => Self {
                code: ins,
                ins: Instruction::Adc,
                mode: AddressingMode::Absolute,
                cycles: 4,
                ins_type: InstructionType::Common,
            },
            0x7D => Self {
                code: ins,
                ins: Instruction::Adc,
                mode: AddressingMode::AbsoluteX,
                cycles: 4,
                ins_type: InstructionType::CrossingPage,
            },
            0x79 => Self {
                code: ins,
                ins: Instruction::Adc,
                mode: AddressingMode::AbsoluteY,
                cycles: 4,
                ins_type: InstructionType::CrossingPage,
            },
            0x61 => Self {
                code: ins,
                ins: Instruction::Adc,
                mode: AddressingMode::IndirectX,
                cycles: 6,
                ins_type: InstructionType::Common,
            },
            0x71 => Self {
                code: ins,
                ins: Instruction::Adc,
                mode: AddressingMode::IndirectY,
                cycles: 5,
                ins_type: InstructionType::CrossingPage,
            },

            // LDY
            0xA0 => Self {
                code: ins,
                ins: Instruction::Ldy,
                mode: AddressingMode::Immediate,
                cycles: 2,
                ins_type: InstructionType::Common,
            },
            0xA4 => Self {
                code: ins,
                ins: Instruction::Ldy,
                mode: AddressingMode::ZeroPage,
                cycles: 3,
                ins_type: InstructionType::Common,
            },
            0xB4 => Self {
                code: ins,
                ins: Instruction::Ldy,
                mode: AddressingMode::ZeroPageX,
                cycles: 4,
                ins_type: InstructionType::Common,
            },
            0xAC => Self {
                code: ins,
                ins: Instruction::Ldy,
                mode: AddressingMode::Absolute,
                cycles: 4,
                ins_type: InstructionType::Common,
            },
            0xBC => Self {
                code: ins,
                ins: Instruction::Ldy,
                mode: AddressingMode::AbsoluteX,
                cycles: 4,
                ins_type: InstructionType::CrossingPage,
            },

            // CPY
            0xC0 => Self {
                code: ins,
                ins: Instruction::Cpy,
                mode: AddressingMode::Immediate,
                cycles: 2,
                ins_type: InstructionType::Common,
            },
            0xC4 => Self {
                code: ins,
                ins: Instruction::Cpy,
                mode: AddressingMode::ZeroPage,
                cycles: 3,
                ins_type: InstructionType::Common,
            },
            0xCC => Self {
                code: ins,
                ins: Instruction::Cpy,
                mode: AddressingMode::Absolute,
                cycles: 4,
                ins_type: InstructionType::Common,
            },

            // CPX
            0xE0 => Self {
                code: ins,
                ins: Instruction::Cpx,
                mode: AddressingMode::Immediate,
                cycles: 2,
                ins_type: InstructionType::Common,
            },
            0xE4 => Self {
                code: ins,
                ins: Instruction::Cpx,
                mode: AddressingMode::ZeroPage,
                cycles: 3,
                ins_type: InstructionType::Common,
            },
            0xEC => Self {
                code: ins,
                ins: Instruction::Cpx,
                mode: AddressingMode::Absolute,
                cycles: 4,
                ins_type: InstructionType::Common,
            },

            // SBC
            0xE9 => Self {
                code: ins,
                ins: Instruction::Sbc,
                mode: AddressingMode::Immediate,
                cycles: 2,
                ins_type: InstructionType::Common,
            },
            0xEB => Self {
                code: ins,
                ins: Instruction::Sbc,
                mode: AddressingMode::Immediate,
                cycles: 2,
                ins_type: InstructionType::Common,
            },
            0xE5 => Self {
                code: ins,
                ins: Instruction::Sbc,
                mode: AddressingMode::ZeroPage,
                cycles: 3,
                ins_type: InstructionType::Common,
            },
            0xF5 => Self {
                code: ins,
                ins: Instruction::Sbc,
                mode: AddressingMode::ZeroPageX,
                cycles: 4,
                ins_type: InstructionType::Common,
            },
            0xED => Self {
                code: ins,
                ins: Instruction::Sbc,
                mode: AddressingMode::Absolute,
                cycles: 4,
                ins_type: InstructionType::Common,
            },
            0xFD => Self {
                code: ins,
                ins: Instruction::Sbc,
                mode: AddressingMode::AbsoluteX,
                cycles: 4,
                ins_type: InstructionType::CrossingPage,
            },
            0xF9 => Self {
                code: ins,
                ins: Instruction::Sbc,
                mode: AddressingMode::AbsoluteY,
                cycles: 4,
                ins_type: InstructionType::CrossingPage,
            },
            0xE1 => Self {
                code: ins,
                ins: Instruction::Sbc,
                mode: AddressingMode::IndirectX,
                cycles: 6,
                ins_type: InstructionType::Common,
            },
            0xF1 => Self {
                code: ins,
                ins: Instruction::Sbc,
                mode: AddressingMode::IndirectY,
                cycles: 5,
                ins_type: InstructionType::CrossingPage,
            },

            // INY
            0xC8 => Self {
                code: ins,
                ins: Instruction::Iny,
                mode: AddressingMode::Implicit,
                cycles: 2,
                ins_type: InstructionType::Common,
            },

            // INX
            0xE8 => Self {
                code: ins,
                ins: Instruction::Inx,
                mode: AddressingMode::Implicit,
                cycles: 2,
                ins_type: InstructionType::Common,
            },
            // DEY
            0x88 => Self {
                code: ins,
                ins: Instruction::Dey,
                mode: AddressingMode::Implicit,
                cycles: 2,
                ins_type: InstructionType::Common,
            },

            // DEX
            0xCA => Self {
                code: ins,
                ins: Instruction::Dex,
                mode: AddressingMode::Implicit,
                cycles: 2,
                ins_type: InstructionType::Common,
            },

            // TAX
            0xAA => Self {
                code: ins,
                ins: Instruction::Tax,
                mode: AddressingMode::Implicit,
                cycles: 2,
                ins_type: InstructionType::Common,
            },

            // TAY
            0xA8 => Self {
                code: ins,
                ins: Instruction::Tay,
                mode: AddressingMode::Implicit,
                cycles: 2,
                ins_type: InstructionType::Common,
            },

            // TXA
            0x8A => Self {
                code: ins,
                ins: Instruction::Txa,
                mode: AddressingMode::Implicit,
                cycles: 2,
                ins_type: InstructionType::Common,
            },

            // TYA
            0x98 => Self {
                code: ins,
                ins: Instruction::Tya,
                mode: AddressingMode::Implicit,
                cycles: 2,
                ins_type: InstructionType::Common,
            },

            // TSX
            0xBA => Self {
                code: ins,
                ins: Instruction::Tsx,
                mode: AddressingMode::Implicit,
                cycles: 2,
                ins_type: InstructionType::Common,
            },

            // TXS
            0x9A => Self {
                code: ins,
                ins: Instruction::Txs,
                mode: AddressingMode::Implicit,
                cycles: 2,
                ins_type: InstructionType::Common,
            },

            // RTI
            0x40 => Self {
                code: ins,
                ins: Instruction::Rti,
                mode: AddressingMode::Implicit,
                cycles: 6,
                ins_type: InstructionType::Common,
            },

            // LSR
            0x4A => Self {
                code: ins,
                ins: Instruction::Lsr,
                mode: AddressingMode::Accumulator,
                cycles: 2,
                ins_type: InstructionType::Common,
            },
            0x46 => Self {
                code: ins,
                ins: Instruction::Lsr,
                mode: AddressingMode::ZeroPage,
                cycles: 5,
                ins_type: InstructionType::Common,
            },
            0x56 => Self {
                code: ins,
                ins: Instruction::Lsr,
                mode: AddressingMode::ZeroPageX,
                cycles: 6,
                ins_type: InstructionType::Common,
            },
            0x4E => Self {
                code: ins,
                ins: Instruction::Lsr,
                mode: AddressingMode::Absolute,
                cycles: 6,
                ins_type: InstructionType::Common,
            },
            0x5E => Self {
                code: ins,
                ins: Instruction::Lsr,
                mode: AddressingMode::AbsoluteX,
                cycles: 7,
                ins_type: InstructionType::Common,
            },

            // ROR
            0x6A => Self {
                code: ins,
                ins: Instruction::Ror,
                mode: AddressingMode::Accumulator,
                cycles: 2,
                ins_type: InstructionType::Common,
            },
            0x66 => Self {
                code: ins,
                ins: Instruction::Ror,
                mode: AddressingMode::ZeroPage,
                cycles: 5,
                ins_type: InstructionType::Common,
            },
            0x76 => Self {
                code: ins,
                ins: Instruction::Ror,
                mode: AddressingMode::ZeroPageX,
                cycles: 6,
                ins_type: InstructionType::Common,
            },
            0x6E => Self {
                code: ins,
                ins: Instruction::Ror,
                mode: AddressingMode::Absolute,
                cycles: 6,
                ins_type: InstructionType::Common,
            },
            0x7E => Self {
                code: ins,
                ins: Instruction::Ror,
                mode: AddressingMode::AbsoluteX,
                cycles: 7,
                ins_type: InstructionType::Common,
            },

            // ROL
            0x2A => Self {
                code: ins,
                ins: Instruction::Rol,
                mode: AddressingMode::Accumulator,
                cycles: 2,
                ins_type: InstructionType::Common,
            },
            0x26 => Self {
                code: ins,
                ins: Instruction::Rol,
                mode: AddressingMode::ZeroPage,
                cycles: 5,
                ins_type: InstructionType::Common,
            },
            0x36 => Self {
                code: ins,
                ins: Instruction::Rol,
                mode: AddressingMode::ZeroPageX,
                cycles: 6,
                ins_type: InstructionType::Common,
            },
            0x2E => Self {
                code: ins,
                ins: Instruction::Rol,
                mode: AddressingMode::Absolute,
                cycles: 6,
                ins_type: InstructionType::Common,
            },
            0x3E => Self {
                code: ins,
                ins: Instruction::Rol,
                mode: AddressingMode::AbsoluteX,
                cycles: 7,
                ins_type: InstructionType::Common,
            },

            // STY
            0x84 => Self {
                code: ins,
                ins: Instruction::Sty,
                mode: AddressingMode::ZeroPage,
                cycles: 3,
                ins_type: InstructionType::Common,
            },
            0x8C => Self {
                code: ins,
                ins: Instruction::Sty,
                mode: AddressingMode::Absolute,
                cycles: 4,
                ins_type: InstructionType::Common,
            },
            0x94 => Self {
                code: ins,
                ins: Instruction::Sty,
                mode: AddressingMode::ZeroPageX,
                cycles: 4,
                ins_type: InstructionType::Common,
            },

            // DEC
            0xC6 => Self {
                code: ins,
                ins: Instruction::Dec,
                mode: AddressingMode::ZeroPage,
                cycles: 5,
                ins_type: InstructionType::Common,
            },

            0xD6 => Self {
                code: ins,
                ins: Instruction::Dec,
                mode: AddressingMode::ZeroPageX,
                cycles: 6,
                ins_type: InstructionType::Common,
            },
            0xCE => Self {
                code: ins,
                ins: Instruction::Dec,
                mode: AddressingMode::Absolute,
                cycles: 6,
                ins_type: InstructionType::Common,
            },
            0xDE => Self {
                code: ins,
                ins: Instruction::Dec,
                mode: AddressingMode::AbsoluteX,
                cycles: 7,
                ins_type: InstructionType::Common,
            },

            // INC
            0xE6 => Self {
                code: ins,
                ins: Instruction::Inc,
                mode: AddressingMode::ZeroPage,
                cycles: 5,
                ins_type: InstructionType::Common,
            },

            0xF6 => Self {
                code: ins,
                ins: Instruction::Inc,
                mode: AddressingMode::ZeroPageX,
                cycles: 6,
                ins_type: InstructionType::Common,
            },
            0xEE => Self {
                code: ins,
                ins: Instruction::Inc,
                mode: AddressingMode::Absolute,
                cycles: 6,
                ins_type: InstructionType::Common,
            },
            0xFE => Self {
                code: ins,
                ins: Instruction::Inc,
                mode: AddressingMode::AbsoluteX,
                cycles: 7,
                ins_type: InstructionType::Common,
            },

            // DOP
            0x04 => Self {
                code: ins,
                ins: Instruction::Dop,
                mode: AddressingMode::ZeroPage,
                cycles: 3,
                ins_type: InstructionType::Common,
            },
            0x14 => Self {
                code: ins,
                ins: Instruction::Dop,
                mode: AddressingMode::ZeroPageX,
                cycles: 4,
                ins_type: InstructionType::Common,
            },
            0x34 => Self {
                code: ins,
                ins: Instruction::Dop,
                mode: AddressingMode::ZeroPageX,
                cycles: 4,
                ins_type: InstructionType::Common,
            },
            0x44 => Self {
                code: ins,
                ins: Instruction::Dop,
                mode: AddressingMode::ZeroPage,
                cycles: 3,
                ins_type: InstructionType::Common,
            },

            0x54 => Self {
                code: ins,
                ins: Instruction::Dop,
                mode: AddressingMode::ZeroPageX,
                cycles: 4,
                ins_type: InstructionType::Common,
            },
            0x64 => Self {
                code: ins,
                ins: Instruction::Dop,
                mode: AddressingMode::ZeroPage,
                cycles: 3,
                ins_type: InstructionType::Common,
            },

            0x74 => Self {
                code: ins,
                ins: Instruction::Dop,
                mode: AddressingMode::ZeroPageX,
                cycles: 4,
                ins_type: InstructionType::Common,
            },

            0x80 => Self {
                code: ins,
                ins: Instruction::Dop,
                mode: AddressingMode::Immediate,
                cycles: 2,
                ins_type: InstructionType::Common,
            },
            0x82 => Self {
                code: ins,
                ins: Instruction::Dop,
                mode: AddressingMode::Immediate,
                cycles: 2,
                ins_type: InstructionType::Common,
            },

            0x89 => Self {
                code: ins,
                ins: Instruction::Dop,
                mode: AddressingMode::Immediate,
                cycles: 2,
                ins_type: InstructionType::Common,
            },

            0xC2 => Self {
                code: ins,
                ins: Instruction::Dop,
                mode: AddressingMode::Immediate,
                cycles: 2,
                ins_type: InstructionType::Common,
            },
            0xD4 => Self {
                code: ins,
                ins: Instruction::Dop,
                mode: AddressingMode::ZeroPageX,
                cycles: 4,
                ins_type: InstructionType::Common,
            },

            0xE2 => Self {
                code: ins,
                ins: Instruction::Dop,
                mode: AddressingMode::Immediate,
                cycles: 2,
                ins_type: InstructionType::Common,
            },

            0xF4 => Self {
                code: ins,
                ins: Instruction::Dop,
                mode: AddressingMode::ZeroPageX,
                cycles: 4,
                ins_type: InstructionType::Common,
            },

            // Top
            0x0C => Self {
                code: ins,
                ins: Instruction::Top,
                mode: AddressingMode::Absolute,
                cycles: 4,
                ins_type: InstructionType::Common,
            },
            0x1C => Self {
                code: ins,
                ins: Instruction::Top,
                mode: AddressingMode::AbsoluteX,
                cycles: 4,
                ins_type: InstructionType::CrossingPage,
            },
            0x3C => Self {
                code: ins,
                ins: Instruction::Top,
                mode: AddressingMode::AbsoluteX,
                cycles: 4,
                ins_type: InstructionType::CrossingPage,
            },
            0x5C => Self {
                code: ins,
                ins: Instruction::Top,
                mode: AddressingMode::AbsoluteX,
                cycles: 4,
                ins_type: InstructionType::CrossingPage,
            },
            0x7C => Self {
                code: ins,
                ins: Instruction::Top,
                mode: AddressingMode::AbsoluteX,
                cycles: 4,
                ins_type: InstructionType::CrossingPage,
            },
            0xDC => Self {
                code: ins,
                ins: Instruction::Top,
                mode: AddressingMode::AbsoluteX,
                cycles: 4,
                ins_type: InstructionType::CrossingPage,
            },
            0xFC => Self {
                code: ins,
                ins: Instruction::Top,
                mode: AddressingMode::AbsoluteX,
                cycles: 4,
                ins_type: InstructionType::CrossingPage,
            },

            // LAX
            0xA7 => Self {
                code: ins,
                ins: Instruction::Lax,
                mode: AddressingMode::ZeroPage,
                cycles: 3,
                ins_type: InstructionType::Common,
            },
            0xB7 => Self {
                code: ins,
                ins: Instruction::Lax,
                mode: AddressingMode::ZeroPageY,
                cycles: 4,
                ins_type: InstructionType::Common,
            },
            0xAF => Self {
                code: ins,
                ins: Instruction::Lax,
                mode: AddressingMode::Absolute,
                cycles: 4,
                ins_type: InstructionType::Common,
            },
            0xBF => Self {
                code: ins,
                ins: Instruction::Lax,
                mode: AddressingMode::AbsoluteY,
                cycles: 4,
                ins_type: InstructionType::CrossingPage,
            },
            0xA3 => Self {
                code: ins,
                ins: Instruction::Lax,
                mode: AddressingMode::IndirectX,
                cycles: 6,
                ins_type: InstructionType::Common,
            },
            0xB3 => Self {
                code: ins,
                ins: Instruction::Lax,
                mode: AddressingMode::IndirectY,
                cycles: 5,
                ins_type: InstructionType::CrossingPage,
            },

            // AAX
            0x87 => Self {
                code: ins,
                ins: Instruction::Aax,
                mode: AddressingMode::ZeroPage,
                cycles: 3,
                ins_type: InstructionType::Common,
            },
            0x97 => Self {
                code: ins,
                ins: Instruction::Aax,
                mode: AddressingMode::ZeroPageY,
                cycles: 4,
                ins_type: InstructionType::Common,
            },
            0x83 => Self {
                code: ins,
                ins: Instruction::Aax,
                mode: AddressingMode::IndirectX,
                cycles: 6,
                ins_type: InstructionType::Common,
            },
            0x8F => Self {
                code: ins,
                ins: Instruction::Aax,
                mode: AddressingMode::Absolute,
                cycles: 4,
                ins_type: InstructionType::Common,
            },

            // DCP
            0xC7 => Self {
                code: ins,
                ins: Instruction::Dcp,
                mode: AddressingMode::ZeroPage,
                cycles: 5,
                ins_type: InstructionType::Common,
            },
            0xD7 => Self {
                code: ins,
                ins: Instruction::Dcp,
                mode: AddressingMode::ZeroPageX,
                cycles: 6,
                ins_type: InstructionType::Common,
            },
            0xCF => Self {
                code: ins,
                ins: Instruction::Dcp,
                mode: AddressingMode::Absolute,
                cycles: 6,
                ins_type: InstructionType::Common,
            },
            0xDF => Self {
                code: ins,
                ins: Instruction::Dcp,
                mode: AddressingMode::AbsoluteX,
                cycles: 7,
                ins_type: InstructionType::Common,
            },
            0xDB => Self {
                code: ins,
                ins: Instruction::Dcp,
                mode: AddressingMode::AbsoluteY,
                cycles: 7,
                ins_type: InstructionType::Common,
            },
            0xC3 => Self {
                code: ins,
                ins: Instruction::Dcp,
                mode: AddressingMode::IndirectX,
                cycles: 8,
                ins_type: InstructionType::Common,
            },
            0xD3 => Self {
                code: ins,
                ins: Instruction::Dcp,
                mode: AddressingMode::IndirectY,
                cycles: 8,
                ins_type: InstructionType::Common,
            },

            // ISC
            0xE7 => Self {
                code: ins,
                ins: Instruction::Isc,
                mode: AddressingMode::ZeroPage,
                cycles: 5,
                ins_type: InstructionType::Common,
            },
            0xF7 => Self {
                code: ins,
                ins: Instruction::Isc,
                mode: AddressingMode::ZeroPageX,
                cycles: 6,
                ins_type: InstructionType::Common,
            },
            0xEF => Self {
                code: ins,
                ins: Instruction::Isc,
                mode: AddressingMode::Absolute,
                cycles: 6,
                ins_type: InstructionType::Common,
            },
            0xFF => Self {
                code: ins,
                ins: Instruction::Isc,
                mode: AddressingMode::AbsoluteX,
                cycles: 7,
                ins_type: InstructionType::Common,
            },
            0xFB => Self {
                code: ins,
                ins: Instruction::Isc,
                mode: AddressingMode::AbsoluteY,
                cycles: 7,
                ins_type: InstructionType::Common,
            },
            0xE3 => Self {
                code: ins,
                ins: Instruction::Isc,
                mode: AddressingMode::IndirectX,
                cycles: 8,
                ins_type: InstructionType::Common,
            },
            0xF3 => Self {
                code: ins,
                ins: Instruction::Isc,
                mode: AddressingMode::IndirectY,
                cycles: 8,
                ins_type: InstructionType::Common,
            },

            // SLO
            0x07 => Self {
                code: ins,
                ins: Instruction::Slo,
                mode: AddressingMode::ZeroPage,
                cycles: 5,
                ins_type: InstructionType::Common,
            },
            0x17 => Self {
                code: ins,
                ins: Instruction::Slo,
                mode: AddressingMode::ZeroPageX,
                cycles: 6,
                ins_type: InstructionType::Common,
            },
            0x0F => Self {
                code: ins,
                ins: Instruction::Slo,
                mode: AddressingMode::Absolute,
                cycles: 6,
                ins_type: InstructionType::Common,
            },
            0x1F => Self {
                code: ins,
                ins: Instruction::Slo,
                mode: AddressingMode::AbsoluteX,
                cycles: 7,
                ins_type: InstructionType::Common,
            },
            0x1B => Self {
                code: ins,
                ins: Instruction::Slo,
                mode: AddressingMode::AbsoluteY,
                cycles: 7,
                ins_type: InstructionType::Common,
            },
            0x03 => Self {
                code: ins,
                ins: Instruction::Slo,
                mode: AddressingMode::IndirectX,
                cycles: 8,
                ins_type: InstructionType::Common,
            },
            0x13 => Self {
                code: ins,
                ins: Instruction::Slo,
                mode: AddressingMode::IndirectY,
                cycles: 8,
                ins_type: InstructionType::Common,
            },

            // RLA
            0x27 => Self {
                code: ins,
                ins: Instruction::Rla,
                mode: AddressingMode::ZeroPage,
                cycles: 5,
                ins_type: InstructionType::Common,
            },
            0x37 => Self {
                code: ins,
                ins: Instruction::Rla,
                mode: AddressingMode::ZeroPageX,
                cycles: 6,
                ins_type: InstructionType::Common,
            },
            0x2F => Self {
                code: ins,
                ins: Instruction::Rla,
                mode: AddressingMode::Absolute,
                cycles: 6,
                ins_type: InstructionType::Common,
            },
            0x3F => Self {
                code: ins,
                ins: Instruction::Rla,
                mode: AddressingMode::AbsoluteX,
                cycles: 7,
                ins_type: InstructionType::Common,
            },
            0x3B => Self {
                code: ins,
                ins: Instruction::Rla,
                mode: AddressingMode::AbsoluteY,
                cycles: 7,
                ins_type: InstructionType::Common,
            },
            0x23 => Self {
                code: ins,
                ins: Instruction::Rla,
                mode: AddressingMode::IndirectX,
                cycles: 8,
                ins_type: InstructionType::Common,
            },
            0x33 => Self {
                code: ins,
                ins: Instruction::Rla,
                mode: AddressingMode::IndirectY,
                cycles: 8,
                ins_type: InstructionType::Common,
            },

            // SRE
            0x47 => Self {
                code: ins,
                ins: Instruction::Sre,
                mode: AddressingMode::ZeroPage,
                cycles: 5,
                ins_type: InstructionType::Common,
            },
            0x57 => Self {
                code: ins,
                ins: Instruction::Sre,
                mode: AddressingMode::ZeroPageX,
                cycles: 6,
                ins_type: InstructionType::Common,
            },
            0x4F => Self {
                code: ins,
                ins: Instruction::Sre,
                mode: AddressingMode::Absolute,
                cycles: 6,
                ins_type: InstructionType::Common,
            },
            0x5F => Self {
                code: ins,
                ins: Instruction::Sre,
                mode: AddressingMode::AbsoluteX,
                cycles: 7,
                ins_type: InstructionType::Common,
            },
            0x5B => Self {
                code: ins,
                ins: Instruction::Sre,
                mode: AddressingMode::AbsoluteY,
                cycles: 7,
                ins_type: InstructionType::Common,
            },
            0x43 => Self {
                code: ins,
                ins: Instruction::Sre,
                mode: AddressingMode::IndirectX,
                cycles: 8,
                ins_type: InstructionType::Common,
            },
            0x53 => Self {
                code: ins,
                ins: Instruction::Sre,
                mode: AddressingMode::IndirectY,
                cycles: 8,
                ins_type: InstructionType::Common,
            },

            // RRA
            0x67 => Self {
                code: ins,
                ins: Instruction::Rra,
                mode: AddressingMode::ZeroPage,
                cycles: 5,
                ins_type: InstructionType::Common,
            },
            0x77 => Self {
                code: ins,
                ins: Instruction::Rra,
                mode: AddressingMode::ZeroPageX,
                cycles: 6,
                ins_type: InstructionType::Common,
            },
            0x6F => Self {
                code: ins,
                ins: Instruction::Rra,
                mode: AddressingMode::Absolute,
                cycles: 6,
                ins_type: InstructionType::Common,
            },
            0x7F => Self {
                code: ins,
                ins: Instruction::Rra,
                mode: AddressingMode::AbsoluteX,
                cycles: 7,
                ins_type: InstructionType::Common,
            },
            0x7B => Self {
                code: ins,
                ins: Instruction::Rra,
                mode: AddressingMode::AbsoluteY,
                cycles: 7,
                ins_type: InstructionType::Common,
            },
            0x63 => Self {
                code: ins,
                ins: Instruction::Rra,
                mode: AddressingMode::IndirectX,
                cycles: 8,
                ins_type: InstructionType::Common,
            },
            0x73 => Self {
                code: ins,
                ins: Instruction::Rra,
                mode: AddressingMode::IndirectY,
                cycles: 8,
                ins_type: InstructionType::Common,
            },

            _ => None?,
        })
    }
    /// 返回时钟周期
    fn invoke(self, bus: &mut Bus) -> Result<u32> {
        let (address, cross_page) = self.mode.addressing(bus)?;
        // 分支语句是否跳转成功
        let extra_info = match self.ins {
            Instruction::Jmp => Self::jmp(bus, self.mode, address),
            Instruction::Ldx => Self::ldx(bus, self.mode, address),
            Instruction::Stx => Self::stx(bus, self.mode, address),
            Instruction::Jsr => Self::jsr(bus, self.mode, address),
            Instruction::Nop => Self::nop(bus, self.mode, address),
            Instruction::Sec => Self::sec(bus, self.mode, address),
            Instruction::Bcs => Self::bcs(bus, self.mode, address),
            Instruction::Clc => Self::clc(bus, self.mode, address),
            Instruction::Bcc => Self::bcc(bus, self.mode, address),
            Instruction::Lda => Self::lda(bus, self.mode, address),
            Instruction::Beq => Self::beq(bus, self.mode, address),
            Instruction::Bne => Self::bne(bus, self.mode, address),
            Instruction::Sta => Self::sta(bus, self.mode, address),
            Instruction::Bit => Self::bit(bus, self.mode, address),
            Instruction::Bvs => Self::bvs(bus, self.mode, address),
            Instruction::Bvc => Self::bvc(bus, self.mode, address),
            Instruction::Bpl => Self::bpl(bus, self.mode, address),
            Instruction::Rts => Self::rts(bus, self.mode, address),
            Instruction::Asl => Self::asl(bus, self.mode, address),
            Instruction::Sei => Self::sei(bus, self.mode, address),
            Instruction::Sed => Self::sed(bus, self.mode, address),
            Instruction::Php => Self::php(bus, self.mode, address),
            Instruction::Pla => Self::pla(bus, self.mode, address),
            Instruction::And => Self::and(bus, self.mode, address),
            Instruction::Cmp => Self::cmp(bus, self.mode, address),
            Instruction::Cld => Self::cld(bus, self.mode, address),
            Instruction::Pha => Self::pha(bus, self.mode, address),
            Instruction::Plp => Self::plp(bus, self.mode, address),
            Instruction::Bmi => Self::bmi(bus, self.mode, address),
            Instruction::Ora => Self::ora(bus, self.mode, address),
            Instruction::Clv => Self::clv(bus, self.mode, address),
            Instruction::Eor => Self::eor(bus, self.mode, address),
            Instruction::Adc => Self::adc(bus, self.mode, address),
            Instruction::Ldy => Self::ldy(bus, self.mode, address),
            Instruction::Cpy => Self::cpy(bus, self.mode, address),
            Instruction::Cpx => Self::cpx(bus, self.mode, address),
            Instruction::Sbc => Self::sbc(bus, self.mode, address),
            Instruction::Iny => Self::iny(bus, self.mode, address),
            Instruction::Inx => Self::inx(bus, self.mode, address),
            Instruction::Dey => Self::dey(bus, self.mode, address),
            Instruction::Dex => Self::dex(bus, self.mode, address),
            Instruction::Tay => Self::tay(bus, self.mode, address),
            Instruction::Tax => Self::tax(bus, self.mode, address),
            Instruction::Txa => Self::txa(bus, self.mode, address),
            Instruction::Tya => Self::tya(bus, self.mode, address),
            Instruction::Tsx => Self::tsx(bus, self.mode, address),
            Instruction::Txs => Self::txs(bus, self.mode, address),
            Instruction::Rti => Self::rti(bus, self.mode, address),
            Instruction::Lsr => Self::lsr(bus, self.mode, address),
            Instruction::Ror => Self::ror(bus, self.mode, address),
            Instruction::Rol => Self::rol(bus, self.mode, address),
            Instruction::Sty => Self::sty(bus, self.mode, address),
            Instruction::Inc => Self::inc(bus, self.mode, address),
            Instruction::Dec => Self::dec(bus, self.mode, address),
            Instruction::Dop => Self::dop(bus, self.mode, address),
            Instruction::Top => Self::top(bus, self.mode, address),
            Instruction::Lax => Self::lax(bus, self.mode, address),
            Instruction::Aax => Self::aax(bus, self.mode, address),
            Instruction::Dcp => Self::dcp(bus, self.mode, address),
            Instruction::Isc => Self::isc(bus, self.mode, address),
            Instruction::Slo => Self::slo(bus, self.mode, address),
            Instruction::Rla => Self::rla(bus, self.mode, address),
            Instruction::Sre => Self::sre(bus, self.mode, address),
            Instruction::Rra => Self::rra(bus, self.mode, address),
        }?;
        Ok(match self.ins_type {
            InstructionType::Common => get_cross_page_cycles(self.cycles, false),
            InstructionType::CrossingPage => get_cross_page_cycles(self.cycles, cross_page),
            InstructionType::Branch => get_branch_cycles(self.cycles, cross_page, extra_info),
        })
    }

    fn jmp(bus: &mut Bus, _mode: AddressingMode, address: u16) -> Result<bool> {
        bus.registers_mut().pc = address;
        Ok(false)
    }
    fn ldx(bus: &mut Bus, mode: AddressingMode, address: u16) -> Result<bool> {
        let data = mode.read(bus, address)?;
        bus.registers_mut().x = data;
        bus.registers_mut().set_z_n_flags(data);
        Ok(false)
    }
    fn stx(bus: &mut Bus, mode: AddressingMode, address: u16) -> Result<bool> {
        mode.write(bus, address, bus.registers().x)?;
        Ok(false)
    }
    fn jsr(bus: &mut Bus, _mode: AddressingMode, address: u16) -> Result<bool> {
        bus.stack_push_word(bus.registers().pc - 1)?;
        bus.registers_mut().pc = address;
        Ok(false)
    }

    fn nop(_bus: &mut Bus, _mode: AddressingMode, _address: u16) -> Result<bool> {
        Ok(false)
    }
    fn sec(bus: &mut Bus, _mode: AddressingMode, _address: u16) -> Result<bool> {
        bus.registers_mut().set_c_flag(true);
        Ok(false)
    }
    fn bcs(bus: &mut Bus, _mode: AddressingMode, address: u16) -> Result<bool> {
        let jmp_success = bus.registers().has_c_flag();
        if jmp_success {
            bus.registers_mut().pc = address;
        }
        Ok(jmp_success)
    }

    fn clc(bus: &mut Bus, _mode: AddressingMode, _address: u16) -> Result<bool> {
        bus.registers_mut().set_c_flag(false);
        Ok(false)
    }
    fn bcc(bus: &mut Bus, _mode: AddressingMode, address: u16) -> Result<bool> {
        let jmp_success = !bus.registers().has_c_flag();
        if jmp_success {
            bus.registers_mut().pc = address;
        }
        Ok(jmp_success)
    }

    fn lda(bus: &mut Bus, mode: AddressingMode, address: u16) -> Result<bool> {
        let data = mode.read(bus, address)?;
        bus.registers_mut().a = data;
        bus.registers_mut().set_z_n_flags(data);
        Ok(false)
    }
    fn beq(bus: &mut Bus, _mode: AddressingMode, address: u16) -> Result<bool> {
        let jmp_success = bus.registers().has_z_flag();
        if jmp_success {
            bus.registers_mut().pc = address;
        }
        Ok(jmp_success)
    }
    fn bne(bus: &mut Bus, _mode: AddressingMode, address: u16) -> Result<bool> {
        let jmp_success = !bus.registers().has_z_flag();
        if jmp_success {
            bus.registers_mut().pc = address;
        }
        Ok(jmp_success)
    }
    fn sta(bus: &mut Bus, mode: AddressingMode, address: u16) -> Result<bool> {
        mode.write(bus, address, bus.registers().a)?;
        Ok(false)
    }
    fn bit(bus: &mut Bus, mode: AddressingMode, address: u16) -> Result<bool> {
        let data = mode.read(bus, address)?;
        let a = bus.registers().a;
        bus.registers_mut().set_z_flag(data & a == 0);
        bus.registers_mut()
            .set_v_flag(data & 0b01000000 == 0b01000000);
        bus.registers_mut().set_n_flag(data >> 7 == 1);
        Ok(false)
    }
    fn bvs(bus: &mut Bus, _mode: AddressingMode, address: u16) -> Result<bool> {
        let jmp_success = bus.registers().has_v_flag();
        if jmp_success {
            bus.registers_mut().pc = address;
        }
        Ok(jmp_success)
    }
    fn bvc(bus: &mut Bus, _mode: AddressingMode, address: u16) -> Result<bool> {
        let jmp_success = !bus.registers().has_v_flag();
        if jmp_success {
            bus.registers_mut().pc = address;
        }
        Ok(jmp_success)
    }
    fn bpl(bus: &mut Bus, _mode: AddressingMode, address: u16) -> Result<bool> {
        let jmp_success = !bus.registers().has_n_flag();

        if jmp_success {
            bus.registers_mut().pc = address;
        }
        Ok(jmp_success)
    }
    fn rts(bus: &mut Bus, _mode: AddressingMode, _address: u16) -> Result<bool> {
        let address = bus.stack_pop_word()?;
        bus.registers_mut().pc = address + 1;
        Ok(false)
    }

    fn sei(bus: &mut Bus, _mode: AddressingMode, _address: u16) -> Result<bool> {
        bus.registers_mut().set_i_flag(true);
        Ok(false)
    }
    fn asl(bus: &mut Bus, mode: AddressingMode, address: u16) -> Result<bool> {
        let mut data = mode.read(bus, address)?;
        bus.registers_mut().set_c_flag(data >> 7 == 1);
        data <<= 1;
        bus.registers_mut().set_z_n_flags(data);
        mode.write(bus, address, data)?;
        Ok(false)
    }

    fn sed(bus: &mut Bus, _mode: AddressingMode, _address: u16) -> Result<bool> {
        bus.registers_mut().set_d_flag(true);
        Ok(false)
    }

    fn php(bus: &mut Bus, _mode: AddressingMode, _address: u16) -> Result<bool> {
        bus.stack_push(bus.registers().p | P_FLAGS_U | P_FLAGS_B)?;
        Ok(false)
    }

    fn pla(bus: &mut Bus, _mode: AddressingMode, _address: u16) -> Result<bool> {
        bus.registers_mut().a = bus.stack_pop()?;
        let a = bus.registers().a;
        bus.registers_mut().set_z_n_flags(a);
        Ok(false)
    }

    fn and(bus: &mut Bus, mode: AddressingMode, address: u16) -> Result<bool> {
        let data = mode.read(bus, address)?;
        let result = bus.registers().a & data;
        bus.registers_mut().a = result;
        bus.registers_mut().set_z_n_flags(result);
        Ok(false)
    }
    fn cmp(bus: &mut Bus, mode: AddressingMode, address: u16) -> Result<bool> {
        let data = mode.read(bus, address)?;
        let result = bus.registers().a as i16 - data as i16;
        bus.registers_mut().set_z_n_flags(result as u8);
        bus.registers_mut().set_c_flag(result >= 0);
        Ok(false)
    }

    fn cld(bus: &mut Bus, _mode: AddressingMode, _address: u16) -> Result<bool> {
        bus.registers_mut().set_d_flag(false);
        Ok(false)
    }

    fn pha(bus: &mut Bus, _mode: AddressingMode, _address: u16) -> Result<bool> {
        bus.stack_push(bus.registers().a)?;
        Ok(false)
    }

    fn plp(bus: &mut Bus, _mode: AddressingMode, _address: u16) -> Result<bool> {
        let p = bus.stack_pop()?;
        bus.registers_mut().p = p;
        bus.registers_mut().set_u_flag(true);
        bus.registers_mut().set_b_flag(false);
        Ok(false)
    }

    fn bmi(bus: &mut Bus, _mode: AddressingMode, address: u16) -> Result<bool> {
        let jmp_success = bus.registers().has_n_flag();

        if jmp_success {
            bus.registers_mut().pc = address;
        }
        Ok(jmp_success)
    }

    fn ora(bus: &mut Bus, mode: AddressingMode, address: u16) -> Result<bool> {
        let data = mode.read(bus, address)?;
        let result = bus.registers().a | data;
        bus.registers_mut().a = result;
        bus.registers_mut().set_z_n_flags(result);
        Ok(false)
    }

    fn clv(bus: &mut Bus, _mode: AddressingMode, _address: u16) -> Result<bool> {
        bus.registers_mut().set_v_flag(false);
        Ok(false)
    }

    fn eor(bus: &mut Bus, mode: AddressingMode, address: u16) -> Result<bool> {
        let data = mode.read(bus, address)?;
        let result = bus.registers().a ^ data;
        bus.registers_mut().a = result;
        bus.registers_mut().set_z_n_flags(result);
        Ok(false)
    }

    fn adc(bus: &mut Bus, mode: AddressingMode, address: u16) -> Result<bool> {
        let data = mode.read(bus, address)?;
        let result =
            bus.registers().a as u16 + data as u16 + (bus.registers().p & P_FLAGS_C) as u16;
        let af = bus.registers().a >> 7;
        let bf = data >> 7;
        let cf = ((result >> 7) & 1) as u8;
        bus.registers_mut().set_v_flag(af == bf && af != cf);
        bus.registers_mut().set_c_flag((result >> 8) & 1 == 1);
        bus.registers_mut().a = result as u8;
        bus.registers_mut().set_z_n_flags(result as u8);
        Ok(false)
    }

    fn ldy(bus: &mut Bus, mode: AddressingMode, address: u16) -> Result<bool> {
        let data = mode.read(bus, address)?;
        bus.registers_mut().y = data;
        bus.registers_mut().set_z_n_flags(data);
        Ok(false)
    }
    fn cpy(bus: &mut Bus, mode: AddressingMode, address: u16) -> Result<bool> {
        let y = bus.registers().y;
        let m = mode.read(bus, address)?;
        let result = (y as i16 - m as i16) as u8;
        bus.registers_mut().set_c_flag(y >= m);
        bus.registers_mut().set_z_flag(y == m);
        bus.registers_mut().set_n_flag(result >> 7 == 1);
        Ok(false)
    }
    fn cpx(bus: &mut Bus, mode: AddressingMode, address: u16) -> Result<bool> {
        let x = bus.registers().x;
        let m = mode.read(bus, address)?;
        let result = (x as i16 - m as i16) as u8;
        bus.registers_mut().set_c_flag(x >= m);
        bus.registers_mut().set_z_flag(x == m);
        bus.registers_mut().set_n_flag(result >> 7 == 1);
        Ok(false)
    }

    fn sbc(bus: &mut Bus, mode: AddressingMode, address: u16) -> Result<bool> {
        let data = mode.read(bus, address)?;
        let result =
            bus.registers().a as i16 - data as i16 - (1 - (bus.registers().p & P_FLAGS_C)) as i16;
        let af = bus.registers().a >> 7;
        let bf = data >> 7;
        let cf = ((result >> 7) & 1) as u8;
        bus.registers_mut()
            .set_v_flag((af == 1 && cf == 0) | (af == 0 && bf == 1 && cf == 1));
        bus.registers_mut().set_c_flag((result >> 8) & 1 == 0);
        bus.registers_mut().a = result as u8;
        bus.registers_mut().set_z_n_flags(result as u8);
        Ok(false)
    }

    fn iny(bus: &mut Bus, _mode: AddressingMode, _address: u16) -> Result<bool> {
        let result = (bus.registers().y as i16 + 1) as u8;
        bus.registers_mut().y = result;
        bus.registers_mut().set_z_n_flags(result);
        Ok(false)
    }

    fn inx(bus: &mut Bus, _mode: AddressingMode, _address: u16) -> Result<bool> {
        let result = (bus.registers().x as i16 + 1) as u8;
        bus.registers_mut().x = result;
        bus.registers_mut().set_z_n_flags(result);
        Ok(false)
    }

    fn dex(bus: &mut Bus, _mode: AddressingMode, _address: u16) -> Result<bool> {
        let result = (bus.registers().x as i16 - 1) as u8;
        bus.registers_mut().x = result;
        bus.registers_mut().set_z_n_flags(result);
        Ok(false)
    }

    fn dey(bus: &mut Bus, _mode: AddressingMode, _address: u16) -> Result<bool> {
        let result = (bus.registers().y as i16 - 1) as u8;
        bus.registers_mut().y = result;
        bus.registers_mut().set_z_n_flags(result);
        Ok(false)
    }

    fn tax(bus: &mut Bus, _mode: AddressingMode, _address: u16) -> Result<bool> {
        let result = bus.registers().a;
        bus.registers_mut().x = result;
        bus.registers_mut().set_z_n_flags(result);
        Ok(false)
    }

    fn tay(bus: &mut Bus, _mode: AddressingMode, _address: u16) -> Result<bool> {
        let result = bus.registers().a;
        bus.registers_mut().y = result;
        bus.registers_mut().set_z_n_flags(result);
        Ok(false)
    }

    fn txa(bus: &mut Bus, _mode: AddressingMode, _address: u16) -> Result<bool> {
        let result = bus.registers().x;
        bus.registers_mut().a = result;
        bus.registers_mut().set_z_n_flags(result);
        Ok(false)
    }

    fn tya(bus: &mut Bus, _mode: AddressingMode, _address: u16) -> Result<bool> {
        let result = bus.registers().y;
        bus.registers_mut().a = result;
        bus.registers_mut().set_z_n_flags(result);
        Ok(false)
    }

    fn tsx(bus: &mut Bus, _mode: AddressingMode, _address: u16) -> Result<bool> {
        let result = bus.registers().sp;
        bus.registers_mut().x = result;
        bus.registers_mut().set_z_n_flags(result);
        Ok(false)
    }

    fn txs(bus: &mut Bus, _mode: AddressingMode, _address: u16) -> Result<bool> {
        let result = bus.registers().x;
        bus.registers_mut().sp = result;
        Ok(false)
    }

    fn rti(bus: &mut Bus, _mode: AddressingMode, _address: u16) -> Result<bool> {
        let result = bus.stack_pop()? | P_FLAGS_U;
        bus.registers_mut().p = result;
        bus.registers_mut().pc = bus.stack_pop_word()?;
        Ok(false)
    }

    fn lsr(bus: &mut Bus, mode: AddressingMode, address: u16) -> Result<bool> {
        let data = mode.read(bus, address)?;
        let result = data >> 1;
        mode.write(bus, address, result)?;
        bus.registers_mut().set_c_flag(data & 1 == 1);
        bus.registers_mut().set_z_n_flags(result);
        Ok(false)
    }
    fn ror(bus: &mut Bus, mode: AddressingMode, address: u16) -> Result<bool> {
        let data = mode.read(bus, address)?;
        let result = (data >> 1) | ((bus.registers().p & P_FLAGS_C) << 7);
        mode.write(bus, address, result)?;
        bus.registers_mut().set_c_flag(data & 1 == 1);
        bus.registers_mut().set_z_n_flags(result);
        Ok(false)
    }
    fn rol(bus: &mut Bus, mode: AddressingMode, address: u16) -> Result<bool> {
        let data = mode.read(bus, address)?;
        let result = (data << 1) | (bus.registers().p & P_FLAGS_C);
        mode.write(bus, address, result)?;
        bus.registers_mut().set_c_flag(data >> 7 == 1);
        bus.registers_mut().set_z_n_flags(result);
        Ok(false)
    }

    fn sty(bus: &mut Bus, mode: AddressingMode, address: u16) -> Result<bool> {
        mode.write(bus, address, bus.registers().y)?;
        Ok(false)
    }
    fn inc(bus: &mut Bus, mode: AddressingMode, address: u16) -> Result<bool> {
        let data = mode.read(bus, address)?;
        let data = ((data as i16) + 1) as u8;
        mode.write(bus, address, data)?;
        bus.registers_mut().set_z_n_flags(data);
        Ok(false)
    }

    fn dec(bus: &mut Bus, mode: AddressingMode, address: u16) -> Result<bool> {
        let data = mode.read(bus, address)?;
        let result = ((data as i16) - 1) as u8;
        mode.write(bus, address, result)?;
        bus.registers_mut().set_z_n_flags(result);
        Ok(false)
    }

    fn dop(_bus: &mut Bus, _mode: AddressingMode, _address: u16) -> Result<bool> {
        Ok(false)
    }
    fn top(_bus: &mut Bus, _mode: AddressingMode, _address: u16) -> Result<bool> {
        Ok(false)
    }

    fn lax(bus: &mut Bus, mode: AddressingMode, address: u16) -> Result<bool> {
        let data = mode.read(bus, address)?;
        bus.registers_mut().a = data;
        bus.registers_mut().x = data;
        bus.registers_mut().set_z_n_flags(data);
        Ok(false)
    }

    fn aax(bus: &mut Bus, mode: AddressingMode, address: u16) -> Result<bool> {
        let data = bus.registers().a & bus.registers().x;
        mode.write(bus, address, data)?;
        Ok(false)
    }
    fn dcp(bus: &mut Bus, mode: AddressingMode, address: u16) -> Result<bool> {
        let data = mode.read(bus, address)?;
        let tmp = (data as i16 - 1) as u8;
        mode.write(bus, address, tmp)?;
        let result = (bus.registers().a as i16 - tmp as i16) as u16;
        bus.registers_mut().set_c_flag(result < 0x0100);
        bus.registers_mut().set_z_n_flags(result as u8);
        Ok(false)
    }
    fn isc(bus: &mut Bus, mode: AddressingMode, address: u16) -> Result<bool> {
        let mut data = mode.read(bus, address)?;
        data = (data as i16 + 1) as u8;
        bus.cpu_write(address, data)?;
        let result =
            bus.registers().a as i16 - data as i16 - 1 + ((bus.registers().p & P_FLAGS_C) as i16);
        let af = bus.registers().a >> 7;
        let bf = data >> 7;
        let cf = (result >> 7) & 1;
        bus.registers_mut()
            .set_v_flag((af == 1 && cf == 0) || (af == 0 && bf == 1 && cf == 1));
        bus.registers_mut().set_c_flag((result >> 8) & 1 != 1);
        let a = result as u8;
        bus.registers_mut().a = a;
        bus.registers_mut().set_z_n_flags(a);
        Ok(false)
    }
    fn slo(bus: &mut Bus, mode: AddressingMode, address: u16) -> Result<bool> {
        let data = mode.read(bus, address)?;
        bus.registers_mut().set_c_flag(data >> 7 == 1);
        let data = ((data as u16) << 1) as u8;
        mode.write(bus, address, data)?;
        bus.registers_mut().a |= data;
        let a = bus.registers().a;
        bus.registers_mut().set_z_n_flags(a);
        Ok(false)
    }
    fn rla(bus: &mut Bus, mode: AddressingMode, address: u16) -> Result<bool> {
        let data = mode.read(bus, address)?;
        let new = (data << 1) | (bus.registers().p & P_FLAGS_C);
        mode.write(bus, address, new)?;
        bus.registers_mut().set_c_flag(data >> 7 == 1);
        bus.registers_mut().a &= new;
        let a = bus.registers().a;
        bus.registers_mut().set_z_n_flags(a);
        Ok(false)
    }
    fn sre(bus: &mut Bus, mode: AddressingMode, address: u16) -> Result<bool> {
        let mut data = mode.read(bus, address)?;
        bus.registers_mut().set_c_flag((data & 1) == 1);
        data >>= 1;
        mode.write(bus, address, data)?;
        bus.registers_mut().a ^= data;
        let a = bus.registers().a;
        bus.registers_mut().set_z_n_flags(a);
        Ok(false)
    }
    fn rra(bus: &mut Bus, mode: AddressingMode, address: u16) -> Result<bool> {
        Self::ror(bus, mode.clone(), address).and(Self::adc(bus, mode, address))
    }
}

fn get_cross_page_cycles(cycles: u32, page_crossed: bool) -> u32 {
    cycles + if page_crossed { 1 } else { 0 }
}
fn get_branch_cycles(cycles: u32, page_crossed: bool, success: bool) -> u32 {
    cycles
        + if success {
            if page_crossed {
                2
            } else {
                1
            }
        } else {
            0
        }
}
