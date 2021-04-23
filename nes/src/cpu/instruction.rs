use super::{AddressingMode, CpuBus, CpuError};

pub(crate) struct InstructionProcessor;

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

            _ => None?,
        })
    }
    /// 返回时钟周期
    fn invoke(self, bus: &mut CpuBus) -> Option<u32> {
        match self {
            Instruction::Jmp(_, ins) => Self::jmp(bus, ins),
            Instruction::Ldx(_, ins) => Self::ldx(bus, ins),
            Instruction::Stx(_, ins) => Self::stx(bus, ins),
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
        Some(instruction.cycles + if instruction.can_cross_page { 1 } else { 0 })
    }
    fn stx(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        let address = instruction.mode.addressing(bus)?;
        if !bus.cpu_write(address.0, bus.registers().x) {
            return None;
        }
        Some(instruction.cycles + if instruction.can_cross_page { 1 } else { 0 })
    }
}
