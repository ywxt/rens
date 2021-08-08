use crate::memory::{Memory,Result,MemoryError};
use std::fmt::Debug;

#[derive(Debug)]
pub struct CpuMemory {
    ram: Box<[u8; Self::SIZE_CPU_MEMORY as usize]>,
}
impl CpuMemory {
    const SIZE_CPU_MEMORY: u16 = 0x2000;

    const NUMBER_CPU_MEMORY_MIRROR: u16 = 0x7FF;

    const ADDRESS_CPU_MEMORY_START: u16 = 0x0;
    const ADDRESS_CPU_MEMORY_END: u16 = 0x2000 - 1;
    const ADDRESS_IO_REGISTER_START: u16 = 0x2000;
    const ADDRESS_IO_REGISTER_END: u16 = 0x4020 - 1;
    pub fn new() -> Self {
        Self {
            ram: Box::new([0; Self::SIZE_CPU_MEMORY as usize]),
        }
    }
}

impl Default for CpuMemory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory for CpuMemory {
    fn read(&self, address: u16) -> Result<u8> {
        match address {
            // CPU RAM
            Self::ADDRESS_CPU_MEMORY_START..=Self::ADDRESS_CPU_MEMORY_END => self
                .ram
                .get((address & Self::NUMBER_CPU_MEMORY_MIRROR) as usize)
                .copied()
                .ok_or(MemoryError::ReadMemory(address)),
            // // IO 寄存器，暂不实现
            Self::ADDRESS_IO_REGISTER_START..=Self::ADDRESS_IO_REGISTER_END => Ok(0),
            _ => Err(MemoryError::ReadMemory(address)),
        }
    }

    fn write(&mut self, address: u16, data: u8) -> Result<()> {
        match address {
            Self::ADDRESS_CPU_MEMORY_START..=Self::ADDRESS_CPU_MEMORY_END => {
                self.ram[(address & Self::NUMBER_CPU_MEMORY_MIRROR) as usize] = data;
                Ok(())
            }
            Self::ADDRESS_IO_REGISTER_START..=Self::ADDRESS_IO_REGISTER_END => Ok(()),
            _ => Err(MemoryError::WriteMemory(address)),
        }
    }
}
