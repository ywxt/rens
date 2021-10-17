mod addressing;
mod bus;
mod error;
mod instruction;
mod memory;
mod stack;

pub use bus::*;
pub use error::*;

use crate::clock::Clock;
use crate::memory::Result;
use instruction::*;
use crate::register::*;
use std::{cell::RefCell, rc::Weak};

#[derive(Debug)]
pub struct Cpu {
    bus: Weak<RefCell<CpuBus>>,
    processor: InstructionProcessor,
    cycles: u32,
    defer_cycles: u32,
}

impl Cpu {
    const VECTOR_RESET: u16 = 0xFFFC;
    const VECTOR_NMI: u16 = 0xFFFA;
    const VECTOR_IRQ_OR_BRK: u16 = 0xFFFE;
    pub fn new(bus: Weak<RefCell<CpuBus>>) -> Self {
        Self {
            bus,
            processor: InstructionProcessor,
            cycles: 0,
            defer_cycles: 0,
        }
    }
    pub fn reset(&mut self) -> Result<()> {
        let bus = self.bus.upgrade().unwrap();
        let mut bus = bus.borrow_mut();
        let pc = bus.cpu_read_word(Self::VECTOR_RESET)?;
        let mut registers = bus.registers_mut();
        registers.a = 0;
        registers.x = 0;
        registers.y = 0;
        registers.clear_flags();
        registers.set_u_flag(true);
        registers.set_i_flag(true);
        registers.sp = 0xFD;
        registers.pc = pc;
        self.defer_cycles = 7;
        Ok(())
    }
    pub fn nmi(&mut self) -> Result<()> {
        let bus = self.bus.upgrade().unwrap();
        let mut bus = bus.borrow_mut();
        let nmi_pc = bus.cpu_read_word(Self::VECTOR_NMI)?;
        let pc = bus.registers().pc;
        let p = bus.registers().p;

        bus.stack_push_word(pc)?;
        bus.stack_push((p | P_FLAGS_U) & !P_FLAGS_B)?;

        let registers = bus.registers_mut();
        registers.set_i_flag(true);
        registers.pc = nmi_pc;
        self.defer_cycles += 7;
        Ok(())
    }
    pub fn irq(&mut self) -> Result<()> {
        let bus = self.bus.upgrade().unwrap();
        let mut bus = bus.borrow_mut();
        if bus.registers().p.has_flag(P_FLAGS_I) {
            return Ok(());
        }
        let irq_pc =bus.cpu_read_word(Self::VECTOR_IRQ_OR_BRK) ?;
        let pc = bus.registers().pc;
        let p = bus.registers().p;
        bus.stack_push_word(pc)?;
        bus.stack_push((p | P_FLAGS_U) & !P_FLAGS_B)?;
        let registers = bus.registers_mut();
        registers.set_i_flag(true);
        registers.pc = irq_pc;
        self.defer_cycles += 7;
        Ok(())
    }

    fn step(&mut self) -> std::result::Result<(), CpuError> {
        let bus = self.bus.upgrade().unwrap();
        let mut bus = bus.borrow_mut();
        let pc = bus.registers().pc;
        bus.registers_mut().pc += 1;
        let op = bus.cpu_read(pc)?; 
        self.defer_cycles = self.processor.process(op, &mut bus)?;
        Ok(())
    }
}

impl Clock for Cpu {
    type Error = CpuError;
    fn clock(&mut self) -> std::result::Result<(), CpuError> {
        if self.defer_cycles == 0 {
            self.step()?;
        }
        self.cycles += 1;
        self.defer_cycles -= 1;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::{Cpu, CpuBus, CpuRegisters};
    use crate::clock::Clock;
    use crate::rom::{make_mapper, NesLoader};
    use regex::{Captures, Regex};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn cpu_test() {
        let loader =
            NesLoader::from_slice(&std::fs::read("test_data/nestest.nes").unwrap()).unwrap();
        let bus = Rc::new(RefCell::new(CpuBus::new(
            make_mapper(
                loader.header().mapper_number(),
                loader.prg().to_vec(),
                loader.chr().to_vec(),
            )
            .unwrap(),
        )));
        let mut cpu = Cpu::new(Rc::downgrade(&bus));
        cpu.reset().unwrap();
        bus.borrow_mut().registers_mut().pc = 0xC000;
        let regex = Regex::new(
            r"(?P<ADDR>[A-Z0-9]{4})\s+([A-Z0-9]{2} )+\s*[*#$=@,()A-Z0-9 ]+A:(?P<A>[A-Z0-9]{2}) X:(?P<X>[A-Z0-9]{2}) Y:(?P<Y>[A-Z0-9]{2}) P:(?P<P>[A-Z0-9]{2}) SP:(?P<SP>[A-Z0-9]{2}) PPU:\s*(?P<PPU>\d+,\s*\d+) CYC:(?P<CYC>\d+)",
        ).unwrap();
        let log = std::fs::read_to_string("test_data/nestest.log").unwrap();
        let mut captures = regex.captures_iter(&log);
        loop {
            if cpu.defer_cycles == 0u32 {
                let capture = match captures.next() {
                    None => break,
                    Some(capture) => capture,
                };
                assert!(check(capture, cpu.cycles, bus.borrow().registers()));
            }

            if let Err(error) = cpu.clock() {
                panic!("{}", error);
            }
        }
    }

    fn check(capture: Captures, cycles: u32, registers: &CpuRegisters) -> bool {
        let result = capture["ADDR"] == format!("{:04X}", registers.pc)
            && capture["A"] == format!("{:02X}", registers.a)
            && capture["X"] == format!("{:02X}", registers.x)
            && capture["Y"] == format!("{:02X}", registers.y)
            && capture["P"] == format!("{:02X}", registers.p)
            && capture["SP"] == format!("{:02X}", registers.sp)
            && capture["CYC"] == format!("{}", cycles);
        if !result {
            println!(
                "{:04X} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} CYC:{}",
                registers.pc,
                registers.a,
                registers.x,
                registers.y,
                registers.p,
                registers.sp,
                cycles
            );
            println!(
                "{} A:{} X:{} Y:{} P:{} SP:{} CYC:{}",
                &capture["ADDR"],
                &capture["A"],
                &capture["X"],
                &capture["Y"],
                &capture["P"],
                &capture["SP"],
                &capture["CYC"]
            );
        }
        result
    }
}
