

use super::{CpuBus, CpuRegisters};

#[derive(PartialEq, Eq, Debug, Hash)]
pub enum AddressingMode {
    ImplicitAddressingMode,
    AccumulatorAddressingMode,
    ImmediateAddressingMode,
    AbsoluteAddressingMode,
    AbsoluteXAddressingMode,
    AbsoluteYAddressingMode,
    ZeroPageAddressingMode,
    ZeroPageXAddressingMode,
    ZeroPageYAddressingMode,
    RelativeAddressingMode,
    IndirectAddressingMode,
    IndirectXAddressingMode,
    IndirectYAddressingMode,
}
#[derive(PartialEq, Eq, Debug)]
pub enum AddressingType {
    Data,
    Address,
}

impl AddressingMode {
    pub fn addressing_type(&self) -> AddressingType {
        match self {
            AddressingMode::ImplicitAddressingMode => AddressingType::Data,
            AddressingMode::AccumulatorAddressingMode => AddressingType::Data,
            AddressingMode::ImmediateAddressingMode => AddressingType::Data,
            AddressingMode::AbsoluteAddressingMode => AddressingType::Address,
            AddressingMode::AbsoluteXAddressingMode => AddressingType::Address,
            AddressingMode::AbsoluteYAddressingMode => AddressingType::Address,
            AddressingMode::ZeroPageAddressingMode => AddressingType::Address,
            AddressingMode::ZeroPageXAddressingMode => AddressingType::Address,
            AddressingMode::ZeroPageYAddressingMode => AddressingType::Address,
            AddressingMode::RelativeAddressingMode => AddressingType::Address,
            AddressingMode::IndirectAddressingMode => AddressingType::Address,
            AddressingMode::IndirectXAddressingMode => AddressingType::Address,
            AddressingMode::IndirectYAddressingMode => AddressingType::Address,
        }
    }
    /// 寻址成功返回 `Some((地址, 是否跨页))`
    pub fn addressing(&self, bus: &CpuBus, registers: &mut CpuRegisters) -> Option<(u16, bool)> {
        match self {
            AddressingMode::ImplicitAddressingMode => Self::implicit_addressing(bus, registers),
            AddressingMode::AccumulatorAddressingMode => {
                Self::accumulator_addressing(bus, registers)
            }
            AddressingMode::ImmediateAddressingMode => Self::immediate_addressing(bus, registers),
            AddressingMode::AbsoluteAddressingMode => Self::absolute_addressing(bus, registers),
            AddressingMode::AbsoluteXAddressingMode => Self::absolute_x_addressing(bus, registers),
            AddressingMode::AbsoluteYAddressingMode => Self::absolute_y_addressing(bus, registers),
            AddressingMode::ZeroPageAddressingMode => Self::zero_page_addressing(bus, registers),
            AddressingMode::ZeroPageXAddressingMode => Self::zero_page_x_addressing(bus, registers),
            AddressingMode::ZeroPageYAddressingMode => Self::zero_page_y_addressing(bus, registers),
            AddressingMode::RelativeAddressingMode => Self::relative_addressing(bus, registers),
            AddressingMode::IndirectAddressingMode => Self::indirect_addressing(bus, registers),
            AddressingMode::IndirectXAddressingMode => Self::indirect_x_addressing(bus, registers),
            AddressingMode::IndirectYAddressingMode => Self::indirect_y_addressing(bus, registers),
        }
    }

