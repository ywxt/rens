use super::CpuBus;

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

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum AddressingType {
    Data,
    Address,
}

impl AddressingMode {
    /// 寻址成功返回 `Some((地址, 是否跨页))`
    pub fn addressing(&self, bus: &mut CpuBus) -> Option<(u16, bool)> {
        match self {
            AddressingMode::ImplicitAddressingMode => Self::implicit_addressing(bus),
            AddressingMode::AccumulatorAddressingMode => Self::accumulator_addressing(bus),
            AddressingMode::ImmediateAddressingMode => Self::immediate_addressing(bus),
            AddressingMode::AbsoluteAddressingMode => Self::absolute_addressing(bus),
            AddressingMode::AbsoluteXAddressingMode => Self::absolute_x_addressing(bus),
            AddressingMode::AbsoluteYAddressingMode => Self::absolute_y_addressing(bus),
            AddressingMode::ZeroPageAddressingMode => Self::zero_page_addressing(bus),
            AddressingMode::ZeroPageXAddressingMode => Self::zero_page_x_addressing(bus),
            AddressingMode::ZeroPageYAddressingMode => Self::zero_page_y_addressing(bus),
            AddressingMode::RelativeAddressingMode => Self::relative_addressing(bus),
            AddressingMode::IndirectAddressingMode => Self::indirect_addressing(bus),
            AddressingMode::IndirectXAddressingMode => Self::indirect_x_addressing(bus),
            AddressingMode::IndirectYAddressingMode => Self::indirect_y_addressing(bus),
        }
    }
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

    pub fn read(&self, bus: &CpuBus, address: u16) -> Option<u8> {
        Some(if self.addressing_type() == AddressingType::Address {
            bus.cpu_read(address)?
        } else {
            address as u8
        })
    }
    pub fn write(&self, bus: &mut CpuBus, address: u16, data: u8) -> bool {
        if *self == Self::AccumulatorAddressingMode {
            bus.registers_mut().a = data;
            true
        } else if self.addressing_type() == AddressingType::Address {
            bus.cpu_write(address, data)
        } else {
            false
        }
    }
    #[allow(clippy::unnecessary_wraps)]
    fn implicit_addressing(_bus: &mut CpuBus) -> Option<(u16, bool)> {
        Some((0, false))
    }
    #[allow(clippy::unnecessary_wraps)]
    fn accumulator_addressing(bus: &mut CpuBus) -> Option<(u16, bool)> {
        Some((bus.registers().a as u16, false))
    }
    fn immediate_addressing(bus: &mut CpuBus) -> Option<(u16, bool)> {
        let pc = bus.registers().pc;
        bus.registers_mut().pc += 1;
        Some((bus.cpu_read(pc)? as u16, false))
    }
    fn absolute_addressing(bus: &mut CpuBus) -> Option<(u16, bool)> {
        let pc = bus.registers().pc;
        bus.registers_mut().pc += 2;
        let address = bus.cpu_read_word(pc)?;
        Some((address, false))
    }
    fn absolute_x_addressing(bus: &mut CpuBus) -> Option<(u16, bool)> {
        let pc = bus.registers().pc;
        bus.registers_mut().pc += 2;
        let address = bus.cpu_read_word(pc)?;
        let result = (address as i32 + bus.registers().x as i32) as u16;
        Some((result, is_page_crossed(address, result)))
    }
    fn absolute_y_addressing(bus: &mut CpuBus) -> Option<(u16, bool)> {
        let pc = bus.registers().pc;
        bus.registers_mut().pc += 2;
        let address = bus.cpu_read_word(pc)?;
        let result = (address as i32 + bus.registers().y as i32) as u16;
        Some((result, is_page_crossed(address, result)))
    }
    fn zero_page_addressing(bus: &mut CpuBus) -> Option<(u16, bool)> {
        let pc = bus.registers().pc;
        bus.registers_mut().pc += 1;
        Some((bus.cpu_read(pc)? as u16, false))
    }
    fn zero_page_x_addressing(bus: &mut CpuBus) -> Option<(u16, bool)> {
        let pc = bus.registers().pc;
        bus.registers_mut().pc += 1;
        Some((
            (bus.cpu_read(pc)? as u16 + bus.registers().x as u16) & 0x00FF,
            false,
        ))
    }
    fn zero_page_y_addressing(bus: &mut CpuBus) -> Option<(u16, bool)> {
        let pc = bus.registers().pc;
        bus.registers_mut().pc += 1;
        Some((
            (bus.cpu_read(pc)? as u16 + bus.registers().y as u16) & 0x00FF,
            false,
        ))
    }
    fn relative_addressing(bus: &mut CpuBus) -> Option<(u16, bool)> {
        let pc = bus.registers().pc;
        bus.registers_mut().pc += 1;
        let offset = bus.cpu_read(pc)? as i8;
        let address = ((bus.registers().pc as i32) + (offset as i32)) as u16;
        Some((address, is_page_crossed(address, bus.registers().pc)))
    }
    fn indirect_addressing(bus: &mut CpuBus) -> Option<(u16, bool)> {
        let pc = bus.registers().pc;
        bus.registers_mut().pc += 2;
        let low = bus.cpu_read_word(pc)? as u16;
        let high = (low & 0xFF00) | ((low + 1) & 0x00FF);
        let address = ((bus.cpu_read(high)? as u16) << 8) | (bus.cpu_read(low)? as u16);
        Some((address, false))
    }
    fn indirect_x_addressing(bus: &mut CpuBus) -> Option<(u16, bool)> {
        let pc = bus.registers().pc;
        bus.registers_mut().pc += 1;
        let indirect = (bus.registers().x as u16 + bus.cpu_read(pc)? as u16) & 0x00FF;
        let low = bus.cpu_read(indirect)? as u16;
        let high = bus.cpu_read((indirect + 1) & 0x00FF)? as u16;
        let address = (high << 8) | low;
        Some((address, is_page_crossed(indirect, address)))
    }
    fn indirect_y_addressing(bus: &mut CpuBus) -> Option<(u16, bool)> {
        let pc = bus.registers().pc;
        bus.registers_mut().pc += 1;
        let indirect = bus.cpu_read(pc)? as u16;
        let low = bus.cpu_read(indirect)? as u16;
        let high = bus.cpu_read((indirect + 1) & 0x00FF)? as u16;
        let address = (high << 8) | low;
        let result = (address as i32 + bus.registers().y as i32) as u16;
        Some((result, is_page_crossed(address, result)))
    }
}

fn is_page_crossed(old: u16, new: u16) -> bool {
    (old & 0xFF00) != (new & 0xFF00)
}
