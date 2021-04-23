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

enum Instruction {
    Jmp(u8, InstructionInfo),
}

struct InstructionInfo {
    mode: AddressingMode,
    cycles: u32,
    can_cross_page: bool,
}

impl Instruction {
    /// 返回寻址模式和时钟周期
    fn from_instruction(ins: u8) -> Option<Self> {
        Some(match ins {
            0x4C => Self::Jmp(
                ins,
                InstructionInfo {
                    mode: AddressingMode::AbsoluteAddressingMode,
                    cycles: 3,
                    can_cross_page: false,
                },
            ),
            0x6c => Self::Jmp(
                ins,
                InstructionInfo {
                    mode: AddressingMode::IndirectAddressingMode,
                    cycles: 5,
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
        }
    }

    fn jmp(bus: &mut CpuBus, instruction: InstructionInfo) -> Option<u32> {
        let address = instruction.mode.addressing(bus)?;
        bus.registers_mut().pc = address.0;
        Some(instruction.cycles)
    }
