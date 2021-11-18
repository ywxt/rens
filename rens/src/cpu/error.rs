use thiserror::Error;

#[derive(Error, Debug)]
pub enum CpuError {
    #[error("Memory error: {0}")]
    Memory(#[from] crate::memory::MemoryError),
    #[error("Unavailable instruction: {0:#04X}")]
    UnknownInstruction(u8),
}
