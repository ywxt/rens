use crate::{memory::Memory, memory::Result, rom::Mapper};

use super::{memory::CpuMemory, stack, CpuRegisters, PpuRegister};
use std::fmt::{Debug, Formatter};

pub struct CpuBus {
    cpu_memory: CpuMemory,
    mapper: Box<dyn Mapper>,
    registers: CpuRegisters,
}

impl CpuBus {
    pub fn new(mapper: Box<dyn Mapper>) -> Self {
        Self {
            cpu_memory: CpuMemory::new(),
            mapper,
            registers: CpuRegisters::new(),
        }
    }

    pub fn cpu_read(&self, address: u16) -> Result<u8> {
        self.cpu_memory
            .read(address)
            .or_else(|_| self.mapper.read(address))
    }
    pub fn cpu_read_word(&self, address: u16) -> Result<u16> {
        self.cpu_memory
            .read_word(address)
            .or_else(|_| self.mapper.read_word(address))
    }
    pub fn cpu_write(&mut self, address: u16, data: u8) -> Result<()> {
        self.cpu_memory
            .write(address, data)
            .or_else(|_| self.mapper.write(address, data))
    }
    pub fn cpu_write_word(&mut self, address: u16, data: u16) -> Result<()> {
        self.cpu_memory
            .write_word(address, data)
            .or_else(|_| self.mapper.write_word(address, data))
    }
    pub fn stack_push(&mut self, data: u8) -> Result<()> {
        stack::push(&mut self.cpu_memory, &mut self.registers, data)
    }
    pub fn stack_push_word(&mut self, data: u16) -> Result<()> {
        stack::push_word(&mut self.cpu_memory, &mut self.registers, data)
    }
    pub fn stack_pop(&mut self) -> Result<u8> {
        stack::pop(&mut self.cpu_memory, &mut self.registers)
    }
    pub fn stack_pop_word(&mut self) -> Result<u16> {
        stack::pop_word(&mut self.cpu_memory, &mut self.registers)
    }
    pub fn registers(&self) -> &CpuRegisters {
        &self.registers
    }
    pub fn registers_mut(&mut self) -> &mut CpuRegisters {
        &mut self.registers
    }

    pub fn ppu_register(&self) -> PpuRegister<'_> {
        PpuRegister { cpu_bus: self }
    }
}
impl Debug for CpuBus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CpuBus")
            .field("cpu_memory", &self.cpu_memory)
            .field("mapper", &format!("Mapper{:03}", self.mapper.number()))
            .field("registers", &self.registers)
            .finish()
    }
}
