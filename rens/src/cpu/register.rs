use crate::cpu::CpuBus;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};

/// 进位标志
pub(super) const P_FLAGS_C: u8 = 1;
/// 零标志
pub(super) const P_FLAGS_Z: u8 = 1 << 1;
/// 中断使能
pub(super) const P_FLAGS_I: u8 = 1 << 2;
/// 十进制，未使用
pub(super) const P_FLAGS_D: u8 = 1 << 3;
/// BRK
pub(super) const P_FLAGS_B: u8 = 1 << 4;

pub(super) const P_FLAGS_U: u8 = 1 << 5;
/// 溢出标志
pub(super) const P_FLAGS_V: u8 = 1 << 6;
/// 负标志
pub(super) const P_FLAGS_N: u8 = 1 << 7;

pub trait Flags:
    Sized
    + Copy
    + BitOr<Output = Self>
    + BitAnd<Output = Self>
    + Not<Output = Self>
    + BitOrAssign
    + BitAndAssign
    + Eq
{
    fn has_flag(&self, flag: Self) -> bool {
        (*self & flag) == flag
    }
    fn add_flag(&mut self, flag: Self) {
        self.set_flag(flag, true);
    }
    fn set_flag(&mut self, flag: Self, when: bool) {
        if when {
            *self |= flag;
        } else {
            *self &= !flag;
        }
    }
}
impl Flags for u8 {}

#[derive(Default, Clone, Debug)]
pub struct CpuRegisters {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub pc: u16,
    pub p: u8,
}

impl CpuRegisters {
    pub fn new() -> Self {
        CpuRegisters::default()
    }
    pub fn set_z_n_flags(&mut self, flag: u8) {
        self.set_z_flag(flag == 0);
        self.set_n_flag(flag >> 7 == 1);
    }

    pub fn set_z_flag(&mut self, when: bool) {
        self.p.set_flag(P_FLAGS_Z, when);
    }
    pub fn set_n_flag(&mut self, when: bool) {
        self.p.set_flag(P_FLAGS_N, when);
    }
    pub fn set_c_flag(&mut self, when: bool) {
        self.p.set_flag(P_FLAGS_C, when);
    }
    pub fn set_v_flag(&mut self, when: bool) {
        self.p.set_flag(P_FLAGS_V, when);
    }
    pub fn set_i_flag(&mut self, when: bool) {
        self.p.set_flag(P_FLAGS_I, when);
    }
    pub fn set_d_flag(&mut self, when: bool) {
        self.p.set_flag(P_FLAGS_D, when);
    }
    pub fn set_b_flag(&mut self, when: bool) {
        self.p.set_flag(P_FLAGS_B, when);
    }
    pub fn set_u_flag(&mut self, when: bool) {
        self.p.set_flag(P_FLAGS_U, when);
    }
    pub fn clear_flags(&mut self) {
        self.p = 0;
    }
    pub fn has_z_flag(&self) -> bool {
        self.p.has_flag(P_FLAGS_Z)
    }
    pub fn has_c_flag(&self) -> bool {
        self.p.has_flag(P_FLAGS_C)
    }
    pub fn has_n_flag(&self) -> bool {
        self.p.has_flag(P_FLAGS_N)
    }
    pub fn has_i_flag(&self) -> bool {
        self.p.has_flag(P_FLAGS_I)
    }
    pub fn has_b_flag(&self) -> bool {
        self.p.has_flag(P_FLAGS_B)
    }
    pub fn has_v_flag(&self) -> bool {
        self.p.has_flag(P_FLAGS_V)
    }
}

pub struct PpuRegister<'a> {
    pub(super) cpu_bus: &'a CpuBus,
}

impl PpuRegister<'_> {
    const PPU_CTRL: u16 = 0x2000;
    const PPU_MASK: u16 = 0x2001;
    const PPU_STATUS: u16 = 0x2002;

    pub fn ppu_ctrl(&self) -> u8 {
        self.cpu_bus.cpu_read(Self::PPU_CTRL).unwrap()
    }
    pub fn ppu_mask(&self) -> u8 {
        self.cpu_bus.cpu_read(Self::PPU_MASK).unwrap()
    }
    pub fn ppu_status(&self) -> u8 {
        self.cpu_bus.cpu_read(Self::PPU_STATUS).unwrap()
    }
}
