use std::convert::TryFrom;

use super::Header;
use super::NesError;

pub type Result<T> = std::result::Result<T, NesError>;
pub struct NesLoader {
    header: Header,
    /// Trainer，header中trainer标志为1时，大小为512字节，否则为0
    trainer: Vec<u8>,
    /// 存放PRG的地方，大小为nx16k，n为header中的prg_size
    prg: Vec<u8>,
    /// 存放CHR的地方，大小为nx8k，n为header中的chr_size
    chr: Vec<u8>,
}
impl NesLoader {
    pub const TRAINER_SIZE: usize = 512;
    pub const PRG_UNIT_SIZE: usize = 16 * 1024;
    pub const CHR_UNIT_SIZE: usize = 8 * 1024;
    pub const HEADER_SIZE: usize = 16;
    pub fn header(&self) -> &Header {
        &self.header
    }
    pub fn trainer(&self) -> &[u8] {
        &self.trainer
    }
    pub fn prg(&self) -> &[u8] {
        &self.prg
    }
    pub fn chr(&self) -> &[u8] {
        &self.chr
    }
    pub fn from_slice(rom: &[u8]) -> Result<Self> {
        if rom.len() < Self::HEADER_SIZE {
            return Err(NesError::InvalidInes(String::from(
                "INES 文件长度必须不小于16字节",
            )));
        }
        let header_bytes = &rom[0..Self::HEADER_SIZE];
        let mut position: usize = 0;
        let header = Header::from_slice(&header_bytes)?;
        position += Self::HEADER_SIZE;
        let trainer = if header.trainer() {
            if rom.len() < position + Self::TRAINER_SIZE {
                return Err(NesError::InvalidInes(String::from("缺少Train段")));
            }
            let vec = Vec::from(&rom[position..position + Self::TRAINER_SIZE]);
            position += Self::TRAINER_SIZE;
            vec
        } else {
            Vec::default()
        };
        if rom.len() < position + Self::PRG_UNIT_SIZE * (header.prg_size() as usize) {
            return Err(NesError::InvalidInes(String::from("缺少PRG段")));
        }
        let prg = Vec::from(
            &rom[position..position + Self::PRG_UNIT_SIZE * (header.prg_size() as usize)],
        );
        position += Self::PRG_UNIT_SIZE * (header.prg_size() as usize);
        if rom.len() < position + Self::CHR_UNIT_SIZE * (header.chr_size() as usize) {
            return Err(NesError::InvalidInes(String::from("缺少CHR段")));
        }
        let chr = Vec::from(
            &rom[position..position + Self::CHR_UNIT_SIZE * (header.chr_size() as usize)],
        );
        // position += Self::CHR_UNIT_SIZE * (header.chr_size() as usize);
        Ok(Self {
            header,
            trainer,
            prg,
            chr,
        })
    }
}
impl TryFrom<&[u8]> for NesLoader {
    type Error = NesError;

    fn try_from(value: &[u8]) -> Result<Self> {
        Self::from_slice(value)
    }
}

#[cfg(test)]
mod tests {
    use super::NesLoader;
    use std::{convert::TryFrom, fs};

    #[test]
    fn test_nes1() {
        let bytes1 = fs::read("./test_data/1.nes").unwrap();
        let ines1 = NesLoader::try_from(&bytes1[..]).unwrap();
        assert_eq!(ines1.header().prg_size(), 0x08);
        assert_eq!(ines1.header().chr_size(), 0x10);
        assert!(!ines1.header().nes_2_format());
        assert!(!ines1.header().trainer());
        assert_eq!(ines1.header().mapper_number(), 0x17);
        assert!(ines1.trainer().is_empty());
        assert_eq!(
            ines1.prg()[..],
            bytes1[16..16 + 0x08 * NesLoader::PRG_UNIT_SIZE]
        );
        assert_eq!(
            ines1.chr()[..],
            bytes1[16 + 0x08 * NesLoader::PRG_UNIT_SIZE
                ..16 + 0x08 * NesLoader::PRG_UNIT_SIZE + 0x10 * NesLoader::CHR_UNIT_SIZE]
        );
    }

    #[test]
    fn test_nes2() {
        let bytes2 = fs::read("./test_data/2.nes").unwrap();
        let ines1 = NesLoader::try_from(&bytes2[..]).unwrap();
        // let ines2 = Ines::try_from(&bytes2[..])?;
        assert_eq!(ines1.header().prg_size(), 0x20);
        assert_eq!(ines1.header().chr_size(), 0x40);
        assert!(!ines1.header().nes_2_format());
        assert!(!ines1.header().trainer());
        assert_eq!(ines1.header().mapper_number(), 0x2D);
        assert!(ines1.trainer().is_empty());
        assert_eq!(
            ines1.prg()[..],
            bytes2[16..16 + 0x20 * NesLoader::PRG_UNIT_SIZE]
        );
        assert_eq!(
            ines1.chr()[..],
            bytes2[16 + 0x20 * NesLoader::PRG_UNIT_SIZE
                ..16 + 0x20 * NesLoader::PRG_UNIT_SIZE + 0x40 * NesLoader::CHR_UNIT_SIZE]
        );
    }
}
