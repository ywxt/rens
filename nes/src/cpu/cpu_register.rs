use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};

/// 进位标志
const P_FLAGS_C: u8 = 1;
/// 零标志
const P_FLAGS_Z: u8 = 1 << 1;
/// 中断使能
const P_FLAGS_I: u8 = 1 << 2;
/// 十进制，未使用
const P_FLAGS_D: u8 = 1 << 3;

const P_FLAGS_B: u8 = 1 << 4;

const P_FLAGS_U: u8 = 1 << 5;
/// 溢出标志
const P_FLAGS_V: u8 = 1 << 6;
/// 负标志
const P_FLAGS_N: u8 = 1 << 7;

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

#[derive(Default, Debug)]
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
        CpuRegisters {
            a: 0,
            x: 0,
            y: 0,
            sp: 0,
            pc: 0,
            p: 0,
        }
    }
    pub fn set_z_n_flags(&mut self, flag: u8) {
        self.set_z_flag(flag);
        self.set_n_flag(flag);
    }

    pub fn set_z_flag(&mut self, flag: u8) {
        self.p.set_flag(P_FLAGS_Z, flag == 0);
    }
    pub fn set_n_flag(&mut self, flag: u8) {
        self.p.set_flag(P_FLAGS_N, flag >> 7 == 1);
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
}
