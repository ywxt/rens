use super::{AddressingMode, CpuBus, CpuError};
use crate::cpu::{P_FLAGS_B, P_FLAGS_C, P_FLAGS_U};

pub(super) struct InstructionProcessor;

impl InstructionProcessor {
    pub fn process(&self, ins: u8, bus: &mut CpuBus) -> Result<u32, CpuError> {
        let instruction =
            Instruction::from_instruction(ins).ok_or(CpuError::UnknownInstructionError(ins))?;
        instruction
            .invoke(bus)
            .ok_or(CpuError::InvokeInstructionError(ins))
    }
}

struct InstructionInfo {
    mode: AddressingMode,
    cycles: u32,
    can_cross_page: bool,
}

enum Instruction {
    Jmp(u8, InstructionInfo),
    Ldx(u8, InstructionInfo),
    Stx(u8, InstructionInfo),
    Jsr(u8, InstructionInfo),
    Nop(u8, InstructionInfo),
    Sec(u8, InstructionInfo),
    Bcs(u8, InstructionInfo),
    Clc(u8, InstructionInfo),
    Bcc(u8, InstructionInfo),
    Lda(u8, InstructionInfo),
    Beq(u8, InstructionInfo),
    Bne(u8, InstructionInfo),
    Sta(u8, InstructionInfo),
    Bit(u8, InstructionInfo),
    Bvs(u8, InstructionInfo),
    Bvc(u8, InstructionInfo),
    Bpl(u8, InstructionInfo),
    Rts(u8, InstructionInfo),
    Sei(u8, InstructionInfo),
    Asl(u8, InstructionInfo),
    Sed(u8, InstructionInfo),
    Php(u8, InstructionInfo),
    Pla(u8, InstructionInfo),
    And(u8, InstructionInfo),
    Cmp(u8, InstructionInfo),
    Cld(u8, InstructionInfo),
    Pha(u8, InstructionInfo),
    Plp(u8, InstructionInfo),
    Bmi(u8, InstructionInfo),
    Ora(u8, InstructionInfo),
    Clv(u8, InstructionInfo),
    Eor(u8, InstructionInfo),
    Adc(u8, InstructionInfo),
    Ldy(u8, InstructionInfo),
}

