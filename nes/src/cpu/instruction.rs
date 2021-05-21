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
        let address = bus.stack_word()?;
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
