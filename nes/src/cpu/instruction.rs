use super::{AddressingMode, CpuBus, CpuError};
use crate::cpu::{P_FLAGS_B, P_FLAGS_C, P_FLAGS_U};

#[derive(Debug)]
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
    Cpy(u8, InstructionInfo),
    Cpx(u8, InstructionInfo),
    Sbc(u8, InstructionInfo),
    Iny(u8, InstructionInfo),
    Inx(u8, InstructionInfo),
    Dey(u8, InstructionInfo),
    Dex(u8, InstructionInfo),
    Tay(u8, InstructionInfo),
    Tax(u8, InstructionInfo),
    Txa(u8, InstructionInfo),
    Tya(u8, InstructionInfo),
    Tsx(u8, InstructionInfo),
    Txs(u8, InstructionInfo),
    Rti(u8, InstructionInfo),
    Lsr(u8, InstructionInfo),
    Ror(u8, InstructionInfo),
    Rol(u8, InstructionInfo),
    Sty(u8, InstructionInfo),
    Inc(u8, InstructionInfo),
    Dec(u8, InstructionInfo),
    Dop(u8, InstructionInfo),
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
                    mode: AddressingMode::AbsoluteAddressingMode,
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
                    mode: AddressingMode::IndirectXAddressingMode,
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

            // CPY
            0xC0 => Self::Cpy(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImmediateAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),
            0xC4 => Self::Cpy(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageAddressingMode,
                    cycles: 3,
                    can_cross_page: false,
                },
            ),
            0xCC => Self::Cpy(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteAddressingMode,
                    cycles: 4,
                    can_cross_page: false,
                },
            ),

            // CPX
            0xE0 => Self::Cpx(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImmediateAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),
            0xE4 => Self::Cpx(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageAddressingMode,
                    cycles: 3,
                    can_cross_page: false,
                },
            ),
            0xEC => Self::Cpx(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteAddressingMode,
                    cycles: 4,
                    can_cross_page: false,
                },
            ),

            // SBC
            0xE9 => Self::Sbc(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImmediateAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),
            0xE5 => Self::Sbc(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageAddressingMode,
                    cycles: 3,
                    can_cross_page: false,
                },
            ),
            0xF5 => Self::Sbc(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageXAddressingMode,
                    cycles: 4,
                    can_cross_page: false,
                },
            ),
            0xED => Self::Sbc(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteAddressingMode,
                    cycles: 4,
                    can_cross_page: false,
                },
            ),
            0xFD => Self::Sbc(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteXAddressingMode,
                    cycles: 4,
                    can_cross_page: true,
                },
            ),
            0xF9 => Self::Sbc(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteYAddressingMode,
                    cycles: 4,
                    can_cross_page: true,
                },
            ),
            0xE1 => Self::Sbc(
                ins,
                InstructionInfo {
                    mode: AddressingMode::IndirectXAddressingMode,
                    cycles: 6,
                    can_cross_page: false,
                },
            ),
            0xF1 => Self::Sbc(
                ins,
                InstructionInfo {
                    mode: AddressingMode::IndirectYAddressingMode,
                    cycles: 5,
                    can_cross_page: true,
                },
            ),

            // INY
            0xC8 => Self::Iny(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImplicitAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),

            // INX
            0xE8 => Self::Inx(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImplicitAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),
            // DEY
            0x88 => Self::Dey(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImplicitAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),

            // DEX
            0xCA => Self::Dex(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImplicitAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),

            // TAX
            0xAA => Self::Tax(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImplicitAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),

            // TAY
            0xA8 => Self::Tay(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImplicitAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),

            // TXA
            0x8A => Self::Txa(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImplicitAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),

            // TYA
            0x98 => Self::Tya(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImplicitAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),

            // TSX
            0xBA => Self::Tsx(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImplicitAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),

            // TXS
            0x9A => Self::Txs(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImplicitAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),

            // RTI
            0x40 => Self::Rti(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImplicitAddressingMode,
                    cycles: 6,
                    can_cross_page: false,
                },
            ),

            // LSR
            0x4A => Self::Lsr(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AccumulatorAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),
            0x46 => Self::Lsr(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageAddressingMode,
                    cycles: 5,
                    can_cross_page: false,
                },
            ),
            0x56 => Self::Lsr(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageXAddressingMode,
                    cycles: 6,
                    can_cross_page: false,
                },
            ),
            0x4E => Self::Lsr(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteAddressingMode,
                    cycles: 6,
                    can_cross_page: false,
                },
            ),
            0x5E => Self::Lsr(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteXAddressingMode,
                    cycles: 7,
                    can_cross_page: false,
                },
            ),

            // ROR
            0x6A => Self::Ror(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AccumulatorAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),
            0x66 => Self::Ror(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageAddressingMode,
                    cycles: 5,
                    can_cross_page: false,
                },
            ),
            0x76 => Self::Ror(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageXAddressingMode,
                    cycles: 6,
                    can_cross_page: false,
                },
            ),
            0x6E => Self::Ror(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteAddressingMode,
                    cycles: 6,
                    can_cross_page: false,
                },
            ),
            0x7E => Self::Ror(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteXAddressingMode,
                    cycles: 7,
                    can_cross_page: false,
                },
            ),

            // ROL
            0x2A => Self::Rol(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AccumulatorAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),
            0x26 => Self::Rol(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageAddressingMode,
                    cycles: 5,
                    can_cross_page: false,
                },
            ),
            0x36 => Self::Rol(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageXAddressingMode,
                    cycles: 6,
                    can_cross_page: false,
                },
            ),
            0x2E => Self::Rol(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteAddressingMode,
                    cycles: 6,
                    can_cross_page: false,
                },
            ),
            0x3E => Self::Rol(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteXAddressingMode,
                    cycles: 7,
                    can_cross_page: false,
                },
            ),

            // STY
            0x84 => Self::Sty(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageAddressingMode,
                    cycles: 3,
                    can_cross_page: false,
                },
            ),
            0x8C => Self::Sty(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteAddressingMode,
                    cycles: 4,
                    can_cross_page: false,
                },
            ),
            0x94 => Self::Sty(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageXAddressingMode,
                    cycles: 4,
                    can_cross_page: false,
                },
            ),

            // DEC
            0xC6 => Self::Dec(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageAddressingMode,
                    cycles: 5,
                    can_cross_page: false,
                },
            ),

            0xD6 => Self::Dec(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageXAddressingMode,
                    cycles: 6,
                    can_cross_page: false,
                },
            ),
            0xCE => Self::Dec(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteAddressingMode,
                    cycles: 6,
                    can_cross_page: false,
                },
            ),
            0xDE => Self::Dec(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteXAddressingMode,
                    cycles: 7,
                    can_cross_page: false,
                },
            ),

            // INC
            0xE6 => Self::Inc(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageAddressingMode,
                    cycles: 5,
                    can_cross_page: false,
                },
            ),

            0xF6 => Self::Inc(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageXAddressingMode,
                    cycles: 6,
                    can_cross_page: false,
                },
            ),
            0xEE => Self::Inc(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteAddressingMode,
                    cycles: 6,
                    can_cross_page: false,
                },
            ),
            0xFE => Self::Inc(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteXAddressingMode,
                    cycles: 7,
                    can_cross_page: false,
                },
            ),

            // DOP
            0x04 => Self::Dop(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageAddressingMode,
                    cycles: 3,
                    can_cross_page: false,
                },
            ),
            0x14 => Self::Dop(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageXAddressingMode,
                    cycles: 4,
                    can_cross_page: false,
                },
            ),
            0x34 => Self::Dop(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageXAddressingMode,
                    cycles: 4,
                    can_cross_page: false,
                },
            ),
            0x44 => Self::Dop(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageAddressingMode,
                    cycles: 3,
                    can_cross_page: false,
                },
            ),

            0x54 => Self::Dop(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageXAddressingMode,
                    cycles: 4,
                    can_cross_page: false,
                },
            ),
            0x64 => Self::Dop(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageAddressingMode,
                    cycles: 3,
                    can_cross_page: false,
                },
            ),

            0x74 => Self::Dop(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageXAddressingMode,
                    cycles: 4,
                    can_cross_page: false,
                },
            ),

            0x80 => Self::Dop(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImmediateAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),
            0x82 => Self::Dop(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImmediateAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),

            0x89 => Self::Dop(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImmediateAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),

            0xC2 => Self::Dop(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImmediateAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),
            0xD4 => Self::Dop(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageXAddressingMode,
                    cycles: 4,
                    can_cross_page: false,
                },
            ),

            0xE2 => Self::Dop(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ImmediateAddressingMode,
                    cycles: 2,
                    can_cross_page: false,
                },
            ),

            0xF4 => Self::Dop(
                ins,
                InstructionInfo {
                    mode: AddressingMode::ZeroPageXAddressingMode,
                    cycles: 4,
                    can_cross_page: false,
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
            Instruction::Cpy(_, ins) => Self::cpy(bus, ins),
            Instruction::Cpx(_, ins) => Self::cpx(bus, ins),
            Instruction::Sbc(_, ins) => Self::sbc(bus, ins),
            Instruction::Iny(_, ins) => Self::iny(bus, ins),
            Instruction::Inx(_, ins) => Self::inx(bus, ins),
            Instruction::Dey(_, ins) => Self::dey(bus, ins),
            Instruction::Dex(_, ins) => Self::dex(bus, ins),
            Instruction::Tay(_, ins) => Self::tay(bus, ins),
            Instruction::Tax(_, ins) => Self::tax(bus, ins),
            Instruction::Txa(_, ins) => Self::txa(bus, ins),
            Instruction::Tya(_, ins) => Self::tya(bus, ins),
            Instruction::Tsx(_, ins) => Self::tsx(bus, ins),
            Instruction::Txs(_, ins) => Self::txs(bus, ins),
            Instruction::Rti(_, ins) => Self::rti(bus, ins),
            Instruction::Lsr(_, ins) => Self::lsr(bus, ins),
            Instruction::Ror(_, ins) => Self::ror(bus, ins),
            Instruction::Rol(_, ins) => Self::rol(bus, ins),
            Instruction::Sty(_, ins) => Self::sty(bus, ins),
            Instruction::Inc(_, ins) => Self::inc(bus, ins),
            Instruction::Dec(_, ins) => Self::dec(bus, ins),
            Instruction::Dop(_, ins) => Self::dop(bus, ins),
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

    fn nop(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        instruction.mode.addressing(bus)?;
        Some(instruction.cycles)
    }
    fn sec(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        instruction.mode.addressing(bus)?;
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

    fn clc(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        instruction.mode.addressing(bus)?;
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
        bus.registers_mut().set_z_flag(data & a == 0);
        bus.registers_mut()
            .set_v_flag(data & 0b01000000 == 0b01000000);
        bus.registers_mut().set_n_flag(data >> 7 == 1);
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

    fn sei(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        instruction.mode.addressing(bus)?;
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

    fn sed(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        instruction.mode.addressing(bus)?;
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

    fn cld(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        instruction.mode.addressing(bus)?;
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

    fn clv(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        instruction.mode.addressing(bus)?;
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
    fn cpy(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        let address = instruction.mode.addressing(bus)?;
        let y = bus.registers().y;
        let m = instruction.mode.read(bus, address.0)?;
        let result = (y as i16 - m as i16) as u8;
        bus.registers_mut().set_c_flag(y >= m);
        bus.registers_mut().set_z_flag(y == m);
        bus.registers_mut().set_n_flag(result >> 7 == 1);
        Some(instruction.cycles)
    }
    fn cpx(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        let address = instruction.mode.addressing(bus)?;
        let x = bus.registers().x;
        let m = instruction.mode.read(bus, address.0)?;
        let result = (x as i16 - m as i16) as u8;
        bus.registers_mut().set_c_flag(x >= m);
        bus.registers_mut().set_z_flag(x == m);
        bus.registers_mut().set_n_flag(result >> 7 == 1);
        Some(instruction.cycles)
    }

    fn sbc(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        let address = instruction.mode.addressing(bus)?;
        let data = instruction.mode.read(bus, address.0)?;
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
        Some(get_cross_page_cycles(instruction, address.1))
    }

    fn iny(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        instruction.mode.addressing(bus)?;
        let result = (bus.registers().y as i16 + 1) as u8;
        bus.registers_mut().y = result;
        bus.registers_mut().set_z_n_flags(result);
        Some(instruction.cycles)
    }

    fn inx(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        instruction.mode.addressing(bus)?;
        let result = (bus.registers().x as i16 + 1) as u8;
        bus.registers_mut().x = result;
        bus.registers_mut().set_z_n_flags(result);
        Some(instruction.cycles)
    }

    fn dex(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        instruction.mode.addressing(bus)?;
        let result = (bus.registers().x as i16 - 1) as u8;
        bus.registers_mut().x = result;
        bus.registers_mut().set_z_n_flags(result);
        Some(instruction.cycles)
    }

    fn dey(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        instruction.mode.addressing(bus)?;
        let result = (bus.registers().y as i16 - 1) as u8;
        bus.registers_mut().y = result;
        bus.registers_mut().set_z_n_flags(result);
        Some(instruction.cycles)
    }

    fn tax(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        instruction.mode.addressing(bus)?;
        let result = bus.registers().a;
        bus.registers_mut().x = result;
        bus.registers_mut().set_z_n_flags(result);
        Some(instruction.cycles)
    }

    fn tay(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        instruction.mode.addressing(bus)?;
        let result = bus.registers().a;
        bus.registers_mut().y = result;
        bus.registers_mut().set_z_n_flags(result);
        Some(instruction.cycles)
    }

    fn txa(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        instruction.mode.addressing(bus)?;
        let result = bus.registers().x;
        bus.registers_mut().a = result;
        bus.registers_mut().set_z_n_flags(result);
        Some(instruction.cycles)
    }

    fn tya(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        instruction.mode.addressing(bus)?;
        let result = bus.registers().y;
        bus.registers_mut().a = result;
        bus.registers_mut().set_z_n_flags(result);
        Some(instruction.cycles)
    }

    fn tsx(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        instruction.mode.addressing(bus)?;
        let result = bus.registers().sp;
        bus.registers_mut().x = result;
        bus.registers_mut().set_z_n_flags(result);
        Some(instruction.cycles)
    }

    fn txs(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        instruction.mode.addressing(bus)?;
        let result = bus.registers().x;
        bus.registers_mut().sp = result;
        Some(instruction.cycles)
    }

    fn rti(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        let result = bus.stack_pop()? | P_FLAGS_U;
        bus.registers_mut().p = result;
        bus.registers_mut().pc = bus.stack_pop_word()?;
        Some(instruction.cycles)
    }

    fn lsr(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        let address = instruction.mode.addressing(bus)?;
        let data = instruction.mode.read(bus, address.0)?;
        let result = data >> 1;
        if !instruction.mode.write(bus, address.0, result) {
            return None;
        }
        bus.registers_mut().set_c_flag(data & 1 == 1);
        bus.registers_mut().set_z_n_flags(result);
        Some(instruction.cycles)
    }
    fn ror(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        let address = instruction.mode.addressing(bus)?;
        let data = instruction.mode.read(bus, address.0)?;
        let result = (data >> 1) | ((bus.registers().p & P_FLAGS_C) << 7);
        if !instruction.mode.write(bus, address.0, result) {
            return None;
        }
        bus.registers_mut().set_c_flag(data & 1 == 1);
        bus.registers_mut().set_z_n_flags(result);
        Some(instruction.cycles)
    }
    fn rol(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        let address = instruction.mode.addressing(bus)?;
        let data = instruction.mode.read(bus, address.0)?;
        let result = (data << 1) | (bus.registers().p & P_FLAGS_C);
        if !instruction.mode.write(bus, address.0, result) {
            return None;
        }
        bus.registers_mut().set_c_flag(data >> 7 == 1);
        bus.registers_mut().set_z_n_flags(result);
        Some(instruction.cycles)
    }

    fn sty(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        let address = instruction.mode.addressing(bus)?;
        if !bus.cpu_write(address.0, bus.registers().y) {
            return None;
        }
        Some(instruction.cycles)
    }
    fn inc(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        let address = instruction.mode.addressing(bus)?;
        let data = instruction.mode.read(bus, address.0)?;
        let data = ((data as i16) + 1) as u8;
        if !instruction.mode.write(bus, address.0, data) {
            return None;
        }
        bus.registers_mut().set_z_n_flags(data);
        Some(instruction.cycles)
    }

    fn dec(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        let address = instruction.mode.addressing(bus)?;
        let data = instruction.mode.read(bus, address.0)?;
        let result = ((data as i16) - 1) as u8;
        if !instruction.mode.write(bus, address.0, result) {
            return None;
        }
        bus.registers_mut().set_z_n_flags(result);
        Some(instruction.cycles)
    }

    fn dop(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        instruction.mode.addressing(bus)?;
        Some(instruction.cycles)
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
