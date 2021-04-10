mod cpu_bus;
mod cpu_memory;
mod cpu_register;
mod cpu_stack;
mod error;

pub use cpu_bus::*;
pub use cpu_memory::*;
pub use cpu_register::*;
pub use cpu_stack::*;
pub use error::*;


use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use crate::clock::Clock;

pub struct Cpu {
    bus: Rc<RefCell<CpuBus>>,
    cycles: u32,
    defer_cycles: u32,
}

impl Cpu {
    const VECTOR_RESET: u16 = 0xFFFC;
    const VECTOR_NMI: u16 = 0xFFFA;
    const VECTOR_IRQ_OR_BRK: u16 = 0xFFFE;
    pub fn reset(&mut self) -> bool {
        let mut bus = self.bus.borrow_mut();
        let pc = match bus.cpu_read_word(Self::VECTOR_RESET) {
            Some(pc) => pc,
            None => return false,
        };
        let registers = bus.registers_mut();
        registers.a = 0;
        registers.x = 0;
        registers.y = 0;
        registers.clear_flags();
        registers.set_u_flag(true);
        registers.set_i_flag(true);
        registers.sp = 0xFD;
        registers.pc = pc;
        self.defer_cycles = 7;
        true
    }
    pub fn nmi(&mut self) -> bool {
        let mut bus = self.bus.borrow_mut();
        let nmi_pc = match bus.cpu_read_word(Self::VECTOR_NMI) {
            Some(pc) => pc,
            None => return false,
        };
        let pc = bus.registers().pc;
        let p = bus.registers().p;

        bus.stack_push_word(pc);
        bus.stack_push((p | P_FLAGS_U) & !P_FLAGS_B);

        let registers = bus.registers_mut();
        registers.set_i_flag(true);
        registers.pc = nmi_pc;
        self.defer_cycles += 7;
        true
    }
    pub fn irq(&mut self) -> bool {
        let mut bus = self.bus.borrow_mut();
        if bus.registers().p.has_flag(P_FLAGS_I) {
            return true;
        }
        let irq_pc = match bus.cpu_read_word(Self::VECTOR_IRQ_OR_BRK) {
            Some(pc) => pc,
            None => return false,
        };
        let pc = bus.registers().pc;
        let p = bus.registers().p;
        bus.stack_push_word(pc);
        bus.stack_push((p | P_FLAGS_U) & !P_FLAGS_B);
        let registers = bus.registers_mut();
        registers.set_i_flag(true);
        registers.pc = irq_pc;
        self.defer_cycles += 7;
        true
    }

    fn step(&mut self) -> Result<(),CpuError> {
        let mut bus = self.bus.borrow_mut();
        let pc = bus.registers().pc;
        bus.registers_mut().pc+=1;
        let op = match bus.cpu_read_word(pc) {
            Some(op) => op,
            None => return Err(CpuError::ReadMemoryAddressError(pc)),
        };
        let cycles = processor.process(op,&mut bus);
        Ok(())
    }

    #[cfg(debug_assertions)]
    pub fn bus(&self) -> Ref<'_, CpuBus> {
        self.bus.borrow()
    }
    #[cfg(debug_assertions)]
    pub fn cycles(&self) -> &u32 {
        &self.cycles
    }
}

impl Clock for Cpu {
    type Error = CpuError;
    fn clock(&mut self) -> Result<(),CpuError> {
        if self.defer_cycles == 0 {
            self.step()?;
        }
        self.cycles += 1;
        self.defer_cycles -= 1;
        Ok(())
    }
}
