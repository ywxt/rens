use crate::memory::Memory;

use super::{cpu_memory::CpuMemory, CpuRegisters};

pub struct CpuStack;

impl CpuStack {
    const ADDRESS_CPU_STACK_START: u16 = 0x0100;
    pub fn push(memory: &mut CpuMemory, registers: &mut CpuRegisters, data: u8) -> bool {
        let sp = registers.sp as u16;
        if memory.write(Self::ADDRESS_CPU_STACK_START + sp, data) {
            registers.sp -= 1;
            true
        } else {
            false
        }
    }
    pub fn push_word(memory: &mut CpuMemory, registers: &mut CpuRegisters, data: u16) -> bool {
        Self::push(memory, registers, (data >> 8) as u8)
            && Self::push(memory, registers, (data & 0x00FF) as u8)
    }
    pub fn pop(memory: &mut CpuMemory, registers: &mut CpuRegisters) -> Option<u8> {
        registers.sp += 1;
        let sp = registers.sp as u16;
        memory.read(Self::ADDRESS_CPU_STACK_START + sp)
    }
    pub fn pop_word(memory: &mut CpuMemory, registers: &mut CpuRegisters) -> Option<u16> {
        let low = Self::pop(memory, registers)? as u16;
        let high = Self::pop(memory, registers)? as u16;
        Some((high << 8) | low)
    }
}
