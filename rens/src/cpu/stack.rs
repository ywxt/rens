use crate::memory::{Memory, Result};

use super::{memory::CpuMemory, CpuRegisters};

const ADDRESS_CPU_STACK_START: u16 = 0x0100;
pub fn push(memory: &mut CpuMemory, registers: &mut CpuRegisters, data: u8) -> Result<()> {
    let sp = registers.sp as u16;
    memory.write(ADDRESS_CPU_STACK_START + sp, data)?;
    registers.sp -= 1;
    Ok(())
}
pub fn push_word(memory: &mut CpuMemory, registers: &mut CpuRegisters, data: u16) -> Result<()> {
    push(memory, registers, (data >> 8) as u8)
        .and_then(|_| push(memory, registers, (data & 0x00FF) as u8))
}
pub fn pop(memory: &mut CpuMemory, registers: &mut CpuRegisters) -> Result<u8> {
    registers.sp += 1;
    let sp = registers.sp as u16;
    memory.read(ADDRESS_CPU_STACK_START + sp)
}
pub fn pop_word(memory: &mut CpuMemory, registers: &mut CpuRegisters) -> Result<u16> {
    let low = pop(memory, registers)? as u16;
    let high = pop(memory, registers)? as u16;
    Ok((high << 8) | low)
}
