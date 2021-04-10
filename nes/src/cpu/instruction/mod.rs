use super::{AddressingMode, CpuBus, CpuError};

pub struct InstructionProcessor;
impl InstructionProcessor {
    pub fn process(&self, ins: u8, bus: &mut CpuBus) -> Result<u32, CpuError> {
        todo!()
    }
}
enum Instruction {
    
}
impl Instruction {

    /// 返回寻址模式和时钟周期
    fn op_codes(&self) -> (AddressingMode,u32) {
        todo!()
    }
    /// 返回额外的时钟周期
    fn invoke(&self) -> u32 {
        todo!()
    }
}