impl Instruction {
    /// 返回寻址模式和时钟周期
    fn from_instruction(ins: u8) -> Option<Self> {
        Some(match ins {
            //JMP
            0x4C => Self::Jmp(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteAddressingMode,
                    cycles: 3,
                    can_cross_page: false,
                },
            ),
            0x6C => Self::Jmp(
                ins,
                InstructionInfo {
                    mode: AddressingMode::IndirectAddressingMode,
                    cycles: 5,
                    can_cross_page: false,
                },
            ),

            // LDX
            0xA2 => Self::Ldx(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImmediateAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),
            0xAE => Self::Ldx(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteAddressingMode,
                    cycles: 4,
                    can_cross_page: false,
                },
            ),
            0xA6 => Self::Ldx(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageAddressingMode,
                    cycles: 3,
                    can_cross_page: false,
                },
            ),
            0xB6 => Self::Ldx(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageYAddressingMode,
                    cycles: 4,
                    can_cross_page: false,
                },
            ),
            0xBE => Self::Ldx(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteYAddressingMode,
                    cycles: 4,
                    can_cross_page: true,
                },
            ),

            // STX
            0x86 => Self::Stx(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageAddressingMode,
                    cycles: 3,
                    can_cross_page: false,
                },
            ),
            0x8E => Self::Stx(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteYAddressingMode,
                    cycles: 4,
                    can_cross_page: false,
                },
            ),
            0x96 => Self::Stx(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageYAddressingMode,
                    cycles: 4,
                    can_cross_page: false,
                },
            ),

            // JSR
            0x20 => Self::Jsr(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteAddressingMode,
                    cycles: 6,
                    can_cross_page: false,
                },
            ),

            //NOP
            0x1A => Self::Nop(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImplicitAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),
            0x3A => Self::Nop(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImplicitAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),
            0x5A => Self::Nop(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImplicitAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),
            0x7A => Self::Nop(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImplicitAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),
            0xDA => Self::Nop(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImplicitAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),
            0xEA => Self::Nop(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImplicitAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),
            0xFA => Self::Nop(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImplicitAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),

            // SEC
            0x38 => Self::Sec(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImplicitAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),

            // BCS
            0xB0 => Self::Bcs(
                ins,
                InstructionInfo {
                    mode: AddressingMode::RelativeAddressingMode,
                    cycles: 2,
                    can_cross_page: true,
                },
            ),

            // CLC
            0x18 => Self::Clc(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImplicitAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),

            // BCC
            0x90 => Self::Bcc(
                ins,
                InstructionInfo {
                    mode: AddressingMode::RelativeAddressingMode,
                    cycles: 2,
                    can_cross_page: true,
                },
            ),

            // LDA
            0xA9 => Self::Lda(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImmediateAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),
            0xA5 => Self::Lda(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageAddressingMode,
                    cycles: 3,
                    can_cross_page: false,
                },
            ),
            0xB5 => Self::Lda(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageXAddressingMode,
                    cycles: 4,
                    can_cross_page: false,
                },
            ),
            0xAD => Self::Lda(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteAddressingMode,
                    cycles: 4,
                    can_cross_page: false,
                },
            ),
            0xBD => Self::Lda(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteXAddressingMode,
                    cycles: 4,
                    can_cross_page: true,
                },
            ),
            0xB9 => Self::Lda(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteYAddressingMode,
                    cycles: 4,
                    can_cross_page: true,
                },
            ),
            0xA1 => Self::Lda(
                ins,
                InstructionInfo {
                    mode: AddressingMode::IndirectAddressingMode,
                    cycles: 6,
                    can_cross_page: false,
                },
            ),
            0xB1 => Self::Lda(
                ins,
                InstructionInfo {
                    mode: AddressingMode::IndirectYAddressingMode,
                    cycles: 5,
                    can_cross_page: true,
                },
            ),

            // BEQ
            0xF0 => Self::Beq(
                ins,
                InstructionInfo {
                    mode: AddressingMode::RelativeAddressingMode,
                    cycles: 2,
                    can_cross_page: true,
                },
            ),

            // BNE
            0xD0 => Self::Bne(
                ins,
                InstructionInfo {
                    mode: AddressingMode::RelativeAddressingMode,
                    cycles: 2,
                    can_cross_page: true,
                },
            ),

            // STA
            0x85 => Self::Sta(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageAddressingMode,
                    cycles: 3,
                    can_cross_page: false,
                },
            ),
            0x95 => Self::Sta(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageXAddressingMode,
                    cycles: 4,
                    can_cross_page: false,
                },
            ),
            0x8D => Self::Sta(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteAddressingMode,
                    cycles: 4,
                    can_cross_page: false,
                },
            ),
            0x9D => Self::Sta(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteXAddressingMode,
                    cycles: 5,
                    can_cross_page: false,
                },
            ),
            0x99 => Self::Sta(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteYAddressingMode,
                    cycles: 5,
                    can_cross_page: false,
                },
            ),
            0x81 => Self::Sta(
                ins,
                InstructionInfo {
                    mode: AddressingMode::IndirectXAddressingMode,
                    cycles: 6,
                    can_cross_page: false,
                },
            ),
            0x91 => Self::Sta(
                ins,
                InstructionInfo {
                    mode: AddressingMode::IndirectYAddressingMode,
                    cycles: 6,
                    can_cross_page: false,
                },
            ),

            // BIT
            0x24 => Self::Bit(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageAddressingMode,
                    cycles: 3,
                    can_cross_page: false,
                },
            ),
            0x2C => Self::Bit(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteAddressingMode,
                    cycles: 4,
                    can_cross_page: false,
                },
            ),

            // BVS
            0x70 => Self::Bvs(
                ins,
                InstructionInfo {
                    mode: AddressingMode::RelativeAddressingMode,
                    cycles: 2,
                    can_cross_page: true,
                },
            ),

            // BVC
            0x50 => Self::Bvc(
                ins,
                InstructionInfo {
                    mode: AddressingMode::RelativeAddressingMode,
                    cycles: 2,
                    can_cross_page: true,
                },
            ),

            // BPL
            0x10 => Self::Bpl(
                ins,
                InstructionInfo {
                    mode: AddressingMode::RelativeAddressingMode,
                    cycles: 2,
                    can_cross_page: true,
                },
            ),

            // RTS
            0x60 => Self::Rts(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImplicitAddressingMode,
                    cycles: 6,
                    can_cross_page: false,
                },
            ),

            // SEI
            0x78 => Self::Sei(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImplicitAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),

            // ASL
            0x0A => Self::Asl(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AccumulatorAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),

            0x06 => Self::Asl(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageAddressingMode,
                    cycles: 5,
                    can_cross_page: false,
                },
            ),

            0x16 => Self::Asl(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageXAddressingMode,
                    cycles: 6,
                    can_cross_page: false,
                },
            ),

            0x0E => Self::Asl(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteAddressingMode,
                    cycles: 6,
                    can_cross_page: false,
                },
            ),

            0x1E => Self::Asl(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteXAddressingMode,
                    cycles: 7,
                    can_cross_page: false,
                },
            ),

            // SED
            0xF8 => Self::Sed(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImplicitAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),

            // PHP
            0x08 => Self::Php(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImplicitAddressingMode,
                    cycles: 3,
                    can_cross_page: false,
                },
            ),
            // PLA
            0x68 => Self::Pla(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImplicitAddressingMode,
                    cycles: 4,
                    can_cross_page: false,
                },
            ),

            // AND
            0x29 => Self::And(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImmediateAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),
            0x25 => Self::And(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageAddressingMode,
                    cycles: 3,
                    can_cross_page: false,
                },
            ),
            0x35 => Self::And(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageXAddressingMode,
                    cycles: 4,
                    can_cross_page: false,
                },
            ),
            0x2D => Self::And(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteAddressingMode,
                    cycles: 4,
                    can_cross_page: false,
                },
            ),
            0x3D => Self::And(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteXAddressingMode,
                    cycles: 4,
                    can_cross_page: true,
                },
            ),
            0x39 => Self::And(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteYAddressingMode,
                    cycles: 4,
                    can_cross_page: true,
                },
            ),
            0x21 => Self::And(
                ins,
                InstructionInfo {
                    mode: AddressingMode::IndirectXAddressingMode,
                    cycles: 6,
                    can_cross_page: false,
                },
            ),
            0x31 => Self::And(
                ins,
                InstructionInfo {
                    mode: AddressingMode::IndirectYAddressingMode,
                    cycles: 5,
                    can_cross_page: true,
                },
            ),

            // CMP
            0xC9 => Self::Cmp(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImmediateAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),
            0xC5 => Self::Cmp(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageAddressingMode,
                    cycles: 3,
                    can_cross_page: false,
                },
            ),
            0xD5 => Self::Cmp(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageXAddressingMode,
                    cycles: 4,
                    can_cross_page: false,
                },
            ),
            0xCD => Self::Cmp(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteAddressingMode,
                    cycles: 4,
                    can_cross_page: false,
                },
            ),
            0xDD => Self::Cmp(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteXAddressingMode,
                    cycles: 4,
                    can_cross_page: true,
                },
            ),
            0xD9 => Self::Cmp(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteYAddressingMode,
                    cycles: 4,
                    can_cross_page: true,
                },
            ),
            0xC1 => Self::Cmp(
                ins,
                InstructionInfo {
                    mode: AddressingMode::IndirectXAddressingMode,
                    cycles: 6,
                    can_cross_page: false,
                },
            ),
            0xD1 => Self::Cmp(
                ins,
                InstructionInfo {
                    mode: AddressingMode::IndirectYAddressingMode,
                    cycles: 5,
                    can_cross_page: true,
                },
            ),
            // CLD
            0xD8 => Self::Cld(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImplicitAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),

            // PHA
            0x48 => Self::Pha(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImplicitAddressingMode,
                    cycles: 3,
                    can_cross_page: false,
                },
            ),

            // PLP
            0x28 => Self::Plp(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImplicitAddressingMode,
                    cycles: 4,
                    can_cross_page: false,
                },
            ),

            // BMI
            0x30 => Self::Bmi(
                ins,
                InstructionInfo {
                    mode: AddressingMode::RelativeAddressingMode,
                    cycles: 2,
                    can_cross_page: true,
                },
            ),

            // ORA
            0x09 => Self::Ora(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImmediateAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),
            0x05 => Self::Ora(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageAddressingMode,
                    cycles: 3,
                    can_cross_page: false,
                },
            ),
            0x15 => Self::Ora(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageXAddressingMode,
                    cycles: 4,
                    can_cross_page: false,
                },
            ),
            0x0D => Self::Ora(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteAddressingMode,
                    cycles: 4,
                    can_cross_page: false,
                },
            ),
            0x1D => Self::Ora(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteXAddressingMode,
                    cycles: 4,
                    can_cross_page: true,
                },
            ),
            0x19 => Self::Ora(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteYAddressingMode,
                    cycles: 4,
                    can_cross_page: true,
                },
            ),
            0x01 => Self::Ora(
                ins,
                InstructionInfo {
                    mode: AddressingMode::IndirectXAddressingMode,
                    cycles: 6,
                    can_cross_page: false,
                },
            ),
            0x11 => Self::Ora(
                ins,
                InstructionInfo {
                    mode: AddressingMode::IndirectYAddressingMode,
                    cycles: 5,
                    can_cross_page: true,
                },
            ),

            // CLV
            0xB8 => Self::Clv(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImplicitAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),

            // EOR
            0x49 => Self::Eor(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImmediateAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),
            0x45 => Self::Eor(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageAddressingMode,
                    cycles: 3,
                    can_cross_page: false,
                },
            ),
            0x55 => Self::Eor(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageXAddressingMode,
                    cycles: 4,
                    can_cross_page: false,
                },
            ),
            0x4D => Self::Eor(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteAddressingMode,
                    cycles: 4,
                    can_cross_page: false,
                },
            ),
            0x5D => Self::Eor(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteXAddressingMode,
                    cycles: 4,
                    can_cross_page: true,
                },
            ),
            0x59 => Self::Eor(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteYAddressingMode,
                    cycles: 4,
                    can_cross_page: true,
                },
            ),
            0x41 => Self::Eor(
                ins,
                InstructionInfo {
                    mode: AddressingMode::IndirectXAddressingMode,
                    cycles: 6,
                    can_cross_page: false,
                },
            ),
            0x51 => Self::Eor(
                ins,
                InstructionInfo {
                    mode: AddressingMode::IndirectYAddressingMode,
                    cycles: 5,
                    can_cross_page: true,
                },
            ),

            // ADC
            0x69 => Self::Adc(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImmediateAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),
            0x65 => Self::Adc(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageAddressingMode,
                    cycles: 3,
                    can_cross_page: false,
                },
            ),
            0x75 => Self::Adc(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageXAddressingMode,
                    cycles: 4,
                    can_cross_page: false,
                },
            ),
            0x6D => Self::Adc(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteAddressingMode,
                    cycles: 4,
                    can_cross_page: false,
                },
            ),
            0x7D => Self::Adc(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteXAddressingMode,
                    cycles: 4,
                    can_cross_page: true,
                },
            ),
            0x79 => Self::Adc(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteYAddressingMode,
                    cycles: 4,
                    can_cross_page: true,
                },
            ),
            0x61 => Self::Adc(
                ins,
                InstructionInfo {
                    mode: AddressingMode::IndirectXAddressingMode,
                    cycles: 6,
                    can_cross_page: false,
                },
            ),
            0x71 => Self::Adc(
                ins,
                InstructionInfo {
                    mode: AddressingMode::IndirectYAddressingMode,
                    cycles: 5,
                    can_cross_page: true,
                },
            ),

            // LDY
            0xA0 => Self::Ldy(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImmediateAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),
            0xA4 => Self::Ldy(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageAddressingMode,
                    cycles: 3,
                    can_cross_page: false,
                },
            ),
            0xB4 => Self::Ldy(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageXAddressingMode,
                    cycles: 4,
                    can_cross_page: false,
                },
            ),
            0xAC => Self::Ldy(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteAddressingMode,
                    cycles: 4,
                    can_cross_page: false,
                },
            ),
            0xBC => Self::Ldy(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteXAddressingMode,
                    cycles: 4,
                    can_cross_page: true,
                },
            ),

            _ => None?,
        })
    }
    /// 返回时钟周期
    fn invoke(self, bus: &mut CpuBus) -> Option<u32> {
        match self {
            Instruction::Jmp(_, ins) => Self::jmp(bus, ins),
            Instruction::Ldx(_, ins) => Self::ldx(bus, ins),
            Instruction::Stx(_, ins) => Self::stx(bus, ins),
            Instruction::Jsr(_, ins) => Self::jsr(bus, ins),
            Instruction::Nop(_, ins) => Self::nop(bus, ins),
            Instruction::Sec(_, ins) => Self::sec(bus, ins),
            Instruction::Bcs(_, ins) => Self::bcs(bus, ins),
            Instruction::Clc(_, ins) => Self::clc(bus, ins),
            Instruction::Bcc(_, ins) => Self::bcc(bus, ins),
            Instruction::Lda(_, ins) => Self::lda(bus, ins),
            Instruction::Beq(_, ins) => Self::beq(bus, ins),
            Instruction::Bne(_, ins) => Self::bne(bus, ins),
            Instruction::Sta(_, ins) => Self::sta(bus, ins),
            Instruction::Bit(_, ins) => Self::bit(bus, ins),
            Instruction::Bvs(_, ins) => Self::bvs(bus, ins),
            Instruction::Bvc(_, ins) => Self::bvc(bus, ins),
            Instruction::Bpl(_, ins) => Self::bpl(bus, ins),
            Instruction::Rts(_, ins) => Self::rts(bus, ins),
            Instruction::Asl(_, ins) => Self::asl(bus, ins),
            Instruction::Sei(_, ins) => Self::sei(bus, ins),
            Instruction::Sed(_, ins) => Self::sed(bus, ins),
            Instruction::Php(_, ins) => Self::php(bus, ins),
            Instruction::Pla(_, ins) => Self::pla(bus, ins),
            Instruction::And(_, ins) => Self::and(bus, ins),
            Instruction::Cmp(_, ins) => Self::cmp(bus, ins),
            Instruction::Cld(_, ins) => Self::cld(bus, ins),
            Instruction::Pha(_, ins) => Self::pha(bus, ins),
            Instruction::Plp(_, ins) => Self::plp(bus, ins),
            Instruction::Bmi(_, ins) => Self::bmi(bus, ins),
            Instruction::Ora(_, ins) => Self::ora(bus, ins),
            Instruction::Clv(_, ins) => Self::clv(bus, ins),
            Instruction::Eor(_, ins) => Self::eor(bus, ins),
            Instruction::Adc(_, ins) => Self::adc(bus, ins),
            Instruction::Ldy(_, ins) => Self::ldy(bus, ins),
        }
    }

    fn jmp(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        let address = instruction.mode.addressing(bus)?;
        bus.registers_mut().pc = address.0;
        Some(instruction.cycles)
    }
    fn ldx(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        let address = instruction.mode.addressing(bus)?;
        let data = instruction.mode.read(bus, address.0)?;
        bus.registers_mut().x = data;
        bus.registers_mut().set_z_n_flags(data);
        Some(get_cross_page_cycles(instruction, address.1))
    }
    fn stx(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        let address = instruction.mode.addressing(bus)?;
        if !bus.cpu_write(address.0, bus.registers().x) {
            return None;
        }
        Some(instruction.cycles)
    }
    fn jsr(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        let address = instruction.mode.addressing(bus)?;
        if !bus.stack_push_word(bus.registers().pc - 1) {
            return None;
        }
        bus.registers_mut().pc = address.0;
        Some(instruction.cycles)
    }
    #[allow(clippy::unnecessary_wraps)]
    fn nop(_bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        Some(instruction.cycles)
    }
    #[allow(clippy::unnecessary_wraps)]
    fn sec(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        bus.registers_mut().set_c_flag(true);
        Some(instruction.cycles)
    }
    fn bcs(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        let jmp_success = bus.registers().has_c_flag();
        let address = instruction.mode.addressing(bus)?;
        if jmp_success {
            bus.registers_mut().pc = address.0;
        }
        Some(get_branch_cycles(instruction, address.1, jmp_success))
    }
    #[allow(clippy::unnecessary_wraps)]
    fn clc(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        bus.registers_mut().set_c_flag(false);
        Some(instruction.cycles)
    }
    fn bcc(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        let jmp_success = !bus.registers().has_c_flag();
        let address = instruction.mode.addressing(bus)?;
        if jmp_success {
            bus.registers_mut().pc = address.0;
        }
        Some(get_branch_cycles(instruction, address.1, jmp_success))
    }

    fn lda(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        let address = instruction.mode.addressing(bus)?;
        let data = instruction.mode.read(bus, address.0)?;
        bus.registers_mut().a = data;
        bus.registers_mut().set_z_n_flags(data);
        Some(get_cross_page_cycles(instruction, address.1))
    }
    fn beq(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        let jmp_success = bus.registers().has_z_flag();
        let address = instruction.mode.addressing(bus)?;
        if jmp_success {
            bus.registers_mut().pc = address.0;
        }
        Some(get_branch_cycles(instruction, address.1, jmp_success))
    }
    fn bne(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        let jmp_success = !bus.registers().has_z_flag();
        let address = instruction.mode.addressing(bus)?;
        if jmp_success {
            bus.registers_mut().pc = address.0;
        }
        Some(get_branch_cycles(instruction, address.1, jmp_success))
    }
    fn sta(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        let address = instruction.mode.addressing(bus)?;
        if !bus.cpu_write(address.0, bus.registers().a) {
            return None;
        }
        Some(instruction.cycles)
    }
    fn bit(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        let address = instruction.mode.addressing(bus)?;
        let data = instruction.mode.read(bus, address.0)?;
        let a = bus.registers().a;
        bus.registers_mut().set_z_flag(data & a);
        bus.registers_mut()
            .set_v_flag(data & 0b01000000 == 0b01000000);
        bus.registers_mut().set_n_flag(data);
        Some(instruction.cycles)
    }
    fn bvs(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        let jmp_success = bus.registers().has_v_flag();
        let address = instruction.mode.addressing(bus)?;
        if jmp_success {
            bus.registers_mut().pc = address.0;
        }
        Some(get_branch_cycles(instruction, address.1, jmp_success))
    }
    fn bvc(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        let jmp_success = !bus.registers().has_v_flag();
        let address = instruction.mode.addressing(bus)?;
        if jmp_success {
            bus.registers_mut().pc = address.0;
        }
        Some(get_branch_cycles(instruction, address.1, jmp_success))
    }
    fn bpl(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        let jmp_success = !bus.registers().has_n_flag();
        let address = instruction.mode.addressing(bus)?;
        if jmp_success {
            bus.registers_mut().pc = address.0;
        }
        Some(get_branch_cycles(instruction, address.1, jmp_success))
    }
    fn rts(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        let address = bus.stack_pop_word()?;
        bus.registers_mut().pc = address + 1;
        Some(instruction.cycles)
    }
    #[allow(clippy::unnecessary_wraps)]
    fn sei(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        bus.registers_mut().set_i_flag(true);
        Some(instruction.cycles)
    }
    fn asl(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        let address = instruction.mode.addressing(bus)?;
        let mut data = instruction.mode.read(bus, address.0)?;
        bus.registers_mut().set_c_flag(data >> 7 == 1);
        data <<= 1;
        bus.registers_mut().set_z_n_flags(data);
        if !instruction.mode.write(bus, address.0, data) {
            return None;
        }
        Some(instruction.cycles)
    }
    #[allow(clippy::unnecessary_wraps)]
    fn sed(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        bus.registers_mut().set_d_flag(true);
        Some(instruction.cycles)
    }

    fn php(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        if !bus.stack_push(bus.registers().p | P_FLAGS_U | P_FLAGS_B) {
            return None;
        }
        Some(instruction.cycles)
    }

    fn pla(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        bus.registers_mut().a = bus.stack_pop()?;
        let a = bus.registers().a;
        bus.registers_mut().set_z_n_flags(a);
        Some(instruction.cycles)
    }

    fn and(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        let address = instruction.mode.addressing(bus)?;
        let data = instruction.mode.read(bus, address.0)?;
        let result = bus.registers().a & data;
        bus.registers_mut().a = result;
        bus.registers_mut().set_z_n_flags(result);
        Some(get_cross_page_cycles(instruction, address.1))
    }
    fn cmp(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        let address = instruction.mode.addressing(bus)?;
        let data = instruction.mode.read(bus, address.0)?;
        let result = bus.registers().a as i16 - data as i16;
        bus.registers_mut().set_z_n_flags(result as u8);
        bus.registers_mut().set_c_flag(result >= 0);
        Some(get_cross_page_cycles(instruction, address.1))
    }
    #[allow(clippy::unnecessary_wraps)]
    fn cld(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        bus.registers_mut().set_d_flag(false);
        Some(instruction.cycles)
    }

    fn pha(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        if !bus.stack_push(bus.registers().a) {
            return None;
        }
        Some(instruction.cycles)
    }

    fn plp(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        let p = bus.stack_pop()?;
        bus.registers_mut().p = p;
        bus.registers_mut().set_u_flag(true);
        bus.registers_mut().set_b_flag(false);
        Some(instruction.cycles)
    }

    fn bmi(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        let jmp_success = bus.registers().has_n_flag();
        let address = instruction.mode.addressing(bus)?;
        if jmp_success {
            bus.registers_mut().pc = address.0;
        }
        Some(get_branch_cycles(instruction, address.1, jmp_success))
    }

    fn ora(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        let address = instruction.mode.addressing(bus)?;
        let data = instruction.mode.read(bus, address.0)?;
        let result = bus.registers().a | data;
        bus.registers_mut().a = result;
        bus.registers_mut().set_z_n_flags(result);
        Some(get_cross_page_cycles(instruction, address.1))
    }

    #[allow(clippy::unnecessary_wraps)]
    fn clv(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        bus.registers_mut().set_v_flag(false);
        Some(instruction.cycles)
    }

    fn eor(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        let address = instruction.mode.addressing(bus)?;
        let data = instruction.mode.read(bus, address.0)?;
        let result = bus.registers().a ^ data;
        bus.registers_mut().a = result;
        bus.registers_mut().set_z_n_flags(result);
        Some(get_cross_page_cycles(instruction, address.1))
    }

    fn adc(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        let address = instruction.mode.addressing(bus)?;
        let data = instruction.mode.read(bus, address.0)?;
        let result =
            bus.registers().a as u16 + data as u16 + (bus.registers().p & P_FLAGS_C) as u16;
        let af = bus.registers().a >> 7;
        let bf = data >> 7;
        let cf = ((result >> 7) & 1) as u8;
        bus.registers_mut().set_v_flag(af == bf && af != cf);
        bus.registers_mut().set_c_flag((result >> 8) & 1 == 1);
        bus.registers_mut().a = result as u8;
        bus.registers_mut().set_z_n_flags(result as u8);
        Some(get_cross_page_cycles(instruction, address.1))
    }

    fn ldy(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        let address = instruction.mode.addressing(bus)?;
        let data = instruction.mode.read(bus, address.0)?;
        bus.registers_mut().y = data;
        bus.registers_mut().set_z_n_flags(data);
        Some(get_cross_page_cycles(instruction, address.1))
    }
}

fn get_cross_page_cycles(ins: InstructionInfo, page_crossed: bool) -> u32 {
    ins.cycles
        + if ins.can_cross_page && page_crossed {
            1
        } else {
            0
        }
}
fn get_branch_cycles(ins: InstructionInfo, page_crossed: bool, success: bool) -> u32 {
    ins.cycles
        + if success {
            if ins.can_cross_page && page_crossed {
                2
            } else {
                1
            }
        } else {
            0
        }
}
