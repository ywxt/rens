use std::convert::TryFrom;

use crate::{error::RomError, ppu::Mirroring};

type Result<T> = std::result::Result<T, RomError>;
/// NES ROM HEAD
/// size: 16 bytes
#[derive(Debug, Clone)]
pub struct Header {
    /// 常量 $4E $45 $53 $1A ("NES" followed by MS-DOS end-of-file)
    nes: [u8; 4],
    /// PRG ROM 大小, 每个单元16k
    prg_size: u8,
    /// CHR ROM 大小，每个单元 8k
    chr_size: u8,
    /// - 0: Mirroring
    ///
    ///   0: 水平镜像
    ///   1: 垂直镜像
    ///
    /// - 1: 卡带上没有带电池的SRAM
    ///
    /// - 2: Trainer标志
    ///
    /// - 3: 忽略Mirroring，启用4-Screen模式
    ///
    /// - 4-7: Mapper 号的低4位
    flags6: u8,
    /// - 0: VS Unisystem
    /// - 1: PlayChoice-10
    /// - 2-3: 如果等于2，则为NES 2.0格式，Flags8-10按照2.0格式读取
    /// - 4-7: Mapper 号的高4位
    flags7: u8,
    /// RPG RAM 大小
    ///
    /// 以下 8-10 NES 2.0 格式均不同，但此程序不支持2.0，故不再列出
    flags8: u8,
    /// TV 系统
    ///  - 0: NTSC
    ///  - 1: PAL
    flags9: u8,
    /// 默认为0，可能有些程序会用到
    flags10: u8,
    /// 默认为0，可能有些程序会用到
    unused: [u8; 5],
}

impl Header {
    pub fn from_slice(value: &[u8]) -> Result<Self> {
        if !Self::is_nes_rom(value) {
            return Err(RomError::InvalidInes(String::from("非NES ROM程序")));
        }
        Ok(Self {
            nes: Self::NES_ASCII,
            prg_size: value[4],
            chr_size: value[5],
            flags6: value[6],
            flags7: value[7],
            flags8: value[8],
            flags9: value[9],
            flags10: value[10],
            unused: {
                let mut unused = [0; 5];
                unused.clone_from_slice(&value[11..]);
                unused
            },
        })
    }
}

impl TryFrom<&[u8]> for Header {
    type Error = RomError;

    fn try_from(value: &[u8]) -> Result<Self> {
        Self::from_slice(value)
    }
}

impl Header {
    pub const NES_ASCII: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];

    pub fn is_nes_rom(header: &[u8]) -> bool {
        header.len() == 16 && header[0..4] == Self::NES_ASCII[..]
    }
    pub fn prg_size(&self) -> u8 {
        self.prg_size
    }
    pub fn chr_size(&self) -> u8 {
        self.chr_size
    }
    pub fn mirroring(&self) -> Mirroring {
        if (self.flags6 >> 3) & 1 == 1 {
            return Mirroring::FourScreen;
        }
        if self.flags6 & 1u8 == 0 {
            Mirroring::Horizontal
        } else {
            Mirroring::Vertical
        }
    }
    pub fn battery_backed(&self) -> bool {
        (self.flags6 >> 1) & 1 == 1
    }
    pub fn trainer(&self) -> bool {
        (self.flags6 >> 2) & 1u8 == 1
    }

    pub fn mapper_number(&self) -> u8 {
        let low = self.flags6 >> 4;
        let high = self.flags7 >> 4;
        (high << 4) | low
    }
    pub fn vs_unisystem(&self) -> bool {
        self.flags7 & 1u8 == 1
    }
    pub fn play_choice_10(&self) -> bool {
        (self.flags7 >> 1) & 1u8 == 1
    }
    pub fn nes_2_format(&self) -> bool {
        ((self.flags7 >> 2) & 0b11) == 0b10
    }
    pub fn prg_ram_size(&self) -> Result<u8> {
        if self.nes_2_format() {
            Err(RomError::InvalidInes(String::from("此操作不支持NES 2.0")))
        } else {
            Ok(self.flags8)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Header;
    use super::Result;

    #[test]
    fn is_nes_rom_test() {
        assert!(Header::is_nes_rom(&[
            0x4Eu8, 0x45u8, 0x53u8, 0x1Au8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8
        ]));
        assert!(!Header::is_nes_rom(&[0u8; 16]));
    }

    #[test]
    fn nes_2_format_test() -> Result<()> {
        let header1 = [
            0x4Eu8,
            0x45u8,
            0x53u8,
            0x1Au8,
            0u8,
            0u8,
            0u8,
            0b0000_1000u8,
            0u8,
            0u8,
            0u8,
            0u8,
            0u8,
            0u8,
            0u8,
            0u8,
        ];
        let header2 = [
            0x4Eu8, 0x45u8, 0x53u8, 0x1Au8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8,
        ];
        let h1 = Header::from_slice(&header1)?;
        let h2 = Header::from_slice(&header2)?;
        assert!(h1.nes_2_format());
        assert!(!h2.nes_2_format());
        Ok(())
    }
}