    fn implicit_addressing(_bus: &CpuBus, _registers: &mut CpuRegisters) -> Option<(u16, bool)> {
        Some((0, false))
    }
    fn accumulator_addressing(_bus: &CpuBus, registers: &mut CpuRegisters) -> Option<(u16, bool)> {
        Some((registers.a as u16, false))
    }
    fn immediate_addressing(bus: &CpuBus, registers: &mut CpuRegisters) -> Option<(u16, bool)> {
        let pc = registers.pc;
        registers.pc += 1;
        Some((bus.cpu_read(pc)? as u16, false))
    }
    fn absolute_addressing(bus: &CpuBus, registers: &mut CpuRegisters) -> Option<(u16, bool)> {
        let pc = registers.pc;
        registers.pc += 2;
        let address = bus.cpu_read_word(pc)?;
        Some((address, false))
    }
    fn absolute_x_addressing(bus: &CpuBus, registers: &mut CpuRegisters) -> Option<(u16, bool)> {
        let pc = registers.pc;
        registers.pc += 2;
        let address = bus.cpu_read_word(pc)?;
        let result = address + registers.x as u16;
        Some((result, is_page_crossed(address, result)))
    }
    fn absolute_y_addressing(bus: &CpuBus, registers: &mut CpuRegisters) -> Option<(u16, bool)> {
        let pc = registers.pc;
        registers.pc += 2;
        let address = bus.cpu_read_word(pc)?;
        let result = address + registers.y as u16;
        Some((result, is_page_crossed(address, result)))
    }
    fn zero_page_addressing(bus: &CpuBus, registers: &mut CpuRegisters) -> Option<(u16, bool)> {
        let pc = registers.pc;
        registers.pc += 1;
        Some(((bus.cpu_read(pc)? as u16) & 0x00FF, false))
    }
    fn zero_page_x_addressing(bus: &CpuBus, registers: &mut CpuRegisters) -> Option<(u16, bool)> {
        let pc = registers.pc;
        registers.pc += 1;
        Some((((bus.cpu_read(pc)? + registers.x) as u16) & 0x00FF, false))
    }
    fn zero_page_y_addressing(bus: &CpuBus, registers: &mut CpuRegisters) -> Option<(u16, bool)> {
        let pc = registers.pc;
        registers.pc += 1;
        Some((((bus.cpu_read(pc)? + registers.y) as u16) & 0x00FF, false))
    }
    fn relative_addressing(bus: &CpuBus, registers: &mut CpuRegisters) -> Option<(u16, bool)> {
        let pc = registers.pc;
        registers.pc += 1;
        let offset = bus.cpu_read(registers.pc)? as i8;
        let address = ((pc as i32) + (offset as i32)) as u16;
        Some((address, is_page_crossed(address, pc)))
    }
    fn indirect_addressing(bus: &CpuBus, registers: &mut CpuRegisters) -> Option<(u16, bool)> {
        let pc = registers.pc;
        registers.pc += 2;
        let low = bus.cpu_read(pc)? as u16;
        let high = (low & 0xFF00) | ((low + 1) & 0x00FF);
        let address = ((bus.cpu_read(high)? as u16) << 8) | (bus.cpu_read(low)? as u16);
        Some((address, false))
    }
    fn indirect_x_addressing(bus: &CpuBus, registers: &mut CpuRegisters) -> Option<(u16, bool)> {
        let pc = registers.pc;
        registers.pc += 2;
        let indirect = (registers.x + bus.cpu_read(pc)?) as u16;
        let low = bus.cpu_read(indirect)? as u16;
        let high = bus.cpu_read(indirect + 1)? as u16;
        let address = (high << 8) | low;
        Some((address, is_page_crossed(indirect, address)))
    }
    fn indirect_y_addressing(bus: &CpuBus, registers: &mut CpuRegisters) -> Option<(u16, bool)> {
        let pc = registers.pc;
        registers.pc += 2;
        let indirect = bus.cpu_read(pc)? as u16;
        let low = bus.cpu_read(indirect)? as u16;
        let high = bus.cpu_read(indirect + 1)? as u16;
        let address = ((high << 8) | low) + registers.y as u16;
        Some((address, is_page_crossed(indirect, address)))
    }
}

fn is_page_crossed(old: u16, new: u16) -> bool {
    (old & 0xFF00) == (new & 0xFF00)
}
