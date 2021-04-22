use super::{AddressingMode, CpuBus, CpuError};

pub(crate) struct InstructionProcessor;

impl InstructionProcessor {
    pub fn process(&self, ins: u8, bus: &mut CpuBus) -> Result<u32, CpuError> {
        let instruction =
            Instruction::from_instruction(ins).ok_or(CpuError::UnknownInstructionError(ins))?;
        instruction
            .invoke(bus)
            .ok_or(CpuError::InvokeInstructionError(ins))
            .map(|ext_cycles| instruction.cycles + ext_cycles)
    }
}

#[derive(Copy, Clone)]
enum InstructionKind {
    Jmp(u8),
}

struct Instruction {
    kind: InstructionKind,
    mode: AddressingMode,
    cycles: u32,
    can_cross_page: bool,
}

impl Instruction {
    /// 返回寻址模式和时钟周期
    fn from_instruction(ins: u8) -> Option<Self> {
        Some(match ins {
            0x4C => Self {
                kind: InstructionKind::Jmp(ins),
                mode: AddressingMode::AbsoluteAddressingMode,
                cycles: 3,
                can_cross_page: false,
            },
            0x6c => Self {
                kind: InstructionKind::Jmp(ins),
                mode: AddressingMode::IndirectAddressingMode,
                cycles: 5,
                can_cross_page: false,
            },
            _ => None?,
        })
    }
    /// 返回额外的时钟周期
    fn invoke(&self, bus: &mut CpuBus) -> Option<u32> {
        let addressing = self.mode.addressing(bus)?;
        match self.kind {
            InstructionKind::Jmp(_) => Self::jmp(
                bus,
                self.mode,
                addressing.0,
                self.can_cross_page,
                addressing.1,
            ),
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    fn jmp(
        bus: &mut CpuBus,
        _addressing: AddressingMode,
        data: u16,
        _can_cross_page: bool,
        _page_crossed: bool,
    ) -> Option<u32> {
        bus.registers_mut().pc = data;
        Some(0)
    }
}
