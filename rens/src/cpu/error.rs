use thiserror::Error;
#[derive(Error, Debug)]
pub enum CpuError {
    #[error("内存错误: {0}")]
    Memory(#[from] crate::memory::MemoryError),
    #[error("无效的指令: {0:#04X}")]
    UnknownInstruction(u8),
}
