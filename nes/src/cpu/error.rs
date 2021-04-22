use thiserror::Error;
#[derive(Error, Debug)]
pub enum CpuError {
    #[error("无法读取地址: {0}")]
    ReadMemoryAddressError(u16),

    #[error("无法写入地址: {0}")]
    WriteMemoryAddressError(u16),

    #[error("无效的指令: {0}")]
    InvalidInstructionError(u8),
    
    #[error("执行`{0}`指令发生错误")]
    InvokeInstructionError(u8),
}
