mod bus;
mod memory;

pub use memory::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Mirroring {
    Horizontal,
    Vertical,
    FourScreen,
}

pub const STD_PALETTE: [PaletteData; 64] = [
    PaletteData::from_rgba([0x7F, 0x7F, 0x7F, 0xFF]),
    PaletteData::from_rgba([0x20, 0x00, 0xB0, 0xFF]),
    PaletteData::from_rgba([0x28, 0x00, 0xB8, 0xFF]),
    PaletteData::from_rgba([0x60, 0x10, 0xA0, 0xFF]),
    PaletteData::from_rgba([0x98, 0x20, 0x78, 0xFF]),
    PaletteData::from_rgba([0xB0, 0x10, 0x30, 0xFF]),
    PaletteData::from_rgba([0xA0, 0x30, 0x00, 0xFF]),
    PaletteData::from_rgba([0x78, 0x40, 0x00, 0xFF]),
    PaletteData::from_rgba([0x48, 0x58, 0x00, 0xFF]),
    PaletteData::from_rgba([0x38, 0x68, 0x00, 0xFF]),
    PaletteData::from_rgba([0x38, 0x6C, 0x00, 0xFF]),
    PaletteData::from_rgba([0x30, 0x60, 0x40, 0xFF]),
    PaletteData::from_rgba([0x30, 0x50, 0x80, 0xFF]),
    PaletteData::from_rgba([0x00, 0x00, 0x00, 0xFF]),
    PaletteData::from_rgba([0x00, 0x00, 0x00, 0xFF]),
    PaletteData::from_rgba([0x00, 0x00, 0x00, 0xFF]),
    PaletteData::from_rgba([0xBC, 0xBC, 0xBC, 0xFF]),
    PaletteData::from_rgba([0x40, 0x60, 0xF8, 0xFF]),
    PaletteData::from_rgba([0x40, 0x40, 0xFF, 0xFF]),
    PaletteData::from_rgba([0x90, 0x40, 0xF0, 0xFF]),
    PaletteData::from_rgba([0xD8, 0x40, 0xC0, 0xFF]),
    PaletteData::from_rgba([0xD8, 0x40, 0x60, 0xFF]),
    PaletteData::from_rgba([0xE0, 0x50, 0x00, 0xFF]),
    PaletteData::from_rgba([0xC0, 0x70, 0x00, 0xFF]),
    PaletteData::from_rgba([0x88, 0x88, 0x00, 0xFF]),
    PaletteData::from_rgba([0x50, 0xA0, 0x00, 0xFF]),
    PaletteData::from_rgba([0x48, 0xA8, 0x10, 0xFF]),
    PaletteData::from_rgba([0x48, 0xA0, 0x68, 0xFF]),
    PaletteData::from_rgba([0x40, 0x90, 0xC0, 0xFF]),
    PaletteData::from_rgba([0x00, 0x00, 0x00, 0xFF]),
    PaletteData::from_rgba([0x00, 0x00, 0x00, 0xFF]),
    PaletteData::from_rgba([0x00, 0x00, 0x00, 0xFF]),
    PaletteData::from_rgba([0xFF, 0xFF, 0xFF, 0xFF]),
    PaletteData::from_rgba([0x60, 0xA0, 0xFF, 0xFF]),
    PaletteData::from_rgba([0x50, 0x80, 0xFF, 0xFF]),
    PaletteData::from_rgba([0xA0, 0x70, 0xFF, 0xFF]),
    PaletteData::from_rgba([0xF0, 0x60, 0xFF, 0xFF]),
    PaletteData::from_rgba([0xFF, 0x60, 0xB0, 0xFF]),
    PaletteData::from_rgba([0xFF, 0x78, 0x30, 0xFF]),
    PaletteData::from_rgba([0xFF, 0xA0, 0x00, 0xFF]),
    PaletteData::from_rgba([0xE8, 0xD0, 0x20, 0xFF]),
    PaletteData::from_rgba([0x98, 0xE8, 0x00, 0xFF]),
    PaletteData::from_rgba([0x70, 0xF0, 0x40, 0xFF]),
    PaletteData::from_rgba([0x70, 0xE0, 0x90, 0xFF]),
    PaletteData::from_rgba([0x60, 0xD0, 0xE0, 0xFF]),
    PaletteData::from_rgba([0x60, 0x60, 0x60, 0xFF]),
    PaletteData::from_rgba([0x00, 0x00, 0x00, 0xFF]),
    PaletteData::from_rgba([0x00, 0x00, 0x00, 0xFF]),
    PaletteData::from_rgba([0xFF, 0xFF, 0xFF, 0xFF]),
    PaletteData::from_rgba([0x90, 0xD0, 0xFF, 0xFF]),
    PaletteData::from_rgba([0xA0, 0xB8, 0xFF, 0xFF]),
    PaletteData::from_rgba([0xC0, 0xB0, 0xFF, 0xFF]),
    PaletteData::from_rgba([0xE0, 0xB0, 0xFF, 0xFF]),
    PaletteData::from_rgba([0xFF, 0xB8, 0xE8, 0xFF]),
    PaletteData::from_rgba([0xFF, 0xC8, 0xB8, 0xFF]),
    PaletteData::from_rgba([0xFF, 0xD8, 0xA0, 0xFF]),
    PaletteData::from_rgba([0xFF, 0xF0, 0x90, 0xFF]),
    PaletteData::from_rgba([0xC8, 0xF0, 0x80, 0xFF]),
    PaletteData::from_rgba([0xA0, 0xF0, 0xA0, 0xFF]),
    PaletteData::from_rgba([0xA0, 0xFF, 0xC8, 0xFF]),
    PaletteData::from_rgba([0xA0, 0xFF, 0xF0, 0xFF]),
    PaletteData::from_rgba([0xA0, 0xA0, 0xA0, 0xFF]),
    PaletteData::from_rgba([0x00, 0x00, 0x00, 0xFF]),
    PaletteData::from_rgba([0x00, 0x00, 0x00, 0xFF]),
];

#[derive(Debug, Eq, PartialEq)]
pub struct PaletteData(u32);

impl PaletteData {
    pub const fn new(data: u32) -> Self {
        PaletteData(data)
    }
    pub const fn from_rgba(color: [u8; 4]) -> Self {
        PaletteData(
            (color[0] as u32) << 24
                | (color[1] as u32) << 16
                | (color[2] as u32) << 8
                | (color[3] as u32),
        )
    }

    const fn color(&self, index: usize) -> u8 {
        ((self.0 >> (index * 8)) & 0xFF) as u8
    }
    pub const fn r(&self) -> u8 {
        self.color(3)
    }
    pub const fn g(&self) -> u8 {
        self.color(2)
    }
    pub const fn b(&self) -> u8 {
        self.color(1)
    }
    pub const fn a(&self) -> u8 {
        self.color(0)
    }
    pub const fn data(&self) -> u32 {
        self.0
    }
}

impl From<u32> for PaletteData {
    fn from(data: u32) -> Self {
        PaletteData(data)
    }
}

impl From<PaletteData> for u32 {
    fn from(data: PaletteData) -> Self {
        data.0
    }
}

impl From<[u8; 4]> for PaletteData {
    fn from(data: [u8; 4]) -> Self {
        PaletteData::from_rgba(data)
    }
}
