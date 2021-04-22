use nes::clock::Clock;
use nes::cpu::{Cpu, CpuBus, CpuRegisters};
use nes::rom::{make_mapper, NesLoader};
use regex::{Captures, Regex};
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn cpu_test() {
    let loader = NesLoader::from_slice(&std::fs::read("test_data/nestest.nes").unwrap()).unwrap();
    let bus = Rc::new(RefCell::new(CpuBus::new(
        make_mapper(
            loader.header().mapper_number(),
            loader.prg().to_vec(),
            loader.chr().to_vec(),
        )
        .unwrap(),
    )));
    let mut cpu = Cpu::new(bus);
    cpu.reset();
    cpu.bus_mut().registers_mut().pc = 0xC000;
    let regex = Regex::new(
        r"(?P<ADDR>[A-Z0-9]{4})\s+([A-Z0-9]{2} )+\s+[*#$=@,()A-Z0-9 ]+A:(?P<A>[A-Z0-9]{2}) X:(?P<X>[A-Z0-9]{2}) Y:(?P<Y>[A-Z0-9]{2}) P:(?P<P>[A-Z0-9]{2}) SP:(?P<SP>[A-Z0-9]{2}) PPU:  (?P<PPU>\d+, \d+) CYC:(?P<CYC>\d+)",
    ).unwrap();
    let log = std::fs::read_to_string("test_data/nestest.log").unwrap();
    let mut captures = regex.captures_iter(&log);
    loop {
        if *cpu.defer_cycles() == 0u32 {
            let capture = match captures.next() {
                None => break,
                Some(capture) => capture,
            };
            assert!(check(capture, *cpu.cycles(), cpu.bus().registers()));
        }

        cpu.clock().unwrap();
    }
}

fn check(capture: Captures, cycles: u32, registers: &CpuRegisters) -> bool {
    println!(
        "{:04X} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} CYC:{}",
        registers.pc, registers.a, registers.x, registers.y, registers.p, registers.sp, cycles
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
    capture["ADDR"] == format!("{:04X}", registers.pc)
        && capture["A"] == format!("{:02X}", registers.a)
        && capture["X"] == format!("{:02X}", registers.x)
        && capture["Y"] == format!("{:02X}", registers.y)
        && capture["P"] == format!("{:02X}", registers.p)
        && capture["SP"] == format!("{:02X}", registers.sp)
        && capture["CYC"] == format!("{}", cycles)
}
