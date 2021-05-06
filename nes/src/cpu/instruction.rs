use super::{AddressingMode, CpuBus, CpuError};

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

pub(super) struct InstructionInfo {
    mode: AddressingMode,
    cycles: u32,
    can_cross_page: bool,
}

pub(super) enum Instruction {
    Jmp(u8, InstructionInfo),
    Ldx(u8, InstructionInfo),
    Stx(u8, InstructionInfo),
    Jsr(u8, InstructionInfo),
    Nop(u8, InstructionInfo),
    Sec(u8, InstructionInfo),
    Bcs(u8, InstructionInfo),
    Clc(u8, InstructionInfo),
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
