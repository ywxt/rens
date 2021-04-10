mod mapper0;

use crate::memory::Memory;

use self::mapper0::Mapper000;

pub fn make_mapper(number: u8, prg_rom: Vec<u8>, chr_rom: Vec<u8>) -> Option<Box<dyn Mapper>> {
    match number {
        0 => Some(Box::new(Mapper000::new(prg_rom, chr_rom))),
        _ => None,
    }
}
pub trait Mapper: Memory {}
