use crate::memory::Memory;

use super::Mapper;

pub struct Mapper0 {
    prg_ram: Vec<u8>,
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    /// NROM-128 最后16KB镜像
    nrom_128: bool,
}

impl Mapper0 {
    const ADDRESS_CHR_BANK_START: u16 = 0x0000;
    const ADDRESS_CHR_BANK_END: u16 = 0x2000 - 1;
    const ADDRESS_PRG_RAM_BANK_START: u16 = 0x6000;
    const ADDRESS_PRG_RAM_BANK_END: u16 = 0x8000 - 1;
    const ADDRESS_PRG_BANK_FIRST_START: u16 = 0x8000;
    const ADDRESS_PRG_BANK_FIRST_END: u16 = 0xC000 - 1;
    const ADDRESS_PRG_BANK_SECOND_START: u16 = 0xC000;
    const ADDRESS_PRG_BANK_SECOND_END: u16 = 0xFFFF;

    const MAPPER_SIZE_PRG_RAM: u16 = 8 * 1024;
    const MAPPER_SIZE_NROM_128: u16 = 16 * 1024;

    pub fn new(prg_rom: Vec<u8>, chr_rom: Vec<u8>) -> Self {
        let nrom_128 = prg_rom.len() == Self::MAPPER_SIZE_NROM_128 as usize; // 16 KiB for NROM-128, 32 KiB for NROM-256 (DIP-28 standard pin out)
        let prg_ram = vec![0; Self::MAPPER_SIZE_PRG_RAM as usize]; // 固定 8K PRG RAM
        Self {
            prg_ram,
            prg_rom,
            chr_rom,
            nrom_128,
        }
    }
}

impl Mapper for Mapper0 {}

impl Memory for Mapper0 {
    fn read(&self, address: u16) -> Option<u8> {
        Some(match address {
            Self::ADDRESS_CHR_BANK_START..=Self::ADDRESS_CHR_BANK_END => {
                self.chr_rom[address as usize]
            }
            Self::ADDRESS_PRG_RAM_BANK_START..=Self::ADDRESS_PRG_RAM_BANK_END => {
                self.prg_ram[(address - Self::ADDRESS_PRG_RAM_BANK_START) as usize]
            }
            Self::ADDRESS_PRG_BANK_FIRST_START..=Self::ADDRESS_PRG_BANK_FIRST_END => {
                self.prg_rom[(address - Self::ADDRESS_PRG_BANK_FIRST_START) as usize]
            }
            Self::ADDRESS_PRG_BANK_SECOND_START..=Self::ADDRESS_PRG_BANK_SECOND_END => {
                if self.nrom_128 {
                    self.prg_rom[(address - Self::ADDRESS_PRG_BANK_SECOND_START) as usize]
                } else {
                    self.prg_rom[(address - Self::ADDRESS_PRG_BANK_FIRST_START) as usize]
                }
            }
            _ => return None,
        })
    }

    fn write(&mut self, address: u16, data: u8) -> bool{
        match address {
            Self::ADDRESS_CHR_BANK_START..=Self::ADDRESS_CHR_BANK_END => {
                self.chr_rom[address as usize] = data;
                true
            }
            Self::ADDRESS_PRG_RAM_BANK_START..=Self::ADDRESS_PRG_RAM_BANK_END => {
                self.prg_ram[(address - Self::ADDRESS_PRG_RAM_BANK_START) as usize] = data;
                true
            }
            Self::ADDRESS_PRG_BANK_FIRST_START..=Self::ADDRESS_PRG_BANK_FIRST_END => {
                self.prg_rom[(address - Self::ADDRESS_PRG_BANK_FIRST_START) as usize] = data;
                true
            }
            Self::ADDRESS_PRG_BANK_SECOND_START..=Self::ADDRESS_PRG_BANK_SECOND_END => {
                if self.nrom_128 {
                    self.prg_rom[(address - Self::ADDRESS_PRG_BANK_SECOND_START) as usize] = data;
                } else {
                    self.prg_rom[(address - Self::ADDRESS_PRG_BANK_FIRST_START) as usize] = data;
                }
                true
            }
            _ => false,
        }
    }
}
