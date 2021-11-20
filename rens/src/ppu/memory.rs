use crate::memory::{Memory, MemoryError, Result};

pub struct PpuMemory {
    memory: Box<[u8; Self::SIZE_PPU_MEMORY]>,
}

impl PpuMemory {
    const SIZE_PPU_MEMORY: usize = 16 * 1024;
    const ADDRESS_PPU_NAME_TABLE_MIRROR_START: u16 = 0x3000;
    const ADDRESS_PPU_NAME_TABLE_MIRROR_END: u16 = 0x3EFF;
    const ADDRESS_PPU_PALETTE_MIRROR_START: u16 = 0x3F20;
    const ADDRESS_PPU_PALETTE_MIRROR_END: u16 = 0x3FFF;
    pub fn new() -> Self {
        PpuMemory {
            memory: Box::new([0; Self::SIZE_PPU_MEMORY]),
        }
    }
}

impl Default for PpuMemory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory for PpuMemory {
    fn read(&self, address: u16) -> Result<u8> {
        let address = address & 0x3FFF;
        let offset = match address {
            Self::ADDRESS_PPU_NAME_TABLE_MIRROR_START..=Self::ADDRESS_PPU_NAME_TABLE_MIRROR_END => {
                0x1000
            }
            Self::ADDRESS_PPU_PALETTE_MIRROR_START..=Self::ADDRESS_PPU_PALETTE_MIRROR_END => 0x20,
            _ => return Err(MemoryError::AddressOutOfRange(address)),
        };
        let address = match address - offset {
            0x3F10 | 0x3F14 | 0x3F18 | 0x3F1C => address - offset - 0x10,
            address => address,
        };
        self.memory
            .get(address as usize)
            .copied()
            .ok_or(MemoryError::ReadMemory(address))
    }

    fn write(&mut self, address: u16, data: u8) -> Result<()> {
        let address = address & 0x3FFF;
        let offset = match address {
            Self::ADDRESS_PPU_NAME_TABLE_MIRROR_START..=Self::ADDRESS_PPU_NAME_TABLE_MIRROR_END => {
                0x1000
            }
            Self::ADDRESS_PPU_PALETTE_MIRROR_START..=Self::ADDRESS_PPU_PALETTE_MIRROR_END => 0x20,
            _ => return Err(MemoryError::AddressOutOfRange(address)),
        };
        let address = match address - offset {
            0x3F10 | 0x3F14 | 0x3F18 | 0x3F1C => address - offset - 0x10,
            address => address,
        };
        self.memory
            .get_mut(address as usize)
            .map(|value| *value = data)
            .ok_or(MemoryError::WriteMemory(address))
    }
}
