use crate::{memory::Memory, rom::Mapper};

use super::{CpuMemory, CpuRegisters, CpuStack};

pub struct CpuBus {
    cpu_memory: CpuMemory,
    mapper: Box<dyn Mapper>,
    register: CpuRegisters,
}
impl CpuBus {
    pub fn cpu_read(&self, address: u16) -> Option<u8> {
        self.cpu_memory
            .read(address)
            .or_else(|| self.mapper.read(address))
    }
    pub fn cpu_read_word(&self, address: u16) -> Option<u16> {
        self.cpu_memory
            .read_word(address)
            .or_else(|| self.mapper.read_word(address))
    }
    pub fn cpu_write(&mut self, address: u16, data: u8) -> bool {
        self.cpu_memory.write(address, data) || self.mapper.write(address, data)
    }
    pub fn cpu_write_word(&mut self, address: u16, data: u16) -> bool {
        self.cpu_memory.write_word(address, data) || self.mapper.write_word(address, data)
    }
    pub fn stack_push(&mut self, data: u8) -> bool {
        CpuStack::push(&mut self.cpu_memory, &mut self.register, data)
    }
    pub fn stack_push_word(&mut self, data: u16) -> bool {
        CpuStack::push_word(&mut self.cpu_memory, &mut self.register, data)
    }
    pub fn stack_pop(&mut self) -> Option<u8> {
        CpuStack::pop(&mut self.cpu_memory, &mut self.register)
    }
    pub fn stack_word(&mut self) -> Option<u16> {
        CpuStack::pop_word(&mut self.cpu_memory, &mut self.register)
    }
    pub fn register(&self) -> &CpuRegisters {
        &self.register
    }
    pub fn register_mut(&mut self) -> &CpuRegisters {
        &mut self.register
    }
}
