use super::CpuBus;
use crate::memory::Result;
#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub enum AddressingMode {
    Implicit,
    Accumulator,
    Immediate,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Indirect,
    IndirectX,
    IndirectY,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum AddressingType {
    Data,
    Address,
}

impl AddressingMode {
    /// 寻址成功返回 `Some((地址, 是否跨页))`
    pub fn addressing(&self, bus: &mut CpuBus) -> Result<(u16, bool)> {
        match self {
            AddressingMode::Implicit => Self::implicit_addressing(bus),
            AddressingMode::Accumulator => Self::accumulator_addressing(bus),
            AddressingMode::Immediate => Self::immediate_addressing(bus),
            AddressingMode::Absolute => Self::absolute_addressing(bus),
            AddressingMode::AbsoluteX => Self::absolute_x_addressing(bus),
            AddressingMode::AbsoluteY => Self::absolute_y_addressing(bus),
            AddressingMode::ZeroPage => Self::zero_page_addressing(bus),
            AddressingMode::ZeroPageX => Self::zero_page_x_addressing(bus),
            AddressingMode::ZeroPageY => Self::zero_page_y_addressing(bus),
            AddressingMode::Relative => Self::relative_addressing(bus),
            AddressingMode::Indirect => Self::indirect_addressing(bus),
            AddressingMode::IndirectX => Self::indirect_x_addressing(bus),
            AddressingMode::IndirectY => Self::indirect_y_addressing(bus),
        }
    }
    pub fn addressing_type(&self) -> AddressingType {
        match self {
            AddressingMode::Implicit => AddressingType::Data,
            AddressingMode::Accumulator => AddressingType::Data,
            AddressingMode::Immediate => AddressingType::Data,
            AddressingMode::Absolute => AddressingType::Address,
            AddressingMode::AbsoluteX => AddressingType::Address,
            AddressingMode::AbsoluteY => AddressingType::Address,
            AddressingMode::ZeroPage => AddressingType::Address,
            AddressingMode::ZeroPageX => AddressingType::Address,
            AddressingMode::ZeroPageY => AddressingType::Address,
            AddressingMode::Relative => AddressingType::Address,
            AddressingMode::Indirect => AddressingType::Address,
            AddressingMode::IndirectX => AddressingType::Address,
            AddressingMode::IndirectY => AddressingType::Address,
        }
    }

    pub fn read(&self, bus: &CpuBus, address: u16) -> Result<u8> {
        if self.addressing_type() == AddressingType::Address {
            bus.cpu_read(address)
        } else {
            Ok(address as u8)
        }
    }
    pub fn write(&self, bus: &mut CpuBus, address: u16, data: u8) -> Result<()> {
        if *self == Self::Accumulator {
            bus.registers_mut().a = data;
            Ok(())
        } else {
            bus.cpu_write(address, data)
        }
    }
    #[allow(clippy::unnecessary_wraps)]
    fn implicit_addressing(_bus: &mut CpuBus) -> Result<(u16, bool)> {
        Ok((0, false))
    }
    #[allow(clippy::unnecessary_wraps)]
    fn accumulator_addressing(bus: &mut CpuBus) -> Result<(u16, bool)> {
        Ok((bus.registers().a as u16, false))
    }
    fn immediate_addressing(bus: &mut CpuBus) -> Result<(u16, bool)> {
        let pc = bus.registers().pc;
        bus.registers_mut().pc += 1;
        Ok((bus.cpu_read(pc)? as u16, false))
    }
    fn absolute_addressing(bus: &mut CpuBus) -> Result<(u16, bool)> {
        let pc = bus.registers().pc;
        bus.registers_mut().pc += 2;
        let address = bus.cpu_read_word(pc)?;
        Ok((address, false))
    }
    fn absolute_x_addressing(bus: &mut CpuBus) -> Result<(u16, bool)> {
        let pc = bus.registers().pc;
        bus.registers_mut().pc += 2;
        let address = bus.cpu_read_word(pc)?;
        let result = (address as i32 + bus.registers().x as i32) as u16;
        Ok((result, is_page_crossed(address, result)))
    }
    fn absolute_y_addressing(bus: &mut CpuBus) -> Result<(u16, bool)> {
        let pc = bus.registers().pc;
        bus.registers_mut().pc += 2;
        let address = bus.cpu_read_word(pc)?;
        let result = (address as i32 + bus.registers().y as i32) as u16;
        Ok((result, is_page_crossed(address, result)))
    }
    fn zero_page_addressing(bus: &mut CpuBus) -> Result<(u16, bool)> {
        let pc = bus.registers().pc;
        bus.registers_mut().pc += 1;
        Ok((bus.cpu_read(pc)? as u16, false))
    }
    fn zero_page_x_addressing(bus: &mut CpuBus) -> Result<(u16, bool)> {
        let pc = bus.registers().pc;
        bus.registers_mut().pc += 1;
        Ok((
            (bus.cpu_read(pc)? as u16 + bus.registers().x as u16) & 0x00FF,
            false,
        ))
    }
    fn zero_page_y_addressing(bus: &mut CpuBus) -> Result<(u16, bool)> {
        let pc = bus.registers().pc;
        bus.registers_mut().pc += 1;
        Ok((
            (bus.cpu_read(pc)? as u16 + bus.registers().y as u16) & 0x00FF,
            false,
        ))
    }
    fn relative_addressing(bus: &mut CpuBus) -> Result<(u16, bool)> {
        let pc = bus.registers().pc;
        bus.registers_mut().pc += 1;
        let offset = bus.cpu_read(pc)? as i8;
        let address = ((bus.registers().pc as i32) + (offset as i32)) as u16;
        Ok((address, is_page_crossed(address, bus.registers().pc)))
    }
    fn indirect_addressing(bus: &mut CpuBus) -> Result<(u16, bool)> {
        let pc = bus.registers().pc;
        bus.registers_mut().pc += 2;
        let low = bus.cpu_read_word(pc)? as u16;
        let high = (low & 0xFF00) | ((low + 1) & 0x00FF);
        let address = ((bus.cpu_read(high)? as u16) << 8) | (bus.cpu_read(low)? as u16);
        Ok((address, false))
    }
    fn indirect_x_addressing(bus: &mut CpuBus) -> Result<(u16, bool)> {
        let pc = bus.registers().pc;
        bus.registers_mut().pc += 1;
        let indirect = (bus.registers().x as u16 + bus.cpu_read(pc)? as u16) & 0x00FF;
        let low = bus.cpu_read(indirect)? as u16;
        let high = bus.cpu_read((indirect + 1) & 0x00FF)? as u16;
        let address = (high << 8) | low;
        Ok((address, is_page_crossed(indirect, address)))
    }
    fn indirect_y_addressing(bus: &mut CpuBus) -> Result<(u16, bool)> {
        let pc = bus.registers().pc;
        bus.registers_mut().pc += 1;
        let indirect = bus.cpu_read(pc)? as u16;
        let low = bus.cpu_read(indirect)? as u16;
        let high = bus.cpu_read((indirect + 1) & 0x00FF)? as u16;
        let address = (high << 8) | low;
        let result = (address as i32 + bus.registers().y as i32) as u16;
        Ok((result, is_page_crossed(address, result)))
    }
}

fn is_page_crossed(old: u16, new: u16) -> bool {
    (old & 0xFF00) != (new & 0xFF00)
}
